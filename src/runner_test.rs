//! Tests for runner module
use crate::runner::*;
use crate::lexer::lex;
use crate::song::{EventType, Song};

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
