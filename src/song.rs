//! song & track

use std::collections::HashMap;
use crate::runner::value_range;
use crate::svalue::SValue;
use crate::mml_def::{self, TieMode};
use crate::sakura_message::{MessageLang, MessageData, MessageKind};
use crate::token::Tokens;

// const
pub const SAKURA_MAX_LOGS: usize = 100; // lines
pub const SAKURA_MAX_LOGS_CHARS: usize = 1024 * 4; // chars

/// Event Type
#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    NoteOn,
    NoteOff,
    ControllChange,
    PitchBend,
    PitchBendRange,
    Voice,
    Meta,
    SysEx,
}

/// Event
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
    pub fn sysex_raw(time: isize, data_v: Vec<u8>) -> Self {
        Self { etype: EventType::SysEx, time, channel: 0, v1: 0, v2: 0, v3: 0, data: Some(data_v) }
    }
    pub fn cc(time: isize, channel: isize, no: isize, value: isize) -> Self {
        Self { etype: EventType::ControllChange, time, channel, v1: no, v2: value, v3:0, data: None }
    }
    /// pitch_bend : 0..16383 (-8192 .. 0 .. 8191)
    pub fn pitch_bend(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::PitchBend, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
    pub fn pitch_bend_range(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::PitchBendRange, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
}

/// NoteInfo
#[derive(Debug)]
pub struct NoteInfo {
    pub no: isize,
    pub flag: isize,
    pub natural: isize,
    pub len_s: String,
    pub qlen: isize,
    pub vel: isize,
    pub t: isize,
    pub o: isize,
    pub slur: isize,
}


#[derive(Debug, Clone)]
pub struct ControllChangeOnNoteWave {
    pub no: isize,
    pub data: Vec<isize>,
}
/// Track
#[derive(Debug)]
pub struct Track {
    pub timepos: isize,
    pub channel: isize,
    pub length: isize,
    pub octave: isize,
    pub velocity: isize,
    pub v_sub: Vec<isize>,
    pub qlen: isize,
    pub timing: isize,
    pub v_rand: isize,
    pub q_rand: isize,
    pub t_rand: isize,
    pub port: isize,
    pub track_key: isize,
    pub tie_mode: TieMode, // Slur(#7)
    pub tie_value: isize,
    pub bend_range: isize,
    pub v_on_time_start: isize,
    pub v_on_time: Option<Vec<isize>>,
    pub v_on_note_index: isize,
    pub v_on_note: Option<Vec<isize>>,
    pub q_on_note_index: isize,
    pub q_on_note: Option<Vec<isize>>,
    pub t_on_note_index: isize,
    pub t_on_note: Option<Vec<isize>>,
    pub cc_on_time_freq: isize,
    pub events: Vec<Event>,
    pub tie_notes: Vec<Event>,
    pub cc_on_note_wave: Vec<ControllChangeOnNoteWave>,
}

impl Track {
    pub fn new(timebase: isize, channel: isize) -> Self {
        let channel = if channel < 0 { 0 } else if channel > 15 { 15 } else { channel };
        Track {
            timepos: 0,
            length: timebase,
            velocity: 100,
            octave: 5,
            qlen: 90,
            timing: 0,
            track_key: 0,
            port: 0,
            tie_mode: TieMode::Port,
            tie_value: 0,
            v_sub: vec![0],
            v_rand: 0,
            q_rand: 0,
            t_rand: 0,
            cc_on_time_freq: 4,
            v_on_time_start: -1,
            v_on_time: None,
            v_on_note_index: 0,
            v_on_note: None,
            q_on_note_index: 0,
            q_on_note: None,
            t_on_note_index: 0,
            t_on_note: None,
            channel,
            events: vec![],
            tie_notes: vec![],
            bend_range: -1,
            cc_on_note_wave: vec![],
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
                EventType::PitchBend => {}, // TODO: #8
                EventType::PitchBendRange => {}, // TODO: #8
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
    pub fn calc_v_on_note(&mut self, def: isize) -> isize {
        // on_note?
        let ia = match &self.v_on_note {
            None => return def,
            Some(pia) => pia.clone()
        };
        if ia.len() == 0 { return def; }
        let v = ia[(self.v_on_note_index as usize) % ia.len()];
        self.v_on_note_index += 1;
        return v;
    }
    pub fn calc_t_on_note(&mut self, def: isize) -> isize {
        // on_note?
        let ia = match &self.t_on_note {
            None => return def,
            Some(pia) => pia.clone()
        };
        if ia.len() == 0 { return def; }
        let t = ia[(self.t_on_note_index as usize) % ia.len()];
        self.t_on_note_index += 1;
        return t;
    }
    pub fn calc_qlen_on_note(&mut self, def: isize) -> isize {
        // on_note?
        let ia = match &self.q_on_note {
            None => return def,
            Some(pia) => pia.clone()
        };
        if ia.len() == 0 { return def; }
        let qlen = ia[(self.q_on_note_index as usize) % ia.len()];
        self.q_on_note_index += 1;
        return qlen;
    }
    pub fn write_cc_on_time(&mut self, cc_no: isize, ia: Vec<isize>) {
        let freq = self.cc_on_time_freq;
        for i in 0..ia.len() / 3 {
            let low = ia[i*3+0];
            let high = ia[i*3+1];
            let len = ia[i*3+2];
            // println!("CC.T={},{},{}", low, high, len);
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
    pub fn remove_cc_on_note_wave(&mut self, no: isize) {
        if self.cc_on_note_wave.len() == 0 { return; }
        let mut new_list: Vec<ControllChangeOnNoteWave> = vec![];
        for cow in self.cc_on_note_wave.iter() {
            if cow.no == no { continue; }
            new_list.push(cow.clone());
        }
        self.cc_on_note_wave = new_list;
    }
    pub fn set_cc_on_note_wave(&mut self, no: isize, ia: Vec<isize>) {
        self.remove_cc_on_note_wave(no);
        let cc_new = ControllChangeOnNoteWave { no, data: ia };
        self.cc_on_note_wave.push(cc_new);
    }
    pub fn write_cc_on_note_wave(&mut self, start_pos: isize) {
        let end_pos = self.timepos;
        // let _wave_len = end_pos - start_pos;
        self.timepos = start_pos;
        for cow in self.cc_on_note_wave.clone().iter() {
            // println!("write_cc_on_note_wave:no={}", cow.no);
            self.write_cc_on_time(cow.no, cow.data.clone());
        }
        self.timepos = end_pos;
    }
}

#[derive(Debug)]
pub struct Flags {
    pub harmony_flag: bool,
    pub harmony_time: isize,
    pub harmony_events: Vec<Event>,
    pub octave_once: isize,
    pub measure_shift: isize,
    pub break_flag: isize, // 0: none 1: break 2: continue 3: return
    pub max_loop: isize,
    pub function_needs_return_value: bool,
}
impl Flags {
    pub fn new() -> Self {
        Flags {
            harmony_flag: false,
            harmony_time: 0,
            harmony_events: vec![],
            octave_once: 0,
            measure_shift: 0,
            break_flag: 0,
            max_loop: 10000,
            function_needs_return_value: false,
        }
    }
}

#[derive(Debug)]
pub struct SFunction {
    pub name: String,
    pub tokens: Tokens,
    pub lineno: isize,
    pub func_id: usize,
    pub arg_names: Vec<String>,
    pub arg_types: Vec<char>, // S: string, I: int, A: array
}

impl SFunction {
    pub fn new(name: &str, tokens: Tokens, func_id: usize, lineno: isize) -> Self {
        Self {
            name: name.to_string(),
            tokens,
            lineno,
            func_id,
            arg_names: vec![],
            arg_types: vec![],
        }
    }
}

/// Song
#[derive(Debug)]
pub struct Song {
    pub debug: bool,
    pub message_data: MessageData,
    pub tracks: Vec<Track>,
    pub tempo: isize,
    pub timebase: isize,
    pub cur_track: usize,
    pub timesig_frac: isize, // 分子
    pub timesig_deno: isize, // 分母
    pub flags: Flags,
    pub rhthm_macro: Vec<String>,
    pub variables_stack: Vec<HashMap<String, SValue>>,
    pub functions: Vec<SFunction>,
    pub system_functions: HashMap<String, mml_def::SystemFunction>,
    pub reserved_words: HashMap<String, u8>,
    pub key_flag: Vec<isize>,
    pub key_shift: isize,
    pub play_from: isize,
    pub v_add: isize,
    pub q_add: isize,
    pub stack: Vec<SValue>,
    pub rand_seed: u32,
    pub device_number: u8,
    pub lineno: isize,
    logs: Vec<String>, // ログ
}

impl Song {
    pub fn new() -> Self {
        let timebase = 96;
        let trk = Track::new(timebase, 0);
        let global_vars = mml_def::init_variables();
        let vars_stack = vec![global_vars];
        let sys_funcs = mml_def::init_system_functions();
        let reserved = mml_def::init_reserved_words(&sys_funcs);

        Self {
            debug: false,
            message_data: MessageData::new(MessageLang::EN),
            timebase,
            tempo: 120,
            tracks: vec![trk],
            cur_track: 0,
            timesig_frac: 4,
            timesig_deno: 4,
            flags: Flags::new(),
            system_functions: sys_funcs,
            rhthm_macro: mml_def::init_rhythm_macro(),
            variables_stack: vars_stack,
            functions: vec![],
            reserved_words: reserved,
            key_flag: vec![0,0,0,0,0,0,0,0,0,0,0,0],
            key_shift: 0,
            play_from: -1,
            logs: vec![],
            v_add: 8,
            q_add: 1,
            stack: vec![],
            rand_seed: 1234567, // Random Seed
            device_number: 0,
            lineno: 0,
        }
    }
    pub fn set_language(&mut self, lang_code: &str) {
        let lang = MessageLang::from(lang_code);
        self.message_data = MessageData::new(lang);
    }
    pub fn get_message(&self, kind: MessageKind) -> &'static str {
        self.message_data.get(kind)
    }
    pub fn get_logs_str(&self) -> String {
        let msg = self.logs.join("\n");
        let chars = msg.chars();
        if chars.count() <= SAKURA_MAX_LOGS_CHARS { return msg; }
        let mut submsg: String = msg.chars().take(SAKURA_MAX_LOGS_CHARS).collect();
        submsg.push_str("...");
        submsg
    }
    pub fn add_log(&mut self, msg: String) {
        if SAKURA_MAX_LOGS <= self.logs.len() { return; } // check max logs
        self.logs.push(msg);
    }
    pub fn get_logs_len(&self) -> usize {
        self.logs.len()
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
    pub fn change_cur_track(&mut self, no: usize) {
        self.cur_track = no as usize;
        // new track ?
        while self.tracks.len() <= self.cur_track {
            // println!("{:?}", v);
            let trk = Track::new(self.timebase, no as isize - 1);
            self.tracks.push(trk);
        }

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
    pub fn variables_contains_key(&self, key: &str) -> bool {
        for vars in self.variables_stack.iter().rev() {
            if vars.contains_key(key) { return true; }
        }
        false
    }
    pub fn variables_insert(&mut self, key: &str, val: SValue) {
        let mut last = self.variables_stack.pop().unwrap();
        last.insert(key.to_string(), val);
        self.variables_stack.push(last);
    }
    pub fn variables_get(&self, key: &str) -> Option<&SValue> {
        for vars in self.variables_stack.iter().rev() {
            match vars.get(key) {
                None => continue,
                Some(val) => return Some(val),
            }
        }
        None
    }
    pub fn variables_modify<F: Fn(SValue)->SValue>(&mut self, key: &str, closure: F) {
        let mut modified = false;
        for vars in self.variables_stack.iter_mut().rev() {
            match vars.get_mut(key) {
                None => continue,
                Some(val) => {
                    modified = true;
                    *val = closure(val.clone());
                }
            }
        }
        if !modified {
            let new_val = closure(SValue::new());
            self.variables_insert(key, new_val);
        }
    }
    pub fn variables_stack_push(&mut self) {
        let vars = HashMap::new();
        self.variables_stack.push(vars);
    }
    pub fn variables_stack_pop(&mut self) -> HashMap<String, SValue> {
        self.variables_stack.pop().unwrap_or(HashMap::new())
    }
}
