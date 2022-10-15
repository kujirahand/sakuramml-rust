use std::fs::File;
use std::io::{Write};

use sakuramml::token::lex;
use sakuramml::song::{exec, Song};
use sakuramml::midi::generate;

fn main() {
    let src = format!("{}{}", 
    "TR=1@4l4 v50 q50 o5 g,100 v100 ab,100>c<b,100ag",
    "TR=2@4l4 v50 q50 o5 b,100>cde,100dc<b,100"
    );
    let src = sakuramml::sutoton::convert(&src);
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
