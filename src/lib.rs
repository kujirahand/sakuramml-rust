pub mod sakura_version;
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

// RustでJavaScriptから使える関数を定義
#[wasm_bindgen]
pub fn get_version() -> String {
    sakura_version::version_str()
}

// JavaScriptの関数をRustで使う
#[wasm_bindgen]
extern {
    // JavaScriptのalert関数をRustで使えるように
    // ログを出力する関数
    pub fn sakura_log(s: &str);
}

// RustでJavaScriptから使える関数を定義
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
