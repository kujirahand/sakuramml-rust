
#[allow(dead_code)]
#[derive(Debug)]
pub enum SValue {
    Int(isize),
    Str(String),
    Array(Vec<SValue>),
    None,
}

impl SValue {
    pub fn new() -> Self {
        Self::None
    }
    pub fn from_i(v: isize) -> Self {
        Self::Int(v)
    }
    pub fn from_s(s: String) -> Self {
        Self::Str(s)
    }
    pub fn to_i(&self) -> isize {
        match self {
            Self::Int(i) => *i,
            Self::Str(s) => s.parse().unwrap_or(0), 
            Self::None => 0,
            _ => 0,
        }
    }
    pub fn to_s(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::Str(s) => s.clone(),
            Self::None => String::new(),
            _ => String::new(),
        }
    }
}