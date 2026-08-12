//! "sakruamml-rust" is a MML/ABC to MIDI compier.
//! This compiler that converts the text of "cde" into MIDI files. 
//! It is a tool that allows you to easily create music.

pub mod sakura_version;
pub mod sakura_message;
pub mod source_cursor;
pub mod sakura_functions;
pub mod token;
pub mod lexer;
pub mod song;
pub mod svalue;
pub mod midi;
pub mod sutoton;
pub mod runner;
pub mod note_length;
pub mod mml_def;
pub mod song_test;

#[cfg(test)]
mod lexer_test;

#[cfg(test)]
mod runner_test;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

/// Debug level - no info
pub const SAKURA_DEBUG_NONE: u32 = 0;
/// Debug level - show info
pub const SAKURA_DEBUG_INFO: u32 = 1;

/// MAX Input Size (3MB)
pub const SAKURA_MAX_INPUT_SIZE: usize = 3 * 1024 * 1024;

// ------------------------------------------
// Sakura Functions for JavaScript
// ------------------------------------------
/// get sakura compiler version info
#[wasm_bindgen]
pub fn get_version() -> String {
    sakura_version::SAKURA_VERSION.to_string()
}

#[wasm_bindgen]
pub fn get_build_number() -> String {
    std::env::var("BUILD_NUMBER").unwrap_or_else(|_| "0".to_string())
}

/// SakuraCompiler Object
#[wasm_bindgen]
pub struct SakuraCompiler {
    song: song::Song,
    log_str: String,
    lang: String,
    debug_level: u32,
    max_input_size: usize,
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
            max_input_size: SAKURA_MAX_INPUT_SIZE,
        }
    }
    /// compile to MIDI data
    pub fn compile(&mut self, source: &str) -> Vec<u8> {
        // 同じコンパイラを再利用しても、前回の曲やログを引き継がない。
        self.song = song::Song::new();
        self.log_str.clear();
        if self.debug_level > 0 {
            self.song.debug = true;
        }
        self.song.set_language(&self.lang);
        if source.len() > self.max_input_size {
            let msg = format!(
                "[ERROR](0) Input size exceeds max_input_size ({} > {})",
                source.len(),
                self.max_input_size
            );
            self.song.add_log(msg);
            let log_text = self.song.get_logs_str();
            self.log_str.push_str(&log_text);
            return vec![];
        }
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
    /// get max input size
    #[wasm_bindgen(getter)]
    pub fn max_input_size(&self) -> usize {
        self.max_input_size
    }
    /// set max input size
    #[wasm_bindgen(setter)]
    pub fn set_max_input_size(&mut self, value: usize) {
        self.max_input_size = value;
    }
    /// dump midi
    pub fn dump_midi(&self, bin: Vec<u8>) -> String {
        midi::dump_midi(&bin, false)
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

#[cfg(test)]
mod public_api_tests {
    use super::*;

    #[test]
    fn compile_returns_midi_data() {
        let result = compile("o4c", SAKURA_DEBUG_NONE);
        assert!(result.log.is_empty());
        assert!(result.bin.starts_with(b"MThd"));
        assert!(result.bin.ends_with(&[0x00, 0xFF, 0x2F, 0x00]));
    }

    #[test]
    fn compiler_can_be_reused_without_mixing_previous_song() {
        let mut compiler = SakuraCompiler::new();
        compiler.compile("o4c");
        let second = compiler.compile("o4d");
        let dump = compiler.dump_midi(second);

        assert_eq!(dump.matches("NoteOn(").count(), 1);
        assert!(dump.contains("NoteOn($32"));
        assert!(!dump.contains("NoteOn($30"));
    }

    #[test]
    fn compiler_rejects_only_inputs_over_the_configured_limit() {
        let mut compiler = SakuraCompiler::new();
        compiler.set_max_input_size(2);

        assert!(compiler.compile("cc").starts_with(b"MThd"));
        assert!(compiler.compile("ccc").is_empty());
        assert!(compiler.get_log().contains("Input size exceeds max_input_size (3 > 2)"));
    }

    #[test]
    fn malformed_mml_does_not_panic() {
        for source in ["Tempo()", "TimeSig()", "SysEx()", "[", "Function F(){"] {
            let result = std::panic::catch_unwind(|| compile(source, SAKURA_DEBUG_NONE));
            assert!(result.is_ok(), "次の入力でパニックしました: {source:?}");
        }
    }
}
