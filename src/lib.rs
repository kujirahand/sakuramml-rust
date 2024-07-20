//! "sakruamml-rust" is a MML/ABC to MIDI compier.
//! This compiler that converts the text of "cde" into MIDI files. 
//! It is a tool that allows you to easily create music.

pub mod sakura_version;
pub mod sakura_message;
pub mod cursor;
pub mod token;
pub mod lexer;
pub mod song;
pub mod svalue;
pub mod midi;
pub mod sutoton;
pub mod runner;
pub mod mml_def;
pub mod song_test;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

/// Debug level - no info
pub const SAKURA_DEBUG_NONE: u32 = 0;
/// Debug level - show info
pub const SAKURA_DEBUG_INFO: u32 = 1;

// ------------------------------------------
// Sakura Functions for JavaScript
// ------------------------------------------
/// get sakura compiler version info
#[wasm_bindgen]
pub fn get_version() -> String {
    sakura_version::SAKURA_VERSION.to_string()
}

/// SakuraCompiler Object
#[wasm_bindgen]
pub struct SakuraCompiler {
    song: song::Song,
    log_str: String,
    lang: String,
    debug_level: u32,
}
#[wasm_bindgen]
impl SakuraCompiler {
    /// new object
    pub fn new() -> Self {
        SakuraCompiler {
            song: song::Song::new(),
            log_str: "".to_string(),
            debug_level: 0,
            lang: "en".to_string(),
        }
    }
    /// compile to MIDI data
    pub fn compile(&mut self, source: &str) -> Vec<u8> {
        if self.debug_level > 0 {
            self.song.debug = true;
        }
        self.song.set_language(&self.lang);
        // convert sutoton
        let source_mml = sutoton::convert(source);
        // parse MML
        let tokens = lexer::lex(&mut self.song, &source_mml, 0);
        // run Tokens
        runner::exec(&mut self.song, &tokens);
        // generate MIDI
        let bin = midi::generate(&mut self.song);
        // get log text
        let log_text = self.song.get_logs_str();
        self.log_str.push_str(&log_text);
        bin
    }
    /// set message language
    pub fn set_language(&mut self, code: &str) {
        self.lang = code.to_string();
    }
    /// get log text
    pub fn get_log(&self) -> String {
        self.log_str.to_string()
    }
    /// set debug level
    pub fn set_debug_level(&mut self, level: u32) {
        self.debug_level = level;
    }
}

/// compile source to MIDI data
#[wasm_bindgen]
pub fn compile_to_midi(source: &str, debug_level: u32) -> Vec<u8> {
    let mut song = song::Song::new();
    if debug_level >= 1 {
        song.debug = true;
    }
    let source_mml = sutoton::convert(source);
    let tokens = lexer::lex(&mut song, &source_mml, 0);
    runner::exec(&mut song, &tokens);
    let bin = midi::generate(&mut song);
    bin
}

// ------------------------------------------
// Functions for Rust Native
// ------------------------------------------
/// compiler result struct
#[derive(Debug)]
pub struct SakuraResult {
    /// MIDI binary data
    pub bin: Vec<u8>,
    /// MIDI binary data
    pub log: String,
}

/// compile source to MIDI data
pub fn compile(source: &str, debug_level: u32) -> SakuraResult {
    let mut song = song::Song::new();
    if debug_level >= 1 {
        song.debug = true;
    }
    let source_mml = sutoton::convert(source);
    let tokens = lexer::lex(&mut song, &source_mml, 0);
    runner::exec(&mut song, &tokens);
    let bin = midi::generate(&mut song);
    let log_text = song.get_logs_str();
    SakuraResult {
        bin,
        log: log_text
    }
}
