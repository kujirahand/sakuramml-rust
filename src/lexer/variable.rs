//! lexer: 変数・ユーザー定義関数の読み取り
use super::*;

pub(super) fn read_def_user_function(cur: &mut SourceCursor, song: &mut Song) -> Token {
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
    // args parameters
    let mut arg_types: Vec<char> = vec![];
    let mut arg_names: Vec<String> = vec![];
    let mut arg_def_values: Vec<SValue> = vec![];
    // check args_str
    let mut acur = SourceCursor::from(&args_str);
    while !acur.is_eos() {
        acur.skip_space();
        // get name
        let mut type_sf = 'I';
        let mut def_v = SValue::from_i(0);
        let mut name = acur.get_word();
        if name.len() == 0 {
            break;
        }
        // get type
        if acur.eq_char(' ') {
            acur.skip_space(); // skip space
            let type_s = name;
            name = acur.get_word();
            if type_s == "Int" || type_s == "INT" || type_s == "I" {
                type_sf = 'I';
            } else if type_s == "Str" || type_s == "STR" || type_s == "S" {
                type_sf = 'S';
                def_v = SValue::from_str("");
            } else if type_s == "Array" || type_s == "ARRAY" || type_s == "A" {
                type_sf = 'A';
                def_v = SValue::from_int_array(vec![]);
            } else {
                let msg = format!("Invalid type: {}", type_s);
                return read_error_cmd(cur, song, &msg);
            }
        }
        // get def value
        acur.skip_space();
        if acur.eq_char('=') {
            acur.next();
            def_v = read_arg_value(&mut acur, song);
            // check def type
            match def_v {
                SValue::Int(_) => type_sf = 'I',
                SValue::Str(_, _) => type_sf = 'S',
                SValue::Array(_) => type_sf = 'A',
                _ => {}
            }
        }
        arg_names.push(name.clone());
        arg_types.push(type_sf);
        arg_def_values.push(def_v.clone());
        song.variables_insert(&name, def_v); // add name to local variables
        acur.skip_space();
        if acur.eq_char(',') {
            acur.next();
            continue;
        }
        break;
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
    let func_val = song
        .variables_get(&func_name)
        .unwrap_or(&SValue::new())
        .clone();
    let func_id = match func_val {
        SValue::UserFunc(func_id) => func_id,
        _ => {
            // system error to analyze function in preprocess
            read_error_cmd(
                cur,
                song,
                &format!("(System error) Define Function: {}", func_name),
            );
            0
        }
    };
    // register function to song.functions
    let mut func_obj = SFunction::new(&func_name, body_tok, func_id, lineno);
    func_obj.arg_names = arg_names;
    func_obj.arg_types = arg_types;
    func_obj.arg_def_values = arg_def_values;
    song.functions[func_id] = func_obj;
    Token::new_empty(&format!("DefineFunction::{}", func_name), lineno)
}

pub(super) fn check_variables(
    cur: &mut SourceCursor,
    song: &mut Song,
    cmd: String,
) -> Option<Token> {
    // increment variable?
    if cur.eq("++") {
        cur.next_n(2);
        return Some(Token::new_const(
            TokenType::ValueInc,
            1,
            Some(cmd),
            TokenValueType::VARIABLE,
        ));
    }
    if cur.eq("--") {
        cur.next_n(2);
        return Some(Token::new_const(
            TokenType::ValueInc,
            -1,
            Some(cmd),
            TokenValueType::VARIABLE,
        ));
    }
    // let?
    cur.skip_space();
    if cur.eq("=") {
        cur.next();
        cur.skip_space();
        // check reserved words
        if song.reserved_words.contains_key(&cmd) {
            let msg = format!(
                "{}: \"{}\"",
                song.get_message(MessageKind::ErrorDefineVariableIsReserved),
                cmd
            );
            return Some(read_error(cur, song, &msg));
        }
        // let str
        if cur.eq_char('{') {
            let body = cur.get_token_nest('{', '}');
            let value_token = Token::new_const(
                TokenType::ConstStr,
                body.len() as isize,
                Some(body),
                TokenValueType::STR,
            );
            // 後続の単独記述をMMLマクロとして字句解析できるよう、
            // Lexerには文字列型のプレースホルダーだけを登録する。
            // 実際の本文はLetVarトークンの実行時に代入する。
            song.variables_insert(&cmd, SValue::from_str(""));
            return Some(Token::new_data_tokens(
                TokenType::LetVar,
                0,
                vec![SValue::from_str(&cmd)],
                vec![value_token],
            ));
        }
        // let calc
        let body_tokens = read_calc_tokens(cur, song).unwrap_or(vec![]);
        let tok = Token::new_data_tokens(
            TokenType::LetVar,
            0,
            vec![SValue::from_str(&cmd)],
            body_tokens,
        );
        song.variables_insert(&cmd, SValue::None);
        return Some(tok);
    }
    // replace string
    else if cur.eq(".s(") {
        cur.next_n(2);
        let args = read_args_tokens(cur, song);
        let mut replace_tok = Token::new_tokens(TokenType::StrVarReplace, 0, args);
        replace_tok.value_s = Some(cmd);
        return Some(replace_tok);
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

pub(super) fn read_variables(
    cur: &mut SourceCursor,
    song: &mut Song,
    name: &str,
    sval: SValue,
) -> Token {
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
                let tok = Token::new(
                    TokenType::Value,
                    LEX_VALUE,
                    vec![SValue::from_s(format!("={}", name))],
                );
                return tok;
            }
        }
        SValue::UserFunc(func_id) => {
            return read_call_function(cur, song, func_id);
        }
        _ => {
            return Token::new_empty(&format!("Could not execute: {}", name), cur.line);
        }
    }
}

pub(super) fn read_call_function(cur: &mut SourceCursor, song: &mut Song, func_id: usize) -> Token {
    cur.skip_space();
    let args: Vec<Token> = read_args_tokens(cur, song);
    // Create token with func_id only in tag field (not value_i to avoid duplication)
    let mut call_func_tok = Token::new(TokenType::CallUserFunction, 0, vec![]);
    call_func_tok.tag = func_id as isize; // func_id is stored in tag field for runtime use
    call_func_tok.children = Some(args);
    call_func_tok
}

pub(super) fn read_def_var(
    cur: &mut SourceCursor,
    song: &mut Song,
    value_type: TokenValueType,
) -> Token {
    cur.skip_space();
    let var_name = cur.get_word();
    if var_name == "" {
        song.add_log(format!(
            "[ERROR]({}): Variable's name should be Upper case like \"Test\".",
            cur.line
        ));
        return Token::new_empty("Failed to def INT", cur.line);
    }
    // check reserved words
    if song.reserved_words.contains_key(&var_name) {
        let msg = format!(
            "{}: \"{}\"",
            song.get_message(MessageKind::ErrorDefineVariableIsReserved),
            var_name
        );
        read_error(cur, song, &msg);
        return Token::new_empty("Failed to def INT", cur.line);
    }
    cur.skip_space();
    // 値を得る
    let tok = match value_type {
        TokenValueType::INT => {
            let mut val_tokens = None;
            if cur.eq_char('=') {
                // 代入文がある場合
                cur.next(); // skip '='
                val_tokens = read_calc_tokens(cur, song);
            }
            // register variable
            song.variables_insert(&var_name, SValue::from_i(0));
            // token
            Token::new_variable(TokenType::DefInt, var_name, val_tokens)
        }
        TokenValueType::STR => {
            // 初期値に空をセット
            let mut val_tokens = None;
            if cur.eq_char('=') {
                // 代入文がある場合
                cur.next(); // skip '='
                val_tokens = read_calc_tokens(cur, song);
            }
            // register variable
            song.variables_insert(&var_name, SValue::from_str(""));
            // token
            Token::new_variable(TokenType::DefStr, var_name, val_tokens)
        }
        TokenValueType::ARRAY => {
            let mut val_tokens = None;
            if cur.eq_char('=') {
                // 代入文がある場合
                cur.next(); // skip '='
                val_tokens = read_calc_tokens(cur, song);
            }
            // register variable
            song.variables_insert(&var_name, SValue::Array(vec![]));
            // token
            Token::new_variable(TokenType::DefArray, var_name, val_tokens)
        }
        _ => {
            song.add_log(format!("[ERROR]({}): Invalid value type.", cur.line));
            return Token::new_empty("Failed to def INT", cur.line);
        }
    };
    tok
}
