use super::cursor::TokenCursor;

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    Number,
    Flag,
    String,
    CommandUpper,
    CommandLower,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    value: i64,
    data: Option<String>,
    args: Option<Vec<Token>>,
}

pub fn char_from_u32(i: u32, def: char) -> char {
    char::from_u32(i).unwrap_or(def)
}
/// 全角記号を半角記号に変換
// https://en.wikipedia.org/wiki/Halfwidth_and_Fullwidth_Forms_(Unicode_block)
pub fn zen2han(c: char) -> char {
    match c {
        // half ascii code
        '\u{0020}'..='\u{007E}' => c,
        // FullWidth
        // '！'..='～' = '\u{FF01}'..='\u{FF5E}'
        '\u{FF01}'..='\u{FF5E}' => char_from_u32(c as u32 - 0xFF01 + 0x21, c),
        // space
        '\u{2002}'..='\u{200B}' => ' ',
        '\u{3000}' | '\u{FEFF}' => ' ',
        // others
        _ => c,
    }
}

pub fn han2zen(c: char) -> char {
    match c {
        // digit
        '0'..='9' => char_from_u32(c as u32 + 0xFF10 - 0x30, c),
        // alphabet
        'A'..='Z' | 'a'..='z' => char_from_u32(c as u32 + 0xFF21 - 0x41 , c),
        // flag
        '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => 
            char_from_u32(c as u32 + 0xFF01 - 0x21, c),
        _ => c
    }
}

pub fn read_digit(cur: &mut TokenCursor, ch1: char) -> Token {
    let mut s = String::new();
    s.push(ch1);
    while !cur.is_eos() {
        let ch = cur.peek().unwrap_or('\0');
        if '0' <= ch && ch <= '9' {
            s.push(ch);
            cur.index += 1;
            continue;
        }
        break;
    }
    Token {
        ttype: TokenType::Number,
        value: s.parse().unwrap_or(0),
        data: None,
        args: None,
    }
}

pub fn lex(src: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut cur = TokenCursor::from(src);
    while !cur.is_eos() {
        let ch = zen2han(cur.next().unwrap_or('\0'));
        println!("{:?}", ch);
        match ch {
            // space
            ' ' | '\t' | '\r' => { /* skip */}
            // ret
            '\n' => {
                cur.line += 1;
            }
            // digit
            '0'..='9' => {
                let tok = read_digit(&mut cur, ch);
                result.push(tok);
            }
            // lower command
            'a'..='z' | '_' => {
                result.push(Token{
                    ttype: TokenType::CommandLower,
                    value: ch as i64,
                    data: None,
                    args: None,
                })
            },
            // string
            '{' => {
                cur.prev();
                let s = cur.get_token_nest('{', '}');
                result.push(Token {
                    ttype: TokenType::String,
                    value: s.len() as i64,
                    data: Some(s),
                    args: None,
                })
            }
            _ => {
                // skip
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zen2han() {
        assert_eq!(zen2han('Ａ'), 'A');
        assert_eq!(zen2han('３'), '3');
        assert_eq!(zen2han('　'), ' ');
    }
    #[test]
    fn test_han2zen() {
        assert_eq!(han2zen('A'), 'Ａ');
        assert_eq!(han2zen('3'), '３');
        assert_eq!(han2zen('!'), '！');
    }
}
