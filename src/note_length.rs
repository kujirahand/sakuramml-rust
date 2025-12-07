use crate::source_cursor::SourceCursor;

/// Calculate note length
pub fn calc_length(len_str: &str, timebase: isize, def_len: isize) -> isize {
    let mut res = def_len;
    if len_str == "" {
        return def_len;
    }
    let mut cur = SourceCursor::from(len_str);
    let mut step_mode = false;
    if cur.eq_char('%') {
        cur.next();
        step_mode = true;
    }
    if cur.is_numeric() || cur.eq_char('-') {
        if step_mode {
            res = cur.get_int(0);
        } else {
            let i = cur.get_int(4);
            res = if i > 0 { timebase * 4 / i } else { 0 };
        }
    }
    if cur.peek_n(0) == '.' {
        if cur.eq("....") {
            cur.next_n(4);
            res += (res as f32 / 2.0 + res as f32 / 4.0 + res as f32 / 8.0 + res as f32 / 16.0) as isize;
        } else if cur.eq("...") { // triple dotted note (三付点音符)
            cur.next_n(3);
            res += (res as f32 / 2.0 + res as f32 / 4.0 + res as f32 / 8.0) as isize;
        } else if cur.eq("..") { // double dotted note (複付点音符)
            cur.next_n(2);
            res += (res as f32 / 2.0 + res as f32 / 4.0) as isize;
        } else { // dotted note
            cur.next();
            res += (res as f32 / 2.0) as isize;
        }
    }
    while !cur.is_eos() {
        let c = cur.peek_n(0);
        if (c != '^') && (c != '+') {
            break;
        }
        cur.next(); // skip '^'
        if cur.eq_char('%') {
            step_mode = true;
            cur.next();
        }
        if cur.is_numeric() || cur.eq_char('-') {
            let mut n = if step_mode {
                cur.get_int(0)
            } else {
                let i = cur.get_int(4);
                if i == 0 {
                    def_len
                } else {
                    timebase * 4 / i
                }
            };
            if cur.eq("....") {
                cur.next_n(4);
                n += (n as f32 / 2.0 + n as f32 / 4.0 + n as f32 / 8.0 + n as f32 / 16.0) as isize;
            } else if cur.eq("...") {
                cur.next_n(3);
                n += (n as f32 / 2.0 + n as f32 / 4.0 + n as f32 / 8.0) as isize;
            } else if cur.eq("..") {
                cur.next_n(2);
                n += (n as f32 / 2.0 + n as f32 / 4.0) as isize;
            } else if cur.peek_n(0) == '.' {
                cur.next();
                n = (n as f32 * 1.5) as isize;
            }
            res += n;
        } else {
            res += def_len;
        }
    }
    res
}

#[cfg(test)]
mod calc_length_tests {
    use super::calc_length;

    #[test]
    fn calc_length_base_test() {
        let timebase: isize = 480;
        let t1: isize = timebase * 4;
        assert_eq!(calc_length("1", timebase, timebase), t1 / 1);
        assert_eq!(calc_length("2", timebase, timebase), t1 / 2);
        assert_eq!(calc_length("4", timebase, timebase), t1 / 4);
        assert_eq!(calc_length("8", timebase, timebase), t1 / 8);
        assert_eq!(calc_length("16", timebase, timebase), t1 / 16);
        assert_eq!(calc_length("24", timebase, timebase), t1 / 24);
    }
    #[test]
    fn calc_length_defaultlen_test() {
        let timebase: isize = 480;
        let t1: isize = timebase * 4;
        let t4: isize = t1 / 4;
        // check default length
        assert_eq!(calc_length("", timebase, timebase), timebase);
        assert_eq!(calc_length("", timebase, t1), t1);
        assert_eq!(calc_length("", timebase, t1/4), t1/4);
        // '^' length
        assert_eq!(calc_length("^", timebase, t4), t4 + t4);
        assert_eq!(calc_length("^^", timebase, t4), t4 + t4 + t4);
        assert_eq!(calc_length("^^^", timebase, t4), t1);
        assert_eq!(calc_length("^8", timebase, t4), t4 + t4/2);
        assert_eq!(calc_length("^8.", timebase, t4), t4 + t1/8 + t1/16);
    }
    #[test]
    fn calc_length_dot_test() {
        let timebase: isize = 480;
        let t1: isize = timebase * 4;
        let t4: isize = t1 / 4;
        let t8: isize = t1 / 8;
        let t16: isize = t1 / 16;
        assert_eq!(calc_length("1.", timebase, t1), t1 + t1/2);
        assert_eq!(calc_length("2.", timebase, t1), t1/2 + t4);
        assert_eq!(calc_length("4.", timebase, t1), t4 + t8);
        assert_eq!(calc_length("8.", timebase, t1), t8 + t16);
        assert_eq!(calc_length("16.", timebase, t1), t16 + t16 / 2);
    }
    #[test]
    fn calc_length_double_dotted_note_test() { // double dotted note (複付点音符)
        let timebase: isize = 480;
        let t1: isize = timebase * 4;
        let t2: isize = t1 / 2;
        let t4: isize = t1 / 4;
        let t8: isize = t1 / 8;
        let t16: isize = t1 / 16;
        // double
        assert_eq!(calc_length("1..", timebase, t4), t1 + t2 + t4);
        assert_eq!(calc_length("2..", timebase, t4), t2 + t4 + t8);
        assert_eq!(calc_length("4..", timebase, t4), t4 + t8 + t16);
        assert_eq!(calc_length("4..^4..", timebase, t4), (t4 + t8 + t16) * 2);
        // triple
        assert_eq!(calc_length("1...", timebase, t4), t1 + t2 + t4 + t8);
        assert_eq!(calc_length("1...^1...", timebase, t4), (t1 + t2 + t4 + t8) * 2);
        assert_eq!(calc_length("1...^1...^1...", timebase, t4), (t1 + t2 + t4 + t8) * 3);
        // dot * 4
        assert_eq!(calc_length("1....", timebase, t4), t1 + t2 + t4 + t8 + t16);
        assert_eq!(calc_length("1....^1....", timebase, t4), (t1 + t2 + t4 + t8 + t16) * 2);
    }
    #[test]
    fn calc_length_calc_test() {
        let timebase: isize = 480;
        let t1: isize = timebase * 4;
        let t4: isize = t1 / 4;
        let t8: isize = t1 / 8;
        let t16: isize = t1 / 16;
        assert_eq!(calc_length("4^4", timebase, t1), t4 + t4);
        assert_eq!(calc_length("4^8", timebase, t4), t4 + t8);
        assert_eq!(calc_length("4^8^8", timebase, t4), t4 + t8 + t8);
        assert_eq!(calc_length("4^8^16", timebase, t4), t4 + t8 + t16);
        // dot
        assert_eq!(calc_length("4.^4.", timebase, t1), (t4 + t8) * 2);
        assert_eq!(calc_length("4.^4.^4.", timebase, t1), (t4 + t8) * 3);
        assert_eq!(calc_length("4.^4.^4.^4.", timebase, t1), (t4 + t8) * 4);
        // default + dot
        assert_eq!(calc_length("^4.", timebase, t4), t4 + (t4 + t8));
    }
    #[test]
    fn test_calc_len() {
        assert_eq!(calc_length("4", 480, 480), 480);
        assert_eq!(calc_length("", 480, 480), 480);
        assert_eq!(calc_length("8", 480, 480), 240);
        assert_eq!(calc_length("8^", 480, 240), 480);
        assert_eq!(calc_length("^^^", 480, 240), 480 * 2);
        assert_eq!(calc_length("4.", 480, 480), 480 + 240);
        assert_eq!(calc_length("4.^", 480, 240), 240 * 4);
    }
    #[test]
    fn test_calc_len2() {
        assert_eq!(calc_length("4", 96, 48), 96);
        assert_eq!(calc_length("", 96, 48), 48);
        assert_eq!(calc_length("^", 96, 48), 96);
        assert_eq!(calc_length("^4", 96, 48), 48 + 96);
    }
    #[test]
    fn test_calc_len3() {
        assert_eq!(calc_length("2", 48, 48), 96);
        assert_eq!(calc_length("4^4", 48, 48), 96);
        assert_eq!(calc_length("4.^8", 48, 48), 96);
        assert_eq!(calc_length("8^4.", 48, 48), 96);
    }
    #[test]
    fn test_calc_len_step() {
        assert_eq!(calc_length("%96", 96, 96), 96);
        assert_eq!(calc_length("4^%1", 96, 96), 97);
        assert_eq!(calc_length("^%2", 96, 96), 98);
        assert_eq!(calc_length("^%-1", 96, 48), 47);
    }
    #[test]
    fn test_calc_len_plus() {
        assert_eq!(calc_length("4+4", 96, 96), 96 * 2);
        assert_eq!(calc_length("8+", 96, 48), 96);
    }
    #[test]
    fn test_calc_len_dot() {
        assert_eq!(calc_length(".", 96, 96), 96 + 48);
    }
}
