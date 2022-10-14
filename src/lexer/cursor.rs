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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testa() {
        let cur = TokenCursor::from("l8cde");
        assert_eq!(cur.is_eos(), false);
    }
}
