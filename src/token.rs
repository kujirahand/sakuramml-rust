use super::svalue::SValue;

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    Error,
    Unknown,
    Note,
    NoteN,
    Track,
    Channel,
    Voice,
    NoteNo,
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
    HarmonyFlag,
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
