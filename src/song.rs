use crate::cursor::TokenCursor;

/// song & track
use super::token::{Token, TokenType};
use super::svalue::SValue;

#[derive(Debug)]
pub enum EventType {
    NoteNo,
    ControllChange,
    PitchBend,
    Voice,
    Meta,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Event {
    pub etype: EventType,
    pub time: isize,
    pub channel: isize,
    pub v1: isize,
    pub v2: isize,
    pub v3: isize,
    pub data: Option<Vec<u8>>,
}
impl Event {
    pub fn note(time: isize, channel: isize, note_no: isize, len: isize, vel: isize) -> Self {
        Self { etype: EventType::NoteNo, time, channel, v1: note_no, v2: len, v3: vel, data: None }
    }
    pub fn voice(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::Voice, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
    pub fn meta(time: isize, v1: isize, v2: isize, v3: isize, data_v: Vec<u8>) -> Self {
        Self { etype: EventType::Meta, time, channel: 0, v1, v2, v3, data: Some(data_v) }
    }
    pub fn cc(time: isize, channel: isize, no: isize, value: isize) -> Self {
        Self { etype: EventType::ControllChange, time, channel, v1: no, v2: value, v3:0, data: None }
    }
    pub fn pitch_bend(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::PitchBend, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct Track {
    timepos: isize,
    length: isize,
    octave: isize,
    velocity: isize,
    qlen: isize,
    timing: isize,
    pub channel: isize,
    pub events: Vec<Event>,
}

impl Track {
    pub fn new(timebase: isize, channel: isize) -> Self {
        Track {
            timepos: 0,
            length: timebase,
            velocity: 100,
            octave: 5,
            qlen: 90,
            timing: 0,
            channel,
            events: vec![],
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Song {
    pub debug: bool,
    pub tracks: Vec<Track>,
    pub timebase: isize,
    pub cur_track: usize,
    pub logs: Vec<String>,
    pub timesig_frac: isize, // 分子
    pub timesig_deno: isize, // 分母
}

impl Song {
    pub fn new() -> Self {
        let timebase = 96;
        let trk = Track::new(timebase, 0);
        Self {
            debug: false,
            timebase,
            tracks: vec![trk],
            cur_track: 0,
            logs: vec![],
            timesig_frac: 4,
            timesig_deno: 4,
        }
    }
    pub fn add_event(&mut self, e: Event) {
        self.tracks[self.cur_track].events.push(e);
    }
}

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

fn data_get_int(data: &Vec<SValue>) -> isize {
    if data.len() == 0 { return 0; }
    data[0].to_i()
}

fn data_get_str(data: &Vec<SValue>) -> String {
    if data.len() == 0 { return String::new(); }
    data[0].to_s()
}

pub fn calc_length(len_str: &str, timebase: isize, def_len: isize) -> isize {
    let mut res = def_len;
    if len_str == "" { return def_len; }
    let mut cur = TokenCursor::from(len_str);
    if cur.is_numeric() {
        let i = cur.get_int(4);
        res = timebase * 4 / i;
        if cur.peek_n(0) == '.' {
            cur.next();
            res = (res as f32 * 1.5) as isize;
        }
    }
    while !cur.is_eos() {
        if cur.peek_n(0) != '^' { break; }
        if cur.is_numeric() {
            let i = cur.get_int(4);
            let mut n = timebase * 4 / i;
            if cur.peek_n(0) == '.' {
                cur.next();
                n = (res as f32 * 1.5) as isize;
            }
            res += n;
        } else {
            res += def_len;
        }
        cur.next()
    }
    res
}

fn exec_note(song: &mut Song, t: &Token) {
    let trk = &mut song.tracks[song.cur_track];
    let data_note_flag = t.data[0].to_i();
    let data_note_len = t.data[1].to_s();
    let mut data_note_qlen = t.data[2].to_i();
    let mut data_note_vel = t.data[3].to_i();
    let o = trk.octave;
    let noteno = o * 12 + t.value + data_note_flag;
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    if data_note_qlen <= 0 { data_note_qlen = trk.qlen; }
    let notelen_real = (notelen as f32 * data_note_qlen as f32 / 100.0) as isize;
    if data_note_vel <= 0 { data_note_vel = trk.velocity; }
    let event = Event::note(trk.timepos, trk.channel, noteno, notelen_real, data_note_vel);
    // println!("- {}: note(no={},len={},vel={})", trk.timepos, noteno, notelen_real, data_note_vel);
    trk.events.push(event);
    trk.timepos += notelen;
}

fn exec_note_n(song: &mut Song, t: &Token) {
    let trk = &mut song.tracks[song.cur_track];
    let data_note_no = t.data[0].to_i();
    let data_note_len = t.data[1].to_s();
    let mut data_note_qlen = t.data[2].to_i();
    let mut data_note_vel = t.data[3].to_i();
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    if data_note_qlen <= 0 { data_note_qlen = trk.qlen; }
    let notelen_real = (notelen as f32 * data_note_qlen as f32 / 100.0) as isize;
    if data_note_vel <= 0 { data_note_vel = trk.velocity; }
    let event = Event::note(trk.timepos, trk.channel, data_note_no, notelen_real, data_note_vel);
    trk.events.push(event);
    trk.timepos += notelen;
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

pub fn exec(song: &mut Song, tokens: &Vec<Token>) -> bool {
    let mut pos = 0;
    let mut loop_stack: Vec<LoopItem> = vec![];
    while pos < tokens.len() {
        let t = &tokens[pos];
        if song.debug {
            println!("{:3}:exec:{:?}", pos, t);
        }
        match t.ttype {
            // Loop controll
            TokenType::LoopBegin => {
                let mut it = LoopItem::new();
                it.start_pos = pos + 1;
                it.count = t.value as usize;
                loop_stack.push(it);
            },
            TokenType::LoopBreak => {
                let mut it = loop_stack.pop().unwrap();
                if it.index == (it.count-1) {
                    if it.end_pos == 0 {
                        for i  in pos..tokens.len() {
                            match &tokens[i].ttype {
                                TokenType::LoopEnd => {
                                    it.end_pos = i;
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
                let trk = &song.tracks[song.cur_track];
                song.add_event(Event::voice(trk.timepos, trk.channel, t.value));
            },
            TokenType::Note => exec_note(song, t),
            TokenType::NoteN => exec_note_n(song, t),
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
                let trk = &song.tracks[song.cur_track];
                let val = t.value * 128;
                song.add_event(Event::pitch_bend(trk.timepos, trk.channel, val));
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
                let trk = &song.tracks[song.cur_track];
                let txt = data_get_str(&t.data);
                let e = Event::meta(trk.timepos, 0xFF, t.value, txt.len() as isize, txt.into_bytes());
                song.add_event(e);
            },
            TokenType::TimeSignature => {
                let trk = &song.tracks[song.cur_track];
                song.timesig_frac = t.data[0].to_i();
                song.timesig_deno = t.data[1].to_i();
                let e = Event::meta(trk.timepos, 0xFF, 0x58, 0x04, vec![
                    song.timesig_frac as u8,
                    (song.timesig_deno as f32).sqrt() as u8,
                    0x18,
                    0x08
                ]);
                song.add_event(e);
            },
            _ => {
                println!("[TODO] {:?}", t);
            }
        }
        pos += 1;
    }
    true
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
}
