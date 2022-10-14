mod token_cursor;

#[derive(Debug)]
pub enum TokenType {
    Number,
    Word,
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    data: String
}

pub fn lex(src: &str) -> Vec<Token> {
    let cur = token_cursor::TokenCursor::from(src);

    let n = Token {
        ttype: TokenType::Number,
        data: String::from("30"),
    };
    return vec![n];
}

