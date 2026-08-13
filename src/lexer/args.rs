//! lexer: 引数の読み取り
use super::*;

/// read const int value
pub(super) fn read_arg_const_int(cur: &mut SourceCursor) -> Option<isize> {
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    if cur.eq_char('(') {
        cur.next();
    }
    cur.skip_space();
    let value: isize;
    let ch = cur.peek_n(0);
    match ch {
        '-' | '0'..='9' | '$' => {
            value = cur.get_int(0);
        }
        _ => return None,
    }
    cur.skip_space();
    if cur.eq_char(')') {
        cur.next();
    }
    Some(value)
}

pub(super) fn read_arg_value(cur: &mut SourceCursor, song: &mut Song) -> SValue {
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
        '-' | '0'..='9' | '$' => {
            let v = cur.get_int(0);
            SValue::from_i(v)
        }
        '=' => {
            cur.next(); // skip =
            read_arg_value(cur, song)
        }
        '(' => {
            cur.next(); // skip (
            let mut args = vec![];
            let mut flag_array = false;
            loop {
                let v = read_arg_value(cur, song);
                args.push(v);
                cur.skip_space();
                if cur.eq_char(',') {
                    cur.next();
                    flag_array = true;
                    continue;
                }
                break;
            }
            if cur.eq_char(')') {
                cur.next();
            }
            if flag_array {
                SValue::from_vec(args)
            } else {
                args.into_iter().next().unwrap_or(SValue::None)
            }
        }
        '{' => {
            let s = cur.get_token_nest('{', '}');
            SValue::from_s(s)
        }
        _ => SValue::None,
    }
}

pub(super) fn read_arg_value_int_array(cur: &mut SourceCursor, song: &mut Song) -> SValue {
    let mut a: Vec<SValue> = vec![];
    loop {
        cur.skip_space();
        // println!("@@@read_arg_value_int_array:{}", cur.peek_n(0));
        let v = read_arg_value(cur, song);
        match v {
            SValue::None => { break; }
            SValue::Array(av) => {
                a.extend(av);
            },
            _ => {
                a.push(v)
            }
        }
        cur.skip_space();
        if !cur.eq_char(',') {
            break;
        }
        cur.next(); // skip ,
    }
    SValue::from_vec(a)
}

pub(super) fn read_arg_int_array(cur: &mut SourceCursor, song: &mut Song) -> SValue {
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

/// on/off の指定を読み取る (.Repeat(on) など)
/// on/off のほか 1/0 などの数値も受け付ける。省略時は on とみなす
pub(super) fn read_arg_on_off(cur: &mut SourceCursor, song: &mut Song) -> SValue {
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    let has_paren = cur.eq_char('(');
    if has_paren {
        cur.next();
    }
    cur.skip_space();
    let result = match cur.peek_n(0) {
        'a'..='z' | 'A'..='Z' => {
            let word = cur.get_word();
            match word.to_lowercase().as_str() {
                "off" | "false" | "no" => SValue::from_b(false),
                "on" | "true" | "yes" => SValue::from_b(true),
                _ => SValue::from_s(format!("={}", word)),
            }
        }
        '-' | '0'..='9' | '$' => SValue::from_b(cur.get_int(0) != 0),
        _ => {
            let value = read_arg_value(cur, song);
            if value.is_none() { SValue::from_b(true) } else { value }
        }
    };
    cur.skip_space();
    if has_paren && cur.eq_char(')') {
        cur.next();
    }
    result
}

pub(super) fn read_args_tokens(cur: &mut SourceCursor, song: &mut Song) -> Vec<Token> {
    cur.skip_space();
    let skip_paren = if cur.eq_char('(') {
        cur.next(); // skip '('
        true
    } else { false };

    let mut tokens = vec![];
    loop {
        cur.skip_space();
        let sub_tokens = read_calc_tokens(cur, song).unwrap_or(vec![]);
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
