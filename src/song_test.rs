//! test file

#[cfg(test)]
mod calc_length_tests {
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
}

#[cfg(test)]
mod note_tests {
    use crate::song::*;
    use crate::lexer::lex;
    use crate::runner::exec;

    fn test_mml(mml: &str) -> Song {
        let mut song = Song::new();
        let tokens = lex(&mut song, mml, 0);
        exec(&mut song, &tokens);
        song
    }

    fn test_mml_event1(mml: &str) -> Event {
        let song = test_mml(mml);
        song.tracks[0].events[0].clone()
    }

    #[test]
    fn note_base_test() {
        // note
        assert_eq!(test_mml_event1("o4c").v1, 48);
        // sharp
        assert_eq!(test_mml_event1("o4c+").v1, 49);
        assert_eq!(test_mml_event1("o4c++").v1, 50);
        assert_eq!(test_mml_event1("o4c+++").v1, 51);
        // flat
        assert_eq!(test_mml_event1("o4c-").v1, 47);
        assert_eq!(test_mml_event1("o4c--").v1, 46);
        assert_eq!(test_mml_event1("o4c---").v1, 45);
        // n command
        assert_eq!(test_mml_event1("n48").v1, 48);
    }
    #[test]
    fn note_key_flag_test() {
        // Key
        assert_eq!(test_mml_event1("Key=0;o4c").v1, 48);
        assert_eq!(test_mml_event1("Key=1;o4c").v1, 49);
        assert_eq!(test_mml_event1("Key=2;o4c").v1, 50);
        // KeyShift
        assert_eq!(test_mml_event1("KeyShift=2;o4c").v1, 50);
        // KeyFlag
        assert_eq!(test_mml_event1("KeyFlag+(c);o4c").v1, 49);
        assert_eq!(test_mml_event1("Key=1;KeyFlag+(c);o4c").v1, 50);
        assert_eq!(test_mml_event1("KeyFlag-(c);o4c").v1, 47);
        // UseKeyShift
        assert_eq!(test_mml_event1("UseKeyShift(on); KeyFlag+(c); o4c").v1, 49);
        assert_eq!(test_mml_event1("UseKeyShift(off); KeyFlag+(c); o4c").v1, 48);
    }
}
