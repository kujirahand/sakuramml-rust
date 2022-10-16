pub mod sakura_version;
pub mod cursor;
pub mod token;
pub mod lexer;
pub mod song;
pub mod svalue;
pub mod midi;
pub mod sutoton;
pub mod runner;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;



// JavaScriptの関数をRustで使う
#[wasm_bindgen]
extern {
    // JavaScriptのalert関数をRustで使えるように
    pub fn sakura_log(s: &str);
}

// RustでJavaScriptから使える関数を定義
#[wasm_bindgen]
pub fn compile(source: &str) -> Vec<u8> {
    let mut song = song::Song::new();
    let source_mml = sutoton::convert(source);
    let tokens = lexer::lex(&mut song, &source_mml);
    runner::exec(&mut song, &tokens);
    let bin = midi::generate(&song);
    let log_text = song.logs.join("\n");
    sakura_log(&log_text);
    return bin;
}
