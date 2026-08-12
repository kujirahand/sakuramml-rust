//! lexer
use crate::source_cursor::SourceCursor;
use crate::note_length::calc_length;
use crate::sakura_message::MessageKind;
use crate::song::{Song, SFunction};
use crate::svalue::SValue;
use crate::token::{zen2han, Token, TokenType, TokenValueType, COMMENT_DEBUG, COMMENT_NORMAL};

mod args;
mod calc;
mod cc;
mod command;
mod error;
mod note;
mod variable;

use args::*;
use calc::*;
use cc::*;
use command::*;
use error::*;
use note::*;
use variable::*;

/// prerpcess ... check user function
fn lex_preprocess(song: &mut Song, cur: &mut SourceCursor) -> bool {
    let tmp_lineno = cur.line;
    while !cur.is_eos() {
        // skip comment
        if cur.eq("/*") {
            cur.get_token_s("*/");
            continue;
        }
        if cur.eq("//") {
            cur.get_token_ch('\n');
            continue;
        }
        // (memom) 上記以外に '#' から始まるコメント記号があるが、
        //  #はマクロやシャープ記号と使っているので、ここでは判定できない
        // check upper case
        if cur.is_upper() {
            let word = cur.get_word();
            // Check defining user function
            if word == "FUNCTION" || word == "Function" {
                cur.skip_space();
                let func_name = cur.get_word();
                // check double definition
                if song.variables_contains_key(&func_name) {
                    let reason = song.get_message(MessageKind::ErrorRedfineFnuction);
                    read_warning(cur, song, &func_name, reason);
                }
                // check reserved words
                if song.reserved_words.contains_key(&func_name) {
                    let msg = format!("{}: \"{}\"", song.get_message(MessageKind::ErrorDefineVariableIsReserved), func_name);
                    read_error(cur, song, &msg);
                }
                // register function name
                let func_id = song.functions.len();
                song.variables_insert(&func_name, SValue::UserFunc(func_id));
                let sfunc = SFunction::new(&func_name, vec![], func_id, 0);
                song.functions.push(sfunc);
                continue;
            }
            if word == "END" || word == "End" { // それ以降をコンパイルしない
                break;
            }
        }
        // peek
        let ch = cur.get_char();
        if ch == '\n' {
            cur.line += 1;
            continue;
        }
    }
    cur.index = 0;
    cur.line = tmp_lineno;
    true
}

/// split source code to tokens
pub fn lex(song: &mut Song, src: &str, lineno: isize) -> Vec<Token> {
    let mut result: Vec<Token> = vec![
        Token::new_lineno(lineno), // init lineno
    ];
    let mut cur = SourceCursor::from(src);
    cur.line = lineno;
    // preprocess
    let _pre = lex_preprocess(song, &mut cur);
    // read
    let mut flag_harmony = false;
    while !cur.is_eos() {
        let ch = zen2han(cur.get_char());
        // println!("lex: ch = {}", ch);
        match ch {
            // <CHAR_COMMANDS>
            /*
            SPACE TAB CR LF ; CHR(0x7C) => // @ space - 空白文字 / ';'や'|'も読み飛ばす
            */
            ' ' | '\t' | '\r' | '|' | ';' => {},
            // ret
            '\n' => {
                cur.line += 1;
                result.push(Token::new_lineno(cur.line));
            },
            // lower command
            'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b' => result.push(read_note(&mut cur, ch)), // @ note - ドレミファソラシ c(l),(q),(v),(t),(o)
            'n' => result.push(read_note_n(&mut cur, song)), // @ note no - 番号を指定して発音 n(no),(l),(q),(v),(t) - (ex) n60
            'r' => result.push(read_rest(&mut cur)),         // @ rest - 休符
            'l' => result.push(read_length(&mut cur, song)), // @ length - 音長の指定 (ex) l4 c
            'o' => result.push(read_octave(&mut cur, song)), // @ octave - 音階の指定 range:0-10 (ex) o6 c
            'p' => result.push(read_pitch_bend_small(&mut cur, song)), // @ pitch bend - ピッチベンドの指定 range:0-127 (center:64) (ex) p64 / (ref) PB(n) は -8192~0~8191
            'q' => result.push(read_qlen(&mut cur, song)), // @ gate rate - ゲートの指定 range:0-100 (ex) q90
            'v' => result.push(read_velocity(&mut cur, song)), // @ velocity - ベロシティ音量の指定 range:0-127 (ex) v100 / v.Random=n
            't' => result.push(read_timing(&mut cur, song)), // @ timing - 発音タイミングの指定 (例 t-1) / t.Random=n
            'y' => result.push(read_cc(&mut cur, song, ch)), // @ Control change - コントロールチェンジ range:0-127 y(cc_no),(value) / (ex) y1,100 / y1.onTime(low,high,len)
            // Upper command
            'A'..='Z' | '_' => {
                cur.prev();
                if cur.eq("End") || cur.eq("END") { // それ移行をコンパイルしない
                    let last_comment = cur.cur2end();
                    cur.next_n(last_comment.len());
                    result.push(Token::new_empty(&last_comment, cur.line));
                    continue;
                }
                result.push(read_upper_command(&mut cur, song));
            },
            '#' => { // @ Macro - マクロ定義 (ex) #A={cdefg}
                cur.prev();
                if cur.eq("##") || cur.eq("# ") || cur.eq("#-") { // なんかみんなが使っているので一行コメントと見なす
                    cur.get_token_ch('\n');
                    result.push(Token::new_lineno(cur.line)); // 改行を消費したので行番号を更新
                    continue;
                }
                result.push(read_upper_command(&mut cur, song));
            },
            // flag
            '@' => result.push(read_voice(&mut cur, song)), // @ Voice select(音色の指定) range:1-128 (format) @(no),(Bank_MSB),(Bank_LSB)
            '>' => result.push(Token::new_value(TokenType::OctaveRel, 1)), // @ Octave up (音階を1つ上げる)
            '<' => result.push(Token::new_value(TokenType::OctaveRel, -1)), // @ Octave down (音階を1つ下げる)
            ')' => result.push(Token::new_value(TokenType::VelocityRel, song.v_add)), // @ velocity up - 音量をvAddの値だけ上げる
            '(' => result.push(Token::new_value(TokenType::VelocityRel, -1 * song.v_add)), // @ velocity down - 音量をvAddの値だけ下げる
            // comment
            /*
            "\/\*" ... "\*\/" => // @ range comment (範囲コメント)
            "///" => // @ line comment for debug(デバッグ用一行コメント/行番号と内容をMetaTextとしてMIDIに埋め込む)
            "//" => // @ line comment (一行コメント)
            "##" => // @ line comment (一行コメント)
            "# " => // @ line comment (一行コメント)
            "#-" => // @ line comment (一行コメント)
            */
            '/' => {
                cur.prev();
                if cur.eq("///") {
                    let lineno = cur.line;
                    let line_comment = cur.get_token_ch('\n');
                    // コメント記号「///」だけを取り除いた本文をMetaTextとして埋め込む (see: runner.rs)
                    let body = line_comment[3..].trim().to_string();
                    let mut tok = Token::new_const(TokenType::Comment, COMMENT_DEBUG, Some(body), TokenValueType::VOID);
                    tok.lineno = lineno;
                    result.push(tok);
                    result.push(Token::new_lineno(cur.line)); // 改行を消費したので行番号を更新
                    continue;
                } else if cur.eq("//") {
                    cur.get_token_ch('\n');
                    result.push(Token::new_lineno(cur.line)); // 改行を消費したので行番号を更新
                    continue;
                } else if cur.eq("/**") {
                    let range_comment = cur.get_token_s("*/");
                    let mut tok = Token::new_const(TokenType::Comment, COMMENT_NORMAL, Some(range_comment), TokenValueType::VOID);
                    tok.lineno = cur.line;
                    result.push(tok);
                    result.push(Token::new_lineno(cur.line)); // 複数行にまたがる場合があるので行番号を更新
                    continue;
                } else if cur.eq("/*") {
                    cur.get_token_s("*/");
                    result.push(Token::new_lineno(cur.line)); // 複数行にまたがる場合があるので行番号を更新
                    continue;
                }
                cur.next();
                // パースエラー
                let err = format!("Could not parse flag '{}'", ch);
                lex_error(&mut cur, song, &err);
                continue;
            }
            '[' => result.push(read_loop(&mut cur, song)), // @ begin of loop - ループ開始 (ex) [4 cdeg]
            ':' => result.push(Token::new_value(TokenType::LoopBreak, 0)), // @ break of loop - ループ最終回に脱出 (ex)　[4 cde:g]e
            ']' => result.push(Token::new_value(TokenType::LoopEnd, 0)),   // @ end of loop - ループ終了
            '\'' => result.push(read_harmony_flag(&mut cur, &mut flag_harmony)), // @ harmony - 和音 (ex) 'ceg' (format) 'ceg'(音長),(ゲート)
            '$' => read_def_rhythm_macro(&mut cur, song), // @ define rhythm macro - リズムマクロ定義 $(char){ defined } (ex) $c{n60,}
            '{' => result.push(read_command_div(&mut cur, song, true)), // @ tuplet - 連符 {note}(len) (ex) {ceg}4 {c^d}
            '`' => result.push(Token::new_value(TokenType::OctaveOnce, 1)), // @ Octave up once - 一度だけ音階を+1する
            '"' => result.push(Token::new_value(TokenType::OctaveOnce, -1)), // @ Octave down once - 一度だけ音階を-1する
            '?' => result.push(Token::new_value(TokenType::PlayFromHere, 0)), // @ play from here - ここから演奏する (=PlayFromHere)
            '&' => result.push(read_tie_error(&mut cur, song)), // @ tie, slur - タイ・スラー(Slurコマンドで動作が変更できる)
            // </CHAR_COMMANDS>
            _ => {
                let msg = format!("{}", ch);
                lex_error(&mut cur, song, &msg);
                cur.next();
            }
        }
    }
    normalize_tokens(result)
}

// Emptyを削除し、Tokensを展開して返す。ただし、Div/Subは実行時にならないと展開結果が分からないため、それは展開しない
fn normalize_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut res = vec![];
    for t in tokens.into_iter() {
        match t.ttype {
            TokenType::Empty => {}
            TokenType::Tokens => match t.children {
                Some(sub_tt) => {
                    let sub_tt2 = normalize_tokens(sub_tt);
                    for tt in sub_tt2.into_iter() {
                        res.push(tt);
                    }
                }
                None => {}
            },
            _ => {
                res.push(t);
            }
        }
    }
    res
}
