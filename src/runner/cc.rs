//! runner: コントロールチェンジ・テンポ・音色の実行
use super::*;

pub(super) fn exec_cc_rpn_nrpn(song: &mut Song, t: &Token, cc1: isize, cc2: isize, cc3: isize) {
    let val = exec_value_int_by_token(song, t);
    let msb = t.data[0].to_i();
    let lsb = t.data[1].to_i();
    if cc1 == 101 && cc2 == 100 && msb == 0 && lsb == 0 {
        trk!(song).bend_range = val;
    }
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc1, msb));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc2, lsb));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc3, val)); 
}

pub(super) fn exec_cc_rpn_nrpn_direct(song: &mut Song, t: &Token, cc1: isize, cc2: isize, cc3: isize) {
    let args = exec_args(song, t.children.as_deref().unwrap_or(&[]));
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

/// テンポの既定値(BPM)
const DEFAULT_TEMPO: isize = 120;

pub(super) fn tempo_change(song: &mut Song, tempo: isize) {
    // TempoChange は Tempo と違い範囲チェックがないため 0 以下が渡り得る。
    // そのままだと MIDI のテンポ(μsec/四分音符)が不正な値になるので既定値に戻す #94
    let tempo = if tempo > 0 { tempo } else { DEFAULT_TEMPO };
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
    let args = exec_args(song, t.children.as_deref().unwrap_or(&[]));
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

/// コントロールチェンジの送信
pub(super) fn exec_control_change(song: &mut Song, t: &Token) {
    let no = t.value_i;
    let val_tokens = t.children.clone().unwrap_or(vec![]);
    let val_v = exec_value(song, &val_tokens);
    let val = val_v.to_i();
    trk!(song).remove_cc_on_note_wave(no);
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, no, val));
}

/// ピッチベンドの送信
pub(super) fn exec_pitch_bend(song: &mut Song, t: &Token) {
    let val = var_extract(&t.data[0], song).to_i();
    trk!(song).pitch_bend = if t.value_i == 0 { val * 128 - 8192 } else { val };
    let val = if t.value_i == 0 { val * 128 } else { val + 8192 };
    song.add_event(Event::pitch_bend(
        trk!(song).timepos,
        trk!(song).channel,
        val,
    ));
}

/// テンポの指定
pub(super) fn exec_tempo(song: &mut Song, t: &Token) {
    let tempo = exec_value_int_by_token(song, t);
    let tempo = value_range(10, tempo, 300);
    tempo_change(song, tempo);
}

/// テンポの変化 (TempoChange)
pub(super) fn exec_tempo_change(song: &mut Song, t: &Token) {
    let data = exec_args(song, t.children.as_deref().unwrap_or(&[]));
    if data.len() == 3 {
        tempo_change_a_to_b(song, data[0].to_i(), data[1].to_i(), data[2].to_i());
    } else if data.len() == 2 {
        tempo_change_a_to_b(song, song.tempo, data[0].to_i(), data[1].to_i());
    } else {
        tempo_change(song, data[0].to_i());
    }
}

/// 時間経過によるCCの変化
pub(super) fn exec_cc_on_time(song: &mut Song, t: &Token) {
    let no = t.value_i;
    let ia = t.data[0].to_int_array();
    trk!(song).remove_cc_on(no);
    trk!(song).write_cc_on_time(no, ia);
}

/// 音符ごとのCCの変化
pub(super) fn exec_cc_on_note(song: &mut Song, t: &Token) {
    let no = t.value_i;
    let ia = t.data[0].to_int_array();
    trk!(song).set_cc_on_note(no, ia);
}

/// 音符ごとのCCの波形変化
pub(super) fn exec_cc_on_note_wave(song: &mut Song, t: &Token) {
    let no = t.value_i;
    let ia = t.data[0].to_int_array();
    trk!(song).set_cc_on_note_wave(no, ia);
}

/// 時間経過によるCC変化の頻度
pub(super) fn exec_cc_on_time_freq(song: &mut Song, t: &Token) {
    trk!(song).cc_on_time_freq = var_extract(&t.data[0], song).to_i();
}

/// 時間経過によるピッチベンドの変化
pub(super) fn exec_pb_on_time(song: &mut Song, t: &Token) {
    trk!(song).write_pb_on_time(t.value_i, t.data[0].to_int_array(), song.timebase);
}
