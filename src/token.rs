use super::svalue::SValue;

/// TokenType
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Unimplemented,
    Error,
    Empty,
    LineNo,
    TimeBase,
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
    QLenRel,
    Velocity,
    VelocityRel,
    Timing,
    CtrlChange,
    CConTime,
    CConNoteWave,
    CConTimeFreq,
    Decresc,
    Tempo,
    TempoChange,
    MetaText,
    GSEffect,
    Port,
    SysEx,
    TimeSignature,
    PitchBend,
    PBonTime,
    LoopBegin,
    LoopEnd,
    LoopBreak,
    Time,
    Rhythm,
    HarmonyBegin,
    HarmonyEnd,
    Tokens, // should run children toknes
    Div,
    Sub,
    KeyFlag,
    KeyShift,
    UseKeyShift,
    TrackKey,
    DefInt,
    DefStr,
    DefArray,
    LetVar,
    PlayFrom,
    PlayFromHere,
    OctaveRandom,
    OctaveOnNote,
    OctaveOnCycle,
    QLenRandom,
    TimingRandom,
    VelocityRandom,
    VelocityOnTime,
    VelocityOnNote,
    VelocityOnCycle,
    QLenOnNote,
    QLenOnCycle,
    LengthOnNote,
    LengthOnCycle,
    TimingOnNote,
    TimingOnCycle,
    MeasureShift,
    TrackSync,
    TieMode,
    If,
    For,
    While,
    Break,
    Continue,
    Calc,
    Value,
    ValueInc,
    SetConfig,
    DefUserFunction,
    CallUserFunction,
    Return,
    Play,
    Include,
    SongVelocityAdd,
    SongQAdd,
    SoundType,
    DeviceNumber,
    ControllChangeCommand,
    RPN,
    RPNCommand,
    NRPN,
    NRPNCommand,
    FadeIO,
    Cresc,
    SysexReset,
    SetRandomSeed,
}

/// Token.value_type
pub const VALUE_UNKNOWN: isize = 0x00;
pub const VALUE_CONST_INT: isize = 0x01;
pub const VALUE_CONST_STR: isize = 0x02;
pub const VALUE_VARIABLE: isize = 0x10;

#[derive(Debug, Clone)]
pub enum TokenValueType {
    VOID,
    INT,
    STR,
    ARRAY,
    VARIABLE,
}


#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: isize,
    pub value_type: TokenValueType,
    pub tag: isize,
    pub data: Vec<SValue>,
    pub children: Option<Vec<Token>>,
    pub lineno: isize,
}

impl Token {
    pub fn new_lineno(lineno: isize) -> Self {
        Self {
            ttype: TokenType::LineNo,
            value: 0,
            value_type: TokenValueType::VOID,
            tag: 0,
            data: vec![],
            children: None,
            lineno,
        }
    }
    pub fn new(ttype: TokenType, value: isize, data: Vec<SValue>) -> Self {
        Self {
            ttype,
            value,
            value_type: TokenValueType::VOID,
            tag: 0,
            data,
            children: None,
            lineno: 0,
        }
    }
    pub fn new_value(ttype: TokenType, value: isize) -> Self {
        Self {
            ttype,
            value,
            value_type: TokenValueType::VOID,
            tag: 0,
            data: vec![],
            children: None,
            lineno: 0,
        }
    }
    pub fn new_value_tag(ttype: TokenType, value: isize, tag: isize) -> Self {
        Self {
            ttype,
            value,
            value_type: TokenValueType::VOID,
            tag,
            data: vec![],
            children: None,
            lineno: 0,
        }
    }
    pub fn new_tokens(ttype: TokenType, value: isize, tokens: Vec<Token>) -> Self {
        Self {
            ttype,
            value,
            value_type: TokenValueType::VOID,
            tag: 0,
            data: vec![],
            children: Some(tokens),
            lineno: 0,
        }
    }
    pub fn new_tokens_lineno(ttype: TokenType, value: isize, tokens: Vec<Token>, lineno: isize) -> Self {
        Self {
            ttype,
            value,
            value_type: TokenValueType::VOID,
            tag: 0,
            data: vec![],
            children: Some(tokens),
            lineno,
        }
    }
    pub fn new_data_tokens(ttype: TokenType, value: isize, data: Vec<SValue>, tokens: Vec<Token>) -> Self {
        Self {
            ttype,
            value,
            value_type: TokenValueType::VOID,
            tag: 0,
            data,
            children: Some(tokens),
            lineno: 0,
        }
    }
    pub fn new_empty(cmd: &str, lineno: isize) -> Self {
        Self {
            ttype: TokenType::Empty,
            value: 0,
            value_type: TokenValueType::VOID,
            tag: 0,
            data: vec![SValue::from_s(cmd.to_string())],
            children: None,
            lineno,
        }
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
        // LineNoは除外
        if self.ttype == TokenType::LineNo {
            return String::new();
        }
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

pub type Tokens = Vec<Token>;

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
