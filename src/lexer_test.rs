//! Tests for the lexer module
use crate::lexer::lex;
use crate::song::Song;
use crate::token::tokens_to_str;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex1() {
        let mut song = Song::new();
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "cdefgab", 0)),
            "[Note,0][Note,2][Note,4][Note,5][Note,7][Note,9][Note,11]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "l4c", 0)),
            "[Length,0][Note,0]"
        );
        assert_eq!(&tokens_to_str(&lex(&mut song, "TR=1", 0)), "[Track,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TR(1)", 0)), "[Track,0]");
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "INT A=1;TR(A)", 0)),
            "[DefInt,0][Track,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "INT A=1;TR=A", 0)),
            "[DefInt,0][Track,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "COPYRIGHT{a}", 0)),
            "[MetaText,2]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "COPYRIGHT={a}", 0)),
            "[MetaText,2]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TimeSig=4,4", 0)),
            "[TimeSignature,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TimeSig=(4,4)", 0)),
            "[TimeSignature,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TimeSig(4,4)", 0)),
            "[TimeSignature,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TIME(1:1:0)", 0)),
            "[Time,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TIME=(1:1:0)", 0)),
            "[Time,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "TIME(1:1:0)", 0)),
            "[Time,0]"
        );
        assert_eq!(&tokens_to_str(&lex(&mut song, "TIME=1:1:0", 0)), "[Time,0]");
    }

    #[test]
    fn test_lex_harmony() {
        let mut song = Song::new();
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "'dg'", 0)),
            "[HarmonyBegin,0][Note,2][Note,7][HarmonyEnd,0]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "'dg'^^^", 0)),
            "[HarmonyBegin,0][Note,2][Note,7][HarmonyEnd,0]"
        );
    }

    #[test]
    fn test_lex_rhythm_macro() {
        let mut song = Song::new();
        assert_eq!(&tokens_to_str(&lex(&mut song, "RHYTHM{b}", 0)), "[NoteN,0]");
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "RHYTHM{(Sub){b}}", 0)),
            "[Sub,0]"
        );
    }

    #[test]
    fn test_lex_cc() {
        let mut song = Song::new();
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "P(10)", 0)),
            "[ControlChange,10]"
        );
        assert_eq!(
            &tokens_to_str(&lex(&mut song, "M(10)", 0)),
            "[ControlChange,1]"
        );
    }

    #[test]
    fn test_lex_debug_comment() {
        use crate::token::{TokenType, COMMENT_DEBUG};
        let mut song = Song::new();
        // 「///」だけデバッグ用コメントとして残る (「//」は消える)
        let tokens = lex(&mut song, "// aaa\ncd\n/// bbb\n", 0);
        let comments: Vec<_> = tokens
            .iter()
            .filter(|t| t.ttype == TokenType::Comment)
            .collect();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].value_i, COMMENT_DEBUG);
        assert_eq!(comments[0].value_s.as_deref(), Some("bbb"));
        assert_eq!(comments[0].lineno, 2);
        // コメント記号「///」だけを除去する (本文先頭の「/」は残す)
        let tokens = lex(&mut song, "/// /path/to", 0);
        let comments: Vec<_> = tokens
            .iter()
            .filter(|t| t.ttype == TokenType::Comment)
            .collect();
        assert_eq!(comments[0].value_s.as_deref(), Some("/path/to"));
    }

    #[test]
    fn test_lex_comment_lineno() {
        use crate::token::TokenType;
        let mut song = Song::new();
        // コメント行は改行ごと読み飛ばすので、LineNoトークンで行番号を補う
        for src in ["// a\nc", "/// a\nc", "## a\nc", "/* a\na */c"] {
            let tokens = lex(&mut song, src, 0);
            let last_lineno = tokens
                .iter()
                .filter(|t| t.ttype == TokenType::LineNo)
                .last()
                .map(|t| t.lineno);
            assert_eq!(last_lineno, Some(1), "src={:?}", src);
        }
    }

    /// End/END命令 (issue #141)
    #[test]
    fn test_end_command() {
        let mut song = Song::new();
        // End以降はコンパイルしない。Endトークンを出力して実行を中断する
        let tokens = lex(&mut song, "cd End efg", 0);
        let s = tokens_to_str(&tokens);
        assert!(s.starts_with("[Note,0][Note,2][End,0]"), "{}", s);
        assert!(!s.contains("[Note,4]"), "{}", s);
        // ENDも同じ
        let tokens = lex(&mut song, "cd END efg", 0);
        let s = tokens_to_str(&tokens);
        assert!(s.starts_with("[Note,0][Note,2][End,0]"), "{}", s);
        // 英数字が続く場合はEnd命令ではない
        let tokens = lex(&mut song, "Int ENDTIME=1;TR(ENDTIME)c", 0);
        let s = tokens_to_str(&tokens);
        assert!(!s.contains("[End,0]"), "{}", s);
        assert!(s.contains("[Note,0]"), "{}", s);
    }

    #[test]
    fn test_timebase() {
        let mut song = Song::new();
        let tokens = lex(&mut song, "TIMEBASE(48)", 0);
        println!("{:?}", tokens);
        assert_eq!(&tokens_to_str(&tokens), "[Comment#TIMEBASE=48]");
    }
}
