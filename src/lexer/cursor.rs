#[derive(Debug)]
pub struct TokenCursor {
    index: usize,
    src: Vec<char>
}

impl TokenCursor {
    pub fn from(source: &str) -> Self {
        Self {
            index: 0,
            src: source.chars().collect(),
        }
    }
    pub fn is_eos(&self) -> bool {
        self.src.len() <=  self.index
    }
    pub fn next(&mut self) -> Option<char> {
        if self.is_eos() { return None; }
        let c = self.src[self.index];
        self.index += 1;
        Some(c)
    }
    pub fn peek(&mut self) -> Option<char> {
        if self.is_eos() { return None; }
        let c = self.src[self.index];
        Some(c)
    }
    pub fn is_numeric(&self) -> bool {
        if self.is_eos() { return false; }
        let c = self.src[self.index];
        return ('0' <= c) && (c <= '9');
    }
    pub fn eq(&self, s: &str) -> bool {
        let s2: Vec<char> = s.chars().collect();
        for i in 0..s2.len() {
            let idx = self.index + i;
            println!("{}={}", self.src[idx], s2[i]);
            if self.src[idx] != s2[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testa() {
        let mut cur = TokenCursor::from("l16cde");
        assert_eq!(cur.is_eos(), false);
        assert_eq!(cur.next(), Some('l'));
        assert_eq!(cur.next(), Some('1'));
        assert_eq!(cur.next(), Some('6'));
        assert_eq!(cur.eq("cde"), true);
    }
}
