use super::svalue::SValue;

/// TokenType
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TokenType {
    Error,
    Empty,
    Print,
    Note,
    NoteN,
    Rest,
    Track,
    Channel,
    Voice,
    Length,
    Octave,
    OctaveRel,
    OctaveOnce,
    QLen,
    Velocity,
    VelocityRel,
    Timing,
    ControllChange,
    CConTime,
    CConTimeFreq,
    Tempo,
    TempoChange,
    MetaText,
    SysEx,
    TimeSignature,
    PitchBend,
    PBonTime,
    LoopBegin,
    LoopEnd,
    LoopBreak,
    Time,
    HarmonyBegin,
    HarmonyEnd,
    Tokens, // should run children toknes
    Div,
    Sub,
    KeyFlag,
    KeyShift,
    TrackKey,
    DefInt,
    DefStr,
    PlayFrom,
    VelocityRandom,
    QLenRandom,
    TimingRandom,
    VelocityOnTime,
    VelocityOnNote,
    QLenOnNote,
    TimingOnNote,
    MeasureShift,
    TrackSync,
    TieMode,
    If,
    For,
    While,
    Calc,
    Value,
    ValueInc,
    SetConfig,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: isize,
    pub tag: isize,
    pub data: Vec<SValue>,
    pub children: Option<Vec<Token>>,
}

impl Token {
    pub fn new(ttype: TokenType, value: isize, data: Vec<SValue>) -> Self {
        Self {
            ttype,
            value,
            tag: 0,
            data,
            children: None,
        }
    }
    pub fn new_value(ttype: TokenType, value: isize) -> Self {
        Self {
            ttype,
            value,
            tag: 0,
            data: vec![],
            children: None,
        }
    }
    pub fn new_value_tag(ttype: TokenType, value: isize, tag: isize) -> Self {
        Self {
            ttype,
            value,
            tag,
            data: vec![],
            children: None,
        }
    }
    pub fn new_tokens(ttype: TokenType, value: isize, tokens: Vec<Token>) -> Self {
        Self {
            ttype,
            value,
            tag: 0,
            data: vec![],
            children: Some(tokens),
        }
    }
    pub fn new_data_tokens(ttype: TokenType, value: isize, data: Vec<SValue>, tokens: Vec<Token>) -> Self {
        Self {
            ttype,
            value,
            tag: 0,
            data,
            children: Some(tokens),
        }
    }
    pub fn new_empty(cmd: &str) -> Self {
        Self::new(TokenType::Empty, 0, vec![SValue::from_s(cmd.to_string())])
    }
    pub fn new_sysex(a: Vec<isize>) -> Self {
        let mut sa: Vec<SValue> = vec![];
        for (i, v) in a.iter().enumerate() {
            if i == 0 && *v == 0xF0 {
                continue;
            }
            sa.push(SValue::from_i(*v));
        }
        Self::new(TokenType::SysEx, 0, sa)
    }
    pub fn to_debug_str(&self) -> String {
        format!("[{:?},{}]", self.ttype, self.value)
    }
}

pub fn tokens_to_str(tokens: &Vec<Token>) -> String {
    let mut s = String::new();
    for t in tokens.iter() {
        s.push_str(&t.to_debug_str());
    }
    s
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
