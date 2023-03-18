//! runner from tokens

use super::cursor::TokenCursor;
use super::lexer::lex;
use super::song::{Event, Song, Track};
use super::svalue::SValue;
use super::token::{Token, TokenType};

#[derive(Debug)]
pub struct LoopItem {
    pub start_pos: usize,
    pub end_pos: usize,
    pub index: usize,
    pub count: usize,
}

impl LoopItem {
    fn new() -> Self {
        LoopItem {
            start_pos: 0,
            end_pos: 0,
            index: 0,
            count: 0,
        }
    }
}

macro_rules! trk {
    ($song:expr) => {
        $song.tracks[$song.cur_track]
    };
}

/// run tokens
pub fn exec(song: &mut Song, tokens: &Vec<Token>) -> bool {
    let mut pos = 0;
    let mut loop_stack: Vec<LoopItem> = vec![];
    while pos < tokens.len() {
        let t = &tokens[pos];
        if song.debug {
            println!("{:3}:exec:{:?}", pos, t);
        }
        match t.ttype {
            TokenType::Empty => {
                // unknown
            }
            TokenType::Error => {
                if song.debug {
                    println!("[RUNTIME.ERROR]");
                }
            }
            TokenType::Print => {
                let msg = var_extract(&t.data[0], song).to_s();
                let msg = format!("[PRINT]({}) {}", t.value, msg);
                if song.debug {
                    println!("{}", msg);
                }
                song.logs.push(msg);
            }
            // Loop controll
            TokenType::LoopBegin => {
                let mut it = LoopItem::new();
                it.start_pos = pos + 1;
                it.count = var_extract(&t.data[0], song).to_i() as usize;
                // println!("loop={}", it.count);
                loop_stack.push(it);
            }
            TokenType::LoopBreak => {
                let mut it = match loop_stack.pop() {
                    None => {
                        pos += 1;
                        continue;
                    }
                    Some(i) => i,
                };
                if it.index == (it.count - 1) {
                    if it.end_pos == 0 {
                        for i in pos..tokens.len() {
                            match &tokens[i].ttype {
                                TokenType::LoopEnd => {
                                    it.end_pos = i + 1;
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    if it.end_pos > 0 {
                        pos = it.end_pos;
                        continue;
                    }
                } else {
                    loop_stack.push(it);
                }
            }
            TokenType::LoopEnd => {
                if loop_stack.len() > 0 {
                    let mut it = loop_stack.pop().unwrap();
                    it.end_pos = pos + 1;
                    it.index += 1;
                    if it.index < it.count {
                        pos = it.start_pos;
                        loop_stack.push(it);
                        continue;
                    }
                }
            }
            TokenType::Track => exec_track(song, t),
            TokenType::Channel => {
                let v = value_range(1, data_get_int(&t.data), 16) - 1; // CH(1 to 16)
                trk!(song).channel = v as isize;
            }
            TokenType::Voice => exec_voice(song, t),
            TokenType::Note => exec_note(song, t),
            TokenType::NoteN => exec_note_n(song, t),
            TokenType::Rest => exec_rest(song, t),
            TokenType::Length => {
                trk!(song).length = calc_length(&t.data[0].to_s(), song.timebase, song.timebase);
            }
            TokenType::Octave => {
                trk!(song).octave = value_range(0, t.value, 10);
            }
            TokenType::OctaveRel => {
                trk!(song).octave = value_range(0, trk!(song).octave + t.value, 10);
            }
            TokenType::VelocityRel => {
                trk!(song).velocity = value_range(0, trk!(song).velocity + t.value, 127);
            }
            TokenType::OctaveOnce => {
                trk!(song).octave = value_range(0, trk!(song).octave + t.value, 10);
                song.flags.octave_once += t.value;
            }
            TokenType::QLen => {
                trk!(song).qlen = value_range(0, t.value, 100);
                trk!(song).q_on_note = None;
            }
            TokenType::Velocity => {
                let ino = t.data[0].to_i();
                if ino > 0 {
                    while trk!(song).v_sub.len() >= ino as usize {
                        trk!(song).v_sub.push(0);
                    }
                    trk!(song).v_sub[ino as usize] = value_range(0, t.value, 127);
                } else {
                    trk!(song).velocity = value_range(0, t.value, 127);
                }
                trk!(song).v_on_time = None;
                trk!(song).v_on_note = None;
            }
            TokenType::Timing => {
                trk!(song).timing = t.value;
                trk!(song).t_on_note = None;
            }
            TokenType::ControllChange => {
                let no = t.data[0].to_i();
                let val = t.data[1].to_i();
                song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, no, val));
            }
            TokenType::PitchBend => {
                let val = var_extract(&t.data[0], song).to_i();
                let val = if t.value == 0 { val * 128 } else { val + 8192 };
                song.add_event(Event::pitch_bend(
                    trk!(song).timepos,
                    trk!(song).channel,
                    val,
                ));
            }
            TokenType::Tempo => {
                let tempo = data_get_int(&t.data);
                tempo_change(song, tempo);
            }
            TokenType::TempoChange => {
                let data: Vec<isize> = (&t.data[0]).to_int_array();
                if data.len() == 3 {
                    tempo_change_a_to_b(song, data[0], data[1], data[2]);
                } else if data.len() == 2 {
                    tempo_change_a_to_b(song, song.tempo, data[0], data[1]);
                } else {
                    tempo_change(song, data[0]);
                }
            }
            TokenType::MetaText => {
                let mut txt = data_get_str(&t.data, song);
                if txt.len() > 128 {
                    txt = txt.chars().take(40).collect();
                }
                let e = Event::meta(
                    trk!(song).timepos,
                    0xFF,
                    t.value,
                    txt.len() as isize,
                    txt.into_bytes(),
                );
                song.add_event(e);
            }
            TokenType::TimeSignature => {
                song.timesig_frac = t.data[0].to_i();
                song.timesig_deno = t.data[1].to_i();
                song.timesig_deno = match song.timesig_deno {
                    2 => 2,
                    4 => 4,
                    8 => 8,
                    16 => 16,
                    _ => {
                        song.logs
                            .push(String::from("[TimeSignature] value must be 2/4/8/16,n"));
                        4
                    }
                };
                let deno_v = match song.timesig_deno {
                    2 => 1,
                    4 => 2,
                    8 => 3,
                    16 => 4,
                    _ => 2,
                };
                let e = Event::meta(
                    trk!(song).timepos,
                    0xFF,
                    0x58,
                    0x04,
                    vec![song.timesig_frac as u8, deno_v as u8, 0x18, 0x08],
                );
                song.add_event(e);
            }
            TokenType::SysEx => {
                let e = Event::sysex(trk!(song).timepos, &t.data);
                song.add_event(e);
            }
            TokenType::Time => exec_time(song, t),
            TokenType::HarmonyBegin => exec_harmony(song, t, true),
            TokenType::HarmonyEnd => exec_harmony(song, t, false),
            TokenType::Tokens => {
                let _ = match &t.children {
                    Some(tokens) => exec(song, tokens),
                    None => false,
                };
            }
            TokenType::Div => exec_div(song, t),
            TokenType::Sub => exec_sub(song, t),
            TokenType::KeyFlag => song.key_flag = t.data[0].to_int_array(),
            TokenType::KeyShift => {
                song.key_shift = var_extract(&t.data[0], song).to_i();
            }
            TokenType::TrackKey => {
                trk!(song).track_key = var_extract(&t.data[0], song).to_i();
            }
            TokenType::DefInt => {
                let var_key = t.data[0].to_s().clone();
                let var_val = var_extract(&t.data[1], song);
                song.variables.insert(var_key, var_val);
            }
            TokenType::DefStr => {
                let var_key = t.data[0].to_s().clone();
                let var_val = var_extract(&t.data[1], song);
                song.variables.insert(var_key, var_val);
            }
            TokenType::PlayFrom => {
                song.play_from = trk!(song).timepos;
            }
            TokenType::VelocityRandom => {
                trk!(song).v_rand = var_extract(&t.data[0], song).to_i();
            }
            TokenType::TimingRandom => {
                trk!(song).t_rand = var_extract(&t.data[0], song).to_i();
            }
            TokenType::QLenRandom => {
                trk!(song).q_rand = var_extract(&t.data[0], song).to_i();
            }
            TokenType::VelocityOnTime => {
                trk!(song).v_on_time_start = trk!(song).timepos;
                trk!(song).v_on_time = Some(t.data[0].to_int_array());
            }
            TokenType::VelocityOnNote => {
                trk!(song).v_on_note_index = 0;
                trk!(song).v_on_note = Some(t.data[0].to_int_array());
            }
            TokenType::TimingOnNote => {
                trk!(song).t_on_note_index = 0;
                trk!(song).t_on_note = Some(t.data[0].to_int_array());
            }
            TokenType::QLenOnNote => {
                trk!(song).q_on_note_index = 0;
                trk!(song).q_on_note = Some(t.data[0].to_int_array());
            }
            TokenType::CConTime => {
                trk!(song).write_cc_on_time(t.value, t.data[0].to_int_array());
            }
            TokenType::CConTimeFreq => {
                trk!(song).cc_on_time_freq = var_extract(&t.data[0], song).to_i();
            }
            TokenType::PBonTime => {
                trk!(song).write_pb_on_time(t.value, t.data[0].to_int_array(), song.timebase);
            }
            TokenType::MeasureShift => {
                song.flags.measure_shift = var_extract(&t.data[0], song).to_i();
            }
            TokenType::TrackSync => song.track_sync(),
        }
        pos += 1;
    }
    true
}

fn var_extract(val: &SValue, song: &mut Song) -> SValue {
    match val {
        SValue::Str(s, _) => {
            if s.starts_with('=') && s.len() >= 2 {
                let key = &s[1..];
                match song.variables.get(key) {
                    Some(v) => v.clone(),
                    None => {
                        let err_msg = format!("[WARN] Undefined: {}", key);
                        song.logs.push(err_msg);
                        SValue::None
                    }
                }
            } else {
                SValue::from_str(&s)
            }
        }
        _ => val.clone(),
    }
}

fn tempo_change_a_to_b(song: &mut Song, a: isize, b: isize, len: isize) {
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

fn tempo_change(song: &mut Song, tempo: isize) {
    song.tempo = tempo;
    let mpq = if tempo > 0 { 60000000 / tempo } else { 120 };
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

fn exec_sub(song: &mut Song, t: &Token) {
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

fn exec_div(song: &mut Song, t: &Token) {
    let len_s = &t.data[0].to_s();
    let cnt = t.value;
    let length_org: isize;
    let timepos_end: isize;
    {
        let mut trk = &mut song.tracks[song.cur_track];
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
        let mut trk = &mut song.tracks[song.cur_track];
        trk.timepos = timepos_end;
        trk.length = length_org;
    }
}

fn exec_harmony(song: &mut Song, t: &Token, flag_begin: bool) {
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
        // parameters
        if note_qlen < 0 {
            note_qlen = trk!(song).qlen;
        }
        let note_len = calc_length(&note_len_s, song.timebase, trk!(song).length);
        // change event length
        while song.flags.harmony_events.len() > 0 {
            let mut e = song.flags.harmony_events.pop().unwrap();
            e.time = song.flags.harmony_time;
            if note_qlen != 0 {
                e.v2 = note_len * note_qlen / 100;
            }
            trk!(song).events.push(e);
        }
        trk!(song).timepos = song.flags.harmony_time + note_len;
        return;
    }
}

fn exec_time(song: &mut Song, t: &Token) {
    // Calc Time (SakuraObj_time2step)
    // (ref) https://github.com/kujirahand/sakuramml-c/blob/68b62cbc101669211c511258ae1cf830616f238e/src/k_main.c#L473
    let mes = t.data[0].to_i() + song.flags.measure_shift;
    let beat = t.data[1].to_i();
    let tick = t.data[2].to_i();
    // calc
    let base = song.timebase * 4 / song.timesig_deno;
    let total = (mes - 1) * (base * song.timesig_frac) + (beat - 1) * base + tick;
    song.tracks[song.cur_track].timepos = total;
}

fn data_get_int(data: &Vec<SValue>) -> isize {
    if data.len() == 0 {
        return 0;
    }
    data[0].to_i()
}

fn data_get_str(data: &Vec<SValue>, song: &mut Song) -> String {
    if data.len() == 0 {
        return String::new();
    }
    var_extract(&data[0], song).to_s()
}

pub fn calc_length(len_str: &str, timebase: isize, def_len: isize) -> isize {
    let mut res = def_len;
    if len_str == "" {
        return def_len;
    }
    let mut cur = TokenCursor::from(len_str);
    let mut step_mode = false;
    if cur.eq_char('%') {
        cur.next();
        step_mode = true;
    }
    if cur.is_numeric() || cur.eq_char('-') {
        if step_mode {
            res = cur.get_int(0);
        } else {
            let i = cur.get_int(4);
            res = if i > 0 { timebase * 4 / i } else { 0 };
        }
        if cur.peek_n(0) == '.' {
            cur.next();
            res = (res as f32 * 1.5) as isize;
        }
    }
    while !cur.is_eos() {
        let c = cur.peek_n(0);
        if (c != '^') && (c != '+') {
            break;
        }
        cur.next(); // skip '^'
        if cur.eq_char('%') {
            step_mode = true;
            cur.next();
        }
        if cur.is_numeric() || cur.eq_char('-') {
            let mut n = if step_mode {
                cur.get_int(0)
            } else {
                let i = cur.get_int(4);
                if i == 0 {
                    def_len
                } else {
                    timebase * 4 / i
                }
            };
            if cur.peek_n(0) == '.' {
                cur.next();
                n = (n as f32 * 1.5) as isize;
            }
            res += n;
        } else {
            res += def_len;
        }
    }
    res
}

fn exec_note(song: &mut Song, t: &Token) {
    // get parameters
    let note_no = (t.value % 12) as isize;
    let data_note_flag = t.data[0].to_i();
    let data_note_natural = t.data[1].to_i();
    let data_note_len = t.data[2].to_s();
    let data_note_qlen = t.data[3].to_i(); // 0
    let data_note_vel = t.data[4].to_i(); // -1
    let data_note_t = t.data[5].to_i(); // isize::MIN
    let data_note_o = t.data[6].to_i(); // -1
    let _data_slur = t.data[7].to_i(); // 0 or 1 --- TODO: #7
                                       // check parameters
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
    let o = if data_note_o >= 0 {
        data_note_o
    } else {
        trk!(song).octave
    };
    let timepos = trk!(song).timepos;
    // calc
    let mut noteno = o * 12 + note_no + data_note_flag;
    noteno += if data_note_natural == 0 {
        song.key_flag[note_no as usize]
    } else {
        0
    };
    noteno += song.key_shift;
    noteno += trk!(song).track_key;
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
    // note len
    let notelen = calc_length(&data_note_len, song.timebase, trk!(song).length);
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    // check range
    let v = value_range(0, v, 127);
    // event
    let event = Event::note(timepos + t, trk!(song).channel, noteno, notelen_real, v);
    // println!("- {}: note(no={},len={},qlen={},v={},t={},o={})", trk.timepos, noteno, notelen_real, qlen, v, t, o);
    trk!(song).timepos += notelen;

    // harmony?
    if song.flags.harmony_flag {
        trk!(song).timepos = song.flags.harmony_time;
        song.flags.harmony_events.push(event);
    } else {
        trk!(song).events.push(event);
    }

    // octave_once?
    if song.flags.octave_once != 0 {
        trk!(song).octave = trk!(song).octave - song.flags.octave_once;
        song.flags.octave_once = 0;
    }
}

fn exec_note_n(song: &mut Song, t: &Token) {
    // parameters
    let data_note_no = var_extract(&t.data[0], song).to_i();
    let data_note_len = var_extract(&t.data[1], song).to_s();
    let data_note_qlen = var_extract(&t.data[2], song).to_i(); // 0
    let data_note_vel = var_extract(&t.data[3], song).to_i(); // -1
    let data_note_t = var_extract(&t.data[4], song).to_i(); // isize::MIN

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
    // calc
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    // range
    let v = value_range(0, v, 127);
    let event = Event::note(
        trk!(song).timepos + t,
        trk!(song).channel,
        data_note_no,
        notelen_real,
        v,
    );
    // println!("- {}: note(no={},len={},qlen={},v={},t={})", trk!(song).timepos, notelen_real, notelen, qlen, v, t);
    // write event
    trk!(song).events.push(event);
    trk!(song).timepos += notelen;
}

fn exec_rest(song: &mut Song, t: &Token) {
    let trk = &mut song.tracks[song.cur_track];
    let data_note_len = t.data[0].to_s();
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    trk.timepos += notelen * t.value;
}

fn exec_voice(song: &mut Song, t: &Token) {
    // voice no
    let no = var_extract(&t.data[0], song).to_i() - 1;
    // bank ?
    match t.data[1] {
        SValue::None => {
            song.add_event(Event::voice(trk!(song).timepos, trk!(song).channel, no));
            return;
        }
        _ => {
            let bank = var_extract(&t.data[1], song).to_i();
            song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, 0, bank)); // msb
            song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, 0x20, 0)); // lsb
            song.add_event(Event::voice(trk!(song).timepos, trk!(song).channel, no));
        }
    };
}
fn exec_track(song: &mut Song, t: &Token) {
    let mut v = data_get_int(&t.data); // TR=0..
    if v < 0 {
        v = 0;
    }
    song.cur_track = v as usize;
    // new track ?
    while song.tracks.len() <= song.cur_track {
        println!("{:?}", v);
        let trk = Track::new(song.timebase, v - 1);
        song.tracks.push(trk);
    }
}

pub fn value_range(min_v: isize, value: isize, max_v: isize) -> isize {
    let mut v = value;
    if v < min_v {
        v = min_v;
    } else if v > max_v {
        v = max_v;
    }
    v
}

/// exec source (easy version)
pub fn exec_easy(src: &str) -> Song {
    let mut song = Song::new();
    let t = &lex(&mut song, src, 0);
    exec(&mut song, &t);
    song
}

#[cfg(test)]
mod tests {
    use crate::song::EventType;

    use super::*;
    #[test]
    fn test_calc_len() {
        assert_eq!(calc_length("4", 480, 480), 480);
        assert_eq!(calc_length("", 480, 480), 480);
        assert_eq!(calc_length("8", 480, 480), 240);
        assert_eq!(calc_length("8^", 480, 240), 480);
        assert_eq!(calc_length("^^^", 480, 240), 480 * 2);
        assert_eq!(calc_length("4.", 480, 480), 480 + 240);
        assert_eq!(calc_length("4.^", 480, 240), 240 * 4);
    }
    #[test]
    fn test_calc_len2() {
        assert_eq!(calc_length("4", 96, 48), 96);
        assert_eq!(calc_length("", 96, 48), 48);
        assert_eq!(calc_length("^", 96, 48), 96);
        assert_eq!(calc_length("^4", 96, 48), 48 + 96);
    }
    #[test]
    fn test_calc_len3() {
        assert_eq!(calc_length("2", 48, 48), 96);
        assert_eq!(calc_length("4^4", 48, 48), 96);
        assert_eq!(calc_length("4.^8", 48, 48), 96);
        assert_eq!(calc_length("8^4.", 48, 48), 96);
    }
    #[test]
    fn test_calc_len_step() {
        assert_eq!(calc_length("%96", 96, 96), 96);
        assert_eq!(calc_length("4^%1", 96, 96), 97);
        assert_eq!(calc_length("^%2", 96, 96), 98);
        assert_eq!(calc_length("^%-1", 96, 48), 47);
    }
    #[test]
    fn test_calc_len_plus() {
        assert_eq!(calc_length("4+4", 96, 96), 96 * 2);
        assert_eq!(calc_length("8+", 96, 48), 96);
    }
    #[test]
    fn test_exec1() {
        assert_eq!(exec_easy("PRINT{1}").get_logs_str(), "[PRINT](0) 1");
        assert_eq!(exec_easy("PRINT{abc}").get_logs_str(), "[PRINT](0) abc");
        assert_eq!(
            exec_easy("STR A={abc} PRINT=A").get_logs_str(),
            "[PRINT](0) abc"
        );
    }
    #[test]
    fn test_exec_harmony() {
        let song = exec_easy("q100 l8 'dg'^^^");
        let e = &song.tracks[0].events[0];
        assert_eq!(e.etype, EventType::NoteOn);
        assert_eq!(e.v2, 96 * 2);
    }
    #[test]
    fn test_exec_track_sync() {
        //
        let song = exec_easy("TR=1 l4 cdef TR=2 c TrackSync;");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96);
        //
        let song = exec_easy("TR=0 l4 c TR=2 cdef TrackSync;");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96 * 4);
    }
    #[test]
    fn test_exec_mes_shift() {
        //
        let song = exec_easy("System.MeasureShift=1;TR=0 TIME(1:1:0)");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96 * 4);
    }
    #[test]
    fn test_lex_macro_str() {
        //
        let song = exec_easy("#A={o#?1} #A(0) c");
        assert_eq!(song.tracks[0].events[0].v1, 0);
        //
        let song = exec_easy("STR AAA={o#?1} AAA(0) d");
        assert_eq!(song.tracks[0].events[0].v1, 2);
        //
        let song = exec_easy("STR BBB={o0 #?1 #?2 #?3} BBB({c},{d},{e})");
        assert_eq!(song.tracks[0].events[0].v1, 0);
        assert_eq!(song.tracks[0].events[1].v1, 2);
        assert_eq!(song.tracks[0].events[2].v1, 4);
    }
}
