mod lexer;
use crate::lexer::token::lex;

fn main() {
    let tok = lex("l8cde");
    println!("lex= {:?}", tok); 
}
