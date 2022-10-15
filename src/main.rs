use sakuramml::token::lex;
use sakuramml::song::{exec, Song};

fn main() {
    let src = sakuramml::sutoton::convert("トラック3ドレミファソラシ");
    println!("{}", src);
    let tokens = lex(&src);
    let mut song = Song::new();
    exec(&mut song, &tokens);

    println!("lex= {:?}", tokens); 
}
