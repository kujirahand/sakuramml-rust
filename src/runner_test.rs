//! Tests for runner module
use crate::runner::*;
use crate::lexer::lex;
use crate::song::Song;

/// Helper macro for accessing current track
macro_rules! trk {
    ($song:expr) => {
        $song.tracks[$song.cur_track]
    };
}

/// Helper function for testing
fn exec_easy(src: &str) -> Song {
    let mut song = Song::new();
    let t = &lex(&mut song, src, 0);
    exec(&mut song, &t);
    song
}

#[cfg(test)]
mod test_for_runner {

    use super::exec_easy;
    use crate::song::EventType;
    #[test]
    fn test_exec1() {
        assert_eq!(exec_easy("PRINT({1})").get_logs_str(), "[PRINT](0) 1");
        assert_eq!(exec_easy("PRINT({abc})").get_logs_str(), "[PRINT](0) abc");
        assert_eq!(
            exec_easy("STR A={ddd} PRINT(A)").get_logs_str(),
            "[PRINT](0) ddd"
        );
    }
    #[test]
    fn test_def_var() {
        // define variable
        let song = exec_easy("INT N=333;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 333");
        // define variable
        let song = exec_easy("INT N; N=333; PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 333");
    }
    #[test]
    fn test_exec_harmony() {
        let song = exec_easy("q100 l8 'dg'^^^");
        let e = &song.tracks[0].events[0];
        assert_eq!(e.etype, EventType::NoteOn);
        assert_eq!(e.v2, 96 * 2);
    }
    #[test]
    fn test_timing_random_keeps_following_notes() {
        let song = exec_easy("t.Random(3) cde");
        let note_count = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .count();
        assert_eq!(note_count, 3);
    }
    #[test]
    fn test_exec_track_sync() {
        //
        let song = exec_easy("TR=1 l4 cdef TR=2 c TrackSync;");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96);
        //
        let song = exec_easy("TR=0 l4 c TR=2 cdef TrackSync;");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96 * 4);
    }
    #[test]
    fn test_exec_mes_shift() {
        //
        let song = exec_easy("System.MeasureShift=1;TR=0 TIME(1:1:0)");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96 * 4);
    }
    #[test]
    fn test_lex_macro_str() {
        //
        let song = exec_easy("#A={o#?1} #A(0) c");
        assert_eq!(song.tracks[0].events[0].v1, 0);
        //
        let song = exec_easy("STR AAA={o#?1} AAA(0) d");
        assert_eq!(song.tracks[0].events[0].v1, 2);
        //
        let song = exec_easy("STR BBB={o0 #?1 #?2 #?3} BBB({c},{d},{e})");
        assert_eq!(song.tracks[0].events[0].v1, 0);
        assert_eq!(song.tracks[0].events[1].v1, 2);
        assert_eq!(song.tracks[0].events[2].v1, 4);
    }
    #[test]
    fn test_exec_for() {
        let song = exec_easy("INT N=0;FOR(I=1;I<=10;I++){N=N+I;} PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 55");
        // break
        let song = exec_easy("INT N=0;FOR(I=1;I<=10;I++){IF(I==3){BREAK} N=N+I;} PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 3");
        // continue
        let song = exec_easy("INT N=0;FOR(I=1;I<=10;I++){IF(I>=3){CONTINUE} N=N+I;} PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 3");
    }
    #[test]
    fn test_exec_while() {
        let song = exec_easy("INT N=0;INT I=1;WHILE(I<=10){N=N+I;I++;} PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 55");
        // break
        let song = exec_easy("INT N=0;INT I=1;WHILE(I<=10){IF(I=3){BREAK}N=N+I;I++;} PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 3");
    }
    #[test]
    fn test_exec_calc() {
        // 1+2*3
        let song = exec_easy("INT N=1+2*3;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 7");
        // (1+2)*3
        let song = exec_easy("INT N=(1+2)*3;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 9");
        // 1>2 false(0)
        let song = exec_easy("PRINT(1>2)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) FALSE");
        // 6/3
        let song = exec_easy("INT N=6/3;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 2");
        // 4/0
        let song = exec_easy("INT N=4/0;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 0");
    }
    #[test]
    fn test_exec_function() {
        // simple call
        let song = exec_easy(&format!("{}\n{}\n{}\n{}\n{}",
            "FUNCTION FOO(A,B){",
            "  INT C=A+B;",
            "  PRINT(C);",
            "}",
            "FOO(3,5)"
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](2) 8");
        // with return
        let song = exec_easy(&format!("{}\n{}\n{}\n{}\n",
            "FUNCTION FOO(A,B){",
            "  RETURN(A+B);",
            "}",
            "PRINT(FOO(3,8));"
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](3) 11");
        // use global variable
        let song = exec_easy(&format!("{}\n{}\n{}\n{}\n{}\n{}\n",
            "INT C=100",
            "FUNCTION FOO(TMP){",
            "  INT C=TMP;",
            "  PRINT(C);",
            "}",
            "FOO(1); PRINT(C);"
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](3) 1\n[PRINT](5) 100");
        // use global variable
        let song = exec_easy(&format!("{}\n{}\n{}\n{}\n",
            "INT C=123",
            "FUNCTION FOO(TMP){ INT C=TMP; Result=TMP; }",
            "FUNCTION BAA(TMP){ INT C=TMP; RETURN(C);  }",
            "PRINT(FOO(100)); PRINT(BAA(200)); PRINT(C);",
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](3) 100\n[PRINT](3) 200\n[PRINT](3) 123");
        // use global variable and return into for-loop
        let song = exec_easy(&format!("{}\n{}\n{}\n{}\n{}\n",
            "PRINT(FOO());",
            "FUNCTION FOO(){",
            "  INT C=0; FOR(INT I=0; I<=3; I++){ IF(I==2){ RETURN(C); } ELSE { C=I; } }",
            "  RETURN(100);",
            "}",
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](0) 1");
    }

    #[test]
    fn test_exec_function_issues71() {
        // First test individual calls
        let song = exec_easy(&format!("{}\n{}\n{}\n",
            "FUNCTION FOO(STR TMP){ Result=1; }",
            "FUNCTION BAA(STR TMP){ Result=0; }",
            "PRINT(FOO({0}));",
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](2) 1");
        
        let song = exec_easy(&format!("{}\n{}\n{}\n",
            "FUNCTION FOO(STR TMP){ Result=1; }",
            "FUNCTION BAA(STR TMP){ Result=0; }",
            "PRINT(BAA({A}));",
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](2) 0");
        
        // Now test multiple calls on same line  
        let song = exec_easy(&format!("{}\n{}\n{}\n",
            "FUNCTION FOO(STR TMP){ Result=1; }",
            "FUNCTION BAA(STR TMP){ Result=0; }",
            "PRINT(FOO({0})); PRINT(BAA({A})); PRINT(BAA({a}));",
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](2) 1\n[PRINT](2) 0\n[PRINT](2) 0");
    }
    #[test]
    fn test_exec_sys_func_mid() {
        // mid
        let song = exec_easy("STR A={abcd};PRINT(MID(A,1,2))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) ab");
        // 文字位置はUTF-8のバイト位置ではなく文字単位で扱う
        let song = exec_easy("STR A={あいうえ};PRINT(MID(A,2,2))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) いう");
        // 範囲外や負数を指定してもパニックしない
        let song = exec_easy("STR A={abc};PRINT(MID(A,99,2));PRINT(MID(A,-1,2))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) \n[PRINT](0) ab");
    }
    #[test]
    fn test_exec_sys_func_replace() {
        // mid
        let song = exec_easy("STR A={abcd};PRINT(REPLACE(A,{ab},{rr}))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) rrcd");
    }
    #[test]
    fn test_lex_macro_extract() {
        let song = exec_easy("STR A={c} PRINT(A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) c");
        let song = exec_easy("#A={c} PRINT(#A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) c");
        // let song = exec_easy("STR A={#?1} A{e}");
        // assert_eq!(song.get_logs_str(), "[PRINT](0) c");
    }
    #[test]
    fn test_array() {
        let song = exec_easy("ARRAY A=(1,2,3) PRINT(A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) (1,2,3)");
        // SizeOf
        let song = exec_easy("ARRAY A=(1,2,3) PRINT(SizeOf(A))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 3");
        // combine
        let song = exec_easy("ARRAY A=(1,1);ARRAY B=(2,2);ARRAY C=(3,3);PRINT((A,B,C))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) ((1,1),(2,2),(3,3))");
        let song = exec_easy("ARRAY A=(1,);ARRAY B=(2,);ARRAY C=(3,);PRINT((A,B,C))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) ((1),(2),(3))");
    }
    #[test]
    fn test_lex_neg_number() {
        let song = exec_easy("PRINT(-1)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) -1");
        let song = exec_easy("PRINT(-50)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) -50");
        let song = exec_easy("INT A=30; PRINT(-A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) -30");
    }
    #[test]
    fn extract_function_args() { // 関数の引数で与えた文字列を関数の中で展開できない #27
        let song = exec_easy("Function EXT_MML(STR AA){ AA }; EXT_MML{ l4cdeg }");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, song.timebase * 4);
        //
        let song = exec_easy("Function EXT_MML(STR AA){ AA }; EXT_MML{ l8 [8c] }");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, song.timebase * 4);
    }
    #[test]
    fn func_def_value() { // 関数の引数に省略値が指定できないでエラーになる #37
        let song = exec_easy("Function EXT_MML(STR AA={l4cdef}){ AA }; EXT_MML");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, song.timebase * 4);
        //
        let song = exec_easy("Function EXT_MML(STR AA={cdef}){ PRINT(AA) }; EXT_MML ");
        assert_eq!(song.get_logs_str(), "[PRINT](0) cdef");
        //
        let song = exec_easy("Function DEF_TEST(AA=1){ PRINT(AA) }; DEF_TEST ");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 1");
    }
    #[test]
    fn test_read_value_hex() { // v1互換の16進数を読めない問題 #48
        let song = exec_easy("INT A=$10; PRINT(A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 16");
        let song = exec_easy("INT A=0x10; PRINT(A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 16");
    }
    #[test]
    fn test_loop() {
        // loop simple
        let song = exec_easy("[4 c4]");
        assert_eq!(trk!(song).timepos, song.timebase * 4);
        // loop break
        let song = exec_easy("[4 c4 : c4] c4");
        assert_eq!(trk!(song).timepos, song.timebase * 8);
        // loop nested
        let song = exec_easy("[4 [2 c4] ]");
        assert_eq!(trk!(song).timepos, song.timebase * 8);
        // loop nested with break
        let song = exec_easy("[4 [2 c4 : c4] ]");
        assert_eq!(trk!(song).timepos, song.timebase * 12);
    }
    #[test]
    fn test_read_system_value() {
        // timebase test
        let song = exec_easy("TIMEBASE(96); c4; PRINT(TIME)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 96");
        let song = exec_easy("TIMEBASE(48); c4; PRINT(TIME)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 48");
        // v
        let song = exec_easy("v120 c4; PRINT(v)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 120");
        // o
        let song = exec_easy("o6 c4; PRINT(o)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 6");
    }
    #[test]
    fn test_mml_returns_current_track_and_song_values() {
        let song = exec_easy(
            "l4 v100 o4 q80 t-3 @25 BR(12) PitchBend(123) Key(2) Port(3)\
             PRINT(MML(l)) PRINT(MML(v)) PRINT(MML(o)) PRINT(MML(q))\
             PRINT(MML(t)) PRINT(MML(@)) PRINT(MML(BR)) PRINT(MML(p%))\
             PRINT(MML(Key)) PRINT(MML(TimeKey)) PRINT(MML(Port))",
        );
        assert_eq!(
            song.get_logs_str(),
            "[PRINT](0) 96\n[PRINT](0) 100\n[PRINT](0) 4\n[PRINT](0) 80\n\
             [PRINT](0) -3\n[PRINT](0) 25\n[PRINT](0) 12\n[PRINT](0) 123\n\
             [PRINT](0) 2\n[PRINT](0) 0\n[PRINT](0) 3"
        );
    }
    #[test]
    fn test_mml_keeps_string_and_variable_arguments_compatible() {
        let song = exec_easy("o6 STR Command={o} PRINT(MML({o})) PRINT(MML(Command))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 6\n[PRINT](0) 6");
    }
    #[test]
    fn test_add_len() {
        // test basic
        let song = exec_easy("l4 c");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96 * 1);
        // test space
        let song = exec_easy("l4 c ^");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96 * 2);
        // test tab
        let song = exec_easy("l4 c \t ^^^");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96 * 4);
    }
    #[test]
    fn test_read_length() { // 改行後の音長を有効にする #60
        let song = exec_easy("l8 c^\n^^");
        assert_eq!(song.tracks[0].timepos, song.timebase * 2);
        let song = exec_easy("l8 c^\n^4");
        assert_eq!(song.tracks[0].timepos, song.timebase * 2);
    }
    #[test]
    fn test_calc_and_or() {
        /*
        let song = exec_easy("PRINT(TRUE&TRUE)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) TRUE");
        //
        let song = exec_easy("PRINT(TRUE&FALSE)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) FALSE");
        //
        let song = exec_easy("PRINT(TRUE&FALSE&TRUE)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) FALSE");
        //
        let song = exec_easy("PRINT(TRUE&TRUE&TRUE)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) TRUE");
        //
        */
        let song = exec_easy("PRINT( (1=1)&TRUE )");
        assert_eq!(song.get_logs_str(), "[PRINT](0) TRUE");
    }
}
// ------------------------------------------

#[cfg(test)]
mod test_review_94 {
    use super::exec_easy;
    use crate::song::EventType;

    #[test]
    fn test_undefined_function_in_calc_keeps_name() {
        // 計算式から未定義の関数を呼ぶと func_name に '=' が付かない。
        // 先頭1文字を無条件に落とすと、別名の変数を参照してしまう。
        // (修正前は FOO -> OO と解釈され 999 が返っていた)
        let song = exec_easy("INT OO=999; PRINT(FOO(1))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) ");
    }

    #[test]
    fn test_tempo_change_zero_writes_valid_mpq() {
        // TempoChange は Tempo と違い範囲チェックがないため 0 が渡り得る。
        // 0以下でも MIDI のテンポ(μsec/四分音符)として妥当な値を書くこと。
        let song = exec_easy("TempoChange(0) c");
        let e = song.tracks[0].events.iter()
            .find(|e| e.etype == EventType::Meta && e.v2 == 0x51)
            .expect("テンポのメタイベントがない");
        // 既定の120BPM => 500000 usec => 0x07A120
        assert_eq!(e.data, Some(vec![0x07u8, 0xA1, 0x20]));
    }
}

#[cfg(test)]
mod test_issue_102 {
    use super::exec_easy;
    use crate::song::EventType;

    #[test]
    fn test_sub_velocity_does_not_hang() {
        let song = exec_easy("v__1(100) cde");

        assert_eq!(song.tracks[0].v_sub, vec![0, 100]);
        let notes = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .collect::<Vec<_>>();
        assert_eq!(notes.len(), 3);
        assert!(notes.iter().all(|event| event.v3 == 127));
    }

    #[test]
    fn test_sub_velocity_layers_are_added_to_notes() {
        let song = exec_easy("v70 v__1(-10) v__3(20) c n(60) v__1(0) d");
        let velocities = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v3)
            .collect::<Vec<_>>();

        assert_eq!(song.tracks[0].v_sub, vec![0, 0, 0, 20]);
        assert_eq!(velocities, vec![80, 80, 90]);
    }

    #[test]
    fn test_sub_velocity_on_note_and_cycle() {
        let song = exec_easy(
            "v.onCycle(!4,70,80) v__1.onCycle(!4,10,20) cde v__1.onNote(-10,-20) fga",
        );
        let velocities = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v3)
            .collect::<Vec<_>>();

        assert_eq!(velocities, vec![80, 100, 80, 70, 50, 60]);
    }

    #[test]
    fn test_sub_velocity_on_time_and_scalar_reset() {
        let song = exec_easy("TimeBase=96 v70 v__1.onTime(0,20,!4) l8 ccc v__1(5) de");
        let velocities = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v3)
            .collect::<Vec<_>>();

        assert_eq!(velocities, vec![70, 80, 70, 75, 75]);
    }

    #[test]
    fn test_sub_velocity_random_and_reset() {
        let song = exec_easy("v70 v__1.Random(20) [8 c] v__1.Random(0) d");
        let velocities = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v3)
            .collect::<Vec<_>>();

        assert_eq!(velocities.len(), 9);
        assert!(velocities[..8]
            .iter()
            .all(|velocity| 60 <= *velocity && *velocity <= 79));
        assert_eq!(velocities[8], 70);
        assert_eq!(song.tracks[0].v_sub_rand, vec![0, 0]);
    }

    #[test]
    fn test_sub_velocity_layer_zero_is_distinct_from_base_velocity() {
        let song = exec_easy("v70 v__0(10) c v__0.onCycle(!4,20,-20) def");
        let velocities = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v3)
            .collect::<Vec<_>>();

        assert_eq!(song.tracks[0].velocity, 70);
        assert_eq!(velocities, vec![80, 90, 50, 90]);
    }

    #[test]
    fn test_single_underscore_still_targets_base_velocity() {
        let song = exec_easy("v70 v_.onCycle(!4,80,60) cd");
        let velocities = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v3)
            .collect::<Vec<_>>();

        assert_eq!(velocities, vec![80, 60]);
    }

    #[test]
    fn test_sub_velocity_on_time_ignores_non_positive_lengths() {
        let song = exec_easy("v70 v__1.onTime(10,20,0,20,30,-1) cd");
        let velocities = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v3)
            .collect::<Vec<_>>();

        assert_eq!(velocities, vec![70, 70]);
    }
}

#[cfg(test)]
mod test_issue_74 {
    use super::exec_easy;
    use crate::song::EventType;

    #[test]
    fn test_cc_on_time_sections_are_written_sequentially() {
        let song = exec_easy("TimeBase=96 M.T(0,120,8,120,0,8)");
        let events = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::ControllChange && event.v1 == 1)
            .map(|event| (event.time, event.v2))
            .collect::<Vec<_>>();

        assert_eq!(events, vec![(0, 0), (4, 60), (8, 120), (12, 60)]);
    }

    #[test]
    fn test_cc_on_time_ignores_non_positive_lengths_without_shifting_following_section() {
        let song = exec_easy("TimeBase=96 M.T(90,100,0,100,110,-4,10,20,8)");
        let events = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::ControllChange && event.v1 == 1)
            .map(|event| (event.time, event.v2))
            .collect::<Vec<_>>();

        assert_eq!(events, vec![(0, 10), (4, 15)]);
    }

    #[test]
    fn test_pitch_bend_on_time_sections_are_written_sequentially() {
        let song = exec_easy("TimeBase=96 PitchBend.T(-8192,0,6,0,8191,6)");
        let times = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::PitchBend)
            .map(|event| event.time)
            .collect::<Vec<_>>();

        assert_eq!(times, vec![0, 3, 6, 9]);
    }

    #[test]
    fn test_pitch_bend_on_time_ignores_non_positive_lengths_without_shifting_following_section() {
        let song = exec_easy("TimeBase=96 PitchBend.T(-8192,0,0,0,8191,-6,100,200,6)");
        let events = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::PitchBend)
            .map(|event| (event.time, event.v1))
            .collect::<Vec<_>>();

        assert_eq!(events, vec![(0, 8292), (3, 8342)]);
    }
}

#[cfg(test)]
mod test_issue_78 {
    use super::exec_easy;
    use crate::song::EventType;

    /// 指定したCC番号のイベント数を数える
    fn count_cc(song: &crate::song::Song, no: isize) -> usize {
        song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::ControllChange && event.v1 == no)
            .count()
    }

    /// ピッチベンドのイベント数を数える
    fn count_pitch_bend(song: &crate::song::Song) -> usize {
        song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::PitchBend)
            .count()
    }

    #[test]
    fn test_cc_on_note_wave_writes_wave_for_each_note() {
        // 音符ごとに .onTime 相当の波形が書き込まれる (#78)
        let song = exec_easy("TimeBase=96 M.onNoteWave(0,127,!4) l4 cd");
        assert_eq!(count_cc(&song, 1), 96 / 4 * 2);
    }

    #[test]
    fn test_cc_on_note_wave_with_tie() {
        // タイ・スラーでつないだ音符でも、先頭の音符で1回書き込まれる (#78)
        let song = exec_easy("TimeBase=96 M.onNoteWave(0,127,!4) l4 c&c d");
        assert_eq!(count_cc(&song, 1), 96 / 4 * 2);
    }

    #[test]
    fn test_cc_on_note_wave_with_harmony() {
        // 和音では音数分ではなく1回だけ書き込まれる (#78)
        let song = exec_easy("TimeBase=96 M.onNoteWave(0,127,!4) l4 'ceg'");
        assert_eq!(count_cc(&song, 1), 96 / 4);
        // 和音の先頭から書き出される
        let first = song.tracks[0]
            .events
            .iter()
            .find(|event| event.etype == EventType::ControllChange)
            .unwrap();
        assert_eq!(first.time, 0);
    }

    #[test]
    fn test_pb_on_note_wave() {
        // PB.onNoteWave は音符ごとにピッチベンドの波形を書き込む (#78)
        let song = exec_easy("TimeBase=96 PB.onNoteWave(-8000,8000,!4) l4 cd");
        assert_eq!(count_pitch_bend(&song), 96 / (96 / 32) * 2);
        let first = song.tracks[0]
            .events
            .iter()
            .find(|event| event.etype == EventType::PitchBend)
            .unwrap();
        assert_eq!(first.time, 0);
        assert_eq!(first.v1, -8000 + 8192);
    }

    #[test]
    fn test_pb_on_note_wave_short_alias_and_reset() {
        // .W の別名で指定でき、PB(値)の単発指定で解除される (#78)
        let song = exec_easy("TimeBase=96 PB.W(-8000,8000,!4) l4 c PB(0) d");
        // 1音符分の波形 + PB(0) の1イベント
        assert_eq!(count_pitch_bend(&song), 96 / (96 / 32) + 1);
    }

    #[test]
    fn test_velocity_on_note_wave_is_reported_as_error() {
        // v.onNoteWave は未対応。無言で v0 にならず、エラーを出す (#78)
        let song = exec_easy("v100 v.onNoteWave(0,127,!4) l4 cd");
        assert!(song.get_logs_str().contains("not supported : v.onNoteWave"));
        let velocities = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v3)
            .collect::<Vec<_>>();
        assert_eq!(velocities, vec![100, 100]);
    }

    #[test]
    fn test_qlen_on_note_wave_is_reported_as_error() {
        // q.onNoteWave も同様に、無言で q0 にならずエラーを出す (#78)
        let song = exec_easy("q90 q.onNoteWave(0,127,!4) l4 c");
        assert!(song.get_logs_str().contains("not supported : q.onNoteWave"));
        assert_eq!(song.tracks[0].qlen, 90);
    }

    #[test]
    fn test_cc_on_time_skips_duplicated_values() {
        // 直前と同じ値は書き込まない (#78)
        let song = exec_easy("TimeBase=96 M.onTime(10,10,!4) l4 c");
        assert_eq!(count_cc(&song, 1), 1);
    }

    #[test]
    fn test_cc_on_note_wave_overwrites_overlapped_range() {
        // 波形が音符より長く重なるとき、古い書き込みは削除され後勝ちになる (#78)
        // 2分音符分の波形を4分音符ごとに書き出すので、1音符目の後半は2音符目に上書きされる
        let song = exec_easy("TimeBase=96 M.onNoteWave(0,127,!2) l4 cd");
        let events = song.tracks[0]
            .events
            .iter()
            .filter(|event| {
                event.etype == EventType::ControllChange && event.v1 == 1
            })
            .collect::<Vec<_>>();
        // 同じ時刻に同じCC番号のイベントが2つ以上ないこと
        for i in 1..events.len() {
            assert_ne!(events[i - 1].time, events[i].time);
        }
        // 2音符目の先頭では、あとから指定した波形の開始値になる
        let at_note2 = events.iter().find(|event| event.time == 96).unwrap();
        assert_eq!(at_note2.v2, 0);
    }

    #[test]
    fn test_pb_on_note_wave_overwrites_overlapped_range() {
        // ピッチベンドでも同様に後勝ちになる (#78)
        let song = exec_easy("TimeBase=96 PB.onNoteWave(-8000,8000,!2) l4 cd");
        let events = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::PitchBend)
            .collect::<Vec<_>>();
        for i in 1..events.len() {
            assert_ne!(events[i - 1].time, events[i].time);
        }
        let at_note2 = events.iter().find(|event| event.time == 96).unwrap();
        assert_eq!(at_note2.v1, -8000 + 8192);
    }

    /// ピッチベンドのイベントを時刻順に取り出す
    fn pitch_bends(song: &crate::song::Song) -> Vec<(isize, isize)> {
        song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::PitchBend)
            .map(|event| (event.time, event.v1))
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_tie_glissando_has_priority_over_pb_on_note_wave() {
        // タイ・スラーと PB.onNoteWave が同時に指定されたら、タイ・スラーを優先する (#78)
        // Slur(0)=グリッサンド。タイでつないだ範囲(0〜192)の波形は書き込まれない
        let song = exec_easy("TimeBase=96 BR(2) Slur(0) PB.onNoteWave(-2000,0,!4) l4 c&d e");
        let bends = pitch_bends(&song);
        // グリッサンドは 96-48=48 から始まる
        // 波形が残っていれば音符の先頭(0)から書き込まれているはずなので、それが無いことを確認する
        assert!(bends.iter().all(|(time, _)| *time >= 48));
        // タイに関係しない次の音符では、波形が書き込まれる
        let at_note3 = bends.iter().find(|(time, _)| *time == 96 * 2).unwrap();
        assert_eq!(at_note3.1, -2000 + 8192);
    }

    #[test]
    fn test_tie_bend_mode_has_priority_over_pb_on_note_wave() {
        // Slur(1)=ベンドモードでは、タイの範囲すべてをタイ側のベンドが占める (#78)
        let song = exec_easy("TimeBase=96 BR(4) Slur(1) PB.onNoteWave(-2000,0,!4) l4 c&d e");
        let in_tie = pitch_bends(&song)
            .into_iter()
            .filter(|(time, _)| *time < 96 * 2)
            .collect::<Vec<_>>();
        // タイの開始・音程の変化・タイの終了の3つだけ
        assert_eq!(in_tie.len(), 3);
        assert_eq!(in_tie[0].1, 8192); // タイの開始でベンドを0に戻す
        assert_eq!(in_tie[1].1, 8192 + 8192 * 2 / 4); // 2半音ぶん上げる
    }

    #[test]
    fn test_pb_on_note_wave_is_kept_when_tie_writes_no_bend() {
        // 同じ音程のタイ(c&c)はベンドを書き込まないので、波形はそのまま残る (#78)
        let song = exec_easy("TimeBase=96 BR(2) Slur(0) PB.onNoteWave(-2000,0,!4) l4 c&c e");
        let bends = pitch_bends(&song);
        assert_eq!(bends[0], (0, -2000 + 8192));
        // タイの音符と次の音符で、2回分の波形が書き込まれる
        assert_eq!(bends.len(), 32 * 2);
    }

    #[test]
    fn test_slur_second_arg_accepts_note_length_notation() {
        // Slur(type, value) の value に音長記法(!8)を書けること (#112)
        // 以前は計算式パーサが "!" を比較演算子として扱っていたため解釈に失敗し、
        // 残った文字が後続のコマンドとして読まれてベロシティが化けていた
        let song = exec_easy("TimeBase=96 BR(2) Slur(0,!8) l4 c&d");
        assert_eq!(song.get_logs_str(), "");
        let notes = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .collect::<Vec<_>>();
        // ベロシティが既定値(100)のまま変化しないこと
        assert_eq!(notes[0].v3, 100);
        // !8 が48ステップとして解釈され、数値で 48 と書いた場合と同じ結果になること
        let bends = pitch_bends(&song);
        let expected = pitch_bends(&exec_easy("TimeBase=96 BR(2) Slur(0,48) l4 c&d"));
        assert_eq!(bends, expected);
        // グリッサンドは音符の48ステップ手前(96-48=48)から始まる。
        // ただし開始時点のベンド値は0で直前の値と同じため出力されず、
        // 最初に書き込まれるイベントは49ステップ目になる
        assert_eq!(bends.first().unwrap().0, 49);
    }
}

/// 先行指定(mml_const.pas の OPTION_*)のテスト (#65)
mod test_issue_65 {
    use super::exec_easy;
    use crate::song::{EventType, Song};

    /// 指定したCC番号の (時間, 値) を取り出す
    fn cc_events(song: &Song, no: isize) -> Vec<(isize, isize)> {
        song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::ControllChange && e.v1 == no)
            .map(|e| (e.time, e.v2))
            .collect()
    }

    /// ピッチベンドの (時間, 値) を取り出す
    fn pitch_bends(song: &Song) -> Vec<(isize, isize)> {
        song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::PitchBend)
            .map(|e| (e.time, e.v1))
            .collect()
    }

    /// ノートオンのベロシティを取り出す
    fn velocities(song: &Song) -> Vec<isize> {
        song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::NoteOn)
            .map(|e| e.v3)
            .collect()
    }

    /// ノートオンの (時間, 長さ) を取り出す
    fn note_times(song: &Song) -> Vec<(isize, isize)> {
        song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::NoteOn)
            .map(|e| (e.time, e.v2))
            .collect()
    }

    #[test]
    fn test_cc_unknown_option_is_error() {
        // 未対応の指定を、無言で `M(10)` に化けさせないこと
        let song = exec_easy("M.Foo(10) c");
        assert_eq!(cc_events(&song, 1).len(), 0);
        assert!(song.get_logs_str().contains("not supported : CC(1).Foo"));
    }

    #[test]
    fn test_pitch_bend_unknown_option_is_error() {
        let song = exec_easy("PB.Foo(10) c");
        assert_eq!(pitch_bends(&song).len(), 0);
        assert!(song.get_logs_str().contains("not supported : PB.Foo"));
    }

    #[test]
    fn test_cc_on_cycle_writes_values_by_step() {
        // .onCycle(ステップ値, 値1, 値2, ...) --- 解除するまでくり返す
        let song = exec_easy("TimeBase=96 M.onCycle(!4,0,127) l8 [8 c]");
        assert_eq!(
            cc_events(&song, 1),
            vec![(0, 0), (96, 127), (192, 0), (288, 127)]
        );
    }

    #[test]
    fn test_note_param_on_cycle_is_time_based() {
        // t.onCycle(!16,0,8) は t.onTime(0,0,!16, 8,8,!16) と同じ意味
        let song = exec_easy("TimeBase=96 t.onCycle(!8,0,8) l8 cdcd");
        let times: Vec<isize> = note_times(&song).iter().map(|(t, _)| *t).collect();
        assert_eq!(times, vec![0, 56, 96, 152]);
    }

    #[test]
    fn test_velocity_on_cycle_is_time_based() {
        let song = exec_easy("TimeBase=96 v.onCycle(!8,50,120) l8 cdcd");
        assert_eq!(velocities(&song), vec![50, 120, 50, 120]);
    }

    #[test]
    fn test_qlen_on_time() {
        // q.onTime は以前は未対応だった
        let song = exec_easy("TimeBase=96 q.onTime(50,100,!1) l4 cdef");
        let qlens: Vec<isize> = note_times(&song).iter().map(|(_, len)| *len).collect();
        assert_eq!(qlens, vec![48, 59, 72, 83]);
    }

    #[test]
    fn test_octave_on_time() {
        let song = exec_easy("TimeBase=96 o.onTime(4,6,!1) l4 cccc");
        let notes: Vec<isize> = song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::NoteOn)
            .map(|e| e.v1)
            .collect();
        assert_eq!(notes, vec![48, 48, 60, 60]);
    }

    #[test]
    fn test_length_random_is_supported() {
        // l.Random は以前は未対応だった
        let song = exec_easy("TimeBase=96 l.Random(8) l4 c");
        assert!(!song.get_logs_str().contains("not supported"));
        assert_eq!(song.tracks[0].l_opt.random, 8);
    }

    #[test]
    fn test_velocity_max_limits_value() {
        // v.Max は先行指定ではなく、値の上限を変える指定
        let song = exec_easy("v.Max(50) v100 c");
        assert_eq!(velocities(&song), vec![50]);
    }

    #[test]
    fn test_qlen_max_limits_value() {
        let song = exec_easy("TimeBase=96 q.Max(50) q100 l4 c");
        let qlens: Vec<isize> = note_times(&song).iter().map(|(_, len)| *len).collect();
        assert_eq!(qlens, vec![48]);
    }

    #[test]
    fn test_note_param_range_limits_value() {
        let song = exec_easy("v.Range(30,60) v100 c v10 d");
        assert_eq!(velocities(&song), vec![60, 30]);
    }

    #[test]
    fn test_cc_range_limits_written_value() {
        let song = exec_easy("TimeBase=96 M.Range(0,50) M.onTime(0,127,!4) c");
        let values: Vec<isize> = cc_events(&song, 1).iter().map(|(_, v)| *v).collect();
        assert!(values.iter().all(|v| *v <= 50));
        assert_eq!(*values.last().unwrap(), 50);
    }

    #[test]
    fn test_cc_delay_shifts_write_position() {
        let song = exec_easy("TimeBase=96 M.Delay(48) M.onTime(0,127,!4) c");
        assert_eq!(cc_events(&song, 1).first().unwrap().0, 48);
    }

    #[test]
    fn test_cc_repeat_makes_on_note_loop() {
        let song = exec_easy("TimeBase=96 M.onNote(1,2) M.Repeat(on) l4 cdef");
        let values: Vec<isize> = cc_events(&song, 1).iter().map(|(_, v)| *v).collect();
        assert_eq!(values, vec![1, 2, 1, 2]);
        // .Repeat(off) なら値を使い切ったところで終わる
        let song = exec_easy("TimeBase=96 M.onNote(1,2) M.Repeat(off) l4 cdef");
        let values: Vec<isize> = cc_events(&song, 1).iter().map(|(_, v)| *v).collect();
        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn test_pitch_bend_on_note() {
        let song = exec_easy("TimeBase=96 PB.onNote(-8192,0,8191) l4 cde");
        assert_eq!(pitch_bends(&song), vec![(0, 0), (96, 8192), (192, 16383)]);
    }

    #[test]
    fn test_cc_on_note_wave_ex_fits_note_length() {
        // .onNoteWaveEx は波形を音符の長さに合わせて伸縮する
        let song = exec_easy("TimeBase=96 M.onNoteWaveEx(0,127,!1) l4 cd");
        let events = cc_events(&song, 1);
        // 1音目(0〜96)と2音目(96〜192)のそれぞれに波形が収まること
        assert!(events.iter().all(|(time, _)| *time < 192));
        assert!(events.iter().any(|(time, _)| *time >= 96));
    }

    #[test]
    fn test_cc_on_note_wave_r_repeats_while_note_is_on() {
        // .onNoteWaveR は音符が鳴っている間くり返す
        let song = exec_easy("TimeBase=96 M.onNoteWaveR(0,120,!8) l2 c");
        let events = cc_events(&song, 1);
        // 2分音符(192)の間に、8分音符(48)の波形が4回くり返される
        let zero_count = events.iter().filter(|(_, v)| *v == 0).count();
        assert_eq!(zero_count, 4);
        assert!(events.iter().all(|(time, _)| *time < 192));
    }

    #[test]
    fn test_cc_sine_writes_wave_once() {
        // .Sine(type,low,high,len,times) --- 0はlow→high→lowの正弦波
        let song = exec_easy("TimeBase=96 CC.Frequency(24) M.Sine(0,0,100,!1,1) c1");
        let events = cc_events(&song, 1);
        assert_eq!(events.first().unwrap(), &(0, 0));
        // 半周した位置(全音符の半分=192)で最大値になる
        assert_eq!(events.iter().find(|(t, _)| *t == 192).unwrap().1, 100);
    }

    #[test]
    fn test_cc_on_note_sine_writes_for_each_note() {
        let song = exec_easy("TimeBase=96 CC.Frequency(24) M.onNoteSine(1,0,100,!4,1) l4 cd");
        let events = cc_events(&song, 1);
        // 音符ごとに書き込まれる
        assert!(events.iter().any(|(t, _)| *t == 0));
        assert!(events.iter().any(|(t, _)| *t == 96));
    }

    #[test]
    fn test_pitch_bend_frequency() {
        // PB.Frequency で書き込み頻度を変えられること
        // 既定は timebase/32 = 3ステップおき
        let song = exec_easy("TimeBase=96 PB.onTime(-8192,8191,!4) c");
        let times: Vec<isize> = pitch_bends(&song).iter().map(|(t, _)| *t).collect();
        assert_eq!(times[0], 0);
        assert_eq!(times[1], 3);
        // 24ステップおきに変更する
        let song = exec_easy("TimeBase=96 PB.Frequency(24) PB.onTime(-8192,8191,!4) c");
        let times: Vec<isize> = pitch_bends(&song).iter().map(|(t, _)| *t).collect();
        assert_eq!(times, vec![0, 24, 48, 72]);
        // p でも同じ設定が使える
        let song = exec_easy("TimeBase=96 p.Frequency(24) p.onTime(0,127,!4) c");
        let times: Vec<isize> = pitch_bends(&song).iter().map(|(t, _)| *t).collect();
        assert_eq!(times, vec![0, 24, 48, 72]);
    }

    #[test]
    fn test_direct_value_clears_reserve() {
        // 値を直接指定すると先行指定は解除される
        let song = exec_easy("TimeBase=96 M.onNote(10,20) l4 c M(0) d");
        let values: Vec<isize> = cc_events(&song, 1).iter().map(|(_, v)| *v).collect();
        assert_eq!(values, vec![10, 0]);
    }
}
