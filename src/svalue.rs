
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum SValue {
    Int(isize),
    Str(String, isize),
    Bool(bool),
    Array(Vec<SValue>),
    IntArray(Vec<isize>),
    StrArray(Vec<String>),
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
    pub fn from_str_array(a: Vec<String>) -> Self {
        SValue::StrArray(a)
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
    pub fn from_b(b: bool) -> Self {
        Self::Bool(b)
    }
    pub fn to_b(&self) -> bool {
        let v = self.to_i();
        return v != 0;
    }
    pub fn to_i(&self) -> isize {
        match self {
            Self::Int(i) => *i,
            Self::Str(s, _) => s.parse().unwrap_or(0),
            Self::Bool(b) => if *b { 1 } else { 0 },
            Self::None => 0,
            _ => 0,
        }
    }
    pub fn to_s(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::Str(s, _) => s.clone(),
            Self::Bool(b) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
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
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
    pub fn eq(&self, v: SValue) -> bool {
        match v {
            Self::Int(vi) => {
                let si = self.to_i();
                return si == vi;
            },
            Self::Str(vs, _) => {
                let ss = self.to_s();
                return ss == vs;
            },
            Self::None => {
                return self.is_none();
            },
            _ => {},
        }
        false
    }
    pub fn ne(&self, v: SValue) -> bool {
        !self.eq(v)
    }
    pub fn gt(&self, v: SValue) -> bool {
        match self {
            Self::Int(i) => {
                return i > &v.to_i();
            },
            Self::Str(s, _) => {
                return s > &v.to_s();
            },
            _ => {},
        }
        false
    }
    pub fn gteq(&self, v: SValue) -> bool {
        match self {
            Self::Int(i) => {
                return i >= &v.to_i();
            },
            Self::Str(s, _) => {
                return s >= &v.to_s();
            },
            _ => {},
        }
        false
    }
    pub fn lt(&self, v: SValue) -> bool {
        match self {
            Self::Int(i) => {
                return i < &v.to_i();
            },
            Self::Str(s, _) => {
                return s < &v.to_s();
            },
            _ => {},
        }
        false
    }
    pub fn div(&self, v: SValue) -> SValue {
        let i1 = self.to_i();
        let i2 = v.to_i();
        if i2 == 0 {
            return SValue::from_i(0);
        }
        SValue::from_i(i1 / i2)
    }
    pub fn lteq(&self, v: SValue) -> bool {
        match self {
            Self::Int(i) => {
                return i <= &v.to_i();
            },
            Self::Str(s, _) => {
                return s <= &v.to_s();
            },
            _ => {},
        }
        false
    }
    pub fn is_s(&self) -> bool {
        match self {
            Self::Str(_, _) => true,
            _ => false,
        }
    }
    pub fn add(&self, v: SValue) -> SValue {
        if self.is_s() || v.is_s() {
            let mut s1 = self.to_s().clone();
            s1.push_str(&v.to_s());
            return Self::Str(s1, 0);
        }
        // check target
        match v {
            Self::Int(vi) => {
                let si = self.to_i();
                return Self::Int(si + vi);
            },
            _ => {},
        }
        // others
        let i1 = self.to_i();
        let i2 = v.to_i();
        SValue::Int(i1 + i2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        // 
        let si = SValue::from_i(100);
        assert_eq!(si.to_i(), 100);
    }
    #[test]
    fn test_comp() {
        let a = SValue::from_i(100);
        let b = SValue::from_i(200);
        assert_eq!(a.lt(b), true);
    }
}
