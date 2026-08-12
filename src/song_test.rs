//! test file
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

    #[test]
    fn normalize_adds_note_off_before_the_next_note_at_the_same_time() {
        let mut track = Track::new(96, 0);
        track.events.push(Event::note(0, 0, 60, 96, 100));
        track.events.push(Event::note(96, 0, 62, 96, 100));

        track.normalize();
        track.events_sort();

        assert_eq!(track.events.len(), 4);
        assert_eq!(track.events[1].time, 96);
        assert_eq!(track.events[1].etype, EventType::NoteOff);
        assert_eq!(track.events[2].time, 96);
        assert_eq!(track.events[2].etype, EventType::NoteOn);
    }

    #[test]
    fn play_from_restores_voice_and_control_change() {
        let mut track = Track::new(96, 2);
        track.events.push(Event::voice(0, 2, 10));
        track.events.push(Event::cc(24, 2, 7, 80));
        track.events.push(Event::note(40, 2, 60, 24, 100));
        track.events.push(Event::note(72, 2, 62, 24, 100));

        track.play_from(48);

        assert!(track.events.iter().any(|e|
            e.etype == EventType::Voice && e.time == 0 && e.channel == 2 && e.v1 == 10));
        assert!(track.events.iter().any(|e|
            e.etype == EventType::ControllChange && e.time == 0 && e.v1 == 7 && e.v2 == 80));
        let notes: Vec<_> = track.events.iter()
            .filter(|e| e.etype == EventType::NoteOn)
            .collect();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].time, 24);
        assert_eq!(notes[0].v1, 62);
    }

    #[test]
    fn on_note_values_stop_or_cycle_as_configured() {
        let mut track = Track::new(96, 0);
        track.v_on_note = Some(vec![10, 20]);
        assert_eq!(track.calc_v_on_note(99), 10);
        assert_eq!(track.calc_v_on_note(99), 20);
        assert_eq!(track.calc_v_on_note(99), 99);

        track.v_on_note = Some(vec![30, 40]);
        track.v_on_note_is_cycle = true;
        assert_eq!(track.calc_v_on_note(99), 30);
        assert_eq!(track.calc_v_on_note(99), 40);
        assert_eq!(track.calc_v_on_note(99), 30);
    }
}
