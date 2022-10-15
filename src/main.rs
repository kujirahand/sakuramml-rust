use std::fs::File;
use std::io::{Write, BufWriter};

use sakuramml::token::lex;
use sakuramml::song::{exec, Song};
use sakuramml::midi::generate;

fn main() {
    let src = sakuramml::sutoton::convert("トラック3ドレミファソラシ");
    println!("{}", src);
    let tokens = lex(&src);
    println!("lex= {:?}", tokens); 
    let mut song = Song::new();
    exec(&mut song, &tokens);
    println!("song= {:?}", song);
    save_to_file(&song, "test.mid");
    println!("ok.");
}

fn save_to_file(song: &Song, path: &str) {
    let mut file = File::create(path).unwrap();
    let buf = generate(song);
    file.write(buf.as_ref()).unwrap();
    file.flush().unwrap();
}
