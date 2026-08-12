//! runner: トラックの演奏パラメータ(音長・オクターブ・音量・ゲート・タイミング)の設定
use super::*;

/// 音長の指定 (l)
pub(super) fn exec_length(song: &mut Song, t: &Token) {
    trk!(song).l_on_note = None;
    trk!(song).length = calc_length(&t.data[0].to_s(), song.timebase, song.timebase);
}

/// オクターブの指定 (o)
pub(super) fn exec_octave(song: &mut Song, t: &Token) {
    trk!(song).o_on_note = None;
    trk!(song).octave = value_range(0, t.value_i, 10);
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
    let ino = t.data[0].to_i();
    if ino >= 0 {
        let index = ino as usize;
        let velocity = value_range(-127, t.value_i, 127);
        trk!(song).set_v_sub(index, velocity);
    } else {
        trk!(song).v_on_note = None;
        trk!(song).v_on_time = None;
        trk!(song).velocity = value_range(0, t.value_i, 127);
    }
}

/// ベロシティの相対変更 ()( )
pub(super) fn exec_velocity_rel(song: &mut Song, t: &Token) {
    trk!(song).velocity = value_range(0, trk!(song).velocity + (song.v_add * t.value_i), 127);
}

/// ゲートの指定 (q)
pub(super) fn exec_qlen(song: &mut Song, t: &Token) {
    trk!(song).q_on_note = None;
    trk!(song).qlen = value_range(0, t.value_i, 100);
    trk!(song).q_on_note = None;
}

/// ゲートの相対変更
pub(super) fn exec_qlen_rel(song: &mut Song, t: &Token) {
    trk!(song).qlen = trk!(song).qlen + (song.q_add * t.value_i);
}

/// 発音タイミングの指定 (t)
pub(super) fn exec_timing(song: &mut Song, t: &Token) {
    trk!(song).t_on_note = None;
    trk!(song).timing = t.value_i;
    trk!(song).t_on_note = None;
}

/// オクターブのランダム変化
pub(super) fn exec_octave_random(song: &mut Song, t: &Token) {
    trk!(song).o_rand = var_extract(&t.data[0], song).to_i();
}

/// ベロシティのランダム変化
pub(super) fn exec_velocity_random(song: &mut Song, t: &Token) {
    let random = var_extract(&t.data[0], song).to_i();
    let index = t.data.get(1).map(|value| value.to_i()).unwrap_or(-1);
    if index >= 0 {
        trk!(song).set_v_sub_random(index as usize, random);
    } else {
        trk!(song).v_rand = random;
    }
}

/// タイミングのランダム変化
pub(super) fn exec_timing_random(song: &mut Song, t: &Token) {
    trk!(song).t_rand = var_extract(&t.data[0], song).to_i();
}

/// ゲートのランダム変化
pub(super) fn exec_qlen_random(song: &mut Song, t: &Token) {
    trk!(song).q_rand = var_extract(&t.data[0], song).to_i();
}

/// 時間経過によるベロシティ変化
pub(super) fn exec_velocity_on_time(song: &mut Song, t: &Token) {
    let values = t.data[0].to_int_array();
    let index = t.data.get(1).map(|value| value.to_i()).unwrap_or(-1);
    if index >= 0 {
        trk!(song).set_v_sub_on_time(index as usize, values);
    } else {
        trk!(song).v_on_note = None;
        trk!(song).v_on_time_start = trk!(song).timepos;
        trk!(song).v_on_time = Some(values);
    }
}

/// 音符ごとのベロシティ変化 (is_cycle=trueで繰り返し)
pub(super) fn exec_velocity_on_note(song: &mut Song, t: &Token, is_cycle: bool) {
    let values = t.data[0].to_int_array();
    let index = t.data.get(1).map(|value| value.to_i()).unwrap_or(-1);
    if index >= 0 {
        trk!(song).set_v_sub_on_note(index as usize, values, is_cycle);
    } else {
        trk!(song).v_on_time = None;
        trk!(song).v_on_note_index = 0;
        trk!(song).v_on_note = Some(values);
        trk!(song).v_on_note_is_cycle = is_cycle;
    }
}

/// 音符ごとのタイミング変化 (is_cycle=trueで繰り返し)
pub(super) fn exec_timing_on_note(song: &mut Song, t: &Token, is_cycle: bool) {
    trk!(song).t_on_note_index = 0;
    trk!(song).t_on_note = Some(t.data[0].to_int_array());
    trk!(song).t_on_note_is_cycle = is_cycle;
}

/// 音符ごとのゲート変化 (is_cycle=trueで繰り返し)
pub(super) fn exec_qlen_on_note(song: &mut Song, t: &Token, is_cycle: bool) {
    trk!(song).q_on_note_index = 0;
    trk!(song).q_on_note = Some(t.data[0].to_int_array());
    trk!(song).q_on_note_is_cycle = is_cycle;
}

/// 音符ごとのオクターブ変化 (is_cycle=trueで繰り返し)
pub(super) fn exec_octave_on_note(song: &mut Song, t: &Token, is_cycle: bool) {
    trk!(song).o_on_note_index = 0;
    trk!(song).o_on_note = Some(t.data[0].to_int_array());
    trk!(song).o_on_note_is_cycle = is_cycle;
}

/// 音符ごとの音長変化 (is_cycle=trueで繰り返し)
pub(super) fn exec_length_on_note(song: &mut Song, t: &Token, is_cycle: bool) {
    trk!(song).l_on_note_index = 0;
    trk!(song).l_on_note = Some(t.data[0].to_int_array());
    trk!(song).l_on_note_is_cycle = is_cycle;
}

/// タイ(&)の動作モードの指定
pub(super) fn exec_tie_mode(song: &mut Song, t: &Token) {
    let args = exec_args(song, t.children.as_ref().unwrap_or(&vec![]));
    if args.len() >= 1 {
        trk!(song).tie_mode = TieMode::from_i(var_extract(&args[0], song).to_i());
    }
    if args.len() >= 2 {
        trk!(song).tie_value = var_extract(&args[1], song).to_i();
    }
}
