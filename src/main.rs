use sakuramml;

fn main() {
    let src = sakuramml::sutoton::convert("トラック3ドレミファソラシ");
    println!("{}", src);
    let tok = sakuramml::token::lex(&src);
    println!("lex= {:?}", tok); 
}
