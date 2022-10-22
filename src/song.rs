/// song & track
use std::collections::HashMap;
use crate::runner::value_range;
use crate::svalue::SValue;
use crate::mml_def;

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    NoteOn,
    NoteOff,
    ControllChange,
    PitchBend,
    Voice,
    Meta,
    SysEx,
}

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
        Self { etype: EventType::NoteOn, time, channel, v1: note_no, v2: len, v3: vel, data: None }
    }
    pub fn voice(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::Voice, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
    pub fn meta(time: isize, v1: isize, v2: isize, v3: isize, data_v: Vec<u8>) -> Self {
        Self { etype: EventType::Meta, time, channel: 0, v1, v2, v3, data: Some(data_v) }
    }
    pub fn sysex(time: isize, data_v: &Vec<SValue>) -> Self {
        let mut a: Vec<u8> = vec![];
        for v in data_v.iter() {
            a.push(v.to_i() as u8);
        }
        Self { etype: EventType::SysEx, time, channel: 0, v1: 0, v2: 0, v3: 0, data: Some(a) }
    }
    pub fn cc(time: isize, channel: isize, no: isize, value: isize) -> Self {
        Self { etype: EventType::ControllChange, time, channel, v1: no, v2: value, v3:0, data: None }
    }
    pub fn pitch_bend(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::PitchBend, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
}


#[derive(Debug)]
pub struct Track {
    pub timepos: isize,
    pub channel: isize,
    pub length: isize,
    pub octave: isize,
    pub velocity: isize,
    pub qlen: isize,
    pub timing: isize,
    pub v_rand: isize,
    pub t_rand: isize,
    pub v_on_time_start: isize,
    pub v_on_time: Option<Vec<isize>>,
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
            v_rand: 0,
            t_rand: 0,
            v_on_time_start: -1,
            v_on_time: None,
            channel,
            events: vec![],
        }
    }

    pub fn split_note_off(&self) -> Vec<Event> {
        let mut events: Vec<Event> = vec![];
        for i in 0..self.events.len() {
            let e = &self.events[i];
            match e.etype {
                EventType::NoteOn => {
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
        events
    }

    pub fn normalize(&mut self) {
        let events: Vec<Event> = self.split_note_off();
        self.events = events;
    }
    pub fn events_sort(&mut self) {
        // sort_byなら要素の順序は保持される
        self.events.sort_by(|a, b| a.time.cmp(&b.time));
    }
    pub fn play_from(&mut self, timepos: isize) {
        let mut events: Vec<Event> = vec![];
        let mut cc_values: Vec<isize> = vec![];
        let mut voice: isize = -1;
        let mut ch: isize = 0;
        for _ in 0..128 { cc_values.push(-1); }
        for e in self.events.iter() {
            match e.etype {
                EventType::Meta | EventType::SysEx => {
                    let mut e2 = e.clone();
                    e2.time -= timepos;
                    if e2.time < 0 { e2.time = 0; }
                    events.push(e2);
                },
                EventType::NoteOn => {
                    let mut e2 = e.clone();
                    e2.time -= timepos;
                    if e2.time < 0 { continue; }
                    events.push(e2);
                },
                EventType::Voice => {
                    let mut e2 = e.clone();
                    e2.time -= timepos;
                    if e2.time < 0 {
                        voice = e2.v1;
                        ch = e2.channel;
                        continue;
                    }
                    events.push(e2);
                },
                EventType::ControllChange => {
                    let mut e2 = e.clone();
                    e2.time -= timepos;
                    if e2.time < 0 {
                        cc_values[e2.v1 as usize] = e2.v2;
                        ch = e2.channel;
                        continue;
                    }
                    events.push(e2);
                },
                EventType::NoteOff => {},
                EventType::PitchBend => {},
            }
        }
        // add cc
        for no in 0..128 {
            if cc_values[no] < 0 { continue; }
            events.push(Event::cc(0, ch, no as isize, cc_values[no as usize]));
        }
        // voice
        if voice >= 0 {
            events.push(Event::voice(0, ch, voice));
        }
        self.events = events;
    }
    pub fn calc_v_on_time(&mut self, def: isize) -> isize {
        let start_time = self.v_on_time_start;
        let cur_time = self.timepos - start_time;
        let mut result = isize::MIN;
        // on_time?
        let ia = match &self.v_on_time {
            None => return def,
            Some(pia) => pia.clone()
        };
        let mut area_time = 0;
        for i in 0..ia.len() / 3 {
            let low = ia[i*3+0];
            let high = ia[i*3+1];
            let len = ia[i*3+2];
            let area_time_to = area_time + len;
            if area_time <= cur_time && cur_time < area_time_to {
                let v = (high - low) as f32 * ((cur_time - area_time) as f32 / len as f32) + low as f32;
                result = v as isize;
            }
            area_time = area_time_to;
        }
        // over ?
        if area_time <= cur_time {
            self.v_on_time = None;
            self.v_on_time_start = -1;
        }
        if result == isize::MIN { result = def; }
        result
    }
    pub fn write_cc_on_time(&mut self, cc_no: isize, ia: Vec<isize>, timebase: isize) {
        let freq = timebase / 32;
        for i in 0..ia.len() / 3 {
            let low = ia[i*3+0];
            let high = ia[i*3+1];
            let len = ia[i*3+2];
            for j in 0..len {
                if (j % freq) == 0 {
                    let v = (high - low) as f32 * (j as f32 / len as f32) + low as f32;
                    let v = value_range(0, v as isize, 127);
                    let e = Event::cc(self.timepos + j, self.channel, cc_no, v);
                    self.events.push(e);
                }
            }
        }
    }
    pub fn write_pb_on_time(&mut self, is_big: isize, ia: Vec<isize>, timebase: isize) {
        let freq = timebase / 32;
        for i in 0..ia.len() / 3 {
            let mut low = ia[i*3+0];
            let mut high = ia[i*3+1];
            if is_big == 0 { // small
                low = low * 128;
                high = high * 128;
            } else { // big
                low += 8192;
                high += 8192;
            }
            // println!("@@@PB.T={},{}", low,high);
            let len = ia[i*3+2];
            for j in 0..len {
                if (j % freq) == 0 {
                    let v = (high - low) as f32 * (j as f32 / len as f32) + low as f32;
                    let v = value_range(0, v as isize, 0x7f7f);
                    let e = Event::pitch_bend(self.timepos + j, self.channel, v);
                    self.events.push(e);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Flags {
    pub harmony_flag: bool,
    pub harmony_time: isize,
    pub harmony_events: Vec<Event>,
    pub octave_once: isize,
    pub measure_shift: isize,
}
impl Flags {
    pub fn new() -> Self {
        Flags {
            harmony_flag: false,
            harmony_time: 0,
            harmony_events: vec![],
            octave_once: 0,
            measure_shift: 0,
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
    pub variables: HashMap<String, SValue>,
    pub key_flag: Vec<isize>,
    pub key_shift: isize,
    pub play_from: isize,
    pub logs: Vec<String>, // ログ
    rand_seed: u32,
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
            rhthm_macro: mml_def::init_rhythm_macro(),
            variables: mml_def::init_variables(),
            key_flag: vec![0,0,0,0,0,0,0,0,0,0,0,0],
            key_shift: 0,
            play_from: -1,
            logs: vec![],
            rand_seed: 1110122942, // Random Seed
        }
    }
    pub fn get_logs_str(&self) -> String {
        self.logs.join("\n")
    }
    pub fn add_log(&mut self, msg: String) {
        self.logs.push(msg);
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
    pub fn play_from_all_track(&mut self) {
        if self.play_from < 0 { return; }
        if self.debug { println!("PLAY_FROM={}", self.play_from); }
        for trk in self.tracks.iter_mut() {
            trk.play_from(self.play_from);
        }
    }
    pub fn calc_rand_value(&mut self, val: isize, rand_v: isize) -> isize {
        let r = self.rand();
        let r = (r as isize) % rand_v - (rand_v / 2);
        val + r
    }
    pub fn rand(&mut self) -> u32 {
        let mut y = self.rand_seed;
        y ^= y << 13;
        y ^= y >> 17;
        y ^= y << 5;
        self.rand_seed = y;
        y
    }
    pub fn track_sync(&mut self) {
        let timepos = self.tracks[self.cur_track].timepos;
        for i in 0..self.tracks.len() {
            self.tracks[i].timepos = timepos;
        }
    }
    pub fn merge_all_events(&mut self) -> Vec<Event> {
        let mut events: Vec<Event> = vec![];
        for trk in self.tracks.iter_mut() {
            let elist = trk.split_note_off();
            for e in elist.into_iter() {
                events.push(e);
            }
        }
        events.sort_by(|a, b| a.time.cmp(&b.time));
        events
    }
}

