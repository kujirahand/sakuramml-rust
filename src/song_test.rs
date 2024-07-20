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

    fn test_mml_log(mml: &str) -> String {
        let song = test_mml(mml);
        let s = song.get_logs_str();
        s.replace("[PRINT](0)", "").trim().to_string()
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
        // TrackKey
        assert_eq!(test_mml_event1("TrackKey=1;o4c").v1, 49);
        assert_eq!(test_mml_event1("TrackKey=1;Key=1;o4c").v1, 50);
        assert_eq!(test_mml_event1("TrackKey=1;Key=1;KeyFlag+(c);o4c").v1, 51);
        // UseKeyShift
        assert_eq!(test_mml_event1("UseKeyShift(on); KeyFlag+(c); o4c").v1, 49);
        assert_eq!(test_mml_event1("UseKeyShift(off); KeyFlag+(c); o4c").v1, 48);
    }
    #[test]
    fn single_char_macro_test() {
        assert_eq!(test_mml_event1("~{x}={o4c};x").v1, 48);
        assert_eq!(test_mml_event1("~{h}={o4c};h").v1, 48);
        assert_eq!(test_mml_event1("~{x}={o4d};x").v1, 50);
    }
    #[test]
    fn str_macro_test() {
        // string macro normal
        assert_eq!(test_mml_event1("#A={o4c};#A").v1, 48);
        assert_eq!(test_mml_event1("#B={o4c};#B").v1, 48);
        assert_eq!(test_mml_event1("#AA={o4c};#AA").v1, 48);
        assert_eq!(test_mml_event1("#AAA={o4c};#AAA").v1, 48);
        assert_eq!(test_mml_event1("#ABC={o4c};#ABC").v1, 48);
        // string macro replace
        assert_eq!(test_mml_event1("#A={o#?1c};#A(4)").v1, 48);
        assert_eq!(test_mml_event1("#A={o4#?1};#A{c}").v1, 48);
        assert_eq!(test_mml_event1("#A={o#?1 #?2};#A(4,{c})").v1, 48);
    }
    #[test]
    fn str_var_test() {
        // normal
        assert_eq!(test_mml_event1("STR A={o4c} A").v1, 48);
        assert_eq!(test_mml_event1("STR AA={o4c} AA").v1, 48);
        assert_eq!(test_mml_event1("STR AAA={o4c} AAA").v1, 48);
        // plus
        assert_eq!(&test_mml_log("STR A={c};STR B={d};PRINT(A+B)"), "cd");
        assert_eq!(&test_mml_log("STR A={c};STR B={d};STR C=A+B; PRINT(C)"), "cd");
        assert_eq!(&test_mml_log("STR A={c};INT B=4; STR C=A+B; PRINT(C)"), "c4");
    }
    #[test]
    fn calc_test() {
        assert_eq!(&test_mml_log("INT A=3;INT B=4; INT C=A+B; PRINT(C)"), "7");
        assert_eq!(&test_mml_log("INT A=9;INT B=4; INT C=A-B; PRINT(C)"), "5");
        assert_eq!(&test_mml_log("INT A=3;INT B=4; INT C=A*B; PRINT(C)"), "12");
        assert_eq!(&test_mml_log("INT A=30;INT B=3; INT C=A/B; PRINT(C)"), "10");
    }
    #[test]
    fn func_test() {
        assert_eq!(&test_mml_log("FUNCTION ADD(INT A, INT B){ PRINT(A+B) }; ADD(3,5)"), "8");
        assert_eq!(&test_mml_log("FUNCTION ADD(INT A, INT B){ Result=(A+B) }; PRINT(ADD(3,5))"), "8"); // Pascal Like Function
        assert_eq!(&test_mml_log("FUNCTION ADD(INT A, INT B){ RETURN(A+B) }; PRINT(ADD(3,5))"), "8");
        // TODO: 引数の省略 (#37)
        // assert_eq!(&test_mml_log("FUNCTION ADD(INT A, INT B=0){ PRINT(A+B) }; ADD(3)"), "3"); // 値の省略
    }
}
