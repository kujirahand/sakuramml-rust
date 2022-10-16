/// song & track

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
    pub timepos: isize,
    pub length: isize,
    pub octave: isize,
    pub velocity: isize,
    pub qlen: isize,
    pub timing: isize,
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
    pub timesig_frac: isize, // 分子
    pub timesig_deno: isize, // 分母
    pub logs: Vec<String>, // ログ
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


