
#[allow(dead_code)]
#[derive(Debug)]
pub enum SValue {
    Int(i64),
    Str(String),
    None,
}

impl SValue {
    pub fn new() -> Self {
        Self::None
    }
    pub fn from_i(v: i64) -> Self {
        Self::Int(v)
    }
    pub fn from_s(s: String) -> Self {
        Self::Str(s)
    }
    pub fn to_i(&self) -> i64 {
        match self {
            Self::Int(i) => *i,
            Self::Str(s) => s.parse().unwrap_or(0), 
            Self::None => 0,
        }
    }
    pub fn to_s(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::Str(s) => s.clone(),
            Self::None => String::new(),
        }
    }
}
