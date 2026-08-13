//! runner: コントロールチェンジ・テンポ・音色の実行
use super::*;

/// 先行指定の書き込みに必要な情報(乱数など)を用意して処理を実行する
/// 乱数は曲全体で1つの系列を使うので、処理のあとに種を書き戻す
pub(super) fn with_write_ctx<F>(song: &mut Song, f: F)
where
    F: FnOnce(&mut Track, &mut WriteCtx),
{
    let timebase = song.timebase;
    let mut seed = song.rand_seed;
    let max_event_bytes = song.max_event_bytes();
    let event_bytes = song.event_bytes();
    let (result_event_bytes, event_limit_exceeded) = {
        let mut ctx = WriteCtx {
            timebase,
            rand_seed: &mut seed,
            max_event_bytes,
            event_bytes,
            event_limit_exceeded: false,
        };
        let trk = &mut song.tracks[song.cur_track];
        f(trk, &mut ctx);
        (ctx.event_bytes, ctx.event_limit_exceeded)
    };
    song.rand_seed = seed;
    song.update_event_budget(result_event_bytes, event_limit_exceeded);
}

/// トークンの value_i から書き込み先を求める
/// 0以上はCC番号、負ならピッチベンド
pub(super) fn write_target_from_value(value_i: isize) -> WriteTarget {
    match value_i {
        WRITE_TARGET_PB_SMALL => WriteTarget::PitchBend(0),
        WRITE_TARGET_PB_BIG => WriteTarget::PitchBend(1),
        no => WriteTarget::CC(no),
    }
}

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
        if song.event_limit_exceeded() {
            trk!(song).timepos = timepos;
            return;
        }
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
    let v1 = var_extract(&t.data[1], song).to_i();
    let v2 = var_extract(&t.data[2], song).to_i();
    let len = calc_length(&len_s, song.timebase, trk!(song).length);
    let ia = vec![v1, v2, len];
    // write EP
    with_write_ctx(song, |trk, ctx| trk.write_cc_on_time(11, ia, ctx));
}

/// フェードイン・フェードアウト
pub(super) fn exec_fade_io(song: &mut Song, t: &Token) {
    let measures = t.data.first().map(|v| var_extract(v, song).to_i()).unwrap_or(0);
    let len = song.timebase * 4 * measures;
    let values = if t.value_i >= 1 {
        vec![0, 127, len]
    } else {
        vec![127, 0, len]
    };
    trk!(song).remove_cc_on(11);
    with_write_ctx(song, |trk, ctx| trk.write_cc_on_time(11, values, ctx));
}

/// コントロールチェンジの送信
pub(super) fn exec_control_change(song: &mut Song, t: &Token) {
    let no = t.value_i;
    let val_tokens = t.children.clone().unwrap_or(vec![]);
    let val_v = exec_value(song, &val_tokens);
    let val = val_v.to_i();
    // 値を直接指定すると先行指定は解除される
    trk!(song).remove_reserve(WriteTarget::CC(no));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, no, val));
}

/// ピッチベンドの送信
pub(super) fn exec_pitch_bend(song: &mut Song, t: &Token) {
    // 単発のピッチベンド指定で、先行指定を解除する
    trk!(song).remove_reserve(WriteTarget::PitchBend(1));
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
    let ia = var_extract(&t.data[0], song).to_int_array();
    trk!(song).remove_cc_on(no);
    with_write_ctx(song, |trk, ctx| trk.write_cc_on_time(no, ia, ctx));
}

/// 音符ごとのCCの変化
pub(super) fn exec_cc_on_note(song: &mut Song, t: &Token) {
    let no = t.value_i;
    let ia = var_extract(&t.data[0], song).to_int_array();
    trk!(song).set_cc_on_note(no, ia);
}

/// 音符ごとのCCの波形変化
pub(super) fn exec_cc_on_note_wave(song: &mut Song, t: &Token) {
    let no = t.value_i;
    let ia = var_extract(&t.data[0], song).to_int_array();
    trk!(song).set_cc_on_note_wave(no, ia);
}

/// 時間経過によるCC・ピッチベンドの書き込み頻度 (.Frequency)
pub(super) fn exec_cc_on_time_freq(song: &mut Song, t: &Token) {
    let freq = var_extract(&t.data[0], song).to_i();
    match write_target_from_value(t.value_i) {
        // CCの頻度はトラック全体で共通
        WriteTarget::CC(_) => trk!(song).cc_on_time_freq = freq,
        WriteTarget::PitchBend(_) => trk!(song).pb_on_time_freq = freq,
    }
}

/// 時間経過によるピッチベンドの変化
pub(super) fn exec_pb_on_time(song: &mut Song, t: &Token) {
    trk!(song).remove_pb_on_note_wave();
    let is_big = t.value_i;
    let ia = var_extract(&t.data[0], song).to_int_array();
    with_write_ctx(song, |trk, ctx| trk.write_pb_on_time(is_big, ia, ctx));
}

/// 音符ごとのピッチベンドの波形変化
pub(super) fn exec_pb_on_note_wave(song: &mut Song, t: &Token) {
    let values = var_extract(&t.data[0], song).to_int_array();
    trk!(song).set_pb_on_note_wave(t.value_i, values);
}

/// 音符ごとのピッチベンドの変化 (PB.onNote/p.onNote)
pub(super) fn exec_pb_on_note(song: &mut Song, t: &Token) {
    let target = WriteTarget::PitchBend(t.value_i);
    let ia = var_extract(&t.data[0], song).to_int_array();
    trk!(song).set_on_note(target, ia);
}

/// 音符ごとの波形変化 --- 音符の長さに合わせて伸縮する (.onNoteWaveEx)
pub(super) fn exec_cc_on_note_wave_ex(song: &mut Song, t: &Token) {
    let target = write_target_from_value(t.value_i);
    let ia = var_extract(&t.data[0], song).to_int_array();
    trk!(song).set_on_note_wave(target, ia, WaveMode::Expand);
}

/// 音符ごとの波形変化 --- 音符が鳴っている間くり返す (.onNoteWaveR)
pub(super) fn exec_cc_on_note_wave_r(song: &mut Song, t: &Token) {
    let target = write_target_from_value(t.value_i);
    let ia = var_extract(&t.data[0], song).to_int_array();
    trk!(song).set_on_note_wave(target, ia, WaveMode::Repeat);
}

/// 現在のトラック時刻まで、周期的な先行指定(.onCycle)を書き出す
/// 音符や休符で時間が進んだあとに呼ぶ。呼ばないと、長い音符の途中や
/// 曲の末尾で周期的な書き込みが止まってしまう
pub(super) fn flush_cc_on_cycle(song: &mut Song) {
    if trk!(song).cc_on_cycle.len() == 0 { return; }
    // 現在位置ちょうどの書き込みは、次の音符の発音時に確定させる
    // (ここで書き込むと、曲の末尾に余分なイベントが増えてしまう)
    let until = trk!(song).timepos - 1;
    with_write_ctx(song, |trk, ctx| trk.write_cc_on_cycle(until, ctx));
}

/// 一定時間ごとの値の先行指定 (.onCycle)
pub(super) fn exec_cc_on_cycle(song: &mut Song, t: &Token) {
    let target = write_target_from_value(t.value_i);
    let args = var_extract(&t.data[0], song).to_int_array();
    if args.len() < 2 {
        runtime_error(song, ".onCycle needs (step, v1, v2, ...)");
        return;
    }
    let len = args[0];
    let values = args[1..].to_vec();
    trk!(song).set_on_cycle(target, len, values);
}

/// .Sine / .onNoteSine の引数を読み取る (type, low, high, len, times)
fn get_sine_args(song: &mut Song, t: &Token) -> Option<OnNoteSine> {
    let target = write_target_from_value(t.value_i);
    let args = var_extract(&t.data[0], song).to_int_array();
    if args.len() < 4 {
        runtime_error(song, ".Sine needs (type, low, high, len [,times])");
        return None;
    }
    Some(OnNoteSine {
        target,
        stype: SineType::from_i(args[0]),
        low: args[1],
        high: args[2],
        len: args[3],
        times: if args.len() >= 5 { args[4] } else { 1 },
    })
}

/// 正弦波を1回書き込む (.Sine)
pub(super) fn exec_cc_sine(song: &mut Song, t: &Token) {
    let sine = match get_sine_args(song, t) {
        Some(v) => v,
        None => return,
    };
    trk!(song).remove_reserve(sine.target);
    with_write_ctx(song, |trk, ctx| {
        trk.write_sine(sine.target, sine.stype, sine.low, sine.high, sine.len, sine.times, ctx)
    });
}

/// 音符ごとに正弦波を書き込む (.onNoteSine)
pub(super) fn exec_cc_on_note_sine(song: &mut Song, t: &Token) {
    let sine = match get_sine_args(song, t) {
        Some(v) => v,
        None => return,
    };
    trk!(song).set_on_note_sine(sine.target, sine);
}

/// 先行指定の効果の遅延時間 (.Delay)
pub(super) fn exec_cc_delay(song: &mut Song, t: &Token) {
    let target = write_target_from_value(t.value_i);
    let v = var_extract(&t.data[0], song).to_i();
    trk!(song).update_write_opt(target, |opt| opt.delay = v);
}

/// 書き込む値をランダムにばらつかせる (.Random)
pub(super) fn exec_cc_random(song: &mut Song, t: &Token) {
    let target = write_target_from_value(t.value_i);
    let v = var_extract(&t.data[0], song).to_i();
    trk!(song).update_write_opt(target, |opt| opt.random = v);
}

/// 書き込む値の下限と上限を設定する (.Range)
pub(super) fn exec_cc_range(song: &mut Song, t: &Token) {
    let target = write_target_from_value(t.value_i);
    let args = var_extract(&t.data[0], song).to_int_array();
    if args.len() < 2 {
        runtime_error(song, ".Range needs (low, high)");
        return;
    }
    let (low, high) = (args[0], args[1]);
    trk!(song).update_write_opt(target, |opt| opt.range = Some((low, high)));
}

/// .onNote などで値をくり返すかどうか (.Repeat)
pub(super) fn exec_cc_repeat(song: &mut Song, t: &Token) {
    let target = write_target_from_value(t.value_i);
    let on = var_extract(&t.data[0], song).to_i() != 0;
    trk!(song).set_repeat(target, on);
}
