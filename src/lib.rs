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

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// ------------------------------------------
// JavaScript Functions for Rust
// ------------------------------------------
#[wasm_bindgen]
extern {
    /// should define log function in JavaScript code
    pub fn sakura_log(s: &str);
}

// ------------------------------------------
// Rust Functions for JavaScript
// ------------------------------------------
/// get sakura compiler version info
#[wasm_bindgen]
pub fn get_version() -> String {
    sakura_version::SAKURA_VERSION.to_string()
}

/// compile source to MIDI data
#[wasm_bindgen]
pub fn compile(source: &str) -> Vec<u8> {
    let mut song = song::Song::new();
    let source_mml = sutoton::convert(source);
    let tokens = lexer::lex(&mut song, &source_mml, 0);
    runner::exec(&mut song, &tokens);
    let bin = midi::generate(&mut song);
    let log_text = song.get_logs_str();
    sakura_log(&log_text);
    return bin;
}

/// SakuraCompiler Object
#[wasm_bindgen]
pub struct SakuraCompiler {
    song: song::Song,
}
#[wasm_bindgen]
impl SakuraCompiler {
    /// new object
    pub fn new() -> Self {
        SakuraCompiler {
            song: song::Song::new(),
        }
    }
    /// compile to MIDI data
    pub fn compile(&mut self, source: &str) -> Vec<u8> {
        let source_mml = sutoton::convert(source);
        let tokens = lexer::lex(&mut self.song, &source_mml, 0);
        runner::exec(&mut self.song, &tokens);
        let bin = midi::generate(&mut self.song);
        let log_text = self.song.get_logs_str();
        sakura_log(&log_text);
        return bin;
    }
    /// set message language
    pub fn set_language(&mut self, code: &str) {
        self.song.set_language(code);
    }
}