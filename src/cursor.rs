#[derive(Debug)]
pub struct TokenCursor {
    pub index: usize,
    src: Vec<char>,
    pub line: isize,
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
            if idx >= self.src.len() { return false; }
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
    pub fn get_token_s(&mut self, splitter: &str) -> String {
        let mut s = String::new();
        while !self.is_eos() {
            if self.eq(splitter) {
                self.index += splitter.chars().count();
                break;
            }
            s.push(self.get_char());
        }
        s
    }
    pub fn get_token_ch(&mut self, splitter: char) -> String {
        let mut s = String::new();
        while !self.is_eos() {
            let ch = self.get_char();
            if ch == '\n' { self.line += 1; }
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
            if ch == '\n' { self.line += 1; }
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
                    if ch == '\n' { self.line += 1; }
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
                '0'..='9' | '.' | '^' | '%' => {
                    res.push(ch);
                    self.index += 1;
                    continue;
                },
                ' ' | '|' => {
                    self.next();
                },
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
        let mut no: isize = 0;
        let mut flag: isize = 1;
        // minus
        if self.eq_char('-') {
            flag = -1;
            self.next();
        }
        // Hex integer?
        if self.eq("0x") {
            self.index += 2; // skip 0x
            // Hex chars?
            let ch = self.peek_n(0);
            match ch {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {},
                _ => { return def; }
            }
            // calc number
            while !self.is_eos() {
                let ch = self.peek_n(0);
                match ch {
                    '0'..='9' => {
                        no = no << 4 | ch as isize - '0' as isize;
                        self.next();
                        continue;
                    },
                    'a'..='f' => {
                        no = (no << 4) | (0x0a + (ch as isize - 'a' as isize));
                        self.next();
                        continue;
                    },
                    'A'..='F' => {
                        no = (no << 4) | (0x0a + (ch as isize - 'A' as isize));
                        self.next();
                        continue;
                    },
                    _ => { break; }
                }
            }
            return no * flag;
        }
        // Oct integer?
        if self.eq("0o") {
            self.index += 2; // skip 0o
            // Oct chars?
            let ch = self.peek_n(0);
            match ch {
                '0'..='8' => {},
                _ => { return def; }
            }
            // calc number
            while !self.is_eos() {
                let ch = self.peek_n(0);
                match ch {
                    '0'..='8' => {
                        no = no * 8 + (ch as isize - '0' as isize);
                        self.next();
                        continue;
                    },
                    _ => { break; }
                }
            }
            return no * flag;
        }
        // check numeric
        if !self.is_numeric() { return def; }
        while !self.is_eos() {
            let ch = self.peek_n(0);
            match ch {
                '0'..='9' => {
                    no = no * 10 + (ch as isize - '0' as isize);
                    self.next();
                },
                _ => break,
            }
        }
        no * flag
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
    fn test_get_token_s() {
        //
        let mut cur = TokenCursor::from("123::456::789");
        assert_eq!(cur.get_token_s("::"), String::from("123"));
        assert_eq!(cur.get_token_s("::"), String::from("456"));
        assert_eq!(cur.get_token_s("::"), String::from("789"));
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
    #[test]
    fn test_get_int() {
        let mut cur = TokenCursor::from("345");
        assert_eq!(cur.get_int(-1), 345);
        //
        let mut cur = TokenCursor::from("l8");
        cur.next();
        assert_eq!(cur.get_int(-1), 8);
        //
        let mut cur = TokenCursor::from("0xFF");
        assert_eq!(cur.get_int(-1), 0xFF);
        //
        let mut cur = TokenCursor::from("0x");
        assert_eq!(cur.get_int(-1), -1);
        //
        let mut cur = TokenCursor::from("0xff");
        assert_eq!(cur.get_int(-1), 0xff);
        //
        let mut cur = TokenCursor::from("0x123");
        assert_eq!(cur.get_int(-1), 0x123);
        //
        let mut cur = TokenCursor::from("a");
        assert_eq!(cur.get_int(-1), -1);
        //
        let mut cur = TokenCursor::from("1234");
        assert_eq!(cur.get_int(-1), 1234);
        //
        let mut cur = TokenCursor::from("0o777");
        assert_eq!(cur.get_int(-1), 0o777);
        //
        let mut cur = TokenCursor::from("-111");
        assert_eq!(cur.get_int(0), -111);
        //
        let mut cur = TokenCursor::from("-0xFF");
        assert_eq!(cur.get_int(0), -0xFF);
    }
}
