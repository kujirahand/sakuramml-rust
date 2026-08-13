//! runner: 音符と休符の実行
use super::*;

fn apply_v_sub_random(song: &mut Song, mut velocity: isize) -> isize {
    let len = trk!(song).v_sub_rand.len();
    for index in 0..len {
        let random = trk!(song).v_sub_rand[index];
        if random > 0 {
            velocity = song.calc_rand_value(velocity, random);
        }
    }
    velocity
}

/// 音符属性の .Random を値に適用する
fn calc_note_param_random(song: &mut Song, target: isize, value: isize) -> isize {
    let trk = &song.tracks[song.cur_track];
    let param = match target {
        NOTE_PARAM_V => &trk.v_opt,
        NOTE_PARAM_Q => &trk.q_opt,
        NOTE_PARAM_T => &trk.t_opt,
        NOTE_PARAM_O => &trk.o_opt,
        _ => &trk.l_opt,
    };
    let random = param.random;
    if random > 0 {
        song.calc_rand_value(value, random)
    } else {
        value
    }
}

/// 音符属性の .Random / .Range / .Max を値に適用する
fn calc_note_param(song: &mut Song, target: isize, value: isize) -> isize {
    let value = calc_note_param_random(song, target, value);
    // 借用の都合で、ランダムを計算したあとに範囲を適用する
    let trk = &song.tracks[song.cur_track];
    let param = match target {
        NOTE_PARAM_V => &trk.v_opt,
        NOTE_PARAM_Q => &trk.q_opt,
        NOTE_PARAM_T => &trk.t_opt,
        NOTE_PARAM_O => &trk.o_opt,
        _ => &trk.l_opt,
    };
    param.apply_limit(value)
}

/// ゲート長(実際の発音ステップ数)を求める (#127)
/// - 割合指定(q90 など): 音長に対する百分率
/// - ステップ指定(q%48 など): ステップ数をそのまま使う(音長を超える指定も可能)
pub(crate) fn calc_gate_len(notelen: isize, qlen: isize, qlen_is_step: bool) -> isize {
    if qlen_is_step {
        // 負の値は resolve_step_qlen で解決済み。念のため0で止める
        return qlen.max(0);
    }
    (notelen as f32 * qlen as f32 / 100.0) as isize
}

/// ステップ指定のゲートが負の値のときは、現在のqの値からの相対指定として解決する (#127)
/// (オリジナル(Pascal版)の SetNoteInfo.subSoutai と同じ動作)
pub(crate) fn resolve_step_qlen(song: &mut Song, qlen: isize, qlen_is_step: bool) -> isize {
    if qlen_is_step && qlen < 0 {
        return trk!(song).qlen.saturating_add(qlen).max(0);
    }
    qlen
}

pub(crate) fn get_note_info_from_token(t: &Token) -> NoteInfo {
    let data = &t.data;
    if data.len() < 8 {
        // broken note
        return NoteInfo {
            no: 0,
            flag: 0,
            natural: 0,
            len_s: "".to_string(),
            qlen: 0,
            qlen_is_step: false,
            vel: 0,
            t: 0,
            o: 0,
            slur: 0,
        };
    }
    let note_no = (t.value_i % 12) as isize;
    let data_note_flag = data[0].to_i();
    let data_note_natural = data[1].to_i();
    let data_note_len = data[2].to_s();
    let data_note_qlen = data[3].to_i(); // 0
    let data_note_vel = data[4].to_i(); // -1
    let data_note_t = data[5].to_i(); // isize::MIN
    let data_note_o = data[6].to_i(); // -1
    let data_slur = data[7].to_i(); // 0 or 1 --- TODO: #7
    let data_qlen_is_step = data.get(8).map(|v| v.to_i()).unwrap_or(0) != 0; // c4,%70 (#127)
    NoteInfo {
        no: note_no,
        flag: data_note_flag,
        natural: data_note_natural,
        len_s: data_note_len,
        qlen: data_note_qlen,
        qlen_is_step: data_qlen_is_step,
        vel: data_note_vel,
        t: data_note_t,
        o: data_note_o,
        slur: data_slur,
    }
}

/// get note info, and shift key
pub(crate) fn set_note_info_with_default_value(note: &mut NoteInfo, song: &mut Song) {
    // set note with track's default value
    // 音符側にゲート指定がなければ、トラックの指定(割合/ステップ)を受け継ぐ (#127)
    if note.qlen == 0 && !note.qlen_is_step {
        note.qlen = trk!(song).qlen;
        note.qlen_is_step = trk!(song).qlen_is_step;
    } else {
        // c4,%-6 のような負のステップ指定は、現在のqの値からの相対指定 (#127)
        note.qlen = resolve_step_qlen(song, note.qlen, note.qlen_is_step);
    }
    if note.vel < 0 {
        note.vel = trk!(song).velocity;
    }
    if note.t == isize::MIN {
        note.t = trk!(song).timing;
    }
    if note.o < 0 {
        note.o = trk!(song).octave;
    }
    // calc note no
    let mut noteno = note.o * 12 + note.no + note.flag;
    // key_shift / key_flag / track_key
    if song.use_key_shift {
        noteno += if note.natural == 0 {
            song.key_flag[(note.no as usize) % 12]
        } else {
            0
        };
        noteno += song.key_shift;
        noteno += trk!(song).track_key;
    }
    note.no = noteno;
}

/// 音符の発音開始時に、先行指定(onNote/onNoteWave)を書き出す
/// タイ・スラーや和音では、つながった音符の先頭で一度だけ呼ぶ
pub(super) fn write_on_note_events(song: &mut Song, start_pos: isize) {
    with_write_ctx(song, |trk, ctx| {
        trk.write_cc_on_cycle(start_pos, ctx);
        trk.write_cc_on_note(start_pos, ctx);
        trk.write_cc_on_note_wave(start_pos, ctx);
    });
}

/// イベント上限で音符を書き込めない場合も、演奏位置と一時状態を確定する。
/// すでに予算を確保済みの和音・タイ音符は、途中までのMIDIとして保持する。
fn finish_note_after_event_limit(song: &mut Song, notelen: isize) {
    trk!(song).timepos = trk!(song).timepos.saturating_add(notelen);
    if song.flags.octave_once != 0 {
        trk!(song).octave = trk!(song).octave.saturating_sub(song.flags.octave_once);
        song.flags.octave_once = 0;
    }

    song.flags.harmony_flag = false;
    let mut pending = std::mem::take(&mut song.flags.harmony_events);
    pending.append(&mut std::mem::take(&mut trk!(song).tie_notes));
    for event in pending {
        song.add_reserved_event(event);
    }
}

pub(super) fn exec_note(song: &mut Song, t: &Token) {
    // get note parameters
    let mut note = get_note_info_from_token(t);
    // get note info, and shift key
    set_note_info_with_default_value(&mut note, song);
    // timepos
    let timepos = trk!(song).timepos;
    let start_pos = timepos;
    // onTime / onNote
    let v = trk!(song).calc_v_on_time(note.vel);
    let v = trk!(song).calc_v_on_note(v);
    let t = trk!(song).calc_t_on_time(note.t);
    let t = trk!(song).calc_t_on_note(t);
    let qlen = trk!(song).calc_qlen_on_time(note.qlen);
    let qlen = trk!(song).calc_qlen_on_note(qlen);
    // ゲートの先行指定は割合指定なので、有効ならステップ指定より優先する (#127)
    // 予約が終わったかどうかは、値を求めたあとでなければ分からない
    let q_reserved = trk!(song).q_opt.has_reserve();
    let qlen_is_step = note.qlen_is_step && !q_reserved;
    let o_abs = trk!(song).calc_o_on_time(-1);
    let o_abs = trk!(song).calc_o_on_note(o_abs);
    // 実際に使うオクターブを求め、.Random と .Range/.Max を適用する
    let mut o_cur = if o_abs != -1 { o_abs } else { note.o };
    if trk!(song).o_opt.random > 0 {
        // octave randomize
        o_cur = song.calc_rand_value(o_cur, trk!(song).o_opt.random);
    }
    let o_cur = trk!(song).o_opt.apply_limit(o_cur);
    if o_cur != note.o {
        // ノート番号のオクターブ部分だけ差し替える(臨時記号や移調はそのまま)
        note.no += (o_cur - note.o) * 12;
    }
    let v = calc_note_param(song, NOTE_PARAM_V, v);
    let t = calc_note_param(song, NOTE_PARAM_T, t);
    // ステップ指定のときは、割合向けの .Range/.Max を適用しない (#127)
    let qlen = if qlen_is_step {
        calc_note_param_random(song, NOTE_PARAM_Q, qlen)
    } else {
        calc_note_param(song, NOTE_PARAM_Q, qlen)
    };
    let v = trk!(song).apply_v_sub(v);
    let v = apply_v_sub_random(song, v);
    // note len
    let mut notelen = calc_length(&note.len_s, song.timebase, trk!(song).length);
    // note len onTime / onNote / onCycle
    let notelen_on_note = trk!(song).calc_l_on_time(-1);
    let notelen_on_note = trk!(song).calc_l_on_note(notelen_on_note);
    if notelen_on_note != -1 {
        // 先行指定の値があれば強制的に上書き
        notelen = notelen_on_note;
    }
    // .Random / .Range / .Max は通常の音長にも適用する
    let notelen = calc_note_param(song, NOTE_PARAM_L, notelen).max(0);
    let notelen_real = calc_gate_len(notelen, qlen, qlen_is_step);
    // check range
    let v = value_range(0, v, trk!(song).v_opt.max_or(127));
    // event
    let event = Event::note(
        timepos.saturating_add(t),
        trk!(song).channel,
        note.no,
        notelen_real,
        v,
    );
    if !song.reserve_event(&event) {
        finish_note_after_event_limit(song, notelen);
        return;
    }
    // println!("- {}: note(no={},len={},qlen={},v={},t={},o={})", trk.timepos, noteno, notelen_real, qlen, v, t, o);
    trk!(song).timepos = trk!(song).timepos.saturating_add(notelen);

    // octave_once?
    if song.flags.octave_once != 0 {
        trk!(song).octave = trk!(song).octave.saturating_sub(song.flags.octave_once);
        song.flags.octave_once = 0;
    }

    // harmony?
    if song.flags.harmony_flag {
        trk!(song).timepos = song.flags.harmony_time;
        // 和音の終わりでゲートを計算し直すので、実際に使った指定を覚えておく (#127)
        if song.flags.harmony_qlen.is_none() {
            song.flags.harmony_qlen = Some((qlen, qlen_is_step, q_reserved));
        }
        song.flags.harmony_events.push(event);
        return;
    }
    // tie or slur?
    if note.slur >= 1 {
        // タイ・スラーの先頭の音符でだけ先行指定を書き出す (#78)
        if trk!(song).tie_notes.len() == 0 {
            write_on_note_events(song, start_pos);
        }
        trk!(song).tie_notes.push(event);
        return;
    }
    if trk!(song).tie_notes.len() > 0 {
        // タイ・スラーの末尾の音符 --- 先行指定は先頭の音符で書き出し済み
        trk!(song).tie_notes.push(event);
        check_tie_notes(song);
        return;
    }
    // onNote / onNoteWave event
    write_on_note_events(song, start_pos);
    // write note event
    song.add_reserved_event(event);
    // 音符の中にある .onCycle の書き込みを確定する
    flush_cc_on_cycle(song);
}

pub(super) fn exec_note_n(song: &mut Song, t: &Token) {
    // parameters
    let data_note_no = var_extract(&t.data[0], song).to_i();
    let data_note_len = var_extract(&t.data[1], song).to_s();
    let data_note_qlen = var_extract(&t.data[2], song).to_i(); // 0
    let data_note_vel = var_extract(&t.data[3], song).to_i(); // -1
    let data_note_t = var_extract(&t.data[4], song).to_i(); // isize::MIN
    let data_qlen_is_step = t.data.get(6).map(|v| v.to_i()).unwrap_or(0) != 0; // n60,4,%70 (#127)
    let start_pos = trk!(song).timepos;
    let track_key = trk!(song).track_key;
    let key_shift = song.key_shift;

    // check parameters
    let notelen = calc_length(&data_note_len, song.timebase, trk!(song).length);
    let notelen = calc_note_param(song, NOTE_PARAM_L, notelen).max(0);
    // ゲート指定 --- 音符側の指定がなければトラックの指定(割合/ステップ)を使う (#127)
    let (qlen, qlen_is_step) = if data_note_qlen != 0 || data_qlen_is_step {
        // 負のステップ指定は、現在のqの値からの相対指定 (#127)
        (
            resolve_step_qlen(song, data_note_qlen, data_qlen_is_step),
            data_qlen_is_step,
        )
    } else {
        (trk!(song).qlen, trk!(song).qlen_is_step)
    };
    let v = if data_note_vel >= 0 {
        data_note_vel
    } else {
        trk!(song).velocity
    };
    let t = if data_note_t != isize::MIN {
        data_note_t
    } else {
        trk!(song).timing
    };
    // onTime / onCycle / onNote
    let v = trk!(song).calc_v_on_time(v);
    let v = trk!(song).calc_v_on_note(v);
    let t = trk!(song).calc_t_on_time(t);
    let t = trk!(song).calc_t_on_note(t);
    let qlen = trk!(song).calc_qlen_on_time(qlen);
    let qlen = trk!(song).calc_qlen_on_note(qlen);
    // ゲートの先行指定は割合指定なので、有効ならステップ指定より優先する (#127)
    // 予約が終わったかどうかは、値を求めたあとでなければ分からない
    let qlen_is_step = qlen_is_step && !trk!(song).q_opt.has_reserve();
    // Random / Range / Max
    let v = calc_note_param(song, NOTE_PARAM_V, v);
    let t = calc_note_param(song, NOTE_PARAM_T, t);
    // ステップ指定のときは、割合向けの .Range/.Max を適用しない (#127)
    let qlen = if qlen_is_step {
        calc_note_param_random(song, NOTE_PARAM_Q, qlen)
    } else {
        calc_note_param(song, NOTE_PARAM_Q, qlen)
    };
    let v = trk!(song).apply_v_sub(v);
    let v = apply_v_sub_random(song, v);
    // calc
    let notelen_real = calc_gate_len(notelen, qlen, qlen_is_step);
    // range
    let v = value_range(0, v, trk!(song).v_opt.max_or(127));
    let event = Event::note(
        trk!(song).timepos.saturating_add(t),
        trk!(song).channel,
        data_note_no + track_key + key_shift,
        notelen_real,
        v,
    );
    if !song.reserve_event(&event) {
        finish_note_after_event_limit(song, notelen);
        return;
    }
    // println!("- {}: note(no={},len={},qlen={},v={},t={})", trk!(song).timepos, notelen_real, notelen, qlen, v, t);
    // onNote / onNoteWave event
    write_on_note_events(song, start_pos);
    // write event
    song.add_reserved_event(event);
    trk!(song).timepos = trk!(song).timepos.saturating_add(notelen);
    // 音符の中にある .onCycle の書き込みを確定する
    flush_cc_on_cycle(song);
}

pub(super) fn exec_rest(song: &mut Song, t: &Token) {
    let trk = &mut song.tracks[song.cur_track];
    let data_note_len = t.data[0].to_s();
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    trk.timepos = trk
        .timepos
        .saturating_add(notelen.saturating_mul(t.value_i));
    // 休符の間にある .onCycle の書き込みを確定する
    flush_cc_on_cycle(song);
}
