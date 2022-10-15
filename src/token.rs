use super::cursor::TokenCursor;
use super::svalue::SValue;

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    Unknown,
    Note,
    Track,
    Channel,
    Voice,
    NoteNo,
    Length,
    Octave,
    OctaveRel,
    QLen,
    Velocity,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub value: isize,
    pub data: Vec<SValue>,
}

impl Token {
    pub fn new_value(ttype: TokenType, value: isize) -> Self {
        Self { ttype, value, data: vec![] }
    }
    pub fn new(ttype: TokenType, value: isize, data: Vec<SValue>) -> Self {
        Self { ttype, value, data }
    }
    pub fn new_unknown() -> Self {
        Self::new(TokenType::Unknown, 0, vec![])
    }
}

pub fn char_from_u32(i: u32, def: char) -> char {
    char::from_u32(i).unwrap_or(def)
}
/// 全角記号を半角記号に変換
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

fn read_arg(cur: &mut TokenCursor) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        '(' => {
            cur.next();
            let r = read_arg(cur);
            cur.skip_space();
            if cur.peek_n(0) == ')' { cur.next(); }
            return r;
        },
        '=' => {
            cur.next();
            read_arg(cur)
        },
        // number
        '0'..='9' => {
            let n = cur.get_int(0);
            SValue::from_i(n)
        },
        '{' => {
            SValue::from_s(cur.get_token_nest('{', '}'))
        }
        _ => {
            SValue::None
        }
    }
}

fn read_command(cur: &mut TokenCursor) -> Token {
    cur.prev(); // back 1char
    let cmd = cur.get_word();
    if cmd == "TR" || cmd == "TRACK" || cmd == "Track" {
        let v = read_arg(cur);
        return Token {
            ttype: TokenType::Track,
            value: 0,
            data: vec![v],
        }
    }
    if cmd == "CH" || cmd == "Channel" {
        let v = read_arg(cur);
        return Token {
            ttype: TokenType::Channel,
            value: 0,
            data: vec![v],
        }
    }
    Token {
        ttype: TokenType::Unknown,
        value: 0,
        data: vec![],
    }
}

fn read_voice(cur: &mut TokenCursor) -> Token {
    let value = read_arg(cur);
    Token::new(TokenType::Voice, value.to_i(), vec![])
}

fn read_length(cur: &mut TokenCursor) -> Token {
    let s = cur.get_note_length();
    Token::new(TokenType::Length, 0, vec![SValue::from_s(s)])
}

fn read_octave(cur: &mut TokenCursor) -> Token {
    let value = read_arg(cur);
    Token::new(TokenType::Octave, value.to_i(), vec![])
}

fn read_qlen(cur: &mut TokenCursor) -> Token {
    let value = read_arg(cur);
    Token::new(TokenType::QLen, value.to_i(), vec![])
}

fn read_velocity(cur: &mut TokenCursor) -> Token {
    let value = read_arg(cur);
    Token::new(TokenType::Velocity, value.to_i(), vec![])
}

fn read_note(cur: &mut TokenCursor, ch: char) -> Token {
    // flag
    let note_flag = match cur.peek_n(0) {
        '+' | '#' => 1,
        '-' => -1,
        _ => 0,
    };
    // length
    let note_len = cur.get_note_length();
    cur.skip_space();
    // qlen
    let qlen = if !cur.eq_char(',') { 0 } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    // veolocity
    let vel = if !cur.eq_char(',') { 0 } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    Token {
        ttype: TokenType::Note,
        value: match ch {
            'c' => 0,
            'd' => 2,
            'e' => 4,
            'f' => 5,
            'g' => 7,
            'a' => 9,
            'b' => 11,
            _ => 0,
        },
        data: vec![
            SValue::from_i(note_flag),
            SValue::from_s(note_len),
            SValue::from_i(qlen),
            SValue::from_i(vel),
        ],
    }
}

pub fn lex(src: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut cur = TokenCursor::from(src);
    while !cur.is_eos() {
        let ch = zen2han(cur.get_char());
        match ch {
            // space
            ' ' | '\t' | '\r' => { },
            // ret
            '\n' => { cur.line += 1; },
            // lower command
            'a'..='g' => result.push(read_note(&mut cur, ch)),
            'l' => result.push(read_length(&mut cur)),
            'o' => result.push(read_octave(&mut cur)),
            'q' => result.push(read_qlen(&mut cur)),
            'v' => result.push(read_velocity(&mut cur)),
            // uppwer command
            'A'..='Z' => result.push(read_command(&mut cur)), 
            // flag
            '@' => result.push(read_voice(&mut cur)),
            '>' => result.push(Token::new_value(TokenType::OctaveRel, 1)),
            '<' => result.push(Token::new_value(TokenType::OctaveRel, -1)),
            // string
            '{' => {
                cur.prev();
                cur.get_token_nest('{', '}');
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
}