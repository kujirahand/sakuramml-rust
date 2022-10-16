use super::svalue::SValue;

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    Error,
    Unknown,
    Note,
    NoteN,
    Rest,
    Track,
    Channel,
    Voice,
    Length,
    Octave,
    OctaveRel,
    QLen,
    Velocity,
    ControllChange,
    Tempo,
    MetaText,
    TimeSignature,
    PitchBend,
    LoopBegin,
    LoopEnd,
    LoopBreak,
    Time,
    HarmonyBegin,
    HarmonyEnd,
    Tokens,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub value: isize,
    pub data: Vec<SValue>,
    pub children: Option<Vec<Token>>,
}

impl Token {
    pub fn new(ttype: TokenType, value: isize, data: Vec<SValue>) -> Self {
        Self { ttype, value, data, children: None }
    }
    pub fn new_value(ttype: TokenType, value: isize) -> Self {
        Self { ttype, value, data: vec![], children: None }
    }
    pub fn new_unknown(cmd: &str) -> Self {
        Self::new(TokenType::Unknown, 0, vec![SValue::from_s(cmd.to_string())])
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
