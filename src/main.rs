use sakuramml;

fn main() {
    let tok = sakuramml::token::lex("l8cde");
    println!("lex= {:?}", tok); 
}
