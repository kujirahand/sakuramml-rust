//! song & track

mod event;
mod flags;
mod function;
mod track;

pub use event::*;
pub use flags::*;
pub use function::*;
pub use track::*;


use std::collections::HashMap;
use crate::runner::value_range;
use crate::sakura_functions;
use crate::svalue::SValue;
use crate::mml_def::{self, TieMode};
use crate::sakura_message::{MessageLang, MessageData, MessageKind};
use crate::token::Tokens;

// const
pub const SAKURA_MAX_LOGS: usize = 100; // lines
pub const SAKURA_MAX_LOGS_CHARS: usize = 1024 * 4; // chars
pub const SAKURA_DEFAULT_RANDOM_SEED: u32 = 3958587042; // random seed

/// Song
#[derive(Debug)]
pub struct Song {
    pub debug: bool,
    pub message_data: MessageData,
    pub tracks: Vec<Track>,
    pub tempo: isize,
    pub timebase_changed: bool, // タイムベースの変更は1度限り許す
    pub timebase: isize,
    pub cur_track: usize,
    pub timesig_frac: isize, // 分子
    pub timesig_deno: isize, // 分母
    pub flags: Flags,
    pub rhthm_macro: Vec<String>,
    pub variables_stack: Vec<HashMap<String, SValue>>,
    pub functions: Vec<SFunction>,
    pub system_functions: HashMap<String, mml_def::SystemFunction>,
    pub calc_functions: HashMap<String, sakura_functions::CallbackCalcFn>,
    pub reserved_words: HashMap<String, u8>,
    pub key_flag: Vec<isize>, // order: [c,c#,d,d#,e,f,f#,g,g#,a,a#,b]
    pub key_shift: isize,
    pub play_from: isize,
    pub v_add: isize,
    pub q_add: isize,
    pub stack: Vec<SValue>,
    pub rand_seed: u32,
    pub device_number: u8,
    pub use_key_shift: bool,
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
            timebase_changed: false,
            tempo: 120,
            tracks: vec![trk],
            cur_track: 0,
            timesig_frac: 4,
            timesig_deno: 4,
            flags: Flags::new(),
            system_functions: sys_funcs,
            calc_functions: mml_def::init_system_calc_functions(),
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
            rand_seed: SAKURA_DEFAULT_RANDOM_SEED, // Random Seed
            device_number: 0x10, // default device number (0x10: General MIDI)
            use_key_shift: true,
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
        let mut last = self.variables_stack.pop().unwrap(); // 現在のスコープを取得
        last.insert(key.to_string(), val); // 現在のスコープに変数を追加
        self.variables_stack.push(last); // スコープを戻す
    }
    pub fn variables_get(&self, key: &str) -> Option<&SValue> {
        // search scope
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
