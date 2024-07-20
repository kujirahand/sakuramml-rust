//! test file

#[cfg(test)]
mod tests {
    use crate::runner::calc_length;
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
        assert_eq!(calc_length("^", timebase, t1), t1*2);
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
        assert_eq!(calc_length("4.", timebase, t1), t4 + t8);
        assert_eq!(calc_length("4..", timebase, t4), t4 + t8 + t16); // 複付点音符
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
    }
}
