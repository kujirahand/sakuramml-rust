use crate::cursor::TokenCursor;

/// song & track
use super::token::{Token, TokenType};
use super::svalue::SValue;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Event {
    Note(isize, isize, isize, isize), // time, NoteNo, Length, Velocity
    CC(isize, isize, isize), // time, No, Value
    PitchBend(isize, isize), // time, No
    Voice(isize, isize), // time, No
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
        let timebase = 960;
        let trk = Track::new(timebase, 0);
        Self {
            timebase,
            tracks: vec![trk],
            cur_track: 0,
        }
    }
    pub fn add_event(&mut self, event: Event) {
        self.tracks[self.cur_track].events.push(event);
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
    let trk = &song.tracks[song.cur_track];
    let data = &t.data;
    let data_note_flag = t.data[0].to_i();
    let data_note_len = t.data[1].to_s();
    let data_note_vel = t.data[2].to_i();
    let o = trk.octave;
    let noteno = o * 12 + t.value + data_note_flag;
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    let mut qlen = data[1].to_i();
    if qlen <= 0 { qlen = trk.qlen; }
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    let mut vel = data_note_vel;
    if vel <= 0 { vel = trk.velocity; }
    let event = Event::Note(trk.timepos, noteno, notelen_real, vel);
    song.add_event(event);
}

fn exec_track(song: &mut Song, t: &Token) {
    let mut v = data_get_int(&t.data) - 1; // TR(1 to xxx)
    if v < 0 { v = 0; }
    song.cur_track = v as usize;
    // new track ?
    while song.tracks.len() >= song.cur_track {
        let no = song.tracks.len() as isize;
        let trk = Track::new(song.timebase, no);
        song.tracks.push(trk);
    }
}

pub fn exec(song: &mut Song, tokens: &Vec<Token>) -> bool {
    let mut pos = 0;
    while pos < tokens.len() {
        let t = &tokens[pos];
        println!("exec:{:?}", t);
        match t.ttype {
            TokenType::Track => exec_track(song, t),
            TokenType::Channel => {
                let mut v = data_get_int(&t.data) - 1; // CH(1 to 16)
                if v < 0 { v = 0; } else if v > 15 { v = 15; }
                song.tracks[song.cur_track].channel = v as isize;
            }
            TokenType::Note => exec_note(song, t),
            _ => {}
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
