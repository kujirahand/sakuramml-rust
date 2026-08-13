//! lexer: 計算式・制御構文の読み取り
use super::*;

pub(super) const LEX_PAREN: isize = 1;

pub(super) const LEX_VALUE: isize = 10;

pub(super) const LEX_MUL_DIV: isize = 20;

pub(super) const LEX_PLUS_MINUS: isize = 30;

pub(super) const LEX_COMPARE: isize = 40;

pub(super) const LEX_OR_AND: isize = 50;

pub(super) fn read_value(cur: &mut SourceCursor, song: &mut Song) -> Option<Token> {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        '(' => {
            // ( calc ) | ( array ) | ( value )
            cur.next(); // skip '('
            let token_opt = read_calc(cur, song);
            // 空っぽなら、空のArrayとして扱う
            if token_opt.is_none() {
                if cur.eq_char(')') {
                    cur.next(); // skip ')'
                    return Some(Token::new_data_tokens(TokenType::MakeArray, 0, vec![], vec![]));
                }
                let msg = song.get_message(MessageKind::MissingParenthesis);
                read_error(cur, song, msg);
                return Some(Token::new_const0());
            }
            // value or array
            let token = token_opt.unwrap_or(Token::new_const0());
            // check array
            cur.skip_space();
            let ch = cur.peek_n(0);
            if ch == ',' {
                cur.next(); // is array
                let mut array_tokens = vec![token];
                while cur.has_next() {
                    let token = match read_calc(cur, song) {
                        Some(t) => t,
                        None => break,
                    };
                    array_tokens.push(token);
                    cur.skip_space();
                    if cur.eq_char(',') {
                        cur.next();
                        continue;
                    }
                }
                if cur.eq_char(')') {
                    cur.next();
                } else {
                    let msg = song.get_message(MessageKind::MissingParenthesis);
                    read_error(cur, song, msg);
                }
                return Some(Token::new_data_tokens(TokenType::MakeArray, 0, vec![], array_tokens));
            }
            if cur.eq_char(')') {
                cur.next();
            } else {
                let msg = song.get_message(MessageKind::MissingParenthesis);
                read_error(cur, song, msg);
            }
            // ( calc )
            let token = Token::new_calc_token('(', LEX_PAREN, vec![token]);
            return Some(token);
        },
        '-' => {
            // is negative number ?
            cur.next();
            if cur.is_numeric() {
                let num = cur.get_int(0);
                return Some(Token::new_const(TokenType::ConstInt, -1 * num, None, TokenValueType::INT));
            }
            // '-' * value
            let token_opt = read_value(cur, song);
            let token = match token_opt {
                Some(token) => token,
                None => {
                    // error
                    Token::new_const(TokenType::ConstInt, 0, None, TokenValueType::INT)
                }
            };
            // -1 * value
            let token_tree = Token::new_calc_token('*', LEX_VALUE, vec![
                    Token::new_const(TokenType::ConstInt, -1, None, TokenValueType::INT),
                    token,
            ]);
            return Some(token_tree);
        },
        '0'..='9' => {
            let num = cur.get_int(0);
            return Some(Token::new_const(TokenType::ConstInt, num, None, TokenValueType::INT));
        },
        '$' => { // v2 compatible hex number
            let num = cur.get_int(0);
            return Some(Token::new_const(TokenType::ConstInt, num, None, TokenValueType::INT));
        },
        '!' => {
            // note length notation (e.g. !8, !4.)
            cur.next(); // skip '!'
            let len_str = cur.get_note_length();
            let num = calc_length(&len_str, song.timebase, song.timebase);
            return Some(Token::new_const(TokenType::ConstInt, num, None, TokenValueType::INT));
        },
        '{' => {
            let str = cur.get_token_nest('{', '}');
            return Some(Token::new_const(TokenType::ConstStr, str.len() as isize, Some(str), TokenValueType::STR));
        },
        '"' => {
            cur.next();
            let str = cur.get_token_ch('"');
            return Some(Token::new_const(TokenType::ConstStr, str.len() as isize, Some(str), TokenValueType::STR));
        },
        'A'..='Z' | '_' | '#' | 'a'..='z' => {
            return Some(read_value_word(cur, song));
        },
        _ => {}
    }
    None
}

pub(super) fn read_value_word(cur: &mut SourceCursor, song: &mut Song) -> Token {
    let mut tok = Token::new(TokenType::Value, LEX_VALUE, vec![]);
    let varname = cur.get_word();
    // println!("read_value_word:{}", varname);
    tok.tag = 0;
    if cur.eq_char('(') {
        // function call or array or macro_expand
        let arg_lineno = cur.line;
        let arg_str = cur.get_token_nest('(', ')');
        // println!("read_calc_args={:?}", arg_str);
        // MML(l) などの引数は値ではなく、参照するMML命令名として渡す。
        // 既知の命令名以外は従来どおり式として解析し、変数指定との互換性を保つ。
        let arg_name = arg_str.trim();
        let arg_tokens = if varname == "MML" && is_mml_state_name(arg_name) {
            vec![Token::new_const(
                TokenType::ConstStr,
                arg_name.len() as isize,
                Some(arg_name.to_string()),
                TokenValueType::STR,
            )]
        } else if is_noteno_func_name(&varname) && is_raw_mml_note(arg_name) {
            // NoteNo(o5e) のように、引数をMMLの音符としてそのまま渡す
            vec![Token::new_const(
                TokenType::ConstStr,
                arg_name.len() as isize,
                Some(arg_name.to_string()),
                TokenValueType::STR,
            )]
        } else {
            lex_calc(song, &arg_str, arg_lineno)
        };
        tok.children = Some(arg_tokens);
        tok.tag = 1; // FUNCTION
        tok.data.push(SValue::from_s(varname.clone()));
        // is user function or array?
        let func_val = song.variables_get(&varname);
        if func_val.is_some() {
            let func_id: SValue = func_val.unwrap_or(&SValue::from_i(0)).clone();
            tok.ttype = TokenType::CallUserFunction;
            tok.tag = func_id.to_i();
        }
        return tok;
    } else {
        // inc & dec
        if cur.eq("++") {
            cur.next_n(2);
            tok.ttype = TokenType::ValueInc;
            tok.value_i = 1;
            tok.data.push(SValue::from_s(varname));
            return tok;
        } else if cur.eq("--") {
            cur.next_n(2);
            tok.ttype = TokenType::ValueInc;
            tok.value_i = -1;
            tok.data.push(SValue::from_s(varname));
            return tok;
        } else {
            // get variable
            let mut tok = Token::new_value(TokenType::GetVariable, 0);
            tok.lineno = cur.line;
            tok.value_type = TokenValueType::VARIABLE;
            tok.value_s = Some(varname);
            return tok;
        }
    }
}

fn is_mml_state_name(name: &str) -> bool {
    matches!(
        name,
        "l"
            | "v"
            | "o"
            | "q"
            | "t"
            | "@"
            | "BR"
            | "p%"
            | "Key"
            | "TimeKey"
            | "Port"
    )
}

fn is_noteno_func_name(name: &str) -> bool {
    matches!(name, "NoteNo" | "NOTENO")
}

/// NoteNoの引数がMMLの音符表記かどうかを調べる
/// 音符・オクターブ関連の文字で始まる場合はMMLと見なし、
/// それ以外(変数名など)は従来どおり計算式として解析する
fn is_raw_mml_note(arg: &str) -> bool {
    match arg.chars().next() {
        Some(c) => matches!(c, 'a'..='g' | 'n' | 'o' | '<' | '>' | '`'),
        None => false,
    }
}

pub(super) fn is_operator_char(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '|' | '&' | '%' | '≠' | '=' | '>' | '<' | '≧' | '≦' | '!' => true,
        _ => false,
    }
}

pub(super) fn read_operator(cur: &mut SourceCursor) -> Option<(char, isize)> {
    cur.skip_space();
    let mut ch = cur.peek_n(0);
    if !is_operator_char(ch) { return None; }
    if cur.eq("//") || cur.eq("/*"){
        return None;
    }
    if cur.eq(">=") {
        cur.next_n(2);
        ch = '≧';
    }
    else if cur.eq("<=") {
        cur.next_n(2);
        ch = '≦';
    }
    else if cur.eq("<>") || cur.eq("!=") {
        cur.next_n(2);
        ch = '≠';
    }
    else if cur.eq("==") {
        cur.next_n(2);
        ch = '=';
    }
    else if cur.eq("&&") { // logical and
        cur.next_n(2);
        ch = '&';
    }
    else if cur.eq("||") { // logical or
        cur.next_n(2);
        ch = '|';
    }
    else {
        cur.next();
    }
    let priority = match ch {
        '+' => LEX_PLUS_MINUS,
        '-' => LEX_PLUS_MINUS,
        '*' => LEX_MUL_DIV,
        '/' => LEX_MUL_DIV,
        '|' => LEX_OR_AND,
        '&' => LEX_OR_AND,
        '%' => LEX_MUL_DIV,
        '≠' => LEX_COMPARE,
        '=' => LEX_COMPARE,
        '>' => LEX_COMPARE,
        '<' => LEX_COMPARE,
        '≧' => LEX_COMPARE,
        '≦' => LEX_COMPARE,
        '!' => LEX_COMPARE,
        _ => { -1 }
    };
    if priority < 0 {
        return None;
    }
    Some((ch, priority))
}

pub(super) fn read_calc_tokens(cur: &mut SourceCursor, song: &mut Song) -> Option<Vec<Token>> {
    match read_calc(cur, song) {
        Some(tok) => { Some(vec![tok]) },
        None => None,
    }
}

pub(super) fn read_calc(cur: &mut SourceCursor, song: &mut Song) -> Option<Token> {
    // read left value
    let mut left_val = match read_value(cur, song) {
        Some(res) => res,
        None => return None,
    };
    // read operator and right value
    while cur.has_next() {
        // read operator
        let (operator_ch, operator_priority) = match read_operator(cur) {
            Some(res) => res,
            None => break,
        };
        // println!("@@@operator_ch={}({})", operator_ch, operator_priority);
        // read right value
        let right_val_o = read_calc(cur, song);
        if right_val_o.is_none() {
            let msg = song.get_message(MessageKind::ErrorMissingValue);
            read_error(cur, song, msg);
        }
        let right_val = right_val_o.unwrap_or(Token::new_empty("ERROR", cur.line));
        
        // replace left_val to CalcTree
        if left_val.ttype != TokenType::CalcTree {
            left_val = Token::new_calc_token(
                operator_ch,
                operator_priority,
                vec![left_val, right_val]);
            continue;
        }
        // check priority
        if left_val.value_i < operator_priority {
            // (examle) 1 + 2 * 3 => [left] (1 + 2) [operator] * [right] 3
            // => (1 + (2 * 3))
            // 元々の左側の演算をばらして、右側にくっつける
            let left_operator = left_val.operator_flag;
            let left_priority = left_val.value_i;
            let mut left_val_children = left_val.children.clone().unwrap_or(vec![]);
            if left_val_children.len() < 2 { // 括弧や値の場合
                // example (1) + 2
                left_val = Token::new_calc_token(
                    operator_ch,
                    operator_priority,
                    vec![left_val, right_val]);
                continue;
            }
            let val2 = left_val_children.pop().unwrap_or(Token::new_const(TokenType::ConstInt, 0, None, TokenValueType::INT));
            let val1 = left_val_children.pop().unwrap_or(Token::new_const(TokenType::ConstInt, 0, None, TokenValueType::INT));
            let val3 = right_val;
            let new_right = Token::new_calc_token(
                operator_ch,
                operator_priority,
                vec![val2, val3]);
            left_val = Token::new_calc_token(
                left_operator,
                left_priority,
                vec![val1, new_right]);
        } else {
            left_val = Token::new_calc_token(
                operator_ch,
                operator_priority,
                vec![left_val, right_val]);
        }
    }
    // println!("@@@read_calc={:?}", left_val.to_debug_str(0));
    Some(left_val)
}

/// lex calc script
pub(super) fn lex_calc(song: &mut Song, src: &str, lineno: isize) -> Vec<Token> {
    let mut cur = SourceCursor::from(src);
    cur.line = lineno;
    let mut result = vec![];
    while !cur.is_eos() {
        let lastpos = cur.index;
        let tokens = read_calc_tokens(&mut cur, song).unwrap_or(vec![]);
        result.extend(tokens);
        if cur.peek().unwrap_or('\0') == ',' {
            cur.next();
            continue;
        }
        if lastpos == cur.index {
            let ch = cur.get_char();
            if song.debug {
                println!("[skip]({}) {}", cur.line, ch);
            }
        }
    }
    result
}

pub(super) fn read_while(cur: &mut SourceCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    cur.skip_space();
    if !cur.eq_char('(') {
        read_error_cmd(cur, song, "WHILE");
        return Token::new_empty("ERROR:WHILE", cur.line);
    }
    // read condition
    if !cur.eq_char('(') {
        read_error_cmd(cur, song, "WHILE");
        return Token::new_empty("ERROR:WHILE", cur.line);
    }
    let cond_s = cur.get_token_nest('(', ')');
    let cond_tok = lex_calc(song, &cond_s, lineno);
    cur.skip_space();
    // read body
    let body_s = cur.get_token_nest('{', '}');
    let body_tok = lex(song, &body_s, lineno);
    // while
    let while_tok = Token::new_tokens_lineno(TokenType::While, 0, vec![
        Token::new_tokens(TokenType::Tokens, 0, cond_tok),
        Token::new_tokens(TokenType::Tokens, 0, body_tok),
    ], lineno);
    while_tok
}

pub(super) fn read_for(cur: &mut SourceCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    cur.skip_space();
    if !cur.eq_char('(') {
        read_error_cmd(cur, song, "FOR");
        return Token::new_empty("ERROR:FOR", cur.line);
    }
    // read init
    cur.next(); // skip '('
    let init_s = cur.get_token_ch(';').trim().to_string();
    let cond_s = cur.get_token_ch(';');
    let inc_s = cur.get_token_ch(')');
    println!("---");
    cur.skip_space();
    if !cur.eq_char('{') {
        read_error_cmd(cur, song, "FOR");
        return Token::new_empty("ERROR:FOR", cur.line);
    }
    let body_s = cur.get_token_nest('{', '}');
    // もし、String型のinit_sが"Int "から始まっていなければ"Int "を足す
    let init_s = if init_s == "" || (init_s.starts_with("Int ") || init_s.starts_with("INT "))  {
        init_s
    } else {
        format!("Int {}", init_s)
    };
    let init_tok = lex(song, &init_s, lineno);
    let cond_tok = lex_calc(song, &cond_s, lineno);
    let inc_tok = lex(song, &inc_s, lineno);
    let body_tok = lex(song, &body_s, lineno);
    let for_tok = Token::new_tokens_lineno(TokenType::For, 0, vec![
        Token::new_tokens(TokenType::Tokens, 0, init_tok),
        Token::new_tokens(TokenType::Tokens, 0, cond_tok),
        Token::new_tokens(TokenType::Tokens, 0, inc_tok),
        Token::new_tokens(TokenType::Tokens, 0, body_tok),
    ], lineno);
    for_tok
}

pub(super) fn read_if(cur: &mut SourceCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    // read condition
    cur.skip_space();
    if !cur.eq_char('(') {
        read_error_cmd(cur, song, "IF");
        return Token::new_empty("ERROR:IF", cur.line);
    }
    let cond = cur.get_token_nest('(', ')');
    let cond_tok = lex_calc(song, &cond, cur.line);
    cur.skip_space();
    if !cur.eq_char('{') {
        read_error_cmd(cur, song, "IF");
        return Token::new_empty("ERROR:IF", cur.line);
    }
    // read then block
    let then_s = cur.get_token_nest('{', '}');
    let then_tok = lex(song, &then_s, cur.line);
    let mut else_tok = vec![];
    cur.skip_space_ret();
    // read else block
    if cur.eq("ELSE") || cur.eq("Else") {
        let else_lineno = cur.line;
        cur.next_n(4); // skip "ELSE"
        cur.skip_space();
        if !cur.eq_char('{') {
            read_error_cmd(cur, song, "IF");
            return Token::new_empty("ERROR:IF:ELSE", else_lineno);
        }
        let else_s = cur.get_token_nest('{', '}');
        else_tok = lex(song, &else_s, else_lineno);
    }
    // println!("cond: {:?}", cond_tok);
    // token
    Token::new_tokens_lineno(TokenType::If, 0, vec![
        Token::new_tokens(TokenType::Tokens, 0, cond_tok),
        Token::new_tokens(TokenType::Tokens, 0, then_tok),
        Token::new_tokens(TokenType::Tokens, 0, else_tok),
    ], lineno)
}
