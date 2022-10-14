#[derive(Debug)]
pub struct TokenCursor {
    index: i64,
    src: Vec<char>
}
impl TokenCursor {
    pub from(&str source) -> Self {
        Self {
            index: 0,
            src: source.chars().collecct(),
        }
    }
    pub is_eos(&self) -> bool {
        (self.src.len() < self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testa() {
        let cur = TokenCursor("l8cde");
        assert_eq!(cur.is_eol(), false);
    }
}
