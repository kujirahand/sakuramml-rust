//! runner: サブルーチン・連符・和音・タイムポインタの実行
use super::*;

pub(super) fn exec_play(song: &mut Song, t: &Token) -> bool {
    let tmp_cur_track = song.cur_track;
    let lineno = t.lineno;
    let start_pos = trk!(song).timepos;
    let mut time_ptr_last = start_pos;
    // play
    let arg_tokens = t.children.clone().unwrap_or(vec![]);
    for (index, arg) in arg_tokens.iter().enumerate() {
        song.change_cur_track(index + 1);
        trk!(song).timepos = start_pos;
        // eval calc
        let src = exec_value(song, std::slice::from_ref(arg)).to_s();
        // println!("play(TR={})({}):{}", index+1, lineno, src);
        // eval tokens
        let tokens = lex(song, &src, lineno);
        exec(song, &tokens);
        // check lastpos
        if trk!(song).timepos > time_ptr_last {
            time_ptr_last = trk!(song).timepos;
        }
    }
    song.track_sync();
    song.cur_track = tmp_cur_track;
    true
}

pub(super) fn exec_sub(song: &mut Song, t: &Token) {
    let timepos_tmp: isize;
    {
        let trk = &song.tracks[song.cur_track];
        timepos_tmp = trk.timepos;
    }
    {
        let _ = match &t.children {
            Some(tokens) => exec(song, tokens),
            None => false,
        };
    }
    {
        let trk = &mut song.tracks[song.cur_track];
        trk.timepos = timepos_tmp;
    }
}

pub(super) fn exec_div(song: &mut Song, t: &Token) {
    let len_s = &t.data[0].to_s();
    let cnt = t.value_i;
    let length_org: isize;
    let timepos_end: isize;
    {
        let trk = &mut song.tracks[song.cur_track];
        let div_len = calc_length(len_s, song.timebase, trk.length);
        let note_len = if cnt > 0 { div_len / cnt } else { 0 };
        timepos_end = trk.timepos + div_len;
        length_org = trk.length;
        trk.length = note_len;
    }
    let _ = match &t.children {
        None => false,
        Some(tokens) => exec(song, tokens),
    };
    // clean
    {
        let trk = &mut song.tracks[song.cur_track];
        trk.timepos = timepos_end;
        trk.length = length_org;
    }
}

pub(super) fn exec_harmony(song: &mut Song, t: &Token, flag_begin: bool) {
    // begin
    if flag_begin {
        song.flags.harmony_flag = true;
        song.flags.harmony_time = song.tracks[song.cur_track].timepos;
        return;
    }
    // end
    if song.flags.harmony_flag {
        song.flags.harmony_flag = false;
        // get harmony length
        let note_len_s = t.data[0].to_s();
        let mut note_qlen = t.data[1].to_i();
        let note_vel = t.data[2].clone();
        // `ceg`4,%70 のようなステップ単位のゲート指定 (#127)
        let mut note_qlen_is_step = t.data.get(3).map(|v| v.to_i()).unwrap_or(0) != 0;
        // parameters
        if note_qlen < 0 && !note_qlen_is_step {
            note_qlen = trk!(song).qlen;
            note_qlen_is_step = trk!(song).qlen_is_step;
        }
        let note_len = calc_length(&note_len_s, song.timebase, trk!(song).length);
        // 和音全体で一度だけ先行指定を書き出す (#78)
        if song.flags.harmony_events.len() > 0 {
            let harmony_time = song.flags.harmony_time;
            write_on_note_events(song, harmony_time);
        }
        // change event length
        while song.flags.harmony_events.len() > 0 {
            let mut e = song.flags.harmony_events.pop().unwrap();
            e.time = song.flags.harmony_time;
            if note_qlen_is_step {
                e.v2 = calc_gate_len(note_len, note_qlen, true);
            } else if note_qlen != 0 {
                e.v2 = note_len * note_qlen / 100;
            }
            if !note_vel.is_none() {
                e.v3 = note_vel.to_i();
            }
            song.add_reserved_event(e);
        }
        trk!(song).timepos = song.flags.harmony_time + note_len;
        return;
    }
}

pub(super) fn exec_get_time(song: &mut Song, t: &Token, cmd: &str) -> isize {
    // Calc Time (SakuraObj_time2step)
    // (ref) https://github.com/kujirahand/sakuramml-c/blob/68b62cbc101669211c511258ae1cf830616f238e/src/k_main.c#L473
    let args = exec_args(song, t.children.as_ref().unwrap_or(&Vec::new()));
    if args.len() == 0 {
        runtime_error(song, &format!("[{}] no arguments", cmd));
        return 0;
    }
    if args.len() == 1 {
        return args[0].to_i();
    }
    if args.len() < 3 {
        runtime_error(song, &format!("[{}] needs 1 or 3 arguments", cmd));
        return 0;
    }
    let mes = args[0].to_i() + song.flags.measure_shift;
    let beat = args[1].to_i();
    let tick = args[2].to_i();

    // calc
    let base = song.timebase * 4 / song.timesig_deno;
    let total = (mes - 1) * (base * song.timesig_frac) + (beat - 1) * base + tick;
    total
}

/// トラックの切り替え
pub(super) fn exec_track(song: &mut Song, t: &Token) {
    let no = exec_value_int_by_token(song, t) as usize;
    song.change_cur_track(no);
}

/// チャンネルの指定
pub(super) fn exec_channel(song: &mut Song, t: &Token) {
    let no = exec_value_int_by_token(song, t);
    let v = value_range(1, no, 16) - 1; // CH(1 to 16)
    trk!(song).channel = v as isize;
}

/// 子トークンをまとめて実行する
pub(super) fn exec_tokens(song: &mut Song, t: &Token) {
    let _ = match &t.children {
        Some(tokens) => exec(song, tokens),
        None => false,
    };
}

/// 調号の指定
pub(super) fn exec_key_flag(song: &mut Song, t: &Token) {
    song.key_flag = var_extract(&t.data[0], song).to_int_array();
}

/// 曲全体のキーシフト
pub(super) fn exec_key_shift(song: &mut Song, t: &Token) {
    song.key_shift = exec_value_int_by_token(song, t);
}

/// トラック単位のキーシフト
pub(super) fn exec_track_key(song: &mut Song, t: &Token) {
    trk!(song).track_key = exec_value_int_by_token(song, t);
}

/// キーシフトを使うかどうかの指定
pub(super) fn exec_use_key_shift(song: &mut Song, t: &Token) {
    song.use_key_shift = t
        .data
        .first()
        .map(|v| var_extract(v, song).to_b())
        .unwrap_or(t.value_i != 0);
}

/// 現在位置を演奏開始位置にする
pub(super) fn exec_play_from_here(song: &mut Song) {
    song.play_from = trk!(song).timepos;
}

/// 曲全体のベロシティ加算値
pub(super) fn exec_song_velocity_add(song: &mut Song, t: &Token) {
    song.v_add = exec_value_int_by_token(song, t);
}

/// 曲全体のゲート加算値
pub(super) fn exec_song_q_add(song: &mut Song, t: &Token) {
    song.q_add = exec_value_int_by_token(song, t);
}

/// 小節番号のシフト
pub(super) fn exec_measure_shift(song: &mut Song, t: &Token) {
    song.flags.measure_shift = exec_value_int_by_token(song, t);
}

/// 各種設定の変更
pub(super) fn exec_set_config(song: &mut Song, t: &Token) {
    let key = t.data[0].to_s();
    let val = &t.data[1];
    if key == "RandomSeed" {
        song.rand_seed = var_extract(val, song).to_i() as u32;
    }
}
