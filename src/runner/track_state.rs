//! runner: トラックの演奏パラメータ(音長・オクターブ・音量・ゲート・タイミング)の設定
use super::*;

/// 音符属性(v/q/t/o/l)の先行指定の状態を取り出す
fn note_param<'a>(song: &'a mut Song, target: isize) -> &'a mut NoteParam {
    let trk = &mut song.tracks[song.cur_track];
    match target {
        NOTE_PARAM_V => &mut trk.v_opt,
        NOTE_PARAM_Q => &mut trk.q_opt,
        NOTE_PARAM_T => &mut trk.t_opt,
        NOTE_PARAM_O => &mut trk.o_opt,
        _ => &mut trk.l_opt,
    }
}

/// 音長の指定 (l)
pub(super) fn exec_length(song: &mut Song, t: &Token) {
    trk!(song).l_opt.clear_reserve();
    trk!(song).length = calc_length(&t.data[0].to_s(), song.timebase, song.timebase);
}

/// オクターブの指定 (o)
pub(super) fn exec_octave(song: &mut Song, t: &Token) {
    trk!(song).o_opt.clear_reserve();
    let value = t
        .data
        .first()
        .map(|v| var_extract(v, song).to_i())
        .unwrap_or(t.value_i);
    trk!(song).octave = value_range(0, value, 10);
}

/// オクターブの相対変更 (> <)
pub(super) fn exec_octave_rel(song: &mut Song, t: &Token) {
    trk!(song).octave = value_range(0, trk!(song).octave + t.value_i, 10);
}

/// 1音だけオクターブを変更する (` ")
pub(super) fn exec_octave_once(song: &mut Song, t: &Token) {
    trk!(song).octave = value_range(0, trk!(song).octave + t.value_i, 10);
    song.flags.octave_once += t.value_i;
}

/// ベロシティの指定 (v)
pub(super) fn exec_velocity(song: &mut Song, t: &Token) {
    let value = t
        .data
        .first()
        .map(|v| var_extract(v, song).to_i())
        .unwrap_or(t.value_i);
    let ino = t.data.get(1).map(|v| v.to_i()).unwrap_or(-1);
    if ino >= 0 {
        let index = ino as usize;
        let velocity = value_range(-127, value, 127);
        trk!(song).set_v_sub(index, velocity);
    } else {
        trk!(song).v_opt.clear_reserve();
        let max = trk!(song).v_opt.max_or(127);
        trk!(song).velocity = value_range(0, value, max);
    }
}

/// ベロシティの相対変更 ()( )
pub(super) fn exec_velocity_rel(song: &mut Song, t: &Token) {
    let max = trk!(song).v_opt.max_or(127);
    trk!(song).velocity = value_range(0, trk!(song).velocity + (song.v_add * t.value_i), max);
}

/// ゲートの指定 (q)
pub(super) fn exec_qlen(song: &mut Song, t: &Token) {
    trk!(song).q_opt.clear_reserve();
    let value = t
        .data
        .first()
        .map(|v| var_extract(v, song).to_i())
        .unwrap_or(t.value_i);
    // q%n --- ステップ単位の指定 (#127)
    // 割合ではないので、0〜100(.Max)の範囲に丸めない
    let is_step = t.data.get(1).map(|v| v.to_i()).unwrap_or(0) != 0;
    if is_step {
        // q%-n は、現在のqの値からの相対指定
        // (オリジナル(Pascal版)の SetNoteInfo.subSoutai と同じ動作)
        let value = if value < 0 {
            trk!(song).qlen.saturating_add(value).max(0)
        } else {
            value
        };
        trk!(song).qlen_is_step = true;
        trk!(song).qlen = value;
        return;
    }
    let max = trk!(song).q_opt.max_or(100);
    trk!(song).qlen_is_step = false;
    trk!(song).qlen = value_range(0, value, max);
}

/// ゲートの相対変更
pub(super) fn exec_qlen_rel(song: &mut Song, t: &Token) {
    trk!(song).qlen = trk!(song).qlen + (song.q_add * t.value_i);
}

/// 発音タイミングの指定 (t)
pub(super) fn exec_timing(song: &mut Song, t: &Token) {
    trk!(song).t_opt.clear_reserve();
    trk!(song).timing = t
        .data
        .first()
        .map(|v| var_extract(v, song).to_i())
        .unwrap_or(t.value_i);
}

/// 音符属性のランダム変化 (.Random)
pub(super) fn exec_note_param_random(song: &mut Song, t: &Token, target: isize) {
    let random = var_extract(&t.data[0], song).to_i();
    let index = t.data.get(1).map(|value| value.to_i()).unwrap_or(-1);
    // サブベロシティ (v__n.Random)
    if target == NOTE_PARAM_V && index >= 0 {
        trk!(song).set_v_sub_random(index as usize, random);
        return;
    }
    note_param(song, target).random = random;
}

/// 時間経過による音符属性の変化 (.onTime)
pub(super) fn exec_note_param_on_time(song: &mut Song, t: &Token, target: isize) {
    let values = exec_int_args(song, t);
    let index = t.data.first().map(|value| value.to_i()).unwrap_or(-1);
    if target == NOTE_PARAM_V && index >= 0 {
        trk!(song).set_v_sub_on_time(index as usize, values);
        return;
    }
    let timepos = trk!(song).timepos;
    note_param(song, target).set_on_time(timepos, values);
}

/// 一定時間ごとの音符属性の変化 (.onCycle) --- (ステップ値, 値1, 値2, ...)
pub(super) fn exec_note_param_on_cycle(song: &mut Song, t: &Token, target: isize) {
    let values = exec_int_args(song, t);
    let index = t.data.first().map(|value| value.to_i()).unwrap_or(-1);
    if values.len() < 2 {
        runtime_error(song, ".onCycle needs (step, v1, v2, ...)");
        return;
    }
    if target == NOTE_PARAM_V && index >= 0 {
        trk!(song).set_v_sub_on_cycle(index as usize, values);
        return;
    }
    let timepos = trk!(song).timepos;
    note_param(song, target).set_on_cycle(timepos, values);
}

/// 音符ごとの音符属性の変化 (.onNote)
pub(super) fn exec_note_param_on_note(song: &mut Song, t: &Token, target: isize) {
    let values = exec_int_args(song, t);
    let index = t.data.first().map(|value| value.to_i()).unwrap_or(-1);
    if target == NOTE_PARAM_V && index >= 0 {
        trk!(song).set_v_sub_on_note(index as usize, values);
        return;
    }
    note_param(song, target).set_on_note(values);
}

/// 音符属性の値の範囲指定 (.Range)
pub(super) fn exec_note_param_range(song: &mut Song, t: &Token) {
    let target = t.value_i;
    let args = exec_int_args(song, t);
    if args.len() < 2 {
        runtime_error(song, ".Range needs (low, high)");
        return;
    }
    note_param(song, target).range = Some((args[0], args[1]));
}

/// 音符属性の先行指定の遅延 (.Delay)
pub(super) fn exec_note_param_delay(song: &mut Song, t: &Token) {
    let target = t.value_i;
    let v = var_extract(&t.data[0], song).to_i();
    note_param(song, target).delay = v;
}

/// 音符属性の .onNote をくり返すかどうか (.Repeat)
pub(super) fn exec_note_param_repeat(song: &mut Song, t: &Token) {
    let target = t.value_i;
    let on = var_extract(&t.data[0], song).to_i() != 0;
    let param = note_param(song, target);
    param.repeat = on;
    // すでに予約されている .onNote にも反映する
    param.on_note_is_cycle = on;
}

/// v/q の値の上限を変更する (.Max)
pub(super) fn exec_note_param_max(song: &mut Song, t: &Token) {
    let target = t.value_i;
    let v = var_extract(&t.data[0], song).to_i();
    note_param(song, target).max = v;
}

/// タイ(&)の動作モードの指定
pub(super) fn exec_tie_mode(song: &mut Song, t: &Token) {
    let args = exec_args(song, t.children.as_deref().unwrap_or(&[]));
    if args.len() >= 1 {
        trk!(song).tie_mode = TieMode::from_i(var_extract(&args[0], song).to_i());
    }
    if args.len() >= 2 {
        trk!(song).tie_value = var_extract(&args[1], song).to_i();
    }
}
