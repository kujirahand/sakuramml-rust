/// song & track
use super::token::{Token, TokenType};

#[allow(dead_code)]
pub struct Track {
    length: isize,
    octave: isize,
    qlen: isize,
    timing: isize,
}

impl Track {
    pub fn new(timebase: isize) -> Self {
        Track {
            length: timebase,
            octave: 5,
            qlen: 90,
            timing: 0,
        }
    }
}

#[allow(dead_code)]
pub struct Song {
    tracks: Vec<Track>,
    timebase: isize,
    cur_track: usize,
}

impl Song {
    pub fn new() -> Self {
        let timebase = 960;
        let trk = Track::new(timebase);
        Self {
            timebase: timebase,
            tracks: vec![trk],
            cur_track: 0,
        }
    }
}

pub fn exec(_song: &mut Song, tokens: &Vec<Token>) -> bool {
    let mut pos = 0;
    while pos < tokens.len() {
        let t = &tokens[pos];
        match t.ttype {
            TokenType::Note => {

            },
            _ => {}
        }
        pos += 1;
    }
    true
}