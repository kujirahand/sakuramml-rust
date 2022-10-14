mod lib;

fn main() {
    let tok = lib::lex("l8cde");
    println!("lex= {:?}", tok); 
}
