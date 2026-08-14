//! Tests for runner module
use crate::lexer::lex;
use crate::runner::*;
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
    fn test_issue121_string_assignment_runs_in_source_order() {
        let song = exec_easy("A={o5c}; A; A={o5d}; A");
        let notes: Vec<isize> = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v1)
            .collect();
        assert_eq!(notes, vec![60, 62]);

        let song = exec_easy("A={o5c}; IF(FALSE){A={o5d}} A");
        let notes: Vec<isize> = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v1)
            .collect();
        assert_eq!(notes, vec![60]);
    }

    #[test]
    fn test_issue121_mml_parameters_resolve_variables_at_runtime() {
        let song = exec_easy(
            "Int NOTE=60; Int BEND=100; Int OCT=5; Int VEL=90; Int GATE=80; Int TIMING=-3; \
             o(OCT) v(VEL) q(GATE) t(TIMING) n(NOTE) PB(BEND)",
        );
        let note = song.tracks[0]
            .events
            .iter()
            .find(|event| event.etype == EventType::NoteOn)
            .unwrap();
        assert_eq!(note.v1, 60);
        assert_eq!(note.v3, 90);
        assert_eq!(song.tracks[0].octave, 5);
        assert_eq!(song.tracks[0].velocity, 90);
        assert_eq!(song.tracks[0].qlen, 80);
        assert_eq!(song.tracks[0].timing, -3);
        let bend = song.tracks[0]
            .events
            .iter()
            .find(|event| event.etype == EventType::PitchBend)
            .unwrap();
        assert_eq!(bend.v1, 100 + 8192);
    }

    #[test]
    fn test_issue121_function_defaults_and_assignments_use_outer_variables() {
        let song = exec_easy(
            "Int BASE=7; Int A=10; \
             Function F(Int X=BASE){A=A+1; A++; Print(X); Print(A)} \
             F(); Print(A)",
        );
        assert_eq!(
            song.get_logs_str(),
            "[PRINT](0) 7\n[PRINT](0) 12\n[PRINT](0) 12"
        );
    }

    #[test]
    fn test_issue121_random_seed_and_boolean_settings_accept_variables() {
        let by_variable = exec_easy(
            "Int SEED=12345; RandomSeed(SEED); Print(Random(1,100)); \
             Int ENABLE=0; UseKeyShift(ENABLE)",
        );
        let by_literal = exec_easy(
            "RandomSeed(12345); Print(Random(1,100)); \
             UseKeyShift(off)",
        );
        assert_eq!(by_variable.get_logs_str(), by_literal.get_logs_str());
        assert!(!by_variable.use_key_shift);
        assert!(!by_literal.use_key_shift);
    }

    #[test]
    fn test_issue121_undefined_variable_emits_warning() {
        let song = exec_easy("Print(UNDEFINED_VALUE); Int A=UNDEFINED_VALUE+1; Print(A)");
        let logs = song.get_logs_str();
        assert!(logs.contains("Undefined: UNDEFINED_VALUE"), "{logs}");
        assert!(logs.contains("[PRINT](0) 1"), "{logs}");
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
        let song = exec_easy(&format!(
            "{}\n{}\n{}\n{}\n{}",
            "FUNCTION FOO(A,B){", "  INT C=A+B;", "  PRINT(C);", "}", "FOO(3,5)"
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](2) 8");
        // with return
        let song = exec_easy(&format!(
            "{}\n{}\n{}\n{}\n",
            "FUNCTION FOO(A,B){", "  RETURN(A+B);", "}", "PRINT(FOO(3,8));"
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](3) 11");
        // use global variable
        let song = exec_easy(&format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n",
            "INT C=100",
            "FUNCTION FOO(TMP){",
            "  INT C=TMP;",
            "  PRINT(C);",
            "}",
            "FOO(1); PRINT(C);"
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](3) 1\n[PRINT](5) 100");
        // use global variable
        let song = exec_easy(&format!(
            "{}\n{}\n{}\n{}\n",
            "INT C=123",
            "FUNCTION FOO(TMP){ INT C=TMP; Result=TMP; }",
            "FUNCTION BAA(TMP){ INT C=TMP; RETURN(C);  }",
            "PRINT(FOO(100)); PRINT(BAA(200)); PRINT(C);",
        ));
        assert_eq!(
            song.get_logs_str(),
            "[PRINT](3) 100\n[PRINT](3) 200\n[PRINT](3) 123"
        );
        // use global variable and return into for-loop
        let song = exec_easy(&format!(
            "{}\n{}\n{}\n{}\n{}\n",
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
        let song = exec_easy(&format!(
            "{}\n{}\n{}\n",
            "FUNCTION FOO(STR TMP){ Result=1; }",
            "FUNCTION BAA(STR TMP){ Result=0; }",
            "PRINT(FOO({0}));",
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](2) 1");

        let song = exec_easy(&format!(
            "{}\n{}\n{}\n",
            "FUNCTION FOO(STR TMP){ Result=1; }",
            "FUNCTION BAA(STR TMP){ Result=0; }",
            "PRINT(BAA({A}));",
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](2) 0");

        // Now test multiple calls on same line
        let song = exec_easy(&format!(
            "{}\n{}\n{}\n",
            "FUNCTION FOO(STR TMP){ Result=1; }",
            "FUNCTION BAA(STR TMP){ Result=0; }",
            "PRINT(FOO({0})); PRINT(BAA({A})); PRINT(BAA({a}));",
        ));
        assert_eq!(
            song.get_logs_str(),
            "[PRINT](2) 1\n[PRINT](2) 0\n[PRINT](2) 0"
        );
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
    fn extract_function_args() {
        // 関数の引数で与えた文字列を関数の中で展開できない #27
        let song = exec_easy("Function EXT_MML(STR AA){ AA }; EXT_MML{ l4cdeg }");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, song.timebase * 4);
        //
        let song = exec_easy("Function EXT_MML(STR AA){ AA }; EXT_MML{ l8 [8c] }");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, song.timebase * 4);
    }
    #[test]
    fn func_def_value() {
        // 関数の引数に省略値が指定できないでエラーになる #37
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
    fn test_read_value_hex() {
        // v1互換の16進数を読めない問題 #48
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
    fn test_noteno_returns_note_number_of_mml() {
        // オクターブ付きの音符・臨時記号・n命令
        let song = exec_easy("Int N = NoteNo(o5e) PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 64");
        let song = exec_easy("PRINT(NoteNo(o4c)) PRINT(NoteNo(o5e+)) PRINT(NoteNo(o5e-))");
        assert_eq!(
            song.get_logs_str(),
            "[PRINT](0) 48\n[PRINT](0) 65\n[PRINT](0) 63"
        );
        let song = exec_easy("PRINT(NOTENO(n60)) PRINT(NoteNo(o5c,,,,6))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 60\n[PRINT](0) 72");
        // 省略時は現在のトラックのオクターブを使う
        let song = exec_easy("o6 PRINT(NoteNo(c)) PRINT(NoteNo(>c)) PRINT(NoteNo(<c))");
        assert_eq!(
            song.get_logs_str(),
            "[PRINT](0) 72\n[PRINT](0) 84\n[PRINT](0) 60"
        );
        // キーシフトを反映する
        let song = exec_easy("Key(2) PRINT(NoteNo(o5c))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 62");
        // 一度だけオクターブを変える「`」も反映する
        let song = exec_easy("o5 PRINT(NoteNo(`c))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 72");
        // オクターブは実行時と同じく0-10に丸める
        let song = exec_easy("PRINT(NoteNo(o99c)) PRINT(NoteNo(o-5c))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 120\n[PRINT](0) 0");
        let song = exec_easy("PRINT(NoteNo(>>>>>>>>>>>c)) PRINT(NoteNo(<<<<<<<<<<<c))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 120\n[PRINT](0) 0");
        // 計算式の中でも使える / 変数に入れたMMLも使える
        let song = exec_easy("STR MMLA={o5e} PRINT(NoteNo(MMLA)) PRINT(NoteNo(o5c) + 1)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 64\n[PRINT](0) 61");
        // NoteNo内のo/nも、通常のRunnerと同じく変数を実行時に解決する
        let song = exec_easy(
            "Int OCT=5; Int NOTE=61; \
             PRINT(NoteNo({o(OCT)c})) PRINT(NoteNo({n(NOTE)})) \
             PRINT(NoteNo({o(OCT)n(NOTE)}))",
        );
        assert_eq!(
            song.get_logs_str(),
            "[PRINT](0) 60\n[PRINT](0) 61\n[PRINT](0) 61"
        );
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
    fn test_read_length() {
        // 改行後の音長を有効にする #60
        let song = exec_easy("l8 c^\n^^");
        assert_eq!(song.tracks[0].timepos, song.timebase * 2);
        let song = exec_easy("l8 c^\n^4");
        assert_eq!(song.tracks[0].timepos, song.timebase * 2);
    }
    #[test]
    fn test_calc_and_or() {
        let song = exec_easy("PRINT( (1=1) && TRUE )");
        assert_eq!(song.get_logs_str(), "[PRINT](0) TRUE");
        let song = exec_easy("PRINT( (1=1) & TRUE )");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 1");
    }
}
// ------------------------------------------

#[cfg(test)]
mod test_issue_130 {
    use super::exec_easy;

    #[test]
    fn test_bitwise_operators() {
        // Bitwise OR |
        let song = exec_easy("Int A = 1 | 2; PRINT(A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 3");

        // Bitwise AND &
        let song = exec_easy("Int B = 3 & 1; PRINT(B)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 1");

        // Bitwise XOR ^
        let song = exec_easy("Int C = 3 ^ 1; PRINT(C)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 2");

        // Additional bitwise calculations
        let song = exec_easy(
            "Int D = 12 & 10; Int E = 12 | 10; Int F = 12 ^ 10; PRINT(D); PRINT(E); PRINT(F)",
        );
        assert_eq!(
            song.get_logs_str(),
            "[PRINT](0) 8\n[PRINT](0) 14\n[PRINT](0) 6"
        );
    }

    #[test]
    fn test_logical_operators() {
        let song = exec_easy("PRINT(1 && 1); PRINT(1 && 0); PRINT(0 || 1); PRINT(0 || 0)");
        assert_eq!(
            song.get_logs_str(),
            "[PRINT](0) TRUE\n[PRINT](0) FALSE\n[PRINT](0) TRUE\n[PRINT](0) FALSE"
        );
    }

    #[test]
    fn test_operator_precedence() {
        // 旧サクラでは &、^、| は同じ優先順位で左から評価する
        // (1 | 2) & 4 = 3 & 4 = 0
        let song = exec_easy("Int A = 1 | 2 & 4; PRINT(A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 0");

        // 括弧は優先順位を上書きする: 1 | (2 & 4) = 1 | 0 = 1
        let song = exec_easy("Int B = 1 | (2 & 4); PRINT(B)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 1");

        // (1 | 2) ^ 3 = 3 ^ 3 = 0
        let song = exec_easy("Int C = 1 | 2 ^ 3; PRINT(C)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 0");

        // + はビット演算より優先される: (1 + 2) & 3 = 3 & 3 = 3
        let song = exec_easy("Int D = 1 + 2 & 3; PRINT(D)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 3");

        // 同じ優先順位の算術演算も左から評価する: (8 - 4) - 2 = 2
        let song = exec_easy("Int E = 8 - 4 - 2; PRINT(E)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 2");

        // 明示的な括弧は維持される: 8 - (4 - 2) = 6
        let song = exec_easy("Int F = 8 - (4 - 2); PRINT(F)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 6");
    }

    #[test]
    fn test_tie_caret_non_regression() {
        let song = exec_easy("l4 c^d");
        assert_eq!(song.tracks[0].events.len(), 2); // Event::Note c^ and Event::Note d
        assert_eq!(song.tracks[0].timepos, song.timebase * 3);

        let song = exec_easy("l4 c^^^");
        assert_eq!(song.tracks[0].events.len(), 1); // Event::Note c^^^
        assert_eq!(song.tracks[0].timepos, song.timebase * 4);
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
        let e = song.tracks[0]
            .events
            .iter()
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
        let song =
            exec_easy("v.onCycle(!4,70,80) v__1.onCycle(!4,10,20) cde v__1.onNote(-10,-20) fga");
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
            .filter(|event| event.etype == EventType::ControllChange && event.v1 == 1)
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

    /// ベンドレンジ(RPN)の (時間, 値) を取り出す
    fn bend_ranges(song: &crate::song::Song) -> Vec<(isize, isize)> {
        song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::PitchBendRange)
            .map(|event| (event.time, event.v1))
            .collect::<Vec<_>>()
    }

    /// ノートオンの (時間, 音程, ゲート) を取り出す
    fn note_events(song: &crate::song::Song) -> Vec<(isize, isize, isize)> {
        let mut notes = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| (event.time, event.v1, event.v2))
            .collect::<Vec<_>>();
        notes.sort();
        notes
    }

    #[test]
    fn test_slur_third_arg_sets_bend_range() {
        // Slur(0, value, range) の第3引数でベンドレンジを指定できること (#7)
        let song = exec_easy("TimeBase=96 Slur(0,48,2) l4 c&d");
        assert_eq!(song.get_logs_str(), "");
        // Slur を書いた位置(0)でベンドレンジが2に設定される
        assert_eq!(bend_ranges(&song), vec![(0, 2)]);
        // BR(2) を明示した場合と同じピッチベンドになる
        let expected = pitch_bends(&exec_easy("TimeBase=96 BR(2) Slur(0,48) l4 c&d"));
        assert_eq!(pitch_bends(&song), expected);
        // range 未指定なら既定の12が使われる (BR指定なしのとき)
        let song = exec_easy("TimeBase=96 Slur(0,48) l4 c&d");
        let expected = pitch_bends(&exec_easy("TimeBase=96 BR(12) Slur(0,48) l4 c&d"));
        assert_eq!(pitch_bends(&song), expected);
    }

    #[test]
    fn test_slur_third_arg_does_not_override_later_br() {
        // Slur(0,…,range) の指定は Slur を書いた位置だけに効き、
        // あとから BR() で変更したベンドレンジを壊さないこと (#7)
        let song = exec_easy("TimeBase=96 Slur(0,48,2) l4 c&d BR(6) e&f");
        // タイの処理でベンドレンジを書き込まないこと
        // (Slur(…,2) の1回だけ。BR(6) は RPN の CC として書き込まれる)
        assert_eq!(bend_ranges(&song), vec![(0, 2)]);
        // 2回目のスラーは BR(6) のレンジで計算される
        let expected = pitch_bends(&exec_easy("TimeBase=96 BR(6) Slur(0,48) l4 e&f"));
        let actual = pitch_bends(&song)
            .into_iter()
            .filter(|(time, _)| *time >= 96 * 2)
            .map(|(time, value)| (time - 96 * 2, value))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_slur_does_not_write_bend_range_for_same_note_tie() {
        // 同じ音程のタイ(c&c)はベンドを書き込まないので、
        // タイの処理でベンドレンジを書き込まないこと (#7)
        let song = exec_easy("TimeBase=96 Slur(0,48,2) l4 c&c");
        // Slur(…,2) を書いた位置の1回だけで、タイの位置には増えない
        assert_eq!(bend_ranges(&song), vec![(0, 2)]);
        // ピッチベンドも書き込まれない
        assert_eq!(pitch_bends(&song), vec![]);
    }

    #[test]
    fn test_slur_omitted_args_keep_previous_value() {
        // 省略した引数は以前の値を保持する (オリジナル実装と同じ)
        let song = exec_easy("TimeBase=96 Slur(3,2) l16 Slur(3) c&e&g&>c");
        let expected = exec_easy("TimeBase=96 Slur(3,2) l16 c&e&g&>c");
        assert_eq!(note_events(&song), note_events(&expected));
    }

    #[test]
    fn test_slur_alpe_max_notes() {
        // Slur(3, value) の value で最大発音音数を指定できること (#7)
        // 4音を16分音符(24ステップ)でつなぎ、同時発音数を2音に制限する
        let song = exec_easy("TimeBase=96 Slur(3,2) l16 c&e&g&>c");
        assert_eq!(song.get_logs_str(), "");
        let notes = note_events(&song);
        assert_eq!(notes.len(), 4);
        // 各音は「1つあとの音符のゲートの終わり」まで伸びる (16分音符のゲートは21)
        // 1音目(0) → 2音目の終わり(24+21=45)
        assert_eq!(notes[0].0 + notes[0].2, 24 + 21);
        assert_eq!(notes[1].0 + notes[1].2, 48 + 21);
        assert_eq!(notes[2].0 + notes[2].2, 72 + 21);
        // 最後の音はそれ以上つながる音がないので、自身のゲートのまま
        assert_eq!(notes[3].0 + notes[3].2, 72 + 21);
        // 同時に鳴るのは2音まで (3音目が鳴り始める48の時点で1音目は終わっている)
        assert!(notes[0].0 + notes[0].2 <= notes[2].0);
    }

    #[test]
    fn test_slur_alpe_keeps_gate_rate() {
        // q50 のように短いゲートでも、伸ばした音のゲート比率が保たれること (#7)
        // (音符の開始位置ではなく、対象の音符のゲートの終わりまで伸ばす)
        let song = exec_easy("TimeBase=96 q50 Slur(3,2) l16 c&e&g&>c");
        let notes = note_events(&song);
        // 16分音符(24ステップ)のゲートは12。1音目は2音目の終わり(24+12=36)まで
        assert_eq!(notes[0].0 + notes[0].2, 36);
        // 3音目が鳴り始める48より前に1音目が終わり、レガートになりきらない
        assert!(notes[0].0 + notes[0].2 < notes[2].0);
    }

    #[test]
    fn test_slur_alpe_max_notes_boundary() {
        // value=1 なら音を伸ばさない(単音ずつ)
        let notes = note_events(&exec_easy("TimeBase=96 Slur(3,1) l16 c&e&g&>c"));
        for (_, _, gate) in notes.iter() {
            assert_eq!(*gate, 21); // 16分音符のゲートのまま
        }
        // value が音符数以上なら、全ての音が最後まで伸びる
        let notes = note_events(&exec_easy("TimeBase=96 Slur(3,10) l16 c&e&g&>c"));
        for (time, _, gate) in notes.iter() {
            assert_eq!(time + gate, 93);
        }
        // 0や負数は未指定として扱い、全ての音が最後まで伸びる
        for src in [
            "TimeBase=96 Slur(3,0) l16 c&e&g&>c",
            "TimeBase=96 Slur(3,-1) l16 c&e&g&>c",
        ] {
            let song = exec_easy(src);
            assert_eq!(song.get_logs_str(), "");
            for (time, _, gate) in note_events(&song) {
                assert_eq!(time + gate, 93);
            }
        }
    }

    #[test]
    fn test_slur_alpe_without_max_notes() {
        // value 未指定なら、従来どおり全ての音が最後まで伸びる
        let song = exec_easy("TimeBase=96 Slur(3) l16 c&e&g&>c");
        for (time, _, gate) in note_events(&song) {
            assert_eq!(time + gate, 93);
        }
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

    /// ノートオンの音程を取り出す
    fn note_numbers(song: &Song) -> Vec<isize> {
        song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::NoteOn)
            .map(|e| e.v1)
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
    fn test_cc_delay_is_not_applied_twice_on_cycle() {
        // .onCycle の書き込み位置に .Delay が二重に足されないこと
        let song = exec_easy("TimeBase=96 M.Delay(12) M.onCycle(!4,0,127) l4 cdcd");
        let times: Vec<isize> = cc_events(&song, 1).iter().map(|(t, _)| *t).collect();
        assert_eq!(times, vec![12, 108, 204, 300]);
    }

    #[test]
    fn test_cc_random_does_not_skip_same_base_value() {
        // .Random 有効時は、基準値が同じ区間でも書き込みが省略されないこと
        // (low==high の区間は基準値が一定なので、重複抑制が効くと1つしか出ない)
        let song = exec_easy("TimeBase=96 CC.Frequency(24) M.onTime(60,60,!1) c1");
        assert_eq!(cc_events(&song, 1).len(), 1);
        // 24ステップおきに、全音符(384ステップ)の間で16回書き込まれる
        let song = exec_easy("TimeBase=96 CC.Frequency(24) M.Random(20) M.onTime(60,60,!1) c1");
        assert_eq!(cc_events(&song, 1).len(), 16);
    }

    #[test]
    fn test_cc_repeat_affects_only_its_own_target() {
        // M.Repeat(on) が他のCCやピッチベンドの .onNote をループ化しないこと
        let song = exec_easy(
            "TimeBase=96 M.onNote(1,2) P.onNote(10,20) PB.onNote(-8192,0) M.Repeat(on) l4 cdef",
        );
        let modulation: Vec<isize> = cc_events(&song, 1).iter().map(|(_, v)| *v).collect();
        let panpot: Vec<isize> = cc_events(&song, 10).iter().map(|(_, v)| *v).collect();
        let bends: Vec<isize> = pitch_bends(&song).iter().map(|(_, v)| *v).collect();
        assert_eq!(modulation, vec![1, 2, 1, 2]);
        assert_eq!(panpot, vec![10, 20]);
        assert_eq!(bends, vec![0, 8192]);
    }

    #[test]
    fn test_cc_on_cycle_continues_inside_a_long_note() {
        // 長い音符の途中でも .onCycle の書き込みが止まらないこと
        let song = exec_easy("TimeBase=96 M.onCycle(!8,0,127) l1 c");
        let times: Vec<isize> = cc_events(&song, 1).iter().map(|(t, _)| *t).collect();
        assert_eq!(times, vec![0, 48, 96, 144, 192, 240, 288, 336]);
    }

    #[test]
    fn test_cc_on_cycle_continues_during_rests() {
        // 休符で時間が進んだ場合も書き込みが続くこと
        let song = exec_easy("TimeBase=96 M.onCycle(!4,0,127) l4 c r r r");
        assert_eq!(
            cc_events(&song, 1),
            vec![(0, 0), (96, 127), (192, 0), (288, 127)]
        );
    }

    #[test]
    fn test_length_random_changes_normal_note_length() {
        // l.Random は先行指定なしの通常の音長にも効くこと
        let song = exec_easy("TimeBase=96 l.Random(8) l4 [8 c]");
        let times: Vec<isize> = note_times(&song).iter().map(|(t, _)| *t).collect();
        // 音長が96のままなら 0,96,192,... と並ぶので、ずれていることを確かめる
        assert!(times.iter().enumerate().any(|(i, t)| *t != i as isize * 96));
        assert!(times.windows(2).all(|w| (w[1] - w[0] - 96).abs() <= 4));
    }

    #[test]
    fn test_length_range_limits_normal_note_length() {
        // l.Range は先行指定なしの通常の音長にも効くこと
        let song = exec_easy("TimeBase=96 l.Range(48,48) l4 cd");
        let times: Vec<isize> = note_times(&song).iter().map(|(t, _)| *t).collect();
        assert_eq!(times, vec![0, 48]);
    }

    #[test]
    fn test_octave_range_limits_normal_octave() {
        // o.Range は o コマンドで直接指定したオクターブにも効くこと
        let song = exec_easy("o.Range(4,4) o6 c");
        assert_eq!(note_numbers(&song), vec![48]);
        // 先行指定で切り替えたオクターブにも効く
        let song = exec_easy("o.Range(4,4) o.onNote(6) c");
        assert_eq!(note_numbers(&song), vec![48]);
    }

    #[test]
    fn test_octave_change_keeps_accidental() {
        // オクターブの差し替えで臨時記号が消えないこと
        let song = exec_easy("o5 c+ o.Range(6,6) c+");
        assert_eq!(note_numbers(&song), vec![61, 73]);
    }

    #[test]
    fn test_direct_value_clears_reserve() {
        // 値を直接指定すると先行指定は解除される
        let song = exec_easy("TimeBase=96 M.onNote(10,20) l4 c M(0) d");
        let values: Vec<isize> = cc_events(&song, 1).iter().map(|(_, v)| *v).collect();
        assert_eq!(values, vec![10, 0]);
    }
}

mod test_issue_129 {
    use super::*;
    use crate::song::{EventType, Song};

    fn cc_events(song: &Song, no: isize) -> Vec<(isize, isize)> {
        song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::ControllChange && e.v1 == no)
            .map(|e| (e.time, e.v2))
            .collect()
    }

    fn note_qlens(song: &Song) -> Vec<isize> {
        song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::NoteOn)
            .map(|e| e.v2)
            .collect()
    }

    fn velocities(song: &Song) -> Vec<isize> {
        song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::NoteOn)
            .map(|e| e.v3)
            .collect()
    }

    #[test]
    fn test_ontime_array_expansion() {
        // Issue #129 講座の例: q.onTime(m, A, m, m, A, m, A, A)
        let song = exec_easy(
            "TimeBase=96 Array m=(10,10,!16); Array A=(100,100,!16); q.onTime(m,A); l16 o6 cdef",
        );
        let qlens = note_qlens(&song);
        assert_eq!(qlens.len(), 4);
        assert_eq!(qlens[0], 2);
        assert_eq!(qlens[1], 24);

        // v.onTime での配列変数展開
        let song = exec_easy("TimeBase=96 Array low=(20,20,!4); Array high=(100,100,!4); v.onTime(low,high); l4 cdef");
        let vels = velocities(&song);
        assert_eq!(vels[0], 20);
        assert_eq!(vels[1], 100);

        // CC / Modulation での配列変数展開
        let song = exec_easy(
            "TimeBase=96 Array p1=(10,10,!4); Array p2=(80,80,!4); M.onTime(p1,p2); l4 cd",
        );
        let m_events = cc_events(&song, 1);
        assert!(!m_events.is_empty());
        assert_eq!(m_events[0].1, 10);
    }

    #[test]
    fn test_array_flattening() {
        // 通常の配列は入れ子を維持し、ArrayFlattenで明示的に平坦化する
        let song = exec_easy("ARRAY A=(1,2); ARRAY B=(3,4); ARRAY C=(A,B,5); ARRAY D=ArrayFlatten(C); PRINT(C); PRINT(SizeOf(C)); PRINT(D); PRINT(SizeOf(D))");
        assert_eq!(
            song.get_logs_str(),
            "[PRINT](0) ((1,2),(3,4),5)\n[PRINT](0) 3\n[PRINT](0) (1,2,3,4,5)\n[PRINT](0) 5"
        );

        // 空配列と深い入れ子を展開し、大文字の別名も利用できる
        let song = exec_easy("ARRAY Empty=(); ARRAY A=(Empty, 1, (2, Empty, (3,4))); ARRAY B=ARRAYFLATTEN(A); PRINT(B); PRINT(SizeOf(B))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) (1,2,3,4)\n[PRINT](0) 4");
    }

    #[test]
    fn test_array_edge_cases() {
        // 文字列数値を含む配列の先行指定展開
        let song = exec_easy("TimeBase=96 Array s=(\"50\",\"100\",!1); q.onTime(s); l4 cd");
        let qlens = note_qlens(&song);
        assert_eq!(qlens[0], 48);
        assert_eq!(qlens[1], 59);
    }
}

mod test_issue_128 {
    use super::*;
    use crate::song::EventType;

    #[test]
    fn test_rrs_sample_accepts_nested_mml_call() {
        let song = exec_easy(
            "Function RRS(Len){
                Int Now_Vol=MML(y7)
                If(Len==0){Len=!32}
                r-%(Len)
                V.T(MML(y7),40,Len)
                r%(Len)
                V(Now_Vol)
            }
            V(90)
            RRS(!4)",
        );
        assert_eq!(song.get_logs_str(), "");
        let values: Vec<isize> = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::ControllChange && event.v1 == 7)
            .map(|event| event.v2)
            .collect();
        assert!(values.iter().any(|value| *value < 90), "{values:?}");
        assert_eq!(values.last(), Some(&90));
    }

    #[test]
    fn test_reserve_arguments_accept_functions_and_expressions() {
        let song = exec_easy(
            "TimeBase=96
            V(90)
            V.T(MML(y7)+10,20*2,!4)
            Function CurrentPan(){RETURN(80)}
            Panpot.onTime(CurrentPan(),127,!4)
            Function StartPB(){RETURN(-4096)}
            PB.T(StartPB(),0,!4)
            Function BaseVel(){RETURN(20)}
            v.onTime(BaseVel()+10,90-10,!4)
            l4c",
        );
        assert_eq!(song.get_logs_str(), "");
        let volume = song.tracks[0]
            .events
            .iter()
            .find(|event| event.etype == EventType::ControllChange && event.v1 == 7)
            .map(|event| event.v2);
        let panpot = song.tracks[0]
            .events
            .iter()
            .find(|event| event.etype == EventType::ControllChange && event.v1 == 10)
            .map(|event| event.v2);
        let pitch_bend = song.tracks[0]
            .events
            .iter()
            .find(|event| event.etype == EventType::PitchBend)
            .map(|event| event.v1);
        let velocity = song.tracks[0]
            .events
            .iter()
            .find(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v3);
        assert_eq!(volume, Some(100));
        assert_eq!(panpot, Some(80));
        assert_eq!(pitch_bend, Some(4096));
        assert_eq!(velocity, Some(30));

        // 従来のイコール形式でも計算式を利用できる
        let song = exec_easy("TimeBase=96 M.T=10+10,40,!4");
        let modulation = song.tracks[0]
            .events
            .iter()
            .find(|event| event.etype == EventType::ControllChange && event.v1 == 1)
            .map(|event| event.v2);
        assert_eq!(modulation, Some(20));
    }

    #[test]
    fn test_reserve_arguments_report_missing_parenthesis() {
        let song = exec_easy("V.T(MML(y7),40,!4");
        assert!(
            song.get_logs_str().contains("Missing Parenthesis")
                || song.get_logs_str().contains("括弧が閉じられていません"),
            "{}",
            song.get_logs_str()
        );
    }

    #[test]
    fn test_mml_cc_uses_current_track_and_time_position() {
        let song = exec_easy("V(10) TIME(5:1:0) V(20) TIME(1:1:0) Print(MML(y7))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 10");

        let song = exec_easy("TR(1) CH(1) V(20) TR(2) CH(1) Print(MML(y7))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 100");
    }

    #[test]
    fn test_mml_cc_defaults_match_legacy_sakura() {
        let song = exec_easy("Print(MML(y7),MML(y10),MML(y11),MML(y1))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 100 64 127 0");
    }
}

/// Issue #127: q%n と音符付随指定の %ゲート (ステップ単位のゲート指定)
#[cfg(test)]
mod test_issue_127 {
    use super::*;
    use crate::song::EventType;

    /// NoteOn の (時間, ゲート長) を取り出す
    fn note_times(song: &Song) -> Vec<(isize, isize)> {
        song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::NoteOn)
            .map(|e| (e.time, e.v2))
            .collect()
    }

    fn note_gates(song: &Song) -> Vec<isize> {
        note_times(song).iter().map(|(_, len)| *len).collect()
    }

    #[test]
    fn test_qlen_step_basic() {
        // 講座の例: TimeBase(96) l4 q%96 / q%48 / q%10
        let song = exec_easy("TimeBase=96 l4 q%96 cdef q%48 cdef q%10 cdef");
        assert_eq!(song.get_logs_str(), "");
        assert_eq!(
            note_gates(&song),
            vec![96, 96, 96, 96, 48, 48, 48, 48, 10, 10, 10, 10]
        );
    }

    #[test]
    fn test_qlen_step_is_absolute_for_any_length() {
        // ステップ指定は音長に関わらず同じゲート長になる
        let song = exec_easy("TimeBase=96 q%30 l4 c l8 d l16 e");
        assert_eq!(note_gates(&song), vec![30, 30, 30]);
        // 割合指定は音長に比例する
        let song = exec_easy("TimeBase=96 q50 l4 c l8 d l16 e");
        assert_eq!(note_gates(&song), vec![48, 24, 12]);
    }

    #[test]
    fn test_qlen_step_with_timebase() {
        // TimeBaseを変えても、ステップ指定はそのままのステップ数になる
        let song = exec_easy("TimeBase=48 l4 q%24 c");
        assert_eq!(note_gates(&song), vec![24]);
        let song = exec_easy("TimeBase=480 l4 q%24 c");
        assert_eq!(note_gates(&song), vec![24]);
    }

    #[test]
    fn test_qlen_step_zero_negative_and_over_length() {
        // 0 --- ゲート長0
        let song = exec_easy("TimeBase=96 l4 q%0 c");
        assert_eq!(note_gates(&song), vec![0]);
        // 負の値 --- 現在のqの値からの相対指定 (qの初期値は90)
        let song = exec_easy("TimeBase=96 l4 q%-1 c");
        assert_eq!(note_gates(&song), vec![89]);
        // 相対指定はくり返し適用され、値を引いた結果もステップ単位になる
        let song = exec_easy("TimeBase=96 l4 q80 q%-1 c q%-1 d");
        assert_eq!(note_gates(&song), vec![79, 78]);
        // 0未満にはならない
        let song = exec_easy("TimeBase=96 l4 q%-200 c");
        assert_eq!(note_gates(&song), vec![0]);
        // 音長を超える値 --- そのまま使う(次の音符と重なる)
        let song = exec_easy("TimeBase=96 l4 q%200 c");
        assert_eq!(note_gates(&song), vec![200]);
        // q.Max は割合指定の上限なので、ステップ指定には影響しない
        let song = exec_easy("TimeBase=96 q.Max(50) l4 q%200 c");
        assert_eq!(note_gates(&song), vec![200]);
    }

    #[test]
    fn test_qlen_step_and_percent_are_switched() {
        // q%n のあとに q n を書くと割合指定に戻る
        let song = exec_easy("TimeBase=96 l4 q%10 c q50 d");
        assert_eq!(note_gates(&song), vec![10, 48]);
        // 音符側の割合指定は、トラックのステップ指定より優先される
        let song = exec_easy("TimeBase=96 l4 q%10 c4,50");
        assert_eq!(note_gates(&song), vec![48]);
        // 変数や式でも指定できる
        let song = exec_easy("TimeBase=96 Int A=70; l4 q%(A) c");
        assert_eq!(note_gates(&song), vec![70]);
        // !n の音長表記でステップ数を指定できる
        let song = exec_easy("TimeBase=96 l4 q%!8 c");
        assert_eq!(note_gates(&song), vec![48]);
    }

    #[test]
    fn test_note_gate_step_argument() {
        // 講座の例: c%96,%70,120,0 は l%96 q%70 v120 t0 c と同じ
        let song = exec_easy("TimeBase=96 c%96,%70,120,0");
        assert_eq!(song.get_logs_str(), "");
        assert_eq!(note_times(&song), vec![(0, 70)]);
        let song2 = exec_easy("TimeBase=96 l%96 q%70 v120 t0 c");
        assert_eq!(note_times(&song), note_times(&song2));
        // ベロシティも指定通りであること
        let vels: Vec<isize> = song.tracks[0]
            .events
            .iter()
            .filter(|e| e.etype == EventType::NoteOn)
            .map(|e| e.v3)
            .collect();
        assert_eq!(vels, vec![120]);
        // %を付けない指定はこれまで通り割合
        let song = exec_easy("TimeBase=96 c4,70");
        assert_eq!(note_gates(&song), vec![67]);
        // 0やマイナスの指定 (マイナスは現在のqの値からの相対指定 --- qの初期値は90)
        let song = exec_easy("TimeBase=96 c4,%0 d4,%-6");
        assert_eq!(note_gates(&song), vec![0, 84]);
    }

    #[test]
    fn test_note_n_and_harmony_gate_step() {
        // n コマンドの音符付随指定
        let song = exec_easy("TimeBase=96 n60,4,%70");
        assert_eq!(note_gates(&song), vec![70]);
        // n コマンドでもトラックのステップ指定を受け継ぐ
        let song = exec_easy("TimeBase=96 q%30 l4 n60");
        assert_eq!(note_gates(&song), vec![30]);
        // 和音のゲート指定
        let song = exec_easy("TimeBase=96 'ceg'4,%70");
        assert_eq!(note_gates(&song), vec![70, 70, 70]);
        // 和音でもトラックのステップ指定を受け継ぐ
        let song = exec_easy("TimeBase=96 q%30 l4 'ceg'");
        assert_eq!(note_gates(&song), vec![30, 30, 30]);
    }

    #[test]
    fn test_qlen_step_note_off_position() {
        // MIDIのNoteOffが (発音位置 + ゲート長) に書かれること
        let song = exec_easy("TimeBase=96 l4 q%48 cd");
        let events = song.tracks[0].split_note_off();
        let note_off: Vec<(isize, isize)> = events
            .iter()
            .filter(|e| e.etype == EventType::NoteOff)
            .map(|e| (e.time, e.v1))
            .collect();
        assert_eq!(note_off, vec![(48, 60), (144, 62)]);
    }

    #[test]
    fn test_qlen_step_with_reserve() {
        // 先行指定(q.onNote)は割合指定なので、ステップ指定より優先される
        let song = exec_easy("TimeBase=96 q%10 q.onNote(50,100) l4 cd");
        assert_eq!(note_gates(&song), vec![48, 96]);
        // 先行指定が終わればステップ指定は解除されている(割合指定の値が残る)
        let song = exec_easy("TimeBase=96 q%10 q.onNote(50) l4 cd");
        assert_eq!(note_gates(&song), vec![48, 48]);
        // .onTime が終わった次の音符からは、ステップ指定に戻る
        // (予約の解除は値を求めたあとに分かるので、境界の1音を割合で計算しないこと)
        let song = exec_easy("TimeBase=96 q%10 q.onTime(50,50,!4) l4 cde");
        assert_eq!(note_gates(&song), vec![48, 10, 10]);
    }

    #[test]
    fn test_harmony_gate_with_reserve() {
        // 和音でも、先行指定(割合)はステップ指定より優先される
        let song = exec_easy("TimeBase=96 q%10 q.onTime(50,100,!1) l4 'ceg'");
        assert_eq!(note_gates(&song), vec![48, 48, 48]);
        // 和音自身のステップ指定より、先行指定を優先する
        let song = exec_easy("TimeBase=96 q.onNote(50) l4 'ceg'4,%70");
        assert_eq!(note_gates(&song), vec![48, 48, 48]);
        // 先行指定がなければ、和音自身のステップ指定を使う
        let song = exec_easy("TimeBase=96 l4 'ceg'4,%70");
        assert_eq!(note_gates(&song), vec![70, 70, 70]);
        // 和音の音長を変えても、ステップ指定はそのままのステップ数になる
        let song = exec_easy("TimeBase=96 l16 q%70 'ceg'4");
        assert_eq!(note_gates(&song), vec![70, 70, 70]);
        // 和音の割合指定は、これまで通り音長に対する割合
        let song = exec_easy("TimeBase=96 l16 'ceg'4,50");
        assert_eq!(note_gates(&song), vec![48, 48, 48]);
    }
}

#[cfg(test)]
mod test_note_param_arg {
    use super::exec_easy;
    use crate::song::EventType;

    /// 引数のないv/q/o/tはエラーにする (書き間違いの検出)
    #[test]
    fn test_missing_argument_error() {
        // `vf+4` は `f+4` の書き間違い。vの引数として f を読み飛ばさない
        let song = exec_easy("o5 v100 vf+4");
        assert!(song.get_logs_str().contains("\"v\""));
        assert!(song.get_logs_str().contains("[ERROR]"));
        // エラーになってもベロシティは変更しない
        assert_eq!(song.tracks[0].velocity, 100);
        // 音符 f+ は演奏される
        let notes: Vec<isize> = song.tracks[0]
            .events
            .iter()
            .filter(|event| event.etype == EventType::NoteOn)
            .map(|event| event.v1)
            .collect();
        assert_eq!(notes, vec![66]);
        // q/o/t も同様
        assert!(exec_easy("q c").get_logs_str().contains("[ERROR]"));
        assert!(exec_easy("o c").get_logs_str().contains("[ERROR]"));
        assert!(exec_easy("t c").get_logs_str().contains("[ERROR]"));
        // 定義済みの変数や、= や () を使った指定はエラーにしない
        let song = exec_easy("Int A=60 vA q=50 o(3) c");
        assert_eq!(song.get_logs_str(), "");
        assert_eq!(song.tracks[0].velocity, 60);
        assert_eq!(song.tracks[0].qlen, 50);
        assert_eq!(song.tracks[0].octave, 3);
    }

    /// 相対指定 (v+n, q+n, o+n, t+n, v++, o--)
    #[test]
    fn test_relative_value() {
        // 数値を指定すると、その値だけ増える
        let song = exec_easy("v40 v+10 q80 q+10 o5 o+2 t0 t+3 c");
        assert_eq!(song.tracks[0].velocity, 50);
        assert_eq!(song.tracks[0].qlen, 90);
        assert_eq!(song.tracks[0].octave, 7);
        assert_eq!(song.tracks[0].timing, 3);
        // 数値がなければ、既定の増減幅(v=vAdd, q=qAdd, o/t=1)だけ増減する
        let song = exec_easy("v40 v++ o5 o-- t0 t- c");
        assert_eq!(song.tracks[0].velocity, 48);
        assert_eq!(song.tracks[0].octave, 4);
        assert_eq!(song.tracks[0].timing, -1);
        // System.vAdd の変更が反映される
        let song = exec_easy("System.vAdd(3) v40 v++ v+ c");
        assert_eq!(song.tracks[0].velocity, 46);
        // '(' ')' はvAddの値だけ増減する
        let song = exec_easy("System.vAdd(5) v40 ) ) ( c");
        assert_eq!(song.tracks[0].velocity, 45);
        // マイナス符号と数値は、これまで通り負の絶対値指定
        let song = exec_easy("t0 t-10 v100 v-10 c");
        assert_eq!(song.tracks[0].timing, -10);
        assert_eq!(song.tracks[0].velocity, 0);
    }
}
