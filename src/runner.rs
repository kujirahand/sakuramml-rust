use super::song::{Song, Track, Event};
use super::token::{Token, TokenType};
use super::svalue::SValue;
use super::cursor::TokenCursor;

#[derive(Debug)]
pub struct LoopItem {
    pub start_pos: usize,
    pub end_pos: usize,
    pub index: usize,
    pub count: usize,
}

impl LoopItem {
    fn new() -> Self {
        LoopItem { start_pos: 0, end_pos: 0, index: 0, count: 0 }
    }
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
            },
            TokenType::Error => {
                if song.debug {
                    println!("[ERROR]");
                }
            },
            // Loop controll
            TokenType::LoopBegin => {
                let mut it = LoopItem::new();
                it.start_pos = pos + 1;
                it.count = var_extract(&t.data[0], song).to_i() as usize;
                // println!("loop={}", it.count);
                loop_stack.push(it);
            },
            TokenType::LoopBreak => {
                let mut it = loop_stack.pop().unwrap();
                if it.index == (it.count-1) {
                    if it.end_pos == 0 {
                        for i  in pos..tokens.len() {
                            match &tokens[i].ttype {
                                TokenType::LoopEnd => {
                                    it.end_pos = i + 1;
                                    break;
                                },
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
            },
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
            },
            TokenType::Track => exec_track(song, t),
            TokenType::Channel => {
                let v = value_range(1, data_get_int(&t.data), 16) - 1; // CH(1 to 16)
                song.tracks[song.cur_track].channel = v as isize;
            },
            TokenType::Voice => {
                let no = var_extract(&t.data[0], song);
                let trk = &song.tracks[song.cur_track];
                song.add_event(Event::voice(trk.timepos, trk.channel, no.to_i()));
            },
            TokenType::Note => exec_note(song, t),
            TokenType::NoteN => exec_note_n(song, t),
            TokenType::Rest => exec_rest(song, t),
            TokenType::Length => {
                let mut trk = &mut song.tracks[song.cur_track];
                trk.length = calc_length(&t.data[0].to_s(), song.timebase, song.timebase);
            },
            TokenType::Octave => {
                let mut trk = &mut song.tracks[song.cur_track];
                trk.octave = value_range(0, t.value, 10);
            },
            TokenType::OctaveRel => {
                let mut trk = &mut song.tracks[song.cur_track];
                trk.octave = value_range(0, trk.octave + t.value, 10);
            },
            TokenType::VelocityRel => {
                let mut trk = &mut song.tracks[song.cur_track];
                trk.velocity = value_range(0, trk.velocity + t.value, 127);
            },
            TokenType::OctaveOnce => {
                let mut trk = &mut song.tracks[song.cur_track];
                trk.octave = value_range(0, trk.octave + t.value, 10);
                song.flags.octave_once += t.value;
            },
            TokenType::QLen => {
                let mut trk = &mut song.tracks[song.cur_track];
                trk.qlen = value_range(0, t.value, 100);
            },
            TokenType::Velocity => {
                let mut trk = &mut song.tracks[song.cur_track];
                trk.velocity = value_range(0, t.value, 127);
            },
            TokenType::ControllChange => {
                let trk = &song.tracks[song.cur_track];
                let no = t.data[0].to_i();
                let val = t.data[1].to_i();
                song.add_event(Event::cc(trk.timepos, trk.channel, no, val));
            },
            TokenType::PitchBend => {
                let val = var_extract(&t.data[0], song).to_i();
                let val = if t.value == 0 { val * 128 } else { val + 8192 };
                song.add_event(Event::pitch_bend(song.tracks[song.cur_track].timepos, song.tracks[song.cur_track].channel, val));
            },
            TokenType::Tempo => {
                let trk = &song.tracks[song.cur_track];
                let tempo = data_get_int(&t.data);
                let mpq = 60000000 / tempo;
                let e = Event::meta(trk.timepos, 0xFF, 0x51, 0x03, vec![
                    (mpq >> 16 & 0xFF) as u8,
                    (mpq >>  8 & 0xFF) as u8,
                    (mpq >>  0 & 0xFF) as u8,
                ]);
                song.add_event(e);
            },
            TokenType::MetaText => {
                let txt = data_get_str(&t.data, song);
                let e = Event::meta(song.tracks[song.cur_track].timepos, 0xFF, t.value, txt.len() as isize, txt.into_bytes());
                song.add_event(e);
            },
            TokenType::TimeSignature => {
                let trk = &song.tracks[song.cur_track];
                song.timesig_frac = t.data[0].to_i();
                song.timesig_deno = t.data[1].to_i();
                song.timesig_deno = match song.timesig_deno {
                    4 => 4,
                    8 => 8,
                    _ => {
                        song.logs.push(String::from("[TimeSignature] value must be 4,4 or 6,8"));
                        4
                    }
                };
                let deno_v = match song.timesig_deno {
                    4 => 2,
                    8 => 3,
                    _ => 2,
                };
                let e = Event::meta(trk.timepos, 0xFF, 0x58, 0x04, vec![
                    song.timesig_frac as u8,
                    deno_v as u8,
                    0x18,
                    0x08
                ]);
                song.add_event(e);
            },
            TokenType::SysEx => {
                let e = Event::sysex(song.tracks[song.cur_track].timepos, &t.data);
                song.add_event(e);
            },
            TokenType::Time => exec_time(song, t),
            TokenType::HarmonyBegin => exec_harmony(song, t, true),
            TokenType::HarmonyEnd => exec_harmony(song, t, false),
            TokenType::Tokens => {
                let _ = match &t.children {
                    Some(tokens) => exec(song, tokens),
                    None => false,
                };
            },
            TokenType::Div => exec_div(song, t),
            TokenType::Sub => exec_sub(song, t),
            TokenType::KeyFlag => song.key_flag = t.data[0].to_int_array(),
            TokenType::DefInt => {
                let var_key = t.data[0].to_s().clone();
                let var_val = var_extract(&t.data[1], song);
                song.variables.insert(var_key, var_val);
            },
            TokenType::DefStr => {
                let var_key = t.data[0].to_s().clone();
                let var_val = var_extract(&t.data[1], song);
                song.variables.insert(var_key, var_val);
            },
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
                    Some(v) => {
                        v.clone()
                    },
                    None => {
                        let err_msg = format!("[WARN] Undefined: {}", key);
                        song.logs.push(err_msg);
                        SValue::None
                    },
                }
            } else {
                SValue::from_str(&s)
            }
        },
        _ => {
            val.clone()
        }
    }
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
        let note_len = div_len / cnt;
        timepos_end = trk.timepos + div_len;
        length_org = trk.length;
        trk.length = note_len;
    }
    let _ = match &t.children {
        None => false,
        Some(tokens) => {
            exec(song, tokens)
        },
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
        let mut trk = &mut song.tracks[song.cur_track];
        let note_len_s = t.data[0].to_s();
        let note_qlen = t.data[1].to_i();
        let note_len = calc_length(&note_len_s, song.timebase, trk.length);
        // change event length
        while song.flags.harmony_events.len() > 0 {
            let mut e = song.flags.harmony_events.pop().unwrap();
            e.time = song.flags.harmony_time;
            if note_qlen != 0 {
                e.v2 = note_len * note_qlen / 100;
            }
            trk.events.push(e);
        }
        trk.timepos = song.flags.harmony_time + note_len;
        return;
    }
}

fn exec_time(song: &mut Song, t: &Token) {
    // Calc Time (SakuraObj_time2step)
    // (ref) https://github.com/kujirahand/sakuramml-c/blob/68b62cbc101669211c511258ae1cf830616f238e/src/k_main.c#L473
    let mes = t.data[0].to_i();
    let beat = t.data[1].to_i();
    let tick = t.data[2].to_i();
    // calc
    let base = song.timebase * 4 / song.timesig_deno;
    let total = (mes-1) * (base * song.timesig_frac) + (beat-1) * base + tick;
    song.tracks[song.cur_track].timepos = total;
}

fn data_get_int(data: &Vec<SValue>) -> isize {
    if data.len() == 0 { return 0; }
    data[0].to_i()
}

fn data_get_str(data: &Vec<SValue>, song: &mut Song) -> String {
    if data.len() == 0 { return String::new(); }
    var_extract(&data[0], song).to_s()
}

pub fn calc_length(len_str: &str, timebase: isize, def_len: isize) -> isize {
    let mut res = def_len;
    if len_str == "" { return def_len; }
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
            res = timebase * 4 / i;
        }
        if cur.peek_n(0) == '.' {
            cur.next();
            res = (res as f32 * 1.5) as isize;
        }
    }
    while !cur.is_eos() {
        if cur.peek_n(0) != '^' { break; }
        cur.next(); // skip '^'
        if cur.eq_char('%') {
            step_mode = true;
            cur.next();
        }
        if cur.is_numeric() || cur.eq_char('-') {
            let mut n = if step_mode {
                cur.get_int(0)
            } else {
                let i = cur.get_int(0);
                if i == 0 { def_len } else { timebase * 4 / i }
            };
            if cur.peek_n(0) == '.' {
                cur.next();
                n = (res as f32 * 1.5) as isize;
            }
            res += n;
        } else {
            res += def_len;
        }
    }
    res
}

fn exec_note(song: &mut Song, t: &Token) {
    let trk = &mut song.tracks[song.cur_track];
    // get parameters
    let note_no = (t.value % 12) as isize;
    let data_note_flag = t.data[0].to_i();
    let data_note_len = t.data[1].to_s();
    let data_note_qlen = t.data[2].to_i(); // 0
    let data_note_vel = t.data[3].to_i(); // -1
    let data_note_t = t.data[4].to_i(); // isize::MIN
    let data_note_o = t.data[5].to_i(); // -1
    // check parameters
    let qlen = if data_note_qlen != 0 { data_note_qlen } else { trk.qlen };
    let v = if data_note_vel >= 0 { data_note_vel } else { trk.velocity };
    let t = if data_note_t != isize::MIN { data_note_t } else { trk.timing };
    let o = if data_note_o >= 0 { data_note_o } else { trk.octave };
    // calc
    let noteno = (o * 12 + note_no + data_note_flag + song.key_flag[note_no as usize]) & 0x7F;
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    let event = Event::note(trk.timepos + t, trk.channel, noteno, notelen_real, v);
    // println!("- {}: note(no={},len={},qlen={},v={},t={},o={})", trk.timepos, noteno, notelen_real, qlen, v, t, o);
    trk.timepos += notelen;
    
    // harmony?
    if song.flags.harmony_flag {
        trk.timepos = song.flags.harmony_time;
        song.flags.harmony_events.push(event);
    } else {
        trk.events.push(event);
    }

    // octave_once?
    if song.flags.octave_once != 0 {
        trk.octave = trk.octave - song.flags.octave_once;
        song.flags.octave_once = 0;
    }
}

fn exec_note_n(song: &mut Song, t: &Token) {
    let data_note_no;
    let data_note_len;
    let data_note_qlen;
    let data_note_vel;
    let data_note_t;
    // block for &mut song borrow checker
    {
        data_note_no = var_extract(&t.data[0], song).to_i();
        data_note_len = var_extract(&t.data[1], song).to_s();
        data_note_qlen = var_extract(&t.data[2], song).to_i(); // 0
        data_note_vel = var_extract(&t.data[3], song).to_i(); // -1
        data_note_t = var_extract(&t.data[4], song).to_i(); // isize::MIN
    }
    // parametes
    let trk = &mut song.tracks[song.cur_track];
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    let qlen = if data_note_qlen != 0 { data_note_qlen } else { trk.qlen };
    let v = if data_note_vel >= 0 { data_note_vel } else { trk.velocity };
    let t = if data_note_t != isize::MIN { data_note_t } else { trk.timing };
    // calc
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    let event = Event::note(trk.timepos + t, trk.channel, data_note_no, notelen_real, v);
    // println!("- {}: note(no={},len={},qlen={},v={},t={})", trk.timepos, notelen_real, notelen, qlen, v, t);
    // write event
    trk.events.push(event);
    trk.timepos += notelen;
}

fn exec_rest(song: &mut Song, t: &Token) {
    let trk = &mut song.tracks[song.cur_track];
    let data_note_len = t.data[0].to_s();
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    trk.timepos += notelen * t.value;
}

fn exec_track(song: &mut Song, t: &Token) {
    let mut v = data_get_int(&t.data) - 1; // TR(1 to xxx)
    if v < 0 { v = 0; }
    song.cur_track = v as usize;
    // new track ?
    while song.tracks.len() <= song.cur_track {
        let no = song.tracks.len() as isize;
        let trk = Track::new(song.timebase, no);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calc_len() {
        assert_eq!(calc_length("4", 480, 480), 480);
        assert_eq!(calc_length("", 480, 480), 480);
        assert_eq!(calc_length("8", 480, 480), 240);
        assert_eq!(calc_length("8^", 480, 240), 480);
        assert_eq!(calc_length("^^^", 480, 240), 480*2);
        assert_eq!(calc_length("4.", 480, 480), 480 + 240);
        assert_eq!(calc_length("4.^", 480, 240), 240*4);
    }
    #[test]
    fn test_calc_le2() {
        assert_eq!(calc_length("4", 96, 48), 96);
        assert_eq!(calc_length("", 96, 48), 48);
        assert_eq!(calc_length("^", 96, 48), 96);
        assert_eq!(calc_length("^4", 96, 48), 48+96);
    }
    #[test]
    fn test_calc_len_step() {
        assert_eq!(calc_length("%96", 96, 96), 96);
        assert_eq!(calc_length("4^%1", 96, 96), 97);
        assert_eq!(calc_length("^%2", 96, 96), 98);
        assert_eq!(calc_length("^%-1", 96, 48), 47);
    }
}
