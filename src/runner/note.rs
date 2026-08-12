//! runner: 音符と休符の実行
use super::*;

pub(super) fn get_note_info_from_token(t: &Token) -> NoteInfo {
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
pub(super) fn set_note_info_with_default_value(note: &mut NoteInfo, song: &mut Song) {
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
    let t = trk!(song).calc_t_on_note(note.t);
    let qlen = trk!(song).calc_qlen_on_note(note.qlen);
    let o_abs = trk!(song).calc_o_on_note(-1);
    if o_abs != -1 {// ノートはそのままでオクターブだけ変える
        note.no = note.no % 12 + o_abs * 12; // set absolute octave
    }
    // Random
    if trk!(song).o_rand > 0 { // octave randomize
        let r = song.calc_rand_value(0, trk!(song).o_rand);
        if r != 0 {
            note.no += r * 12;
        }
    }
    let v = if trk!(song).v_rand > 0 {
        song.calc_rand_value(v, trk!(song).v_rand)
    } else {
        v
    };
    let t = if trk!(song).t_rand > 0 {
        song.calc_rand_value(t, trk!(song).t_rand)
    } else {
        t
    };
    let qlen = if trk!(song).q_rand > 0 {
        song.calc_rand_value(qlen, trk!(song).q_rand)
    } else {
        qlen
    };
    let v_sub_rand = trk!(song).v_sub_rand.clone();
    let mut v = trk!(song).apply_v_sub(v);
    for random in v_sub_rand {
        if random > 0 {
            v = song.calc_rand_value(v, random);
        }
    }
    // note len
    let mut notelen = calc_length(&note.len_s, song.timebase, trk!(song).length);
    // note len onNote / onCycle
    let notelen_on_note = trk!(song).calc_l_on_note(-1);
    if notelen_on_note != -1 { // onNote / onCycle の値があれば強制的に上書き
        notelen = notelen_on_note;
    }
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    // check range
    let v = value_range(0, v, 127);
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
        trk!(song).tie_notes.push(event);
        return;
    }
    if trk!(song).tie_notes.len() > 0 {
        trk!(song).tie_notes.push(event);
        check_tie_notes(song);
        return;
    }
    // onNote event
    trk!(song).write_cc_on_note(start_pos);
    // onNoteWave event
    trk!(song).write_cc_on_note_wave(start_pos);
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
    // onTime / onNote
    let v = trk!(song).calc_v_on_time(v);
    let v = trk!(song).calc_v_on_note(v);
    let t = trk!(song).calc_t_on_note(t);
    let qlen = trk!(song).calc_qlen_on_note(qlen);
    // Random
    let v = if trk!(song).v_rand > 0 {
        song.calc_rand_value(v, trk!(song).v_rand)
    } else {
        v
    };
    let t = if trk!(song).t_rand > 0 {
        song.calc_rand_value(t, trk!(song).t_rand)
    } else {
        t
    };
    let qlen = if trk!(song).q_rand > 0 {
        song.calc_rand_value(qlen, trk!(song).q_rand)
    } else {
        qlen
    };
    let v_sub_rand = trk!(song).v_sub_rand.clone();
    let mut v = trk!(song).apply_v_sub(v);
    for random in v_sub_rand {
        if random > 0 {
            v = song.calc_rand_value(v, random);
        }
    }
    // calc
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    // range
    let v = value_range(0, v, 127);
    let event = Event::note(
        trk!(song).timepos + t,
        trk!(song).channel,
        data_note_no + track_key + key_shift,
        notelen_real,
        v,
    );
    // println!("- {}: note(no={},len={},qlen={},v={},t={})", trk!(song).timepos, notelen_real, notelen, qlen, v, t);
    // onNoteWave event
    trk!(song).write_cc_on_note_wave(start_pos);
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
