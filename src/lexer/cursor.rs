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
    }
}
