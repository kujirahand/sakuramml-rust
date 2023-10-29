//! lexer

use std::vec;

use crate::cursor::TokenCursor;
use crate::runner::calc_length;
use crate::sakura_message::MessageKind;
use crate::song::{Song, SFunction};
use crate::svalue::SValue;
use crate::token::{self, zen2han, Token, TokenType};

const LEX_MAX_ERROR: usize = 30;

/// prerpcess 
pub fn lex_preprocess(song: &mut Song, cur: &mut TokenCursor) -> bool {
    let tmp_lineno = cur.line;
    while !cur.is_eos() {
        // skip comment
        if cur.eq("/*") {
            cur.get_token_s("*/");
            continue;
        }
        if cur.eq("//") {
            cur.get_token_ch('\n');
            continue;
        }
        if cur.eq("##") || cur.eq("# ") || cur.eq("#-") { // なんか、みんながよく使っているのでコメントと見なす
            cur.get_token_ch('\n');
            continue;
        }
        // check upper case
        if cur.is_upper() {
            let word = cur.get_word();
            // Check defining user function
            if word == "FUNCTION" || word == "Function" {
                cur.skip_space();
                let func_name = cur.get_word();
                // check double definition
                if song.variables_contains_key(&func_name) {
                    let reason = song.get_message(MessageKind::ErrorRedfineFnuction);
                    read_warning(cur, song, &func_name, reason);
                }
                // check reserved words
                if song.reserved_words.contains_key(&func_name) {
                    let msg = format!("{}: \"{}\"", song.get_message(MessageKind::ErrorDefineVariableIsReserved), func_name);
                    read_error(cur, song, &msg);
                }
                // register function name
                let func_id = song.functions.len();
                song.variables_insert(&func_name, SValue::UserFunc(func_id));
                let sfunc = SFunction::new(&func_name, vec![], func_id, 0);
                song.functions.push(sfunc);
                continue;
            }
            if word == "END" || word == "End" { // それ以降をコンパイルしない
                break;
            }
        }
        let ch = cur.get_char();
        if ch == '\n' {
            cur.line += 1;
        }
    }
    cur.index = 0;
    cur.line = tmp_lineno;
    true
}

/// split source code to tokens
pub fn lex(song: &mut Song, src: &str, lineno: isize) -> Vec<Token> {
    let mut result: Vec<Token> = vec![
        Token::new_lineno(lineno), // init lineno
    ];
    let mut cur = TokenCursor::from(src);
    cur.line = lineno;
    // preprocess
    let _pre = lex_preprocess(song, &mut cur);
    // read
    let mut flag_harmony = false;
    while !cur.is_eos() {
        let ch = zen2han(cur.get_char());
        // println!("lex: ch = {}", ch);
        match ch {
            // <CHAR_COMMANDS>
            // space
            ' ' | '\t' | '\r' | '|' | ';' => {}, // @ 空白文字
            // ret
            '\n' => {
                cur.line += 1;
                result.push(Token::new_lineno(cur.line));
            },
            // lower command
            'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b' => result.push(read_note(&mut cur, ch)), // @ ドレミファソラシ c(音長),(ゲート),(音量),(タイミング),(音階)
            'n' => result.push(read_note_n(&mut cur, song)), // @ 番号を指定して発音(例: n36) n(番号),(音長),(ゲート),(音量),(タイミング)
            'r' => result.push(read_rest(&mut cur)),         // @ 休符
            'l' => result.push(read_length(&mut cur)),       // @ 音長の指定(例 l4)
            'o' => result.push(read_octave(&mut cur, song)), // @ 音階の指定(例 o5) 範囲:0-10
            'p' => result.push(read_pitch_bend_small(&mut cur, song)), // @ ピッチベンドの指定 範囲:0-127 (63が中央)
            'q' => result.push(read_qlen(&mut cur, song)), // @ ゲートの指定 (例 q90) 範囲:0-100
            'v' => result.push(read_velocity(&mut cur, song)), // @ ベロシティ音量の指定 範囲:0-127 / v.Random=n
            't' => result.push(read_timing(&mut cur, song)), // @ 発音タイミングの指定 (例 t-1) / t.Random=n
            'y' => result.push(read_cc(&mut cur, song)), // @ コントロールチェンジの指定 (例 y1,100) 範囲:0-127 / y1.onTime(low,high,len)
            // uppwer command
            'A'..='Z' | '_' => {
                cur.prev();
                if cur.eq("End") || cur.eq("END") { // それ移行をコンパイルしない
                    let last_comment = cur.cur2end();
                    cur.next_n(last_comment.len());
                    result.push(Token::new_empty(&last_comment, cur.line));
                    continue;
                }
                result.push(read_upper_command(&mut cur, song))
            },
            '#' => {
                cur.prev();
                if cur.eq("##") || cur.eq("# ") || cur.eq("#-") { // なんかみんなが使っているので一行コメントと見なす
                    cur.get_token_ch('\n');
                    continue;
                }
                result.push(read_upper_command(&mut cur, song))
            },
            // flag
            '@' => result.push(read_voice(&mut cur, song)), // @ 音色の指定 範囲:1-128 (書式) @(no),(Bank_LSB),(Bank_MSB)
            '>' => result.push(Token::new_value(TokenType::OctaveRel, 1)), // @ 音階を1つ上げる
            '<' => result.push(Token::new_value(TokenType::OctaveRel, -1)), // @ 音階を1つ下げる
            ')' => result.push(Token::new_value(TokenType::VelocityRel, song.v_add)), // @ 音量をvAddの値だけ上げる
            '(' => result.push(Token::new_value(TokenType::VelocityRel, -1 * song.v_add)), // @ 音量をvAddの値だけ下げる
            // comment
            /*
            "//" => // @ 一行コメント
            "/*" .. "*/" => // @ 範囲コメント
            "##" => // @ 一行コメント
            "# " => // @ 一行コメント
            "#-" => // @ 一行コメント
             */
            '/' => {
                if cur.eq_char('/') {
                    cur.get_token_ch('\n');
                } else if cur.eq_char('*') {
                    cur.get_token_s("*/");
                }
            }
            '[' => result.push(read_loop(&mut cur, song)), // @ ループ開始 (例 [4 cdeg])
            ':' => result.push(Token::new_value(TokenType::LoopBreak, 0)), // @ ループ最終回に脱出 (例　[4 cde:g]e)
            ']' => result.push(Token::new_value(TokenType::LoopEnd, 0)),   // @ ループ終了
            '\'' => result.push(read_harmony_flag(&mut cur, &mut flag_harmony)), // @ 和音 (例 'ceg') 'ceg'(音長),(ゲート)
            '$' => read_def_rhythm_macro(&mut cur, song), // @ リズムマクロ定義 $文字{定義内容}
            '{' => result.push(read_command_div(&mut cur, song, true)), // @ 連符 (例 {ceg}4) {c^d}(音長)
            '`' => result.push(Token::new_value(TokenType::OctaveOnce, 1)), // @ 一度だけ音階を+1する
            '"' => result.push(Token::new_value(TokenType::OctaveOnce, -1)), // @ 一度だけ音階を-1する
            '?' => result.push(Token::new_value(TokenType::PlayFromHere, 0)), // @ ここから演奏する (=PlayFromHere)
            '&' => {}, // @ タイ・スラー(Slurコマンドで動作が変更できる)
            // </CHAR_COMMANDS>
            _ => {
                let msg = format!("{}", ch);
                lex_error(&mut cur, song, &msg);
                cur.next();
            }
        }
    }
    normalize_tokens(result)
}

fn lex_error(cur: &mut TokenCursor, song: &mut Song, msg: &str) {
    // make error log
    let near = cur.peek_str_n(8).replace('\n', "↵");
    let log = format!(
        "[ERROR]({}) {}: \"{}\" {} \"{}\"",
        cur.line,
        song.get_message(MessageKind::UnknownChar),
        msg,
        song.get_message(MessageKind::Near),
        near
    );
    if song.debug { println!("{}", log); }
    // add to logs
    if song.get_logs_len() == LEX_MAX_ERROR {
        song.add_log(format!(
            "[ERROR]({}) {}",
            cur.line,
            song.get_message(MessageKind::TooManyErrorsInLexer)
        ));
    } else if song.get_logs_len() < LEX_MAX_ERROR {
        song.add_log(log);
    }
}

/// read Upper case commands
fn read_upper_command(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let mut cmd = cur.get_word();
    // Systemの場合は"."に続く
    if cmd == "System" || cmd == "SYSTEM" {
        cmd = "System".to_string(); // convert "SYSTEM" to "System"
        if cur.eq_char('.') {
            cur.next();
            cmd.push('.');
        }
        cmd.push_str(&cur.get_word());
    }
    // PlayFromも"."が続く場合がある
    if cmd == "PlayFrom" {
        if cur.eq_char('.') {
            cur.next();
            cmd.push('.');
            cmd.push_str(&cur.get_word());
        }
    }

    let lineno = cur.line;
    // SystemFunction ?
    if let Some(f) = song.system_functions.get(&cmd) {
        let arg_t = f.arg_type;
        let token_t = f.token_type;
        let tag1 = f.tag1;
        let tag2 = f.tag2;
        match arg_t {
            'I' | 'S' | 'A' => {
                if cur.eq_char('=') { cur.next(); }
                let args = read_args_tokens(cur, song);
                return Token::new_tokens_lineno(token_t, tag1, args, lineno);
            },
            '_' => { // no paramerter
                return Token::new_tokens_lineno(token_t, tag1, vec![], lineno);
            },
            _ => {
                // 例外的に読み取り処理が必要な特別コマンド
                match token_t {
                    TokenType::Rhythm => return read_command_rhythm(cur, song),
                    TokenType::Div => return read_command_div(cur, song, false),
                    TokenType::Sub => return read_command_sub(cur, song),
                    TokenType::KeyFlag => return read_key_flag(cur, song),
                    TokenType::DefInt => return read_def_int(cur, song),
                    TokenType::DefStr => return read_def_str(cur, song),
                    TokenType::Play => return read_play(cur, song),
                    TokenType::TimeBase => return read_timebase(cur, song),
                    TokenType::Include => return read_include(cur, song),
                    TokenType::ControllChangeCommand => return read_command_cc(cur, tag1, song),
                    TokenType::PitchBend => return read_command_pitch_bend_big(cur, song),
                    TokenType::RPNCommand => return read_rpn_command(cur, tag1, tag2, song),
                    TokenType::NRPNCommand => return read_nrpn_command(cur, tag1, tag2, song),
                    TokenType::FadeIO => return read_fadein(cur, song, tag1),
                    TokenType::Cresc => return read_decres(cur, song, tag1),
                    TokenType::SysExCommand => {
                        match tag1 {
                            0 => return Token::new_sysex(vec![0x7E, 0x7F, 0x9, 0x1, 0xF7]),
                            1 => return Token::new_sysex(vec![0x41, 0x10, 0x42, 0x12, 0x40, 0x00, 0x7F, 0x00, 0x41, 0xF7]),
                            2 => return Token::new_sysex(vec![0x43, 0x10, 0x4c, 0x00, 0x00, 0x7e, 0x00, 0xf7]),
                            _ => {},
                        }
                    },
                    TokenType::If => return read_if(cur, song),
                    TokenType::For => return read_for(cur, song),
                    TokenType::While => return read_while(cur, song),
                    TokenType::Return => {
                        cur.skip_space();
                        let values = if cur.eq_char('(') {
                            read_args_tokens(cur, song)
                        } else {
                            vec![Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_i(0)])]
                        };
                        return Token::new_tokens(TokenType::Return, 0, values);
                    },
                    TokenType::SetRandomSeed => {
                        let v = read_arg_value(cur, song);
                        song.rand_seed = v.to_i() as u32;
                        return Token::new(TokenType::SetConfig, 0, vec![
                            SValue::from_str("RandomSeed"),
                            v
                        ]);
                    },
                    TokenType::DefUserFunction => return read_def_user_function(cur, song),
                    _ => {
                        println!("[SYSTEM_ERROR] FUNCTION NOT SET : {}", cmd);
                    },
                }
            }
        };
    }
    //
    // check variable
    //
    match check_variables(cur, song, cmd.clone()) {
        Some(res) => return res,
        None => {}
    }
    read_error_cmd(cur, song, &cmd);
    return Token::new_empty(&cmd, cur.line);
}

fn read_def_user_function(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    // get function name
    let func_name = cur.get_word();
    cur.skip_space();
    // get args
    if !cur.eq_char('(') {
        return read_error_cmd(cur, song, "FUNCTION");
    }
    // check args
    song.variables_stack_push();
    let args_str = cur.get_token_nest('(', ')');
     let args: Vec<&str> = args_str.split(",").collect();
     let mut arg_types: Vec<char> = vec![];
     let mut arg_names: Vec<String> = vec![];
    for i in 0..args.len() {
        let name = args[i].trim().to_string();
        if name.len() == 0 { continue; }
        // include space?
        if name.contains(" ") {
            // split string by " "
            let splited: Vec<&str> = name.split(" ").collect();
            let name_s = splited[0].trim();
            let type_s = splited[1].trim();
            // get type
            let mut type_sf = 'I';
            if type_s == "Str" || type_s == "STR" || type_s == "S" { type_sf = 'S'; }
            if type_s == "Int" || type_s == "INT" || type_s == "I" { type_sf = 'I'; }
            if type_s == "Array" || type_s == "ARRAY" || type_s == "A" { type_sf = 'A'; }
            song.variables_insert(name_s, SValue::new());
            arg_types.push(type_sf);
            arg_names.push(name_s.to_string());
        } else {
            // only name
            song.variables_insert(&name, SValue::Int(0));
            arg_types.push('i');
            arg_names.push(name);
        }
    }
    // get body
    cur.skip_space_ret();
    if !cur.eq_char('{') {
        return read_error_cmd(cur, song, "FUNCTION");
    }
    let lineno = cur.line;
    let body_s = cur.get_token_nest('{', '}');
    let body_tok = lex(song, &body_s, lineno);
    song.variables_stack_pop(); // destroy local variables
    // register variables
    let func_val = song.variables_get(&func_name).unwrap_or(&SValue::new()).clone();
    let func_id = match func_val {
        SValue::UserFunc(func_id) => func_id,
        _ => {
            // system error to analyze function in preprocess
            read_error_cmd(cur, song, &format!("(System error) Define Function: {}", func_name));
            0
        }
    };
    // register function to song.functions
    let mut func_obj = SFunction::new(&func_name, body_tok, func_id, lineno);
    func_obj.arg_names = arg_names;
    func_obj.arg_types = arg_types;
    song.functions[func_id] = func_obj;
    Token::new_empty(&format!("DefineFunction::{}", func_name), lineno)
}

fn read_error_cmd(cur: &mut TokenCursor, song: &mut Song, cmd: &str) -> Token {
    let near = cur.peek_str_n(8).replace('\n', "↵");
    let error_log = format!(
        "[ERROR]({}) {} \"{}\" {} \"{}\"",
        cur.line,
        song.get_message(MessageKind::ScriptSyntaxError),
        cmd,
        song.get_message(MessageKind::Near),
        near,
    );
    if song.debug { println!("{}", error_log); }
    song.add_log(error_log);
    return Token::new_empty("ERROR", cur.line);
}

fn read_include(cur: &mut TokenCursor, _song: &mut Song) -> Token {
    // @ 未実装
    cur.skip_space();
    let filename = if cur.eq_char('(') {
        cur.get_token_nest('(', ')')
    } else {
        "".to_string()
    };
    return Token::new_empty(&format!("Unimplemented Include({})", filename), cur.line);
}

fn read_error(cur: &mut TokenCursor, song: &mut Song, msg: &str) -> Token {
    let near = cur.peek_str_n(8).replace('\n', "↵");
    song.add_log(format!(
        "[ERROR]({}) {} {} \"{}\"",
        cur.line,
        msg,
        song.get_message(MessageKind::Near),
        near,
    ));
    return Token::new_empty("ERROR", cur.line);
}


fn read_warning(cur: &mut TokenCursor, song: &mut Song, cmd: &str, reason: &str) -> Token {
    let near = cur.peek_str_n(8).replace('\n', "↵");
    song.add_log(format!(
        "[WARN]({}) {} \"{}\" {} : {} \"{}\"",
        cur.line,
        song.get_message(MessageKind::ScriptSyntaxWarning),
        cmd,
        reason,
        song.get_message(MessageKind::Near),
        near,
    ));
    return Token::new_empty("ERROR", cur.line);
}


// --- lex calc script ---
const LEX_VALUE: isize = 1;
// const LEX_NOT: isize = 2;
const LEX_PAREN_L: isize = 3;
const LEX_PAREN_R: isize = 4;
const LEX_MUL_DIV: isize = 20;
const LEX_PLUS_MINUS: isize = 30;
const LEX_COMPARE: isize = 40;
const LEX_OR_AND: isize = 50;

fn is_operator(c: isize) -> bool {
    match c {
        LEX_COMPARE | LEX_MUL_DIV | LEX_PLUS_MINUS | LEX_OR_AND => true,
        _ => false,
    }
}

fn read_calc_can_continue(cur: &mut TokenCursor, parent_level: i32) -> bool {
    if parent_level > 0 { return true; }
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        '+' | '-' | '*' | '/' | '|' | '&' | '%' | '≠' | '=' | '>' | '<' | '≧' | '≦' | '!' => true,
        _ => false,
    }
}

fn read_calc_token(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    let tokens = read_calc(cur, song);
    Token::new_tokens_lineno(TokenType::Tokens, 0, tokens, lineno)
}

fn read_calc(cur: &mut TokenCursor, song: &mut Song) -> Vec<Token> {
    let mut tokens = vec![];
    let mut paren_level:i32 = 0;
    while !cur.is_eos() {
        cur.skip_space();
        let ch = zen2han(cur.peek().unwrap_or('\0'));
        match ch {
            // blank
            '\t' => { cur.next(); }, // comment
            // break chars
            '\n' => { cur.line += 1; break; },
            ';' => break,
            ',' => break,
            '(' => {
                tokens.push(Token::new(TokenType::Calc, LEX_PAREN_L, vec![]));
                paren_level += 1;
                cur.next();
            },
            ')' => {
                if paren_level == 0 { break; }
                paren_level -=1;
                tokens.push(Token::new(TokenType::Calc, LEX_PAREN_R, vec![]));
                cur.next();
                if !read_calc_can_continue(cur, paren_level) { break; }
            },
            // value
            '0'..='9' => {
                let num = cur.get_int(0);
                let mut tok_num = Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_i(num)]);
                tok_num.tag = 0;
                tok_num.value_type = token::VALUE_CONST_INT;
                tokens.push(tok_num);
                if !read_calc_can_continue(cur, paren_level) { break; }
            },
            'A'..='Z' | '_' | '#' | 'a'..='z' => {
                let mut tok = Token::new(TokenType::Value, LEX_VALUE, vec![]);
                let varname = cur.get_word();
                let varname_flag = format!("={}", varname);
                // println!("read_calc:{}", varname);
                tok.tag = 0;
                if cur.eq_char('(') {
                    // function call
                    let arg_lineno = cur.line;
                    let arg_str = cur.get_token_nest('(', ')');
                    // println!("read_calc_args={:?}", arg_str);
                    let arg_tokens = lex_calc(song, &arg_str, arg_lineno);
                    tok.children = Some(arg_tokens);
                    tok.tag = 1; // FUNCTION
                    tok.data.push(SValue::from_s(varname.clone()));
                    // is user function?
                    let func_val = song.variables_get(&varname);
                    if func_val.is_some() {
                        let func_id: SValue = func_val.unwrap_or(&SValue::from_i(0)).clone();
                        tok.ttype = TokenType::CallUserFunction;
                        tok.tag = func_id.to_i();
                    }
                    tokens.push(tok);
                } else {
                    // inc & dec
                    if cur.eq("++") {
                        tok.ttype = TokenType::ValueInc;
                        tok.value = 1;
                        tok.data.push(SValue::from_s(varname));
                        tokens.push(tok);
                    } else if cur.eq("--") {
                        tok.ttype = TokenType::ValueInc;
                        tok.value = -1;
                        tok.data.push(SValue::from_s(varname));
                        tokens.push(tok);
                    } else {
                        // ref variable
                        tok.data.push(SValue::from_s(varname_flag));
                        tok.value_type = token::VALUE_VARIABLE;
                        tokens.push(tok);
                    }
                }
                if !read_calc_can_continue(cur, paren_level) { break; }
            },
            '"' => {
                cur.next(); // skip "
                let str_s = cur.get_token_to_double_quote();
                let tok = Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_s(str_s)]);
                tokens.push(tok);
                if !read_calc_can_continue(cur, paren_level) { break; }
            },
            '{' => {
                let str_s = cur.get_token_nest('{', '}');
                let mut tok = Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_s(str_s)]);
                tok.value_type = token::VALUE_CONST_STR;
                tokens.push(tok);
                if !read_calc_can_continue(cur, paren_level) { break; }
            },
            // value or operator
            '-' => {
                cur.next(); // skip '-'
                let ch2 = cur.peek().unwrap_or('\0');
                if ch2 >= '0' && ch2 <= '9' {
                    let num = cur.get_int(0) * -1;
                    tokens.push(Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_i(num)]));
                    if !read_calc_can_continue(cur, paren_level) { break; }
                } else {
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_PLUS_MINUS, ch as isize));
                }
            },
            // operator
            '/' => {
                if cur.eq("//") { break; }
                if cur.eq("/*") {
                    cur.get_token_s("*/");
                    continue;
                }
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_MUL_DIV, ch as isize));
                cur.next();
            },
            '*' | '%' => {
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_MUL_DIV, ch as isize));
                cur.next();
            },
            '+' => {
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_PLUS_MINUS, ch as isize));
                cur.next();
            },
            '|' => {
                if cur.eq("||") {
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_OR_AND, ch as isize));
                }
                cur.next();
            },
            '&' => {
                cur.next_n(if cur.eq("&&") { 2 } else { 1 });
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_OR_AND, ch as isize));
            },
            '≠' => {
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≠' as isize));
                cur.next();
            }
            '≧' => {
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≧' as isize));
                cur.next();
            }
            '≦' => {
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≦' as isize));
                cur.next();
            }
            '=' => {
                cur.next_n(if cur.eq("==") { 2 } else { 1 });
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, ch as isize));
            },
            '!' => {
                if cur.eq("!=") {
                    cur.next_n(2);
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≠' as isize));
                } else {
                    // timebase length
                    cur.next(); // skip !
                    let len_str = cur.get_note_length();
                    let val = SValue::from_i(calc_length(&len_str, song.timebase, song.timebase));
                    tokens.push(Token::new(TokenType::Value, LEX_VALUE, vec![val]));
                }
            },
            '>' | '<' => {
                if cur.eq("<>") | cur.eq("><") {
                    cur.next_n(2);
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≠' as isize));
                }
                else if cur.eq(">=") {
                    cur.next_n(2);
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≧' as isize));
                }
                else if cur.eq("<=") {
                    cur.next_n(2);
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≦' as isize));
                }
                else if cur.eq("<") {
                    cur.next();
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '<' as isize));
                }
                else if cur.eq(">") {
                    cur.next();
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '>' as isize));
                }
            },
            _ => {
                // 計算時に使えない文字列があれば抜ける
                // let msg = format!("{}", ch);
                // lex_error(cur, song, &msg);
                break;
            }
        }
    }
    // convert to postfix
    infix_to_postfix(cur, song, tokens)
}

fn infix_to_postfix(cur: &mut TokenCursor, song: &mut Song, tokens: Vec<Token>) -> Vec<Token> {
    let mut result = vec![];
    let mut stack:Vec<Token> = vec![];
    for token in tokens.into_iter() {
        if token.value == LEX_VALUE {
            result.push(token);
            continue;
        }
        if is_operator(token.value) {
            while let Some(top) = stack.last().cloned() {
                if top.value == LEX_PAREN_L || top.value > token.value {
                    break;
                }
                result.push(stack.pop().unwrap());
            }
            stack.push(token);
            continue;
        }
        else if token.value == LEX_PAREN_L {
            stack.push(token);
            continue;
        }
        else if token.value == LEX_PAREN_R {
            while let Some(top) = stack.last().cloned() {
                if top.value == LEX_PAREN_L {
                    break;
                }
                result.push(stack.pop().unwrap());
            }
            stack.pop();
            continue;
        } else {
            let msg = format!("Invalid token type: {:?}", token.value);
            lex_error(cur, song, &msg);
            result.clear();
            return vec![];
        }
    }
    while let Some(op) = stack.pop() {
        if op.value == LEX_PAREN_L {
            let msg = format!("Unmatched parenthesis");
            lex_error(cur, song, &msg);
            result.clear();
            return vec![];
        }
        result.push(op);
    }
    result
}

/// lex calc script
fn lex_calc(song: &mut Song, src: &str, lineno: isize) -> Vec<Token> {
    let mut cur = TokenCursor::from(src);
    cur.line = lineno;
    let mut result = vec![];
    while !cur.is_eos() {
        let lastpos = cur.index;
        let tokens = read_calc(&mut cur, song);
        result.extend(tokens);
        if cur.peek().unwrap_or('\0') == ',' {
            cur.next();
            continue;
        }
        if lastpos == cur.index {
            let ch = cur.get_char();
            println!("[skip]({}) {}", cur.line, ch);
        }
    }
    result
}

fn read_while(cur: &mut TokenCursor, song: &mut Song) -> Token {
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

fn read_for(cur: &mut TokenCursor, song: &mut Song) -> Token {
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
    let inc_tok = lex_calc(song, &inc_s, lineno);
    let body_tok = lex(song, &body_s, lineno);
    let for_tok = Token::new_tokens_lineno(TokenType::For, 0, vec![
        Token::new_tokens(TokenType::Tokens, 0, init_tok),
        Token::new_tokens(TokenType::Tokens, 0, cond_tok),
        Token::new_tokens(TokenType::Tokens, 0, inc_tok),
        Token::new_tokens(TokenType::Tokens, 0, body_tok),
    ], lineno);
    for_tok
}

fn read_if(cur: &mut TokenCursor, song: &mut Song) -> Token {
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

fn check_variables(cur: &mut TokenCursor, song: &mut Song, cmd: String) -> Option<Token> {
    // increment variable?
    if cur.eq("++") {
        cur.next_n(2);
        return Some(Token::new(TokenType::ValueInc, 1, vec![SValue::from_s(cmd)]));
    }
    if cur.eq("--") {
        cur.next_n(2);
        return Some(Token::new(TokenType::ValueInc, -1, vec![SValue::from_s(cmd)]));
    }
    // let?
    cur.skip_space();
    if cur.eq("=") {
        cur.next();
        cur.skip_space();
        // check reserved words
        if song.reserved_words.contains_key(&cmd) {
            let msg = format!("{}: \"{}\"", song.get_message(MessageKind::ErrorDefineVariableIsReserved), cmd);
            return Some(read_error(cur, song, &msg));
        }
        // let str
        if cur.eq_char('{') {
            let body = cur.get_token_nest('{', '}');
            song.variables_insert(&cmd, SValue::from_str_and_tag(&body, cur.line));
            return Some(Token::new_empty("DefStr", cur.line));
        }
        // let calc
        let body_tokens = read_calc(cur, song);
        let tok = Token::new_data_tokens(
            TokenType::LetVar, 0, 
            vec![SValue::from_str(&cmd)],
            vec![Token::new_tokens(TokenType::Tokens, 0, body_tokens)]);
        song.variables_insert(&cmd, SValue::None);
        return Some(tok);
    }

    // variables?
    match song.variables_get(&cmd) {
        Some(sval) => {
            // get variable
            return Some(read_variables(cur, song, &cmd, sval.clone()));
        }
        None => {}
    };
    None
}

fn read_variables(cur: &mut TokenCursor, song: &mut Song, name: &str, sval: SValue) -> Token {
    match sval {
        SValue::Str(_src_org, _line_no) => {
            // replace macro?
            cur.skip_space();
            if cur.eq_char('(') || cur.eq_char('{') {
                let args = read_args_tokens(cur, song);
                let mut tok = Token::new_tokens(TokenType::Value, LEX_VALUE, args);
                tok.tag = 1; // Macro
                tok.data = vec![SValue::from_s(format!("={}", name))];
                tok.lineno = cur.line;
                return tok;
            } else {
                let tok = Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_s(format!("={}", name))]);
                return tok;
            }
        }
        SValue::UserFunc(func_id) => { return read_call_function(cur, song, func_id); },
        _ => { return Token::new_empty(&format!("Could not execute: {}", name), cur.line); }
    }
}

fn read_call_function(cur: &mut TokenCursor, song: &mut Song, func_id: usize) -> Token {
    cur.skip_space();
    let args: Vec<Token> = read_args_tokens(cur, song);
    let mut call_func_tok = Token::new(TokenType::CallUserFunction, func_id as isize, vec![]);
    call_func_tok.children = Some(args);
    call_func_tok
}

// Emptyを削除し、Tokensを展開して返す。ただし、Div/Subは実行時にならないと展開結果が分からないため、それは展開しない
fn normalize_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut res = vec![];
    for t in tokens.into_iter() {
        match t.ttype {
            TokenType::Empty => {}
            TokenType::Tokens => match t.children {
                Some(sub_tt) => {
                    let sub_tt2 = normalize_tokens(sub_tt);
                    for tt in sub_tt2.into_iter() {
                        res.push(tt);
                    }
                }
                None => {}
            },
            _ => {
                res.push(t);
            }
        }
    }
    res
}

fn read_arg_value(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        'A'..='Z' | '_' => {
            let var_name = cur.get_word();
            SValue::from_s(format!("={}", var_name)) // ref: variable
        }
        '!' => {
            // timebase length
            cur.next(); // skip !
            let len_str = cur.get_note_length();
            SValue::from_i(calc_length(&len_str, song.timebase, song.timebase))
        }
        '-' | '0'..='9' => {
            let v = cur.get_int(0);
            SValue::from_i(v)
        }
        '=' => {
            cur.next(); // skip =
            read_arg_value(cur, song)
        }
        '(' => {
            cur.next(); // skip (
            let v = read_arg_value(cur, song);
            cur.skip_space();
            if cur.eq_char(')') {
                cur.next();
            }
            v
        }
        '{' => {
            let s = cur.get_token_nest('{', '}');
            SValue::from_s(s)
        }
        _ => SValue::None,
    }
}

fn read_arg_value_int_array(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    let mut a: Vec<isize> = vec![];
    loop {
        cur.skip_space();
        let v = read_arg_value(cur, song);
        a.push(v.to_i());
        cur.skip_space();
        if !cur.eq_char(',') {
            break;
        }
        cur.next(); // skip ,
    }
    SValue::from_int_array(a)
}

fn read_arg_int_array(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        '(' => {
            cur.next(); // skip '('
            let sv = read_arg_value_int_array(cur, song);
            cur.skip_space();
            if cur.peek_n(0) == ')' {
                cur.next();
            }
            return sv;
        }
        '=' => {
            cur.next();
            read_arg_value_int_array(cur, song)
        }
        _ => SValue::None,
    }
}

fn read_args_tokens(cur: &mut TokenCursor, song: &mut Song) -> Vec<Token> {
    cur.skip_space();
    let skip_paren = if cur.eq_char('(') {
        cur.next(); // skip '('
        true
    } else { false };

    let mut tokens = vec![];
    loop {
        cur.skip_space();
        let sub_tokens = read_calc(cur, song);
        tokens.push(Token::new_tokens(TokenType::Tokens, 0, sub_tokens));
        
        // has next value?
        cur.skip_space();
        if cur.eq_char(',') || cur.eq_char(':') {
            cur.next(); // skip ',' or ':'
        } else {
            break;
        }
    }
    if skip_paren {
        cur.skip_space();
        if cur.eq_char(')') {
            cur.next(); // skip ')'
        } else {
            song.add_log(format!("[ERROR]({}) {}", cur.line, song.get_message(MessageKind::MissingParenthesis)));
        }
    }
    tokens
}

fn read_harmony_flag(cur: &mut TokenCursor, flag_harmony: &mut bool) -> Token {
    // begin
    if !*flag_harmony {
        *flag_harmony = true;
        return Token::new(TokenType::HarmonyBegin, 0, vec![]);
    }
    // end
    *flag_harmony = false;
    let mut len_s = SValue::None;
    let mut qlen = SValue::from_i(-1);
    let mut vel = SValue::None;
    if cur.is_numeric() || cur.eq_char('^') {
        len_s = SValue::from_s(cur.get_note_length());
    }
    cur.skip_space();
    if cur.eq_char(',') {
        cur.next();
        qlen = SValue::from_i(cur.get_int(-1));
        if cur.eq_char(',') {
            cur.next();
            vel = SValue::from_i(cur.get_int(-1));
        }
    }
    Token::new(TokenType::HarmonyEnd, 0, vec![len_s, qlen, vel])
}

fn scan_chars(s: &str, c: char) -> isize {
    let mut cnt = 0;
    for ch in s.chars() {
        if ch == c {
            cnt += 1;
        }
    }
    cnt
}

fn read_timebase(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let v = read_arg_value(cur, song);
    song.timebase = v.to_i();
    if song.timebase <= 48 {
        song.timebase = 48;
    }
    Token::new_empty(&format!("TIMEBASE={}", v.to_i()), cur.line)
}

fn read_key_flag(cur: &mut TokenCursor, _song: &mut Song) -> Token {
    let mut flag = 1;
    let mut key_flag = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let key_flag_index_a = [0, 2, 4, 5, 7, 9, 11];
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    cur.skip_space();
    // flag
    match cur.peek_n(0) {
        '+' | '#' => {
            cur.next();
            flag = 1;
        }
        '-' => {
            cur.next();
            flag = -1;
        }
        _ => {}
    }
    // check note
    cur.skip_space();
    if cur.eq_char('(') {
        cur.next();
    }
    let mut idx = 0;
    while !cur.is_eos() {
        cur.skip_space();
        // numeric value
        if cur.eq_char('0') || cur.eq_char('1') || cur.eq("-1") {
            let v = cur.get_int(0);
            key_flag[key_flag_index_a[idx]] = v;
            idx += 1;
            if idx >= 8 {
                break;
            }
            cur.skip_space();
            if cur.eq_char(',') {
                cur.next();
            }
            continue;
        }
        // note name value
        match cur.peek_n(0) {
            'c' => {
                cur.next();
                key_flag[0] = flag;
            }
            'd' => {
                cur.next();
                key_flag[2] = flag;
            }
            'e' => {
                cur.next();
                key_flag[4] = flag;
            }
            'f' => {
                cur.next();
                key_flag[5] = flag;
            }
            'g' => {
                cur.next();
                key_flag[7] = flag;
            }
            'a' => {
                cur.next();
                key_flag[9] = flag;
            }
            'b' => {
                cur.next();
                key_flag[11] = flag;
            }
            _ => break,
        }
    }
    cur.skip_space();
    if cur.eq_char(')') {
        cur.next();
    }
    // token
    let tok = Token::new(
        TokenType::KeyFlag,
        0,
        vec![SValue::from_int_array(key_flag)],
    );
    tok
}

fn read_def_int(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let var_name = cur.get_word();
    if var_name == "" {
        song.add_log(format!(
            "[ERROR]({}): INT command should use Upper case like \"Test\".",
            cur.line
        ));
        return Token::new_empty("Failed to def INT", cur.line);
    }
    // check reserved words
    if song.reserved_words.contains_key(&var_name) {
        let msg = format!("{}: \"{}\"", song.get_message(MessageKind::ErrorDefineVariableIsReserved), var_name);
        read_error(cur, song, &msg);
        return Token::new_empty("Failed to def INT", cur.line);
    }
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    // get line
    let val_tokens = read_calc(cur, song);
    // token
    let tok = Token::new_data_tokens(
        TokenType::DefInt,
        0,
        vec![SValue::from_s(var_name)],
        val_tokens,
    );
    tok
}

fn read_play(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    let arg_tokens = read_args_tokens(cur, song);
    let play_tok = Token::new_tokens_lineno(TokenType::Play, 0, arg_tokens, lineno);
    play_tok
}

fn read_def_str(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let var_name = cur.get_word();
    if var_name == "" {
        song.add_log(format!(
            "[ERROR]({}): STR command should use Upper case like \"Test\"",
            cur.line
        ));
        return Token::new_empty("Failed to def STR", cur.line);
    }
    // check reserved words
    if song.reserved_words.contains_key(&var_name) {
        let msg = format!("{}: \"{}\"", song.get_message(MessageKind::ErrorDefineVariableIsReserved), var_name);
        read_error(cur, song, &msg);
        return Token::new_empty("Failed to def STR", cur.line);
    }
    cur.skip_space();
    let line_no = cur.line;
    let mut init_value_tokens = vec![];
    let mut data_str = "".to_string();
    if cur.eq_char('=') {
        cur.next();
        cur.skip_space();
        let ch = cur.peek().unwrap_or('\0');
        if ch == '"' && ch == '{' {
            // get normal string
            data_str = if ch == '{' { cur.get_token_nest('{', '}') } else { cur.next(); cur.get_token_ch(ch) };
            data_str = format!("{}{}{}", '{', data_str, '}');
            init_value_tokens = lex_calc(song, &data_str, line_no);
        } else {
            // get calc
            let tok = read_calc_token(cur, song);
            init_value_tokens = vec![tok];
        }
    }
    let var_value = SValue::from_str_and_tag(&data_str, line_no);
    let mut tok = Token::new_tokens(TokenType::DefStr, 0, init_value_tokens);
    tok.data = vec![SValue::from_s(var_name.clone())];
    song.variables_insert(&var_name, var_value);
    tok
}

fn read_command_sub(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let block = cur.get_token_nest('{', '}');
    let tokens = lex(song, &block, cur.line);
    let mut tok = Token::new(TokenType::Sub, 0, vec![]);
    tok.children = Some(tokens);
    tok
}

fn read_command_div(cur: &mut TokenCursor, song: &mut Song, need2back: bool) -> Token {
    // is 1char command
    if need2back {
        cur.prev();
    } else {
        cur.skip_space();
    }
    let block = cur.get_token_nest('{', '}');
    let len_s = cur.get_note_length();
    let tokens = lex(song, &block, cur.line);
    // count note
    let mut cnt = 0;
    for t in tokens.iter() {
        match t.ttype {
            TokenType::Note => {
                cnt += 1;
                cnt += scan_chars(&t.data[1].to_s(), '^');
            }
            TokenType::NoteN => {
                cnt += 1;
                cnt += scan_chars(&t.data[1].to_s(), '^');
            }
            TokenType::Div => {
                cnt += 1;
                cnt += scan_chars(&t.data[0].to_s(), '^');
            }
            TokenType::Rest => {
                cnt += 1;
                cnt += scan_chars(&t.data[0].to_s(), '^');
            }
            _ => {}
        }
    }
    let mut tok = Token::new(TokenType::Div, cnt, vec![SValue::from_s(len_s)]);
    tok.children = Some(tokens);
    tok
}

fn read_command_rhythm(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let mut result = String::new();
    cur.skip_space();
    let line_start = cur.line;
    let block = cur.get_token_nest('{', '}');
    // extract macro
    let mut macro_cur = TokenCursor::from(&block);
    macro_cur.line = line_start;
    while !macro_cur.is_eos() {
        if macro_cur.eq("Sub") || macro_cur.eq("SUB") {
            result.push_str("SUB");
            macro_cur.index += 3;
            continue;
        }
        let ch = macro_cur.get_char();
        match ch {
            '(' => {
                // 丸カッコの中は置換しない
                macro_cur.prev();
                let src = macro_cur.get_token_nest('(', ')');
                result.push_str(&src);
            }
            '\u{0040}'..='\u{007f}' => {
                let m = &song.rhthm_macro[ch as usize - 0x40];
                if m == "" {
                    result.push(ch);
                } else {
                    result.push_str(m);
                }
            }
            _ => {
                result.push(ch);
            }
        }
    }
    let mut t = Token::new_value(TokenType::Tokens, 0);
    t.children = Some(lex(song, &result, cur.line));
    t
}

fn read_def_rhythm_macro(cur: &mut TokenCursor, song: &mut Song) {
    let ch = cur.get_char(); // macro char
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    cur.skip_space();
    let s = cur.get_token_nest('{', '}');
    if 0x40 <= ch as u8 && ch as u8 <= 0x7F {
        song.rhthm_macro[ch as usize - 0x40] = s;
    } else {
        song.add_log(format!(
            "[ERROR]({}) could not define Rhythm macro '{}' ",
            cur.line, ch
        ));
    }
}

fn read_fadein(cur: &mut TokenCursor, song: &mut Song, dir: isize) -> Token {
    let arg = read_arg_value(cur, song);
    let ia = if dir >= 1 {
        SValue::from_int_array(vec![0, 127, song.timebase * 4 * arg.to_i()])
    } else {
        SValue::from_int_array(vec![127, 0, song.timebase * 4 * arg.to_i()])
    };
    return Token::new(TokenType::CConTime, 11, vec![ia]);
}

fn read_decres(cur: &mut TokenCursor, song: &mut Song, dir: isize) -> Token {
    let mut v1 = SValue::from_i(if dir < 0 { 127 } else {  40 });
    let mut v2 = SValue::from_i(if dir < 0 {  40 } else { 127 });
    // skip =
    cur.skip_space();
    if cur.eq_char('=') { cur.next(); }
    // length
    let len_s = cur.get_note_length();
    cur.skip_space();
    if cur.eq_char(',') {
        cur.next(); cur.skip_space();
        v1 = read_arg_value(cur, song);
        cur.skip_space();
        if cur.eq_char(',') {
            cur.next(); cur.skip_space();
            v2 = read_arg_value(cur, song);
        }
    }
    return Token::new(TokenType::Decresc, 0, vec![
        SValue::from_s(len_s), v1, v2
    ]);
}

fn read_command_cc(cur: &mut TokenCursor, no: isize, song: &mut Song) -> Token {
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        if cmd == "onTime" || cmd == "T" {
            let ia = read_arg_int_array(cur, song);
            return Token::new(TokenType::CConTime, no, vec![ia]);
        } else if cmd == "Frequency" {
            let a = read_arg_value(cur, song);
            return Token::new(TokenType::CConTimeFreq, 0, vec![a]);
        } else if cmd == "onNoteWave" || cmd == "W" {
            let ia = read_arg_int_array(cur, song);
            return Token::new(TokenType::CConNoteWave, no, vec![ia]);
        } else if cmd == "onNoteWaveEx" || cmd == "WE" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            song.add_log(format!("[WARN]({}) not supported : onNoteWaveEx", cur.line));
            return Token::new_empty("not supported : onNoteWave", cur.line);
        } else if cmd == "onCycle" || cmd == "C" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            song.add_log(format!("[WARN]({}) not supported : onCycle", cur.line));
            return Token::new_empty("not supported : onCycle", cur.line);
        }
    }
    if cur.eq_char('=') { cur.next(); }
    let arg_token1 = read_calc_token(cur, song);
    return Token::new_tokens(TokenType::CtrlChange, no, vec![arg_token1]);
}

fn read_rpn_command(cur: &mut TokenCursor, msb: isize, lsb: isize, song: &mut Song) -> Token {
    let args = read_args_tokens(cur, song);
    let token = Token::new_data_tokens(TokenType::RPNCommand, 0, vec![SValue::Int(msb), SValue::Int(lsb)], args);
    token
}

fn read_nrpn_command(cur: &mut TokenCursor, msb: isize, lsb: isize, song: &mut Song) -> Token {
    let args = read_args_tokens(cur, song);
    let token = Token::new_data_tokens(TokenType::NRPNCommand, 0, vec![SValue::Int(msb), SValue::Int(lsb)], args);
    token
}

fn read_voice(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let args = read_args_tokens(cur, song);
    Token::new_tokens(TokenType::Voice, 0, args)
}

fn read_length(cur: &mut TokenCursor) -> Token {
    let s = cur.get_note_length();
    Token::new(TokenType::Length, 0, vec![SValue::from_s(s)])
}

fn read_octave(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Octave, value.to_i(), vec![])
}

fn read_qlen(cur: &mut TokenCursor, song: &mut Song) -> Token {
    if cur.eq("++") {
        cur.next_n(2);
        return Token::new(TokenType::QLenRel, 1, vec![]);
    }
    if cur.eq("--") {
        cur.next_n(2);
        return Token::new(TokenType::QLenRel, -1, vec![]);
    }
    if cur.eq("__") {
        // dummy
        cur.next();
        cur.next();
        cur.get_int(0);
    } else if cur.eq("_") {
        cur.next();
        cur.get_int(0);
    }
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        if cmd == "Random" {
            let r = read_arg_value(cur, song);
            return Token::new(TokenType::QLenRandom, 0, vec![r]);
        }
        if cmd == "onTime" || cmd == "T" {
            let _av = read_arg_int_array(cur, song);
            return Token::new_empty(&format!("[ERROR]({}) q.onTime not supported", cur.line), cur.line);
        }
        if cmd == "onNote" || cmd == "N" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::QLenOnNote, 0, vec![av]);
        }
        if cmd == "onCycle" || cmd == "C" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            return Token::new_empty("not supported : onCycle", cur.line);
        }
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::QLen, value.to_i(), vec![])
}

fn read_velocity(cur: &mut TokenCursor, song: &mut Song) -> Token {
    if cur.eq("++") {
        cur.next_n(2);
        return Token::new(TokenType::VelocityRel, 1, vec![]);
    }
    if cur.eq("--") {
        cur.next_n(2);
        return Token::new(TokenType::VelocityRel, -1, vec![]);
    }
    let mut ino = -1;
    if cur.eq("__") {
        // sub velocity
        cur.next();
        cur.next();
        ino = cur.get_int(0);
    } else if cur.eq("_") {
        cur.next();
        cur.get_int(0);
        ino = 0;
    }
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        if cmd == "Random" {
            let r = read_arg_value(cur, song);
            return Token::new(TokenType::VelocityRandom, 0, vec![r]);
        }
        if cmd == "onTime" || cmd == "T" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::VelocityOnTime, 0, vec![av]);
        }
        if cmd == "onNote" || cmd == "N" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::VelocityOnNote, 0, vec![av]);
        }
        if cmd == "onCycle" || cmd == "C" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            return Token::new_empty("not supported : onCycle", cur.line);
        }
    }
    // v(no)
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Velocity, value.to_i(), vec![SValue::from_i(ino)])
}

fn read_timing(cur: &mut TokenCursor, song: &mut Song) -> Token {
    if cur.eq("__") {
        // dummy
        cur.next();
        cur.next();
        cur.get_int(0);
    } else if cur.eq_char('_') {
        cur.next();
    }
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        // t.Random ?
        if cmd == "Random" {
            cur.index += 7;
            let r = read_arg_value(cur, song);
            return Token::new(TokenType::TimingRandom, 0, vec![r]);
        }
        if cmd == "onNote" || cmd == "N" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::TimingOnNote, 0, vec![av]);
        }
        if cmd == "onCycle" || cmd == "C" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            return Token::new_empty("not supported : onCycle", cur.line);
        }
    }
    // t(no)
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Timing, value.to_i(), vec![])
}

fn read_command_pitch_bend_big(cur: &mut TokenCursor, song: &mut Song) -> Token {
    if cur.eq(".onTime") || cur.eq(".T") {
        if cur.eq(".onTime") {
            cur.index += ".onTime".len();
        } else {
            cur.index += ".T".len();
        }
        let ia = read_arg_int_array(cur, song);
        return Token::new(TokenType::PBonTime, 1, vec![ia]);
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::PitchBend, 1, vec![value])
}

fn read_pitch_bend_small(cur: &mut TokenCursor, song: &mut Song) -> Token {
    if cur.eq(".onTime") || cur.eq(".T") {
        if cur.eq(".onTime") {
            cur.index += ".onTime".len();
        } else {
            cur.index += ".T".len();
        }
        let ia = read_arg_int_array(cur, song);
        return Token::new(TokenType::PBonTime, 0, vec![ia]);
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::PitchBend, 0, vec![value])
}

fn read_cc(cur: &mut TokenCursor, song: &mut Song) -> Token {
    // red CC no
    let no = read_arg_value(cur, song);

    // .onTime
    if cur.eq_char('.') {
        cur.next();
        let cmd = cur.get_word();
        if cmd == "onTime" || cmd == "T" {
            let ia = read_arg_int_array(cur, song);
            return Token::new(TokenType::CConTime, no.to_i(), vec![ia]);
        }
        if cmd == "onNoteWave" || cmd == "W" {
            let ia = read_arg_int_array(cur, song);
            return Token::new(TokenType::CConNoteWave, no.to_i(), vec![ia]);
        }
        if cmd == "onCycle" || cmd == "C" {
            let _ia = read_arg_int_array(cur, song);
            return Token::new_empty("NOT SUPPORTED", cur.line);
        }
    }

    cur.skip_space();
    if !cur.eq_char(',') && !cur.eq_char('(') {
        return Token::new(
            TokenType::Error,
            0,
            vec![SValue::from_s(format!(
                "[ERROR]({}): Faild to set Controll Change",
                cur.line + 1
            ))],
        );
    }
    cur.next();
    let val = read_arg_value(cur, song);
    Token::new_tokens(TokenType::CtrlChange, no.to_i(), vec![
        Token::new(TokenType::Value, 0, vec![val]),
    ])
}

fn read_loop(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let value = if cur.is_numeric() || cur.eq_char('=') || cur.eq_char('(') {
        read_arg_value(cur, song)
    } else {
        SValue::from_i(2)
    };
    Token::new(TokenType::LoopBegin, 0, vec![value])
}

fn read_rest(cur: &mut TokenCursor) -> Token {
    // '*'
    if cur.eq_char('*') {
        cur.next();
    }
    // length
    let mut dir = 1;
    if cur.eq_char('-') {
        cur.next();
        dir = -1;
    }
    let note_len = cur.get_note_length();
    cur.skip_space();
    Token::new(TokenType::Rest, dir, vec![SValue::from_s(note_len)])
}

fn read_note_n(cur: &mut TokenCursor, song: &mut Song) -> Token {
    // note no
    let note_no = read_arg_value(cur, song);
    cur.skip_space();
    if cur.eq_char(',') {
        cur.next();
    }
    // length
    let note_len = cur.get_note_length();
    cur.skip_space();
    // qlen
    let qlen = if !cur.eq_char(',') {
        0
    } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    cur.skip_space();
    // velocity
    let vel = if !cur.eq_char(',') {
        -1
    } else {
        cur.next();
        cur.skip_space();
        if cur.eq_char('+') {
            cur.next();
        } // 現状 +/- を無視する (TODO)
        cur.get_int(-1)
    };
    cur.skip_space();
    // timing
    let timing = if !cur.eq_char(',') {
        isize::MIN
    } else {
        cur.next();
        cur.skip_space();
        if cur.eq_char('+') {
            cur.next();
        }
        cur.get_int(isize::MIN)
    };
    // Slur or Tie
    let mut slur = SValue::None;
    if cur.eq_char('&') {
        cur.next(); // skip &
        cur.skip_space();
        slur = SValue::Int(1);
    }
    Token::new(
        TokenType::NoteN,
        0,
        vec![
            note_no,
            SValue::from_s(note_len),
            SValue::from_i(qlen),
            SValue::from_i(vel),
            SValue::from_i(timing),
            slur,
        ],
    )
}

fn read_note(cur: &mut TokenCursor, ch: char) -> Token {
    // flag
    let mut note_flag = 0;
    let mut flag_natual = false;
    loop {
        match cur.peek_n(0) {
            '+' | '#' => {
                note_flag += 1;
                cur.next();
            }
            '-' => {
                note_flag -= 1;
                cur.next();
            }
            '*' => {
                cur.next();
                flag_natual = true;
            }
            _ => break,
        }
    }
    // length
    let note_len = cur.get_note_length();
    cur.skip_space();
    // qlen
    let qlen = if !cur.eq_char(',') {
        0
    } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    cur.skip_space();
    // veolocity
    let vel = if !cur.eq_char(',') {
        -1
    } else {
        cur.next();
        cur.skip_space();
        if cur.eq_char('+') {
            cur.next();
        } // 現状 +/- を無視する (TODO)
        cur.get_int(0)
    };
    cur.skip_space();
    // timing
    let timing = if !cur.eq_char(',') {
        isize::MIN
    } else {
        cur.next();
        cur.skip_space();
        cur.get_int(isize::MIN)
    };
    // octave
    let octabe = if !cur.eq_char(',') {
        -1
    } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    // Slur or Tie
    let mut slur = SValue::None;
    if cur.eq_char('&') {
        cur.next(); // skip &
        cur.skip_space();
        slur = SValue::Int(1);
    }
    Token::new(
        TokenType::Note,
        match ch {
            'c' => 0,
            'd' => 2,
            'e' => 4,
            'f' => 5,
            'g' => 7,
            'a' => 9,
            'b' => 11,
            _ => 0,
        },
        vec![
            SValue::from_i(note_flag),
            SValue::from_i(if flag_natual { 1 } else { 0 }),
            SValue::from_s(note_len),
            SValue::from_i(qlen),
            SValue::from_i(vel),
            SValue::from_i(timing),
            SValue::from_i(octabe),
            slur,
        ],
    )
}

#[cfg(test)]
mod tests {
    use crate::token::tokens_to_str;

    use super::*;
    #[test]
    fn test_lex1() {
        let mut song = Song::new();
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "cdefgab", 0)),
            "[Note,0][Note,2][Note,4][Note,5][Note,7][Note,9][Note,11]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "l4c", 0)),
            "[Length,0][Note,0]"
        );
        assert_eq!(&tokens_to_str(&lex(&mut song, "TR=1", 0)), "[Track,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TR(1)", 0)), "[Track,0]");
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "INT A=1;TR(A)", 0)),
            "[DefInt,0][Track,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "INT A=1;TR=A", 0)),
            "[DefInt,0][Track,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "COPYRIGHT{a}", 0)),
            "[MetaText,2]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "COPYRIGHT={a}", 0)),
            "[MetaText,2]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TimeSig=4,4", 0)),
            "[TimeSignature,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TimeSig=(4,4)", 0)),
            "[TimeSignature,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TimeSig(4,4)", 0)),
            "[TimeSignature,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TIME(1:1:0)", 0)),
            "[Time,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TIME=(1:1:0)", 0)),
            "[Time,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TIME(1:1:0)", 0)),
            "[Time,0]"
        );
        assert_eq!(&tokens_to_str(&lex(&mut song, "TIME=1:1:0", 0)), "[Time,0]");
    }
    #[test]
    fn test_lex_harmony() {
        let mut song = Song::new();
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "'dg'", 0)),
            "[HarmonyBegin,0][Note,2][Note,7][HarmonyEnd,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "'dg'^^^", 0)),
            "[HarmonyBegin,0][Note,2][Note,7][HarmonyEnd,0]"
        );
    }
    #[test]
    fn test_lex_rhythm_macro() {
        let mut song = Song::new();
        assert_eq!(&tokens_to_str(&lex(&mut song, "RHYTHM{b}", 0)), "[NoteN,0]");
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "RHYTHM{(Sub){b}}", 0)),
            "[Sub,0]"
        );
    }
    #[test]
    fn test_lex_cc() {
        let mut song = Song::new();
        assert_eq!(&tokens_to_str(&lex(&mut song, "P(10)", 0)), "[CtrlChange,10]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "M(10)", 0)), "[CtrlChange,1]");
    }
}
