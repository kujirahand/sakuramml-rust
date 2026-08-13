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
            "v.onCycle(70,80) v__1.onCycle(10,20) cde v__1.onNote(-10,-20) fga",
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
        let song = exec_easy("v70 v__0(10) c v__0.onCycle(20,-20) def");
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
        let song = exec_easy("v70 v_.onCycle(80,60) cd");
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
}
