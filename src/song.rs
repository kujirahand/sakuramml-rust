/// song & track

#[derive(Debug, Clone)]
pub enum EventType {
    NoteNo,
    NoteOff,
    ControllChange,
    PitchBend,
    Voice,
    Meta,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
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
    pub fn normalize(&mut self) {
        let mut events: Vec<Event> = vec![];
        for i in 0..self.events.len() {
            let e = &self.events[i];
            match e.etype {
                EventType::NoteNo => {
                    events.push(e.clone());
                    let mut noteoff = e.clone();
                    noteoff.etype = EventType::NoteOff;
                    noteoff.time = e.time + e.v2;
                    events.push(noteoff);
                },
                _ => {
                    events.push(e.clone());
                }
            }
        }
        self.events = events;
    }
    pub fn events_sort(&mut self) {
        self.events.sort_by(|a, b| a.time.cmp(&b.time));
    }
}

#[derive(Debug)]
pub struct Flags {
    pub harmony_flag: bool,
    pub harmony_time: isize,
    pub harmony_events: Vec<Event>,
}
impl Flags {
    pub fn new() -> Self {
        Flags {
            harmony_flag: false,
            harmony_time: 0,
            harmony_events: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Song {
    pub debug: bool,
    pub tracks: Vec<Track>,
    pub timebase: isize,
    pub cur_track: usize,
    pub timesig_frac: isize, // 分子
    pub timesig_deno: isize, // 分母
    pub flags: Flags,
    pub rhthm_macro: Vec<String>,
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
            timesig_frac: 4,
            timesig_deno: 4,
            flags: Flags::new(),
            rhthm_macro: init_rhythm_macro(),
            logs: vec![],
        }
    }
    pub fn add_event(&mut self, e: Event) {
        self.tracks[self.cur_track].events.push(e);
    }
    pub fn normalize_and_sort(&mut self) {
        for trk in self.tracks.iter_mut() {
            trk.normalize();
            trk.events_sort();
        }
    }
}

fn init_rhythm_macro() -> Vec<String> {
    // Rhythm macro ... 1 char macro
    let mut rhthm_macro: Vec<String> = vec![];
    for _ in 0x40..=0x7F {
        rhthm_macro.push(String::new());
    }
    // set
    rhthm_macro['b' as usize - 0x40] = String::from("n36,");
    rhthm_macro['s' as usize - 0x40] = String::from("n38,");
    rhthm_macro['h' as usize - 0x40] = String::from("n42,");
    rhthm_macro['H' as usize - 0x40] = String::from("n44,");
    rhthm_macro['o' as usize - 0x40] = String::from("n46,");
    rhthm_macro['c' as usize - 0x40] = String::from("n49,");
    //
    rhthm_macro
}
