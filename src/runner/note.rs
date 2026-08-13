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

/// 音符属性の .Random / .Range / .Max を値に適用する
fn calc_note_param(song: &mut Song, target: isize, value: isize) -> isize {
    let trk = &song.tracks[song.cur_track];
    let param = match target {
        NOTE_PARAM_V => &trk.v_opt,
        NOTE_PARAM_Q => &trk.q_opt,
        NOTE_PARAM_T => &trk.t_opt,
        NOTE_PARAM_O => &trk.o_opt,
        _ => &trk.l_opt,
    };
    let random = param.random;
    let value = if random > 0 {
        song.calc_rand_value(value, random)
    } else {
        value
    };
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

pub(crate) fn get_note_info_from_token(t: &Token) -> NoteInfo {
    let data = &t.data;
    if data.len() < 8 { // broken note
        return NoteInfo {
            no: 0,
            flag: 0,
            natural: 0,
            len_s: "".to_string(),
            qlen: 0,
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
    NoteInfo {
        no: note_no,
        flag: data_note_flag,
        natural: data_note_natural,
        len_s: data_note_len,
        qlen: data_note_qlen,
        vel: data_note_vel,
        t: data_note_t,
        o: data_note_o,
        slur: data_slur,
    }
}

/// get note info, and shift key
pub(crate) fn set_note_info_with_default_value(note: &mut NoteInfo, song: &mut Song) {
    // set note with track's default value
    if note.qlen == 0 {
        note.qlen = trk!(song).qlen;
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
    let o_abs = trk!(song).calc_o_on_time(-1);
    let o_abs = trk!(song).calc_o_on_note(o_abs);
    if o_abs != -1 {// ノートはそのままでオクターブだけ変える
        note.no = note.no % 12 + o_abs * 12; // set absolute octave
    }
    // Random
    if trk!(song).o_opt.random > 0 { // octave randomize
        let r = song.calc_rand_value(0, trk!(song).o_opt.random);
        if r != 0 {
            note.no += r * 12;
        }
    }
    let v = calc_note_param(song, NOTE_PARAM_V, v);
    let t = calc_note_param(song, NOTE_PARAM_T, t);
    let qlen = calc_note_param(song, NOTE_PARAM_Q, qlen);
    let v = trk!(song).apply_v_sub(v);
    let v = apply_v_sub_random(song, v);
    // note len
    let mut notelen = calc_length(&note.len_s, song.timebase, trk!(song).length);
    // note len onTime / onNote / onCycle
    let notelen_on_note = trk!(song).calc_l_on_time(-1);
    let notelen_on_note = trk!(song).calc_l_on_note(notelen_on_note);
    if notelen_on_note != -1 { // 先行指定の値があれば強制的に上書き
        notelen = calc_note_param(song, NOTE_PARAM_L, notelen_on_note);
    }
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    // check range
    let v = value_range(0, v, trk!(song).v_opt.max_or(127));
    // event
    let event = Event::note(timepos + t, trk!(song).channel, note.no, notelen_real, v);
    // println!("- {}: note(no={},len={},qlen={},v={},t={},o={})", trk.timepos, noteno, notelen_real, qlen, v, t, o);
    trk!(song).timepos += notelen;

    // octave_once?
    if song.flags.octave_once != 0 {
        trk!(song).octave = trk!(song).octave - song.flags.octave_once;
        song.flags.octave_once = 0;
    }

    // harmony?
    if song.flags.harmony_flag {
        trk!(song).timepos = song.flags.harmony_time;
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
    trk!(song).events.push(event);
}

pub(super) fn exec_note_n(song: &mut Song, t: &Token) {
    // parameters
    let data_note_no = var_extract(&t.data[0], song).to_i();
    let data_note_len = var_extract(&t.data[1], song).to_s();
    let data_note_qlen = var_extract(&t.data[2], song).to_i(); // 0
    let data_note_vel = var_extract(&t.data[3], song).to_i(); // -1
    let data_note_t = var_extract(&t.data[4], song).to_i(); // isize::MIN
    let start_pos = trk!(song).timepos;
    let track_key = trk!(song).track_key;
    let key_shift = song.key_shift;

    // check parameters
    let notelen = calc_length(&data_note_len, song.timebase, trk!(song).length);
    let qlen = if data_note_qlen != 0 {
        data_note_qlen
    } else {
        trk!(song).qlen
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
    // Random / Range / Max
    let v = calc_note_param(song, NOTE_PARAM_V, v);
    let t = calc_note_param(song, NOTE_PARAM_T, t);
    let qlen = calc_note_param(song, NOTE_PARAM_Q, qlen);
    let v = trk!(song).apply_v_sub(v);
    let v = apply_v_sub_random(song, v);
    // calc
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    // range
    let v = value_range(0, v, trk!(song).v_opt.max_or(127));
    let event = Event::note(
        trk!(song).timepos + t,
        trk!(song).channel,
        data_note_no + track_key + key_shift,
        notelen_real,
        v,
    );
    // println!("- {}: note(no={},len={},qlen={},v={},t={})", trk!(song).timepos, notelen_real, notelen, qlen, v, t);
    // onNote / onNoteWave event
    write_on_note_events(song, start_pos);
    // write event
    trk!(song).events.push(event);
    trk!(song).timepos += notelen;
}

pub(super) fn exec_rest(song: &mut Song, t: &Token) {
    let trk = &mut song.tracks[song.cur_track];
    let data_note_len = t.data[0].to_s();
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    trk.timepos += notelen * t.value_i;
}
