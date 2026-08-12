//! runner: コントロールチェンジ・テンポ・音色の実行
use super::*;

pub(super) fn exec_cc_rpn_nrpn(song: &mut Song, t: &Token, cc1: isize, cc2: isize, cc3: isize) {
    let val = exec_value_int_by_token(song, t);
    let msb = t.data[0].to_i();
    let lsb = t.data[1].to_i();
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc1, msb));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc2, lsb));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc3, val)); 
}

pub(super) fn exec_cc_rpn_nrpn_direct(song: &mut Song, t: &Token, cc1: isize, cc2: isize, cc3: isize) {
    let args = exec_args(song, t.children.as_ref().unwrap_or(&vec![]));
    if args.len() != 3 {
        runtime_error(song, "RPN/NRPN needs 3 arguments");
        return;
    }
    let msb = args[0].to_i();
    let lsb = args[1].to_i();
    let val = args[2].to_i();
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc1, msb));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc2, lsb));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc3, val)); 
}

pub(super) fn tempo_change_a_to_b(song: &mut Song, a: isize, b: isize, len: isize) {
    let step = (song.timebase * 4) / 16;
    let step_cnt = len / step;
    let width = b - a;
    let timepos = trk!(song).timepos;
    for i in 0..step_cnt {
        let v = (a as f32) + (width as f32) * (i as f32 / step_cnt as f32);
        tempo_change(song, v as isize);
        trk!(song).timepos += step;
    }
    trk!(song).timepos = timepos + len;
    tempo_change(song, b);
    trk!(song).timepos = timepos;
}

pub(super) fn tempo_change(song: &mut Song, tempo: isize) {
    let tempo = if tempo > 0 { tempo } else { 120 };
    song.tempo = tempo;
    let mpq = 60000000 / tempo;
    let e = Event::meta(
        trk!(song).timepos,
        0xFF,
        0x51,
        0x03,
        vec![
            (mpq >> 16 & 0xFF) as u8,
            (mpq >> 8 & 0xFF) as u8,
            (mpq >> 0 & 0xFF) as u8,
        ],
    );
    song.add_event(e);
}

pub(super) fn exec_voice(song: &mut Song, t: &Token) {
    // voice no
    let args = exec_args(song, t.children.as_ref().unwrap_or(&vec![]));
    let no = if args.len() >= 1 { args[0].to_i() } else { 1 };
    let no = value_range(1, no, 128) - 1;
    let bank_msb = if args.len() >= 2 { args[1].to_i() } else { 0 };
    let bank_lsb = if args.len() >= 3 { args[2].to_i() } else { 0 };
    trk!(song).program_change = no + 1;
    // bank ?
    if args.len() == 1 {
        song.add_event(Event::voice(trk!(song).timepos, trk!(song).channel, no));
    } else {
        song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, 0x00, bank_msb)); // msb
        song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, 0x20, bank_lsb)); // lsb
        song.add_event(Event::voice(trk!(song).timepos, trk!(song).channel, no));
        // println!("voice: no={}, bank_msb={}, bank_lsb={}", no, bank_msb, bank_lsb);
    }
}

pub(super) fn exec_decres(song: &mut Song, t: &Token) {
    let mut len_s = t.data[0].to_s();
    if len_s == "" { len_s = "1".to_string(); }
    let v1 = t.data[1].to_i();
    let v2 = t.data[2].to_i();
    let len = calc_length(&len_s, song.timebase, trk!(song).length);
    let ia = vec![v1, v2, len];
    // write EP
    trk!(song).write_cc_on_time(11, ia);
}
