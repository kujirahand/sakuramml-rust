#[derive(Debug)]
pub struct TokenCursor {
    pub index: usize,
    src: Vec<char>,
    pub line: usize
}

impl TokenCursor {
    pub fn from(source: &str) -> Self {
        Self {
            index: 0,
            src: source.chars().collect(),
            line: 0,
        }
    }
    pub fn is_eos(&self) -> bool {
        self.src.len() <=  self.index
    }
    pub fn get_char(&mut self) -> char {
        if self.is_eos() { return '\0'; }
        let ch = self.src[self.index];
        self.index += 1;
        return ch;
    }
    pub fn next(&mut self) {
        if self.is_eos() { return }
        self.index += 1;
    }
    pub fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }
    pub fn peek(&mut self) -> Option<char> {
        if self.is_eos() { return None; }
        let c = self.src[self.index];
        Some(c)
    }
    pub fn peek_n(&self, n: usize) -> char {
        let idx = self.index + n;
        if self.src.len() <= idx { return '\0'; }
        self.src[idx]
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
            if self.src[idx] != s2[i] {
                return false;
            }
        }
        true
    }
    pub fn eq_char(&self, ch1: char) -> bool {
        if self.is_eos() { return false; }
        let ch2 = self.peek_n(0);
        return ch2 == ch1;
    }
    pub fn get_token_ch(&mut self, splitter: char) -> String {
        let mut s = String::new();
        while !self.is_eos() {
            let ch = self.get_char();
            if ch == splitter {
                break
            }
            s.push(ch);
        }
        s
    }
    pub fn get_token_nest(&mut self, open_ch: char, close_ch: char) -> String {
        let mut s: String = String::new();
        let mut level: usize = 0;
        // top
        if self.peek_n(0) == open_ch {
            level += 1;
            self.index += 1;
        }
        while !self.is_eos() {
            let ch = self.get_char();
            if ch == open_ch {
                level += 1;
            }
            else if ch == close_ch {
                level -= 1;
                // last?
                if level == 0 { break; }
            }
            s.push(ch);
        }
        s
    }
    pub fn cur2end(&self) -> String {
        let mut res = String::new();
        for i in self.index..self.src.len() {
            res.push(self.src[i]);
        }
        res
    }
    pub fn skip_space_ret(&mut self) {
        while !self.is_eos() {
            let ch = self.peek_n(0);
            match ch {
                '\r' | '\n' | '\t' | ' ' => {
                    self.index += 1;
                },
                _ => { break; }
            }
        }
    }
    pub fn skip_space(&mut self) {
        while !self.is_eos() {
            let ch = self.peek_n(0);
            match ch {
                '\t' | ' ' => {
                    self.index += 1;
                },
                _ => { break; }
            }
        }
    }
    pub fn get_note_length(&mut self) -> String {
        let mut res = String::new();
        while !self.is_eos() {
            let ch = self.peek_n(0);
            match ch {
                '0'..='9' | '.' | '^' => {
                    res.push(ch);
                    self.index += 1;
                    continue;
                }
                _ => { break; }
            }
        }
        res
    }
    pub fn get_word(&mut self) -> String {
        let mut res = String::new();
        while !self.is_eos() {
            let ch = self.peek_n(0);
            match ch {
                'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => {
                    res.push(ch);
                    self.index += 1;
                    continue;
                }
                _ => { break; }
            }
        }
        res
    }
    pub fn get_int(&mut self, def: isize) -> isize {
        let mut s = String::new();
        while !self.is_eos() {
            let ch = self.peek().unwrap_or('\0');
            if '0' <= ch && ch <= '9' {
                s.push(ch);
                self.index += 1;
                continue;
            }
            break;
        }
        s.parse().unwrap_or(def)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        // 
        let mut cur = TokenCursor::from("l16cde");
        assert_eq!(cur.is_eos(), false);
        assert_eq!(cur.get_char(), 'l');
        assert_eq!(cur.get_char(), '1');
        assert_eq!(cur.get_char(), '6');
        assert_eq!(cur.get_char(), 'c');
        assert_eq!(cur.get_char(), 'd');
        assert_eq!(cur.get_char(), 'e');
        assert_eq!(cur.get_char(), '\0');
        assert_eq!(cur.get_char(), '\0');
        //
        let mut cur = TokenCursor::from("l16cde");
        assert_eq!(cur.get_char(), 'l');
        assert_eq!(cur.eq("16"), true);
    }
    #[test]
    fn test_basic2() {
        let mut cur = TokenCursor::from("l16cde");
        assert_eq!(cur.get_char(), 'l');
        assert_eq!(cur.is_numeric(), true);
        assert_eq!(cur.eq("16"), true);
        cur.index += 2;
        assert_eq!(cur.eq("cde"), true);
    }
    #[test]
    fn test_skip() {
        let mut cur = TokenCursor::from("   cde");
        cur.skip_space();
        assert_eq!(cur.cur2end(), String::from("cde"));
    }
    #[test]
    fn test_get_token_ch() {
        //
        let mut cur = TokenCursor::from("123,456,789");
        assert_eq!(cur.get_token_ch(','), String::from("123"));
        assert_eq!(cur.get_token_ch(','), String::from("456"));
        assert_eq!(cur.get_token_ch(','), String::from("789"));
        //
        let mut cur = TokenCursor::from("123,456,789");
        assert_eq!(cur.get_token_ch('*'), String::from("123,456,789"));
    }
    #[test]
    fn test_get_token_nest() {
        //
        let mut cur = TokenCursor::from("{abc}");
        assert_eq!(cur.get_token_nest('{', '}'), String::from("abc"));
        //
        let mut cur = TokenCursor::from("{aaa{bbb}ccc}");
        assert_eq!(cur.get_token_nest('{', '}'), String::from("aaa{bbb}ccc"));
    }
}
