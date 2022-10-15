use crate::cursor::TokenCursor;

/// song & track
use super::token::{Token, TokenType};
use super::svalue::SValue;

#[derive(Debug)]
pub enum EventType {
    NoteNo,
    CC,
    PitchBend,
    Voice,
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
}
impl Event {
    pub fn note(time: isize, channel: isize, note_no: isize, len: isize, vel: isize) -> Self {
        Self { etype: EventType::NoteNo, time, channel, v1: note_no, v2: len, v3: vel }
    }
    pub fn voice(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::Voice, time, channel, v1: value, v2: 0, v3: 0 }
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
    pub tracks: Vec<Track>,
    pub timebase: isize,
    pub cur_track: usize,
}

impl Song {
    pub fn new() -> Self {
        let timebase = 96;
        let trk = Track::new(timebase, 0);
        Self {
            timebase,
            tracks: vec![trk],
            cur_track: 0,
        }
    }
    pub fn add_event(&mut self, e: Event) {
        self.tracks[self.cur_track].events.push(e);
    }
}

fn data_get_int(data: &Vec<SValue>) -> isize {
    if data.len() == 0 { return 0; }
    data[0].to_i()
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
    let data = &t.data;
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
    println!("- {}: note(no={},len={},vel={})", trk.timepos, noteno, notelen_real, data_note_vel);
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
    while pos < tokens.len() {
        let t = &tokens[pos];
        println!("{:3}:exec:{:?}", pos, t);
        match t.ttype {
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
            _ => {
                println!("TODO");
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
