use std::vec;

use super::svalue::SValue;

/// TokenType
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Unimplemented,
    Error,
    Empty,
    LineNo,
    Comment,
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
    ControlChange,
    CConTime,
    CConNote,
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
    CalcTree,
    ConstInt,
    ConstStr,
    MakeArray,
    GetVariable,
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
    ControlChangeCommand,
    RPN,
    RPNCommand,
    NRPN,
    NRPNCommand,
    FadeIO,
    Cresc,
    SysexReset,
    SysExCommand,
    SetRandomSeed,
    DirectSMF,
    NoteOn,
    NoteOff,
    StrVarReplace,
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
    pub value_i: isize,
    pub value_s: Option<String>,
    pub value_type: TokenValueType,
    pub tag: isize,
    pub operator_flag: char,
    pub data: Vec<SValue>,
    pub children: Option<Vec<Token>>,
    pub lineno: isize,
}

impl Token {
    pub fn new_lineno(lineno: isize) -> Self {
        Self {
            ttype: TokenType::LineNo,
            value_i: 0,
            value_s: None,
            value_type: TokenValueType::VOID,
            tag: 0,
            operator_flag: '\0',
            data: vec![],
            children: None,
            lineno,
        }
    }
    pub fn new(ttype: TokenType, value: isize, data: Vec<SValue>) -> Self {
        Self {
            ttype,
            value_i: value,
            value_s: None,
            value_type: TokenValueType::VOID,
            tag: 0,
            operator_flag: '\0',
            data,
            children: None,
            lineno: 0,
        }
    }
    pub fn new_const0() -> Self {
        Self {
            ttype: TokenType::ConstInt,
            value_i: 0,
            value_s: None,
            value_type: TokenValueType::VOID,
            tag: 0,
            operator_flag: '\0',
            data: vec![],
            children: None,
            lineno: 0,
        }
    }
    pub fn new_const(ttype: TokenType, value_i: isize, value_s: Option<String>, value_type: TokenValueType) -> Self {
        Self {
            ttype,
            value_i,
            value_s,
            value_type,
            tag: 0,
            operator_flag: '\0',
            data: vec![],
            children: None,
            lineno: 0,
        }
    }
    pub fn new_variable(ttype: TokenType, var_name: String, init_tokens: Option<Vec<Token>>) -> Self {
        Self {
            ttype,
            value_i: 0,
            value_s: Some(var_name),
            value_type: TokenValueType::VOID,
            tag: 0,
            operator_flag: '\0',
            data: vec![],
            children: init_tokens,
            lineno: 0,
        }
    }
    pub fn new_value(ttype: TokenType, value: isize) -> Self {
        Self {
            ttype,
            value_i: value,
            value_s: None,
            value_type: TokenValueType::VOID,
            tag: 0,
            operator_flag: '\0',
            data: vec![],
            children: None,
            lineno: 0,
        }
    }
    pub fn new_value_tag(ttype: TokenType, value: isize, tag: isize) -> Self {
        Self {
            ttype,
            value_i: value,
            value_s: None,
            value_type: TokenValueType::VOID,
            tag,
            operator_flag: '\0',
            data: vec![],
            children: None,
            lineno: 0,
        }
    }
    pub fn new_tokens(ttype: TokenType, value_i: isize, tokens: Vec<Token>) -> Self {
        Self {
            ttype,
            value_i,
            value_s: None,
            value_type: TokenValueType::VOID,
            tag: 0,
            operator_flag: '\0',
            data: vec![],
            children: Some(tokens),
            lineno: 0,
        }
    }
    pub fn new_calc_token(operator_ch: char, priority: isize, children: Vec<Token>) -> Self {
        Self {
            ttype: TokenType::CalcTree,
            value_i: priority,
            value_s: None,
            value_type: TokenValueType::VOID,
            tag: 0,
            operator_flag: operator_ch,
            data: vec![],
            children: Some(children),
            lineno: 0,
        }
    }
    pub fn new_tokens_lineno(ttype: TokenType, value: isize, tokens: Vec<Token>, lineno: isize) -> Self {
        Self {
            ttype,
            value_i: value,
            value_s: None,
            value_type: TokenValueType::VOID,
            tag: 0,
            operator_flag: '\0',
            data: vec![],
            children: Some(tokens),
            lineno,
        }
    }
    pub fn new_data_tokens(ttype: TokenType, value: isize, data: Vec<SValue>, tokens: Vec<Token>) -> Self {
        Self {
            ttype,
            value_i: value,
            value_s: None,
            value_type: TokenValueType::VOID,
            tag: 0,
            operator_flag: '\0',
            data,
            children: Some(tokens),
            lineno: 0,
        }
    }
    pub fn new_empty(cmd: &str, lineno: isize) -> Self {
        Self {
            ttype: TokenType::Empty,
            value_i: 0,
            value_s: Some(String::from(cmd)),
            value_type: TokenValueType::VOID,
            tag: 0,
            operator_flag: '\0',
            data: vec![],
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
    pub fn to_debug_str(&self, level: isize) -> String {
        // LineNoは除外
        if self.ttype == TokenType::LineNo {
            return String::new();
        }
        match self.ttype {
            TokenType::CalcTree => {
                if let Some(children) = &self.children {
                    let mut s = String::new();
                    for t in children.iter() {
                        s.push_str(&t.to_debug_str(level+1));
                    }
                    return format!(
                        "[{:?} {}({}){{{}}}]",
                        self.ttype,
                        self.operator_flag,
                        self.value_i,
                        s);
                }
                let line = format!("[{:?},{}]", self.ttype, self.value_i);
                return line;
            },
            _ => {},
        }
        let line = format!("[{:?},{}]", self.ttype, self.value_i);
        line
    }
}

// indent line for debug print
fn indent_line(src: &str, level: isize) -> String {
    let part0 = " │  ";
    let part1 = " ├──";
    let mut s = String::new();
    for i in 0..level {
        if i == level-1 {
            s.push_str(part1);
            continue;
        }
        s.push_str(part0);
    }
    s.push_str(src);
    s
}

pub fn tokens_to_str(tokens: &Vec<Token>) -> String {
    let mut s = String::new();
    for t in tokens.iter() {
        s.push_str(&t.to_debug_str(0));
    }
    s
}

fn opt_str_short(s: &Option<String>, head: &str) -> String {
    match s {
        Some(s) => format!("{}{}", head, s),
        None => String::from(""),
    }
}

pub fn tokens_to_debug_str(tokens: &Vec<Token>, level: isize) -> String {
    let mut lineno = 0;
    let mut s = String::new();
    for t in tokens.iter() {
        if t.ttype == TokenType::LineNo {
            lineno = t.lineno;
        }
        let line = format!("[{:?}](i:{}{},line:{})\n", t.ttype, t.value_i, opt_str_short(&t.value_s, ",s:"), lineno);
        s.push_str(&indent_line(&line, level));
        if let Some(children) = &t.children {
            s.push_str(&tokens_to_debug_str(children, level+1));
        }
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
