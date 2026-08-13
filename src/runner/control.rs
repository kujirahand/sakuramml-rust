//! runner: 条件分岐とループの実行
use super::*;

pub(super) fn exec_if(song: &mut Song, t: &Token) -> bool {
    let children = match &t.children {
        Some(tokens) => tokens,
        None => return false,
    };
    if children.len() < 3 {
        return false;
    }
    let cond_token = &children[0];
    let true_token = &children[1];
    let false_token = &children[2];
    // eval cond
    let cond = cond_token.children.clone().unwrap();
    let cond_val = exec_value(song, &cond);
    // exec true or false
    if cond_val.to_i() != 0 {
        let tokens = true_token.children.clone().unwrap();
        exec(song, &tokens);
    } else {
        let tokens = false_token.children.clone().unwrap();
        exec(song, &tokens);
    }
    true
}

pub(super) fn exec_while(song: &mut Song, t: &Token) -> bool {
    let children = match &t.children {
        Some(tokens) => tokens,
        None => return false,
    };
    if children.len() < 2 {
        return false;
    }
    let cond_token = &children[0];
    let body_token = &children[1];
    let mut counter = 0;
    // loop
    loop {
        // eval cond
        let cond = cond_token.children.clone().unwrap();
        let cond_val = exec_value(song, &cond);
        if cond_val.to_b() == false {
            break;
        }
        // exec body
        let body = body_token.children.clone().unwrap();
        exec(song, &body);
        if song.event_limit_exceeded() {
            break;
        }
        // check counter
        counter += 1;
        if counter > song.flags.max_loop {
            song.add_log(format!(
                "[ERROR]({}) {} WHILE(>{})",
                t.lineno,
                song.get_message(MessageKind::LoopTooManyTimes),
                song.flags.max_loop
            ));
            break;
        }
        // check break flag
        match song.flags.break_flag {
            1 => {
                song.flags.break_flag = 0;
                break;
            }
            2 => {
                song.flags.break_flag = 0;
                continue;
            }
            3 => {
                break;
            }
            _ => {}
        }
    }
    true
}

pub(super) fn exec_for(song: &mut Song, t: &Token) -> bool {
    let children = match &t.children {
        Some(tokens) => tokens,
        None => return false,
    };
    if children.len() < 4 {
        return false;
    }
    let init_token = &children[0];
    let cond_token = &children[1];
    let inc_token = &children[2];
    let body_token = &children[3];
    // eval init
    let init = init_token.children.clone().unwrap();
    exec(song, &init);
    let mut counter = 0;
    // loop
    loop {
        // eval cond
        let cond = cond_token.children.clone().unwrap();
        let cond_val = exec_value(song, &cond);
        if cond_val.to_b() == false {
            break;
        }
        // exec body
        let body = body_token.children.clone().unwrap();
        exec(song, &body);
        if song.event_limit_exceeded() {
            break;
        }
        // check loop counter
        counter += 1;
        if counter > song.flags.max_loop {
            song.add_log(format!(
                "[ERROR]({}) {} FOR(>{})",
                t.lineno,
                song.get_message(MessageKind::LoopTooManyTimes),
                song.flags.max_loop
            ));
            break;
        }
        // inc
        let inc_tokens = inc_token.children.clone().unwrap();
        // check break or continue
        if song.flags.break_flag == 1 {
            // break
            song.flags.break_flag = 0;
            break;
        }
        if song.flags.break_flag == 2 {
            // continue
            song.flags.break_flag = 0;
            exec(song, &inc_tokens); // eval inc
            continue;
        }
        // eval inc
        exec(song, &inc_tokens); // eval inc
    }
    true
}

/// Return文 - 戻り値を設定する (実行の中断は呼び出し側で行う)
pub(super) fn exec_return(song: &mut Song, t: &Token) {
    let val_tokens = t.children.clone().unwrap();
    let val = exec_value(song, &val_tokens);
    song.variables_insert("Result", val);
}
