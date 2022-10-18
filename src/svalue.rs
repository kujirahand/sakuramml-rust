
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum SValue {
    Int(isize),
    Str(String, isize),
    Array(Vec<SValue>),
    IntArray(Vec<isize>),
    None,
}

impl SValue {
    pub fn new() -> Self {
        Self::None
    }
    pub fn from_int_array_to_svalue_array(a: Vec<isize>) -> Self {
        let mut sa: Vec<SValue> = vec![];
        for v in a.iter() {
            sa.push(SValue::from_i(*v));
        }
        SValue::Array(sa)
    }
    pub fn from_int_array(a: Vec<isize>) -> Self {
        SValue::IntArray(a)
    }
    pub fn from_i(v: isize) -> Self {
        Self::Int(v)
    }
    pub fn from_s(s: String) -> Self {
        Self::Str(s, 0)
    }
    pub fn from_str(s: &str) -> Self {
        Self::Str(String::from(s), 0)
    }
    pub fn from_str_and_tag(s: &str, tag: isize) -> Self {
        Self::Str(String::from(s), tag)
    }
    pub fn to_i(&self) -> isize {
        match self {
            Self::Int(i) => *i,
            Self::Str(s, _) => s.parse().unwrap_or(0), 
            Self::None => 0,
            _ => 0,
        }
    }
    pub fn to_s(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::Str(s, _) => s.clone(),
            Self::None => String::new(),
            _ => String::new(),
        }
    }
    pub fn get_str_and_tag(&self) -> (String, isize) {
        match self {
            Self::Int(i) => (i.to_string(), 0),
            Self::Str(s, no) => { (s.clone(), *no) },
            Self::None => (String::new(), 0),
            _ => (String::new(), 0),
        }
    }
    pub fn to_int_array(&self) -> Vec<isize> {
        match self {
            Self::Array(a) => {
                let mut res: Vec<isize> = vec![];
                for v in a.iter() {
                    res.push(v.to_i());
                }
                res
            },
            Self::IntArray(a) => a.clone(),
            _ => {
                vec![self.to_i()]
            }
        }
    }
    pub fn to_array(&self) -> Vec<SValue> {
        match self {
            Self::Array(a) => a.clone(),
            Self::IntArray(a) => {
                let mut res: Vec<SValue> = vec![];
                for v in a.iter() {
                    res.push(SValue::from_i(*v));
                }
                res
            },
            _ => {
                vec![self.clone()]
            }
        }
    }
}
