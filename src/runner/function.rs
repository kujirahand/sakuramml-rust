//! runner: ユーザー定義関数・システム関数・変数展開の実行
use super::*;

pub(super) fn exec_userfunc_or_array_or_macro(song: &mut Song, t: &Token) -> bool {
    // check value is array?
    if t.data.len() > 0 {
        let name = t.data[0].to_s();
        let var = song.variables_get(&name).unwrap_or(&SValue::None).clone();
        match var {
            // is Array
            SValue::Array(a) => {
                // get arg
                let args_tokens = t.children.clone().unwrap();
                let args: Vec<SValue> = exec_args(song, &args_tokens);
                if args.len() == 0 {
                    runtime_error(song, &format!("get Array({}) element needs arguments", name));
                    return false;
                }
                let index = args[0].to_i() as usize;
                if a.len() <= index {
                    runtime_error(song, &format!("Array({}) index out of range", name));
                    return false;
                }
                let v = a[index].clone();
                song.stack.push(v);
                return true;
            },
            // is String macro >>> exec string
            SValue::Str(src, _) => {
                // get arg
                let args_tokens = t.children.clone().unwrap();
                let args: Vec<SValue> = exec_args(song, &args_tokens);
                if args.len() == 0 {
                    runtime_error(song, &format!("get String({}) element needs arguments", name));
                    return false;
                }
                // replace string
                let mut s = src.clone();
                for (i, v) in args.iter().enumerate() {
                    let varname = format!("#?{}", i + 1);
                    s = s.replace(&varname, &v.to_s());
                }
                let tokens = lex(song, &s, t.lineno);
                return exec(song, &tokens);
            },
            _ => {}
        }
    }
    // check func_id
    let func_id = t.tag as usize;
    if song.functions.len() <= func_id {
        runtime_error(song, &format!("broken func_id={} in exec_call_user_function", func_id));
        return false;
    }
    // println!("call_user_function[{}]={}::{:?}", func_id, song.functions[func_id].name, song.functions[func_id].arg_def_values);
    
    song.variables_stack_push();
    // eval args
    let args_tokens = t.children.clone().unwrap();
    let args: Vec<SValue> = exec_args(song, &args_tokens);
    // set local variables
    for i in 0..song.functions[func_id].arg_names.len() {
        let varname = &song.functions[func_id].arg_names[i].clone();
        let mut v: SValue = if i < args.len() { args[i].clone() } else { SValue::None };
        v = match v {
            SValue::None => song.functions[func_id].arg_def_values[i].clone(),
            _ => v,
        };
        song.variables_insert(varname, v);
    }
    // eval function
    let tokens = song.functions[func_id].tokens.clone();
    let tmp_break_flag = song.flags.break_flag;
    // println!("func_body={:?}", tokens);
    let eval_result = exec(song, &tokens);
    song.flags.break_flag = tmp_break_flag;
    let vars = song.variables_stack_pop();
    if song.flags.function_needs_return_value {
        let return_val = vars.get("Result");
        song.stack.push(return_val.unwrap_or(&SValue::None).clone());
    }
    eval_result
}

pub(super) fn exec_sys_function(song: &mut Song, t: &Token) -> bool {
    let args_tokens = t.children.clone().unwrap_or(vec![]);
    let args:Vec<SValue> = exec_args(song, &args_tokens);
    let func_name = if t.data.len() > 0 { t.data[0].to_s() } else { "".to_string() };
    // is user function?
    let func_val = song.variables_get(&func_name).unwrap_or(&SValue::new()).clone();
    match func_val {
        SValue::UserFunc(_func_id) => {
            if exec_userfunc_or_array_or_macro(song, t) { return true; }
        },
        _ => {}, // maybe system function
    }
    //
    // todo: https://sakuramml.com/wiki/index.php?%E7%B5%84%E3%81%BF%E8%BE%BC%E3%81%BF%E9%96%A2%E6%95%B0
    //
    // 参照できるシステム関数
    if let Some(cb) = song.calc_functions.get(&func_name) {
        let result = cb(song, args);
        song.stack.push(result);
    } else {
        // macro ("=var_name")
        let func_name2 = if func_name.len() >= 2 { func_name[1..].to_string() } else { func_name };
        let args = t.children.clone().unwrap_or(vec![]);
        let args = exec_args(song, &args);
        let val = song.variables_get(&func_name2).unwrap_or(&SValue::new()).clone();
        let mut val_s = val.to_s();
        for (index, arg) in args.iter().enumerate() {
            let macro_n = format!("#?{}", index+1);
            let target = arg.clone().to_s();
            val_s = val_s.replace(&macro_n, &target);
        }
        // println!("macro={}//{:?}", val_s, t);
        if song.flags.function_needs_return_value {
            song.stack.push(SValue::from_s(val_s));
        } else {
            // exec macro
            let tokens = lex(song, &val_s, t.lineno);
            exec(song, &tokens);
        }
    }
    true
}

pub(super) fn get_system_value(cmd: &str, song: &Song) -> Option<SValue> {
    // <SYSTEM_REF>
    if cmd == "TR" || cmd == "TRACK" || cmd == "Track" { // @ get current track no - 現在のトラック番号を得る
        let tr = song.cur_track as isize;
        return Some(SValue::from_i(tr));
    }
    if cmd == "CH" || cmd == "CHANNEL" { // @ get current channel no - 現在のチャンネル番号を得る
        let ch = trk!(song).channel + 1; // range: 1-16
        return Some(SValue::from_i(ch));
    }
    if cmd == "TIME" || cmd == "Time" || cmd == "TIMEPOS" || cmd == "TIMEPTR" { // @ get time posision - 現在のタイムポインタ値を得る
        let v = trk!(song).timepos;
        return Some(SValue::from_i(v));
    }
    if cmd == "TEMPO" || cmd == "Tempo" || cmd == "BPM" { // @ get tempo - 現在のテンポ値を得る
        let v = song.tempo;
        return Some(SValue::from_i(v));
    }
    if cmd == "KEY" || cmd == "KEY_SHIFT" { // @ get key shift - 現在のキーシフト値を得る
        let v = song.key_shift;
        return Some(SValue::from_i(v));
    }
    if cmd == "TR_KEY" || cmd == "TrackKey" { // @ get track key shift - 現在のトラックごとのキーシフト値を得る
        let v = trk!(song).track_key;
        return Some(SValue::from_i(v));
    }
    if cmd == "TIMEBASE" || cmd == "Timebase" { // @ get timebase - 現在のタイムベース値を得る
        let v = song.timebase;
        return Some(SValue::from_i(v));
    }
    if cmd == "l" { // @ get length - 現在のlの値を得る
        let v = trk!(song).length;
        return Some(SValue::from_i(v));
    }
    if cmd == "v" { // @ get velocity - 現在のvの値を得る
        let v = trk!(song).velocity;
        return Some(SValue::from_i(v));
    }
    if cmd == "q" { // @ get gate rate - 現在のqの値を得る
        let v = trk!(song).qlen;
        return Some(SValue::from_i(v));
    }
    if cmd == "o" { // @ get octave rate - 現在のoの値を得る
        let v = trk!(song).octave;
        return Some(SValue::from_i(v));
    }
    // </SYSTEM_REF>
    None
}

pub(super) fn var_extract(val: &SValue, song: &mut Song) -> SValue {
    match val {
        // String
        SValue::Str(s, _) => {
            if s.starts_with('=') && s.len() >= 2 {
                let key = &s[1..];
                match song.variables_get(key) {
                    Some(v) => v.clone(),
                    None => {
                        match get_system_value(key, song) {
                            Some(v) => return v,
                            None => {
                                let err_msg = format!("[WARN]({}) Undefined: {}", song.lineno, key);
                                song.add_log(err_msg);
                                SValue::None
                            },
                        }
                    }
                }
            } else {
                SValue::from_str(&s)
            }
        },
        // Other value
        _ => val.clone(),
    }
}
