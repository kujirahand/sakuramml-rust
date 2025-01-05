//! Source reader

#[derive(Debug)]
pub struct TokenCursor {
    /// position
    pub index: usize,
    src: Vec<char>,
    /// line number
    pub line: isize,
}

impl TokenCursor {
    /// cursor from source
    pub fn from(source: &str) -> Self {
        Self {
            index: 0,
            src: source.chars().collect(),
            line: 0,
        }
    }
    /// is eos
    pub fn is_eos(&self) -> bool {
        self.src.len() <=  self.index
    }
    pub fn has_next(&self) -> bool {
        self.index < self.src.len()
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
    pub fn next_n(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
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
    /// peek [index+n] char
    pub fn peek_n(&self, n: usize) -> char {
        let idx = self.index + n;
        if self.src.len() <= idx { return '\0'; }
        self.src[idx]
    }
    pub fn peek_str_n(&self, n: usize) -> String {
        let mut s = String::new();
        for i in 0..n {
            let idx = self.index + i;
            if idx >= self.src.len() { return s; }
            s.push(self.src[idx]);
        }
        s
    }
    /// replace current char
    pub fn replace_char(&mut self, ch: char) {
        if self.is_eos() { return; }
        self.src[self.index] = ch;
    }
    pub fn is_numeric(&self) -> bool {
        if self.is_eos() { return false; }
        let c = self.src[self.index];
        return ('0' <= c) && (c <= '9');
    }
    pub fn is_upper(&self) -> bool {
        if self.is_eos() { return false; }
        let c = self.src[self.index];
        return ('A' <= c) && (c <= 'Z');
    }
    pub fn is_lower(&self) -> bool {
        if self.is_eos() { return false; }
        let c = self.src[self.index];
        return ('a' <= c) && (c <= 'z');
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
            if self.eq_char('\n') {
                self.line += 1;
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
    pub fn get_token_to_double_quote(&mut self) -> String {
        let mut s = String::new();
        while !self.is_eos() {
            let ch = self.get_char();
            if ch == '\n' { self.line += 1; }
            if ch == '\\' {
                let ch2 = self.get_char();
                if ch2 != '\0' { s.push(ch2); }
                continue;
            }
            if ch == '"' { break; }
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
            self.next();
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
                '/' => {
                    if self.eq("//") {
                        self.get_token_ch('\n');
                        continue;
                    }
                    if self.eq("/*") {
                        self.get_token_s("*/");
                        continue;
                    }
                    break;
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
                '/' => {
                    if self.eq("/*") {
                        self.get_token_s("*/");
                        continue;
                    }
                    break;
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
                '0'..='9' | '.' | '^' | '%' | '-' | '+' => {
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
        if self.eq_char('#') {
            res.push('#');
            self.next();
        }
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
    pub fn get_hex(&mut self, def: isize, check_flag: bool) -> isize {
        let mut no: isize = 0;
        let mut flag: isize = 1;
        if check_flag {
            // minus
            if self.eq_char('-') {
                flag = -1;
                self.next();
            }
            // hex flag
            if self.eq_char('$') {
                self.next();
            }
            if self.eq("0x") {
                self.index += 2;
            }
        }
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
    pub fn get_int(&mut self, def: isize) -> isize {
        let mut no: isize = 0;
        let mut flag: isize = 1;
        // minus
        if self.eq_char('-') {
            flag = -1;
            self.next();
        }
        // Hex integer?
        if self.eq("0x") || self.eq_char('$') {
            return flag * self.get_hex(def, true);
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
        //
        let mut cur = TokenCursor::from("(abc(ddd))aaa");
        assert_eq!(cur.get_token_nest('(', ')'), String::from("abc(ddd)"));
        //
        let mut cur = TokenCursor::from("(abc(ddd)e)aaa");
        assert_eq!(cur.get_token_nest('(', ')'), String::from("abc(ddd)e"));
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
        //
        let mut cur = TokenCursor::from("$FF");
        assert_eq!(cur.get_int(0), 0xFF);
        //
        let mut cur = TokenCursor::from("-$FF");
        assert_eq!(cur.get_int(0), -0xFF);
    }
    #[test]
    fn test_get_word() {
        let mut cur = TokenCursor::from("ABC");
        assert_eq!(&cur.get_word(), "ABC");
        let mut cur = TokenCursor::from("_ABC");
        assert_eq!(&cur.get_word(), "_ABC");
        let mut cur = TokenCursor::from("Abc");
        assert_eq!(&cur.get_word(), "Abc");
        let mut cur = TokenCursor::from("A123");
        assert_eq!(&cur.get_word(), "A123");
        let mut cur = TokenCursor::from("#abc#def");
        assert_eq!(&cur.get_word(), "#abc");
        let mut cur = TokenCursor::from("#abc(30)");
        assert_eq!(&cur.get_word(), "#abc");
    }
    #[test]
    fn test_peek_str_n() {
        let cur = TokenCursor::from("abcdefg");
        assert_eq!(cur.peek_str_n(1), "a".to_string());
        assert_eq!(cur.peek_str_n(3), "abc".to_string());
        assert_eq!(cur.peek_str_n(5), "abcde".to_string());
        let cur = TokenCursor::from("ドレミ");
        assert_eq!(cur.peek_str_n(1), "ド".to_string());
        assert_eq!(cur.peek_str_n(3), "ドレミ".to_string());
        assert_eq!(cur.peek_str_n(5), "ドレミ".to_string());
    }
    #[test]
    fn test_get_token_to_dq() {
        // simple
        let mut cur = TokenCursor::from("\"abc\"");
        cur.next(); // skip '"'
        assert_eq!(cur.get_token_to_double_quote(), "abc".to_string());
        // multi line
        let mut cur = TokenCursor::from("\"abc\ndef\"");
        cur.next(); // skip '"'
        assert_eq!(cur.get_token_to_double_quote(), "abc\ndef".to_string());
        // escape char
        let mut cur = TokenCursor::from("\"abc\\\"def\"");
        cur.next(); // skip '"'
        assert_eq!(cur.get_token_to_double_quote(), "abc\"def".to_string());
    }
}
