//! runner: 変数の定義・参照と値の計算
use super::*;

/// Int変数の定義
pub(super) fn exec_def_int(song: &mut Song, t: &Token) {
    match &t.value_s {
        None => { runtime_error(song, "[SYSTEM ERROR][DefInt] variable name is empty"); return; },
        Some(var_name) => {
            let val = exec_value(song, &t.children.clone().unwrap_or(vec![]));
            if val.is_array() {
                let msg = format!("{}: {}",
                    song.get_message(MessageKind::ErrorTypeMismatch),
                    var_name);
                runtime_error(song, &msg);
            }
            song.variables_insert(var_name, val);
        }
    }
}

/// Str変数の定義
pub(super) fn exec_def_str(song: &mut Song, t: &Token) {
    match &t.value_s {
        None => { runtime_error(song, "[SYSTEM ERROR][DefStr] variable name is empty"); return; },
        Some(var_name) => {
            let val = exec_value(song, &t.children.clone().unwrap_or(vec![]));
            song.variables_insert(var_name, val);
        }
    }
}

/// Array変数の定義
pub(super) fn exec_def_array(song: &mut Song, t: &Token) {
    match &t.value_s {
        None => { runtime_error(song, "[SYSTEM ERROR][DefArray] variable name is empty"); return; },
        Some(var_name) => {
            let val = exec_value(song, &t.children.clone().unwrap_or(vec![]));
            song.variables_insert(var_name, val);
        }
    }
}

/// 変数の値を取得してスタックに積む
pub(super) fn exec_get_variable(song: &mut Song, t: &Token) {
    match &t.value_s {
        None => {
            runtime_error(song, "[SYSTEM ERROR][GetVariable] variable name is empty");
            return;
        },
        Some(var_name) => {
            // get variable's value
            let val = song.variables_get(&var_name);
            // println!("GetVariable: {}={:?}", var_name, vals);
            let val = match val {
                Some(v) => v.clone(),
                None => {
                    match get_system_value(var_name, &song) {
                        Some(v) => v,
                        None => SValue::None,
                    }
                }
            };
            song.stack.push(val);
        }
    }
}

/// 変数への代入
pub(super) fn exec_let_var(song: &mut Song, t: &Token) {
    let var_key = t.data[0].to_s();
    let val_tokens = t.children.clone().unwrap_or(vec![]);
    let val = exec_value(song, &val_tokens);
    song.variables_insert(&var_key, val);
}

/// 文字列変数の置換
pub(super) fn exec_str_var_replace(song: &mut Song, t: &Token) {
    let var_key = t.value_s.clone().unwrap_or(String::from("ERROR"));
    let args = exec_args(song, &t.children.clone().unwrap_or(vec![]));
    if args.len() >= 2 {
        let mut val_s = song.variables_get(&var_key).unwrap_or(&SValue::None).to_s();
        val_s = val_s.replace(&args[0].to_s(), &args[1].to_s());
        song.variables_insert(&var_key, SValue::from_s(val_s));
    }
}

/// 変数のインクリメント
pub(super) fn exec_value_inc(song: &mut Song, t: &Token) {
    let varname = t.value_s.clone().unwrap_or(String::new());
    let val_inc = t.value_i;
    let val = song.variables_get(&varname).unwrap_or(&SValue::Int(0));
    song.variables_insert(&varname, SValue::from_i(val.to_i() + val_inc));
    // let val = song.variables_get(&varname).unwrap_or(&SValue::Int(0));
    // println!("inc={}={}", varname, val.to_i());
}

/// 整数定数をスタックに積む
pub(super) fn exec_const_int(song: &mut Song, t: &Token) {
    song.stack.push(SValue::from_i(t.value_i));
}

/// 文字列定数をスタックに積む
pub(super) fn exec_const_str(song: &mut Song, t: &Token) {
    song.stack.push(SValue::from_s(t.value_s.clone().unwrap_or(String::new())));
}

/// 配列リテラルの生成
pub(super) fn exec_make_array(song: &mut Song, t: &Token) {
    match &t.children {
        None => {
            song.stack.push(SValue::Array(vec![]));
            return;
        },
        Some(tokens) => {
            let mut a: Vec<SValue> = vec![];
            for tok in tokens {
                let v = exec_value(song, &vec![tok.clone()]);
                a.push(v);
            }
            song.stack.push(SValue::Array(a));
        }
    }
}

/// 値の展開 (変数参照・関数呼び出し)
pub(super) fn exec_value_token(song: &mut Song, t: &Token) {
    // extract value
    // t.value_i ... (ex) LEX_VALUE (lexer.rs) 計算の時に使う
    // t.data ... (ex) [SValue::S("=A")]
    // t.tag ... 関数管理に使う (0: 値 / 1以上: 関数)
    // t.value_type ... 値の種類 tokens::VALUE_XXXX
    // check is variable?
    let val = match t.value_type {
        TokenValueType::VARIABLE => var_extract(&t.data[0], song),
        _ => {
            if t.tag == 0 && t.data.len() > 0 {
                // exec value
                let v = var_extract(&t.data[0], song);
                let vs = v.to_s().clone();
                // println!("lex={:?}", vs);
                let tokens = lex(song, &vs, t.lineno);
                exec(song, &tokens);
                song.stack.pop().unwrap_or(SValue::None)
            } else {
                // user function or system function ref
                exec_sys_function(song, t);
                song.stack.pop().unwrap_or(SValue::None)
            }
        },
    };
    if song.flags.function_needs_return_value {
        song.stack.push(val);
    }
}

/// 計算木の実行
pub(super) fn exec_calc_tree(song: &mut Song, t: &Token) {
    if t.operator_flag == '\0' { // dummy calc
        match &t.children {
            Some(tokens) => {
                exec(song, tokens);
            },
            None => {},
        }
        return;
    }
    // get flag char
    let flag = t.operator_flag;
    let values = exec_args(song, t.children.as_ref().unwrap_or(&vec![]));
    // only 1 value
    if flag == '!' { // flag "!(val)"
        let v = if values.len() >= 1 { values[0].to_b() } else { false };
        song.stack.push(SValue::from_b(!v));
        return;
    }
    // 2 values
    // println!("[calc_tree]{}({:?})", flag, values);
    let a = if values.len() >= 1 { values[0].clone() } else { SValue::None };
    let b = if values.len() >= 2 { values[1].clone() } else { SValue::None };
    let mut c = SValue::None;
    match flag {
        '(' => c = a.clone(), // nop
        '&' => c = SValue::from_b(a.to_b() && b.to_b()), // logical and
        '|' => c = SValue::from_b(a.to_b() || b.to_b()), // logical or
        '=' => c = SValue::from_b(a.eq(b)),
        '≠' => c = SValue::from_b(a.ne(b)), // !=
        '>' => c = SValue::from_b(a.gt(b)),
        '≧' => c = SValue::from_b(a.gteq(b)),
        '<' => c = SValue::from_b(a.lt(b)),
        '≦' => c = SValue::from_b(a.lteq(b)),
        '+' => c = a.add(b),
        '-' => c = SValue::from_i(a.to_i() - b.to_i()),
        '*' => c = SValue::from_i(a.to_i() * b.to_i()),
        '/' => c = a.div(b),
        '%' => c = SValue::from_i(a.to_i() % b.to_i()),
        _ => {
            song.add_log(String::from("[Calc] unknown flag"));
        }
    }
    song.stack.push(c);
}
