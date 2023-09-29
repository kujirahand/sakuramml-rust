//! lexer

use crate::cursor::TokenCursor;
use crate::runner::calc_length;
use crate::sakura_message::MessageKind;
use crate::song::{Song, SFunction};
use crate::svalue::SValue;
use crate::token::{zen2han, Token, TokenType};

const LEX_MAX_ERROR: usize = 30;

/// prerpcess 
pub fn lex_preprocess(song: &mut Song, cur: &mut TokenCursor) -> bool {
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
        if cur.eq("##") || cur.eq("# ") || cur.eq("#-") { // なんか、みんながよく使っているのでコメントと見なす
            cur.get_token_ch('\n');
            continue;
        }
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
        let ch = cur.get_char();
        if ch == '\n' {
            cur.line += 1;
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
    let mut cur = TokenCursor::from(src);
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
            // space
            ' ' | '\t' | '\r' | '|' | ';' => {}, // @ 空白文字
            // ret
            '\n' => {
                cur.line += 1;
                result.push(Token::new_lineno(cur.line));
            },
            // lower command
            'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b' => result.push(read_note(&mut cur, ch)), // @ ドレミファソラシ c(音長),(ゲート),(音量),(タイミング),(音階)
            'n' => result.push(read_note_n(&mut cur, song)), // @ 番号を指定して発音(例: n36) n(番号),(音長),(ゲート),(音量),(タイミング)
            'r' => result.push(read_rest(&mut cur)),         // @ 休符
            'l' => result.push(read_length(&mut cur)),       // @ 音長の指定(例 l4)
            'o' => result.push(read_octave(&mut cur, song)), // @ 音階の指定(例 o5) 範囲:0-10
            'p' => result.push(read_pitch_bend_small(&mut cur, song)), // @ ピッチベンドの指定 範囲:0-127 (63が中央)
            'q' => result.push(read_qlen(&mut cur, song)), // @ ゲートの指定 (例 q90) 範囲:0-100
            'v' => result.push(read_velocity(&mut cur, song)), // @ ベロシティ音量の指定 範囲:0-127 / v.Random=n
            't' => result.push(read_timing(&mut cur, song)), // @ 発音タイミングの指定 (例 t-1) / t.Random=n
            'y' => result.push(read_cc(&mut cur, song)), // @ コントロールチェンジの指定 (例 y1,100) 範囲:0-127 / y1.onTime(low,high,len)
            // uppwer command
            'A'..='Z' | '_' => {
                cur.prev();
                if cur.eq("End") || cur.eq("END") { // それ移行をコンパイルしない
                    let last_comment = cur.cur2end();
                    cur.next_n(last_comment.len());
                    result.push(Token::new_empty(&last_comment, cur.line));
                    continue;
                }
                result.push(read_upper_command(&mut cur, song))
            },
            '#' => {
                cur.prev();
                if cur.eq("##") || cur.eq("# ") || cur.eq("#-") { // なんかみんなが使っているので一行コメントと見なす
                    cur.get_token_ch('\n');
                    continue;
                }
                result.push(read_upper_command(&mut cur, song))
            },
            // flag
            '@' => result.push(read_voice(&mut cur, song)), // @ 音色の指定 範囲:1-128 (書式) @(no),(Bank_LSB),(Bank_MSB)
            '>' => result.push(Token::new_value(TokenType::OctaveRel, 1)), // @ 音階を1つ上げる
            '<' => result.push(Token::new_value(TokenType::OctaveRel, -1)), // @ 音階を1つ下げる
            ')' => result.push(Token::new_value(TokenType::VelocityRel, song.v_add)), // @ 音量を8つ上げる
            '(' => result.push(Token::new_value(TokenType::VelocityRel, -1 * song.v_add)), // @ 音量を8つ下げる
            // comment
            /*
            "//" => // @ 一行コメント
            "/*" .. "*/" => // @ 範囲コメント
            "##" => // @ 一行コメント
            "# " => // @ 一行コメント
            "#-" => // @ 一行コメント
             */
            '/' => {
                if cur.eq_char('/') {
                    cur.get_token_ch('\n');
                } else if cur.eq_char('*') {
                    cur.get_token_s("*/");
                }
            }
            '[' => result.push(read_loop(&mut cur, song)), // @ ループ開始 (例 [4 cdeg])
            ':' => result.push(Token::new_value(TokenType::LoopBreak, 0)), // @ ループ最終回に脱出 (例　[4 cde:g]e)
            ']' => result.push(Token::new_value(TokenType::LoopEnd, 0)),   // @ ループ終了
            '\'' => result.push(read_harmony_flag(&mut cur, &mut flag_harmony)), // @ 和音 (例 'ceg') 'ceg'(音長),(ゲート)
            '$' => read_def_rhythm_macro(&mut cur, song), // @ リズムマクロ定義 $文字{定義内容}
            '{' => result.push(read_command_div(&mut cur, song, true)), // @ 連符 (例 {ceg}4) {c^d}(音長)
            '`' => result.push(Token::new_value(TokenType::OctaveOnce, 1)), // @ 一度だけ音階を+1する
            '"' => result.push(Token::new_value(TokenType::OctaveOnce, -1)), // @ 一度だけ音階を-1する
            '?' => result.push(Token::new_value(TokenType::PlayFrom, 0)), // @ ここから演奏する (=PLAY_FROM)
            '&' => {}, // @ タイ・スラー(Slurコマンドで動作が変更できる)
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

fn lex_error(cur: &mut TokenCursor, song: &mut Song, msg: &str) {
    if song.get_logs_len() == LEX_MAX_ERROR {
        song.add_log(format!(
            "[ERROR]({}) {}",
            cur.line,
            song.get_message(MessageKind::TooManyErrorsInLexer)
        ));
    } else if song.get_logs_len() < LEX_MAX_ERROR {
        let near = cur.peek_str_n(8).replace('\n', "↵");
        let log = format!(
            "[ERROR]({}) {}: \"{}\" {} \"{}\"",
            cur.line,
            song.get_message(MessageKind::UnknownChar),
            msg,
            song.get_message(MessageKind::Near),
            near
        );
        song.add_log(log);
    }
}

/// read Upper case commands
fn read_upper_command(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let mut cmd = cur.get_word();
    // Systemの場合は"."に続く
    if cmd == "System" || cmd == "SYSTEM" {
        cmd = "System".to_string(); // convert "SYSTEM" to "System"
        if cur.eq_char('.') {
            cur.next();
            cmd.push('.');
        }
        cmd.push_str(&cur.get_word());
    }
    // PlayFromも"."が続く場合がある
    if cmd == "PlayFrom" {
        if cur.eq_char('.') {
            cur.next();
            cmd.push('.');
            cmd.push_str(&cur.get_word());
        }
    }

    // <UPPER_COMMANDS>
    /*
    if cmd == "End" || cmd == "END" { } 
    // @ それ移行をコンパイルしない
    */
    // Track & Channel
    if cmd == "TR" || cmd == "TRACK" || cmd == "Track" {
        // @ トラック変更　TR=番号 範囲:0-
        let v = read_arg_value(cur, song);
        return Token::new(TokenType::Track, 0, vec![v]);
    }
    if cmd == "CH" || cmd == "Channel" {
        // @ チャンネル変更 CH=番号 範囲:1-16
        let v = read_arg_value(cur, song);
        return Token::new(TokenType::Channel, 0, vec![v]);
    }
    if cmd == "TIME" || cmd == "Time" {
        // @ タイム変更 TIME(節:拍:ステップ)
        return read_command_time(cur, song);
    }
    if cmd == "RHYTHM" || cmd == "Rhythm" || cmd == "R" {
        // @ リズムモード
        return read_command_rhythm(cur, song);
    }
    if cmd == "RYTHM" || cmd == "Rythm" {
        // @ リズムモード(v1の綴りミス対処[^^;]) RHYTHM または R と同じ
        return read_command_rhythm(cur, song);
    }
    if cmd == "DIV" || cmd == "Div" {
        // @ 連符 (例 DIV{ceg} )
        return read_command_div(cur, song, false);
    }
    if cmd == "SUB" || cmd == "Sub" || cmd == "S" {
        // @ タイムポインタを戻す (例 SUB{ceg} egb)
        return read_command_sub(cur, song);
    }

    if cmd == "KF" || cmd == "KeyFlag" {
        // @ 臨時記号を設定 - KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note)
        return read_key_flag(cur, song);
    }
    if cmd == "KEY" || cmd == "Key" || cmd == "KeyShift" {
        // @ ノート(cdefgab)のキーをn半音シフトする (例 KEY=3 cde)
        return read_command_key(cur, song);
    }
    if cmd == "TR_KEY" || cmd == "TrackKey" {
        // @ トラック毎、ノート(cdefgab)のキーをn半音シフトする (例 TrackKey=3 cde)
        return read_command_track_key(cur, song);
    }
    if cmd == "INT" || cmd == "Int" {
        // @ 変数を定義 (例 INT TestValue=30)
        return read_def_int(cur, song);
    }
    if cmd == "STR" || cmd == "Str" {
        // @ 文字列変数を定義 (例 STR A={cde})
        return read_def_str(cur, song);
    }
    if cmd == "PLAY" || cmd == "Play" {
        // @ 複数トラックを１度に書き込む (例 PLAY={aa},{bb},{cc})
        return read_play(cur, song);
    }
    if cmd == "PRINT" || cmd == "Print" {
        // @ 文字を出力する (例 PRINT{"cde"} )(例 INT AA=30;PRINT(AA))
        return read_print(cur, song);
    }
    if cmd == "PlayFrom.SysEx" || cmd == "PlayFrom.CtrlChg" {
        // @ 未実装
        let _v = read_arg_value(cur, song);
        println!("not supported : {}", cmd);
    }
    if cmd == "PLAY_FROM" || cmd == "PlayFrom" {
        // @ ここから演奏する　(?と同じ意味)
        return read_playfrom(cur, song);
    }
    if cmd == "System.MeasureShift" {
        // @ 小節番号をシフトする (例 System.MeasureShift(1))
        return read_command_mes_shift(cur, song);
    }
    if cmd == "System.KeyFlag" {
        // @ 臨時記号を設定 - KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note)
        return read_key_flag(cur, song);
    }
    if cmd == "System.TimeBase" || cmd == "TIMEBASE" || cmd == "Timebase" || cmd == "TimeBase" {
        // @ タイムベースを設定 (例 TIMEBASE=96)
        return read_timebase(cur, song);
    }
    if cmd == "TRACK_SYNC" || cmd == "TrackSync" {
        // @ 全てのトラックのタイムポインタを同期する
        return Token::new_value(TokenType::TrackSync, 0);
    }
    if cmd == "SLUR" || cmd == "Slur" {
        // @ タイ・スラー記号(&)の動作を変更する(0:グリッサンド/1:ベンド/2:ゲート/3:アルペジオ)
        let args: SValue = read_arg_value_int_array(cur, song);
        return Token::new(
            TokenType::TieMode,
            0,
            vec![args]);
    }
    if cmd == "System.Include" || cmd == "Include" || cmd == "INCLUDE" {
        // @ 未実装
        cur.skip_space();
        let v = if cur.eq_char('(') {
            cur.get_token_nest('(', ')')
        } else {
            "".to_string()
        };
        return Token::new_empty(&v, cur.line);
    }
    if cmd == "System.vAdd" || cmd == "vAdd" {
        // @ ベロシティの相対変化(と)の変化値を指定する (例 System.vAdd(8))
        return read_v_add(cur, song);
    }
    if cmd == "System.qAdd" || cmd == "qAdd" {
        // @ 未定義
        read_arg_value(cur, song);
        return Token::new_empty("qAdd", cur.line);
    }
    if cmd == "SoundType" || cmd == "SOUND_TYPE" {
        // @ 未実装
        return Token::new(
            TokenType::Empty,
            0,
            read_arg_int_array(cur, song).to_array(),
        );
    }
    // Voice Change
    if cmd == "VOICE" || cmd == "Voice" {
        // @ モジュレーション 範囲: 0-127
        return read_voice(cur, song);
    }

    // controll change
    if cmd == "M" || cmd == "Modulation" {
        // @ モジュレーション 範囲: 0-127
        return read_command_cc(cur, 1, song);
    }
    if cmd == "PT" || cmd == "PortamentoTime" {
        // @ ポルタメント 範囲: 0-127
        return read_command_cc(cur, 5, song);
    }
    if cmd == "V" || cmd == "MainVolume" {
        // @ メインボリューム 範囲: 0-127
        return read_command_cc(cur, 7, song);
    }
    if cmd == "P" || cmd == "Panpot" {
        // @ パンポット 範囲: 0-63-127
        return read_command_cc(cur, 10, song);
    }
    if cmd == "EP" || cmd == "Expression" {
        // @ エクスプレッション音量 範囲: 0-127
        return read_command_cc(cur, 11, song);
    }
    if cmd == "PS" || cmd == "PortamentoSwitch" {
        // @ ポルタメントスイッチ
        return read_command_cc(cur, 65, song);
    }
    if cmd == "REV" || cmd == "Reverb" {
        // @ リバーブ 範囲: 0-127
        return read_command_cc(cur, 91, song);
    }
    if cmd == "CHO" || cmd == "Chorus" {
        // @ コーラス 範囲: 0-127
        return read_command_cc(cur, 93, song);
    }
    if cmd == "VAR" || cmd == "Variation" {
        // @ バリエーション 範囲: 0-127
        return read_command_cc(cur, 94, song);
    }

    if cmd == "PB" || cmd == "PitchBend" {
        // @ ピッチベンドを指定 範囲: -8192~0~8191の範囲
        return read_command_pitch_bend_big(cur, song);
    }
    if cmd == "BR" || cmd == "PitchBendSensitivity" {
        // @ ピッチベンドの範囲を設定 範囲: 0-12半音
        return read_command_rpn(cur, 0, 0, song);
    }
    if cmd == "RPN" {
        // @ RPNを書き込む (例 RPN=0,1,64)
        return read_command_rpn_n(cur, song);
    }
    if cmd == "NRPN" {
        // @ NRPNを書き込む (例 NRPN=1,0x64,10)
        return read_command_nrpn_n(cur, song);
    }
    if cmd == "FineTune" {
        // @ チューニングの微調整 範囲:0-64-127 (-100 - 0 - +99.99セント）
        return read_command_rpn(cur, 0, 1, song);
    }
    if cmd == "CoarseTune" {
        // @ 半音単位のチューニング 範囲:40-64-88 (-24 - 0 - 24半音)
        return read_command_rpn(cur, 0, 2, song);
    }
    if cmd == "VibratoRate" {
        // @ 音色の編集(GS/XG) 範囲: 0-127
        return read_command_nrpn(cur, 1, 8, song);
    }
    if cmd == "VibratoDepth" {
        // @ 音色の編集(GS/XG) 範囲: 0-127
        return read_command_nrpn(cur, 1, 9, song);
    }
    if cmd == "VibratoDelay" {
        // @ 音色の編集(GS/XG) 範囲: 0-127
        return read_command_nrpn(cur, 1, 10, song);
    }
    if cmd == "FilterCutoff" {
        // @ 音色の編集(GS/XG) 範囲: 0-127
        return read_command_nrpn(cur, 1, 0x20, song);
    }
    if cmd == "FilterResonance" {
        // @ 音色の編集(GS/XG) 範囲: 0-127
        return read_command_nrpn(cur, 1, 0x21, song);
    }
    if cmd == "EGAttack" {
        // @ 音色の編集(GS/XG) 範囲: 0-127
        return read_command_nrpn(cur, 1, 0x63, song);
    }
    if cmd == "EGDecay" {
        // @ 音色の編集(GS/XG) 範囲: 0-127
        return read_command_nrpn(cur, 1, 0x64, song);
    }
    if cmd == "EGRelease" {
        // @ 音色の編集(GS/XG) 範囲: 0-127
        return read_command_nrpn(cur, 1, 0x66, song);
    }

    if cmd == "Fadein" || cmd == "FADEIN" {
        // @ 小節数を指定してフェードインする (例: Fadein(1))
        return read_fadein(cur, song, 1);
    }
    if cmd == "Fadeout" || cmd == "FADEOUT" {
        // @ 小節数を指定してフェードアウトする (例: Fadeout(1))
        return read_fadein(cur, song, -1);
    }
    if cmd == "Decresc" || cmd == "DECRESC" {
        // @ デクレッシェンドを表現 (書式) Decresc([[[len],v1],v2]) だんだん小さく。エクスプレッションをlen(ｎ分音符指定で)の間に、v1からv2へ変更する。lenを省略すると全音符の長さになる。
        return read_decres(cur, song, -1);
    }
    if cmd == "Cresc" || cmd == "CRESC" {
        // @ クレッシェンドを表現 (書式) Cresc([[[len],v1],v2]) だんだん大きく。エクスプレッションをlen(ｎ分音符指定で)の間に、v1からv2へ変更する。lenを省略すると全音符の長さになる。
        return read_decres(cur, song, 1);
    }

    // SysEx
    if cmd == "ResetGM" {
        // @ GMリセットを送信
        return Token::new_sysex(vec![0x7E, 0x7F, 0x9, 0x1, 0xF7]);
    }
    if cmd == "ResetGS" {
        // @ GSリセットを送信
        return Token::new_sysex(vec![
            0x41, 0x10, 0x42, 0x12, 0x40, 0x00, 0x7F, 0x00, 0x41, 0xF7,
        ]);
    }
    if cmd == "ResetXG" {
        // @ XGリセットを送信
        return Token::new_sysex(vec![0x43, 0x10, 0x4c, 0x00, 0x00, 0x7e, 0x00, 0xf7]);
    }

    // meta events
    if cmd == "TEMPO" || cmd == "Tempo" || cmd == "T" {
        // @ テンポの指定
        let v = read_arg_value(cur, song);
        return Token::new(TokenType::Tempo, 0, vec![v]);
    }
    if cmd == "TempoChange" {
        // @ テンポを連続で変更する (書式) TempoChange(開始値,終了値, !長さ)
        return read_tempo_change(cur, song);
    }
    if cmd == "TimeSignature" || cmd == "TimeSig" || cmd == "TIMESIG" || cmd == "System.TimeSignature" {
        // @ 拍子の指定
        cur.skip_space();
        if cur.eq_char('=') { cur.next(); }
        cur.skip_space();
        if cur.eq_char('(') { cur.next(); }
        let mut frac = read_arg_value(cur, song);
        cur.skip_space();
        if cur.eq_char(',') { cur.next(); }
        let mut deno = read_arg_value(cur, song);
        cur.skip_space();
        if cur.eq_char(')') { cur.next(); }
        // check error
        if frac.to_i() == 0 || deno.to_i() == 0 {
            frac = SValue::from_i(4);
            deno = SValue::from_i(4);
        }
        return Token::new(TokenType::TimeSignature, 0, vec![frac, deno]);
    }
    if cmd == "MetaText" || cmd == "TEXT" || cmd == "Text" {
        // @ メタテキスト (例 TEXT{"abcd"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 1, vec![v]);
    }
    if cmd == "COPYRIGHT" || cmd == "Copyright" {
        // @ メタテキスト著作権 (例 COPYRIGHT{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 2, vec![v]);
    }
    if cmd == "TRACK_NAME" || cmd == "TrackName" {
        // @ 曲名 (例 TRACK_NAME{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 3, vec![v]);
    }
    if cmd == "InstrumentName" {
        // @ 楽器名 (例 InstrumentName{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 4, vec![v]);
    }
    if cmd == "LYRIC" || cmd == "Lyric" {
        // @ メタテキスト歌詞 (例 LYRIC{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 5, vec![v]);
    }
    if cmd == "MAKER" || cmd == "Marker" {
        // @ マーカー (例 MAKER{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 6, vec![v]);
    }
    if cmd == "CuePoint" {
        // @ キューポイント (例 CuePoint{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 7, vec![v]);
    }
    // SCRIPT
    if cmd == "IF" || cmd == "If" {
        // @ IF文 (書式) IF(条件){ … }ELSE{ … }
        return read_if(cur, song);
    }
    if cmd == "FOR" || cmd == "For" {
        // @ FOR文 (書式) FOR(初期化式; 条件; 増加式){ … }
        return read_for(cur, song);
    }
    if cmd == "WHILE" || cmd == "While" {
        // @ WHILE文 (書式) WHILE(条件){ … }
        return read_while(cur, song);
    }
    if cmd == "EXIT" || cmd == "Exit" || cmd == "BREAK" || cmd == "Break" {
        // @ BREAK文 FOR/WHILEを抜ける
        return Token::new(TokenType::Break, 0, vec![]);
    }
    if cmd == "CONTINUE" || cmd == "Continue" {
        // @ CONTINUE文 FOR/WHILEを続ける
        return Token::new(TokenType::Continue, 0, vec![]);
    }
    if cmd == "RETURN" || cmd == "Return" {
        // @ RETURN(戻り値) 関数を抜ける
        cur.skip_space();
        let values = if cur.eq_char('(') {
            read_args_tokens(cur, song)
        } else {
            vec![Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_i(0)])]
        };
        return Token::new_tokens(TokenType::Return, 0, values);
    }
    if cmd == "RandomSeed" || cmd == "RANDOM_SEED" {
        // @ 乱数の種を設定する (例 RandomSeed=1234)
        let v = read_arg_value(cur, song);
        song.rand_seed = v.to_i() as u32;
        return Token::new(TokenType::SetConfig, 0, vec![
            SValue::from_str("RandomSeed"),
            v
        ]);
    }
    if cmd == "FUNCTION" || cmd == "Function" {
        // @ 関数を定義する (未実装)
        return read_def_user_function(cur, song);
    }
    // </UPPER_COMMANDS>
    
    // check variable
    match check_variables(cur, song, cmd.clone()) {
        Some(res) => return res,
        None => {}
    }
    read_error_cmd(cur, song, &cmd);
    return Token::new_empty(&cmd, cur.line);
}

fn read_def_user_function(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    // get function name
    let func_name = cur.get_word();
    cur.skip_space();
    // get args
    if !cur.eq_char('(') {
        return read_error_cmd(cur, song, "FUNCTION");
    }
    // check args
    song.variables_stack_push();
    let args_str = cur.get_token_nest('(', ')');
     let args: Vec<&str> = args_str.split(",").collect();
     let mut arg_types: Vec<char> = vec![];
     let mut arg_names: Vec<String> = vec![];
    for i in 0..args.len() {
        let name = args[i].trim().to_string();
        if name.len() == 0 { continue; }
        // include space?
        if name.contains(" ") {
            // split string by " "
            let splited: Vec<&str> = name.split(" ").collect();
            let name_s = splited[0].trim();
            let type_s = splited[1].trim();
            // get type
            let mut type_sf = 'I';
            if type_s == "Str" || type_s == "STR" || type_s == "S" { type_sf = 'S'; }
            if type_s == "Int" || type_s == "INT" || type_s == "I" { type_sf = 'I'; }
            if type_s == "Array" || type_s == "ARRAY" || type_s == "A" { type_sf = 'A'; }
            song.variables_insert(name_s, SValue::new());
            arg_types.push(type_sf);
            arg_names.push(name_s.to_string());
        } else {
            // only name
            song.variables_insert(&name, SValue::Int(0));
            arg_types.push('i');
            arg_names.push(name);
        }
    }
    // get body
    cur.skip_space_ret();
    if !cur.eq_char('{') {
        return read_error_cmd(cur, song, "FUNCTION");
    }
    let lineno = cur.line;
    let body_s = cur.get_token_nest('{', '}');
    let body_tok = lex(song, &body_s, lineno);
    song.variables_stack_pop(); // destroy local variables
    // register variables
    let func_val = song.variables_get(&func_name).unwrap_or(&SValue::new()).clone();
    let func_id = match func_val {
        SValue::UserFunc(func_id) => func_id,
        _ => {
            // system error to analyze function in preprocess
            read_error_cmd(cur, song, &format!("(System error) Define Function: {}", func_name));
            0
        }
    };
    // register function to song.functions
    let mut func_obj = SFunction::new(&func_name, body_tok, func_id, lineno);
    func_obj.arg_names = arg_names;
    func_obj.arg_types = arg_types;
    song.functions[func_id] = func_obj;
    Token::new_empty(&format!("DefineFunction::{}", func_name), lineno)
}

fn read_error_cmd(cur: &mut TokenCursor, song: &mut Song, cmd: &str) -> Token {
    let near = cur.peek_str_n(8).replace('\n', "↵");
    song.add_log(format!(
        "[ERROR]({}) {} \"{}\" {} \"{}\"",
        cur.line,
        song.get_message(MessageKind::ScriptSyntaxError),
        cmd,
        song.get_message(MessageKind::Near),
        near,
    ));
    return Token::new_empty("ERROR", cur.line);
}

fn read_error(cur: &mut TokenCursor, song: &mut Song, msg: &str) -> Token {
    let near = cur.peek_str_n(8).replace('\n', "↵");
    song.add_log(format!(
        "[ERROR]({}) {} {} \"{}\"",
        cur.line,
        msg,
        song.get_message(MessageKind::Near),
        near,
    ));
    return Token::new_empty("ERROR", cur.line);
}


fn read_warning(cur: &mut TokenCursor, song: &mut Song, cmd: &str, reason: &str) -> Token {
    let near = cur.peek_str_n(8).replace('\n', "↵");
    song.add_log(format!(
        "[WARN]({}) {} \"{}\" {} : {} \"{}\"",
        cur.line,
        song.get_message(MessageKind::ScriptSyntaxWarning),
        cmd,
        reason,
        song.get_message(MessageKind::Near),
        near,
    ));
    return Token::new_empty("ERROR", cur.line);
}


// --- lex calc script ---
const LEX_VALUE: isize = 1;
const LEX_NOT: isize = 2;
const LEX_PAREN_L: isize = 3;
const LEX_PAREN_R: isize = 4;
const LEX_MUL_DIV: isize = 20;
const LEX_PLUS_MINUS: isize = 30;
const LEX_COMPARE: isize = 40;
const LEX_OR_AND: isize = 50;

fn is_operator(c: isize) -> bool {
    match c {
        LEX_COMPARE | LEX_MUL_DIV | LEX_PLUS_MINUS | LEX_OR_AND => true,
        _ => false,
    }
}

fn read_calc_can_continue(cur: &mut TokenCursor, parent_level: i32) -> bool {
    if parent_level > 0 { return true; }
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        '+' | '-' | '*' | '/' | '|' | '&' | '%' | '≠' | '=' | '>' | '<' | '≧' | '≦' | '!' => true,
        _ => false,
    }
}

fn read_calc_token(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    let tokens = read_calc(cur, song);
    Token::new_tokens_lineno(TokenType::Tokens, 0, tokens, lineno)
}

fn read_calc(cur: &mut TokenCursor, song: &mut Song) -> Vec<Token> {
    let mut tokens = vec![];
    let mut paren_level:i32 = 0;
    while !cur.is_eos() {
        cur.skip_space();
        let ch = zen2han(cur.peek().unwrap_or('\0'));
        match ch {
            // blank
            '\t' => { cur.next(); }, // comment
            // break chars
            '\n' => { cur.line += 1; break; },
            ';' => break,
            ',' => break,
            '(' => {
                tokens.push(Token::new(TokenType::Calc, LEX_PAREN_L, vec![]));
                paren_level += 1;
                cur.next();
            },
            ')' => {
                if paren_level == 0 { break; }
                paren_level -=1;
                tokens.push(Token::new(TokenType::Calc, LEX_PAREN_R, vec![]));
                cur.next();
                if !read_calc_can_continue(cur, paren_level) { break; }
            },
            // value
            '0'..='9' => {
                let num = cur.get_int(0);
                tokens.push(Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_i(num)]));
                if !read_calc_can_continue(cur, paren_level) { break; }
            },
            'A'..='Z' | '_' | '#' | 'a'..='z' => {
                let mut tok = Token::new(TokenType::Value, LEX_VALUE, vec![]);
                let varname = cur.get_word();
                let varname_flag = format!("={}", varname);
                // println!("read_calc:{}", varname);
                tok.tag = 0;
                if cur.eq_char('(') {
                    // function call
                    let arg_lineno = cur.line;
                    let arg_str = cur.get_token_nest('(', ')');
                    // println!("read_calc_args={:?}", arg_str);
                    let arg_tokens = lex_calc(song, &arg_str, arg_lineno);
                    tok.children = Some(arg_tokens);
                    tok.tag = 1; // FUNCTION
                    tok.data.push(SValue::from_s(varname.clone()));
                    // is user function?
                    let func_val = song.variables_get(&varname);
                    if func_val.is_some() {
                        let func_id: SValue = func_val.unwrap_or(&SValue::from_i(0)).clone();
                        tok.ttype = TokenType::CallUserFunction;
                        tok.tag = func_id.to_i();
                    }
                    tokens.push(tok);
                } else {
                    // inc & dec
                    if cur.eq("++") {
                        tok.ttype = TokenType::ValueInc;
                        tok.value = 1;
                        tok.data.push(SValue::from_s(varname));
                        tokens.push(tok);
                    } else if cur.eq("--") {
                        tok.ttype = TokenType::ValueInc;
                        tok.value = -1;
                        tok.data.push(SValue::from_s(varname));
                        tokens.push(tok);
                    } else {
                        // ref variable
                        tok.data.push(SValue::from_s(varname_flag));
                        tokens.push(tok);
                    }
                }
                if !read_calc_can_continue(cur, paren_level) { break; }
            },
            '"' => {
                cur.next(); // skip "
                let str_s = cur.get_token_to_double_quote();
                let tok = Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_s(str_s)]);
                tokens.push(tok);
                if !read_calc_can_continue(cur, paren_level) { break; }
            },
            '{' => {
                let str_s = cur.get_token_nest('{', '}');
                let tok = Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_s(str_s)]);
                tokens.push(tok);
                if !read_calc_can_continue(cur, paren_level) { break; }
            },
            // value or operator
            '-' => {
                cur.next(); // skip '-'
                let ch2 = cur.peek().unwrap_or('\0');
                if ch2 >= '0' && ch2 <= '9' {
                    let num = cur.get_int(0) * -1;
                    tokens.push(Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_i(num)]));
                    if !read_calc_can_continue(cur, paren_level) { break; }
                } else {
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_PLUS_MINUS, ch as isize));
                }
            },
            // operator
            '/' => {
                if cur.eq("//") { break; }
                if cur.eq("/*") {
                    cur.get_token_s("*/");
                    continue;
                }
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_MUL_DIV, ch as isize));
                cur.next();
            },
            '*' | '%' => {
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_MUL_DIV, ch as isize));
                cur.next();
            },
            '+' => {
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_PLUS_MINUS, ch as isize));
                cur.next();
            },
            '|' => {
                if cur.eq("||") {
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_OR_AND, ch as isize));
                }
                cur.next();
            },
            '&' => {
                cur.next_n(if cur.eq("&&") { 2 } else { 1 });
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_OR_AND, ch as isize));
            },
            '≠' => {
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≠' as isize));
                cur.next();
            }
            '≧' => {
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≧' as isize));
                cur.next();
            }
            '≦' => {
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≦' as isize));
                cur.next();
            }
            '=' => {
                cur.next_n(if cur.eq("==") { 2 } else { 1 });
                tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, ch as isize));
            },
            '!' => {
                if cur.eq("!=") {
                    cur.next_n(2);
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≠' as isize));
                } else {
                    cur.next();
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_NOT, ch as isize));
                }
            },
            '>' | '<' => {
                if cur.eq("<>") | cur.eq("><") {
                    cur.next_n(2);
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≠' as isize));
                }
                else if cur.eq(">=") {
                    cur.next_n(2);
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≧' as isize));
                }
                else if cur.eq("<=") {
                    cur.next_n(2);
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '≦' as isize));
                }
                else if cur.eq("<") {
                    cur.next();
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '<' as isize));
                }
                else if cur.eq(">") {
                    cur.next();
                    tokens.push(Token::new_value_tag(TokenType::Calc, LEX_COMPARE, '>' as isize));
                }
            },
            _ => {
                // 計算時に使えない文字列があれば抜ける
                // let msg = format!("{}", ch);
                // lex_error(cur, song, &msg);
                break;
            }
        }
    }
    // convert to postfix
    infix_to_postfix(cur, song, tokens)
}

fn infix_to_postfix(cur: &mut TokenCursor, song: &mut Song, tokens: Vec<Token>) -> Vec<Token> {
    let mut result = vec![];
    let mut stack:Vec<Token> = vec![];
    for token in tokens.into_iter() {
        if token.value == LEX_VALUE {
            result.push(token);
            continue;
        }
        if is_operator(token.value) {
            while let Some(top) = stack.last().cloned() {
                if top.value == LEX_PAREN_L || top.value > token.value {
                    break;
                }
                result.push(stack.pop().unwrap());
            }
            stack.push(token);
            continue;
        }
        else if token.value == LEX_PAREN_L {
            stack.push(token);
            continue;
        }
        else if token.value == LEX_PAREN_R {
            while let Some(top) = stack.last().cloned() {
                if top.value == LEX_PAREN_L {
                    break;
                }
                result.push(stack.pop().unwrap());
            }
            stack.pop();
            continue;
        } else {
            let msg = format!("Invalid token type: {:?}", token.value);
            lex_error(cur, song, &msg);
            result.clear();
            return vec![];
        }
    }
    while let Some(op) = stack.pop() {
        if op.value == LEX_PAREN_L {
            let msg = format!("Unmatched parenthesis");
            lex_error(cur, song, &msg);
            result.clear();
            return vec![];
        }
        result.push(op);
    }
    result
}

/// lex calc script
fn lex_calc(song: &mut Song, src: &str, lineno: isize) -> Vec<Token> {
    let mut cur = TokenCursor::from(src);
    cur.line = lineno;
    let mut result = vec![];
    while !cur.is_eos() {
        let lastpos = cur.index;
        let tokens = read_calc(&mut cur, song);
        result.extend(tokens);
        if cur.peek().unwrap_or('\0') == ',' {
            cur.next();
            continue;
        }
        if lastpos == cur.index {
            let ch = cur.get_char();
            println!("[skip]({}) {}", cur.line, ch);
        }
    }
    result
}

fn read_while(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    cur.skip_space();
    if !cur.eq_char('(') {
        read_error_cmd(cur, song, "WHILE");
        return Token::new_empty("ERROR:WHILE", cur.line);
    }
    // read condition
    if !cur.eq_char('(') {
        read_error_cmd(cur, song, "WHILE");
        return Token::new_empty("ERROR:WHILE", cur.line);
    }
    let cond_s = cur.get_token_nest('(', ')');
    let cond_tok = lex_calc(song, &cond_s, lineno);
    cur.skip_space();
    // read body
    let body_s = cur.get_token_nest('{', '}');
    let body_tok = lex(song, &body_s, lineno);
    // while
    let while_tok = Token::new_tokens_lineno(TokenType::While, 0, vec![
        Token::new_tokens(TokenType::Tokens, 0, cond_tok),
        Token::new_tokens(TokenType::Tokens, 0, body_tok),
    ], lineno);
    while_tok
}

fn read_for(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    cur.skip_space();
    if !cur.eq_char('(') {
        read_error_cmd(cur, song, "FOR");
        return Token::new_empty("ERROR:FOR", cur.line);
    }
    // read init
    cur.next(); // skip '('
    let init_s = cur.get_token_ch(';').trim().to_string();
    let cond_s = cur.get_token_ch(';');
    let inc_s = cur.get_token_ch(')');
    cur.skip_space();
    if !cur.eq_char('{') {
        read_error_cmd(cur, song, "FOR");
        return Token::new_empty("ERROR:FOR", cur.line);
    }
    let body_s = cur.get_token_nest('{', '}');
    // もし、String型のinit_sが"Int "から始まっていなければ"Int "を足す
    let init_s = if init_s == "" || (init_s.starts_with("Int ") || init_s.starts_with("INT "))  {
        init_s
    } else {
        format!("Int {}", init_s)
    };
    let init_tok = lex(song, &init_s, lineno);
    let cond_tok = lex_calc(song, &cond_s, lineno);
    let inc_tok = lex_calc(song, &inc_s, lineno);
    let body_tok = lex(song, &body_s, lineno);
    let for_tok = Token::new_tokens_lineno(TokenType::For, 0, vec![
        Token::new_tokens(TokenType::Tokens, 0, init_tok),
        Token::new_tokens(TokenType::Tokens, 0, cond_tok),
        Token::new_tokens(TokenType::Tokens, 0, inc_tok),
        Token::new_tokens(TokenType::Tokens, 0, body_tok),
    ], lineno);
    for_tok
}

fn read_if(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    // read condition
    cur.skip_space();
    if !cur.eq_char('(') {
        read_error_cmd(cur, song, "IF");
        return Token::new_empty("ERROR:IF", cur.line);
    }
    let cond = cur.get_token_nest('(', ')');
    let cond_tok = lex_calc(song, &cond, cur.line);
    cur.skip_space();
    if !cur.eq_char('{') {
        read_error_cmd(cur, song, "IF");
        return Token::new_empty("ERROR:IF", cur.line);
    }
    // read then block
    let then_s = cur.get_token_nest('{', '}');
    let then_tok = lex(song, &then_s, cur.line);
    let mut else_tok = vec![];
    cur.skip_space_ret();
    // read else block
    if cur.eq("ELSE") || cur.eq("Else") {
        let else_lineno = cur.line;
        cur.next_n(4); // skip "ELSE"
        cur.skip_space();
        if !cur.eq_char('{') {
            read_error_cmd(cur, song, "IF");
            return Token::new_empty("ERROR:IF:ELSE", else_lineno);
        }
        let else_s = cur.get_token_nest('{', '}');
        else_tok = lex(song, &else_s, else_lineno);
    }
    // println!("cond: {:?}", cond_tok);
    // token
    Token::new_tokens_lineno(TokenType::If, 0, vec![
        Token::new_tokens(TokenType::Tokens, 0, cond_tok),
        Token::new_tokens(TokenType::Tokens, 0, then_tok),
        Token::new_tokens(TokenType::Tokens, 0, else_tok),
    ], lineno)
}

fn check_variables(cur: &mut TokenCursor, song: &mut Song, cmd: String) -> Option<Token> {
    // increment variable?
    if cur.eq("++") {
        cur.next_n(2);
        return Some(Token::new(TokenType::ValueInc, 1, vec![SValue::from_s(cmd)]));
    }
    if cur.eq("--") {
        cur.next_n(2);
        return Some(Token::new(TokenType::ValueInc, -1, vec![SValue::from_s(cmd)]));
    }
    // let?
    cur.skip_space();
    if cur.eq("=") {
        cur.next();
        cur.skip_space();
        // check reserved words
        if song.reserved_words.contains_key(&cmd) {
            let msg = format!("{}: \"{}\"", song.get_message(MessageKind::ErrorDefineVariableIsReserved), cmd);
            return Some(read_error(cur, song, &msg));
        }
        // let str
        if cur.eq_char('{') {
            let body = cur.get_token_nest('{', '}');
            song.variables_insert(&cmd, SValue::from_str_and_tag(&body, cur.line));
            return Some(Token::new_empty("DefStr", cur.line));
        }
        // let calc
        let body_tokens = read_calc(cur, song);
        let tok = Token::new_data_tokens(
            TokenType::LetVar, 0, 
            vec![SValue::from_str(&cmd)],
            vec![Token::new_tokens(TokenType::Tokens, 0, body_tokens)]);
        song.variables_insert(&cmd, SValue::None);
        return Some(tok);
    }

    // variables?
    match song.variables_get(&cmd) {
        Some(sval) => {
            // get variable
            return Some(read_variables(cur, song, &cmd, sval.clone()));
        }
        None => {}
    };
    None
}

fn read_variables(cur: &mut TokenCursor, song: &mut Song, name: &str, sval: SValue) -> Token {
    match sval {
        SValue::Str(_src_org, _line_no) => {
            // replace macro?
            cur.skip_space();
            if cur.eq_char('(') {
                let args = read_args_tokens(cur, song);
                let mut tok = Token::new_tokens(TokenType::Value, LEX_VALUE, args);
                tok.tag = 1; // Macro
                tok.data = vec![SValue::from_s(format!("={}", name))];
                return tok;
            } else {
                let tok = Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_s(format!("={}", name))]);
                return tok;
            }
        }
        SValue::UserFunc(func_id) => { return read_call_function(cur, song, func_id); },
        _ => { return Token::new_empty(&format!("Could not execute: {}", name), cur.line); }
    }
}

fn read_call_function(cur: &mut TokenCursor, song: &mut Song, func_id: usize) -> Token {
    cur.skip_space();
    let args: Vec<Token> = read_args_tokens(cur, song);
    let mut call_func_tok = Token::new(TokenType::CallUserFunction, func_id as isize, vec![]);
    call_func_tok.children = Some(args);
    call_func_tok
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

fn read_arg_value(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        'A'..='Z' | '_' => {
            let var_name = cur.get_word();
            SValue::from_s(format!("={}", var_name)) // ref: variable
        }
        '!' => {
            // timebase length
            cur.next(); // skip !
            let len_str = cur.get_note_length();
            SValue::from_i(calc_length(&len_str, song.timebase, song.timebase))
        }
        '-' | '0'..='9' => {
            let v = cur.get_int(0);
            SValue::from_i(v)
        }
        '=' => {
            cur.next(); // skip =
            read_arg_value(cur, song)
        }
        '(' => {
            cur.next(); // skip (
            let v = read_arg_value(cur, song);
            cur.skip_space();
            if cur.eq_char(')') {
                cur.next();
            }
            v
        }
        '{' => {
            let s = cur.get_token_nest('{', '}');
            SValue::from_s(s)
        }
        _ => SValue::None,
    }
}

fn read_arg_value_int_array(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    let mut a: Vec<isize> = vec![];
    loop {
        cur.skip_space();
        let v = read_arg_value(cur, song);
        a.push(v.to_i());
        cur.skip_space();
        if !cur.eq_char(',') {
            break;
        }
        cur.next(); // skip ,
    }
    SValue::from_int_array(a)
}

fn read_arg_int_array(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        '(' => {
            cur.next(); // skip '('
            let sv = read_arg_value_int_array(cur, song);
            cur.skip_space();
            if cur.peek_n(0) == ')' {
                cur.next();
            }
            return sv;
        }
        '=' => {
            cur.next();
            read_arg_value_int_array(cur, song)
        }
        _ => SValue::None,
    }
}

fn read_args_vec(cur: &mut TokenCursor, song: &mut Song) -> Vec<SValue> {
    cur.skip_space();
    let skip_paren = if cur.eq_char('(') {
        cur.next(); // skip '('
        true
    } else { false };
    let mut a: Vec<SValue> = vec![];
    loop {
        cur.skip_space();
        let v = read_arg_value(cur, song);
        a.push(v);
        cur.skip_space();
        if !cur.eq_char(',') {
            break;
        }
        cur.next(); // skip ,
    }
    if skip_paren {
        cur.skip_space();
        if cur.eq_char(')') {
            cur.next(); // skip ')'
        } else {
            song.add_log(format!("[ERROR]({}) {}", cur.line, song.get_message(MessageKind::MissingParenthesis)));
        }
    }
    a
}

fn read_args_tokens(cur: &mut TokenCursor, song: &mut Song) -> Vec<Token> {
    cur.skip_space();
    let skip_paren = if cur.eq_char('(') {
        cur.next(); // skip '('
        true
    } else { false };

    let mut tokens = vec![];
    loop {
        cur.skip_space();
        let sub_tokens = read_calc(cur, song);
        tokens.push(Token::new_tokens(TokenType::Tokens, 0, sub_tokens));
        
        cur.skip_space();
        if !cur.eq_char(',') {
            break;
        }
        cur.next(); // skip ,
    }
    if skip_paren {
        cur.skip_space();
        if cur.eq_char(')') {
            cur.next(); // skip ')'
        } else {
            song.add_log(format!("[ERROR]({}) {}", cur.line, song.get_message(MessageKind::MissingParenthesis)));
        }
    }
    tokens
}


fn read_arg_str(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        '(' => {
            cur.next(); // skip '('
            let sv = read_arg_value(cur, song);
            cur.skip_space();
            if cur.peek_n(0) == ')' {
                cur.next();
            }
            return sv;
        }
        '=' => {
            cur.next();
            read_arg_value(cur, song)
        }
        '{' => read_arg_value(cur, song),
        _ => SValue::None,
    }
}

fn read_harmony_flag(cur: &mut TokenCursor, flag_harmony: &mut bool) -> Token {
    // begin
    if !*flag_harmony {
        *flag_harmony = true;
        return Token::new(TokenType::HarmonyBegin, 0, vec![]);
    }
    // end
    *flag_harmony = false;
    let mut len_s = SValue::None;
    let mut qlen = SValue::from_i(-1);
    let mut vel = SValue::None;
    if cur.is_numeric() || cur.eq_char('^') {
        len_s = SValue::from_s(cur.get_note_length());
    }
    cur.skip_space();
    if cur.eq_char(',') {
        cur.next();
        qlen = SValue::from_i(cur.get_int(-1));
        if cur.eq_char(',') {
            cur.next();
            vel = SValue::from_i(cur.get_int(-1));
        }
    }
    Token::new(TokenType::HarmonyEnd, 0, vec![len_s, qlen, vel])
}

fn scan_chars(s: &str, c: char) -> isize {
    let mut cnt = 0;
    for ch in s.chars() {
        if ch == c {
            cnt += 1;
        }
    }
    cnt
}

fn read_timebase(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let v = read_arg_value(cur, song);
    song.timebase = v.to_i();
    if song.timebase <= 48 {
        song.timebase = 48;
    }
    Token::new_empty(&format!("TIMEBASE={}", v.to_i()), cur.line)
}

fn read_v_add(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let v = read_arg_value(cur, song);
    song.v_add = v.to_i();
    Token::new_empty("vAdd", cur.line)
}

fn read_key_flag(cur: &mut TokenCursor, _song: &mut Song) -> Token {
    let mut flag = 1;
    let mut key_flag = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let key_flag_index_a = [0, 2, 4, 5, 7, 9, 11];
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    cur.skip_space();
    // flag
    match cur.peek_n(0) {
        '+' | '#' => {
            cur.next();
            flag = 1;
        }
        '-' => {
            cur.next();
            flag = -1;
        }
        _ => {}
    }
    // check note
    cur.skip_space();
    if cur.eq_char('(') {
        cur.next();
    }
    let mut idx = 0;
    while !cur.is_eos() {
        cur.skip_space();
        // numeric value
        if cur.eq_char('0') || cur.eq_char('1') || cur.eq("-1") {
            let v = cur.get_int(0);
            key_flag[key_flag_index_a[idx]] = v;
            idx += 1;
            if idx >= 8 {
                break;
            }
            cur.skip_space();
            if cur.eq_char(',') {
                cur.next();
            }
            continue;
        }
        // note name value
        match cur.peek_n(0) {
            'c' => {
                cur.next();
                key_flag[0] = flag;
            }
            'd' => {
                cur.next();
                key_flag[2] = flag;
            }
            'e' => {
                cur.next();
                key_flag[4] = flag;
            }
            'f' => {
                cur.next();
                key_flag[5] = flag;
            }
            'g' => {
                cur.next();
                key_flag[7] = flag;
            }
            'a' => {
                cur.next();
                key_flag[9] = flag;
            }
            'b' => {
                cur.next();
                key_flag[11] = flag;
            }
            _ => break,
        }
    }
    cur.skip_space();
    if cur.eq_char(')') {
        cur.next();
    }
    // token
    let tok = Token::new(
        TokenType::KeyFlag,
        0,
        vec![SValue::from_int_array(key_flag)],
    );
    tok
}

fn read_def_int(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let var_name = cur.get_word();
    if var_name == "" {
        song.add_log(format!(
            "[ERROR]({}): INT command should use Upper case like \"Test\".",
            cur.line
        ));
        return Token::new_empty("Failed to def INT", cur.line);
    }
    // check reserved words
    if song.reserved_words.contains_key(&var_name) {
        let msg = format!("{}: \"{}\"", song.get_message(MessageKind::ErrorDefineVariableIsReserved), var_name);
        read_error(cur, song, &msg);
        return Token::new_empty("Failed to def INT", cur.line);
    }
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    // get line
    let val_tokens = read_calc(cur, song);
    // token
    let tok = Token::new_data_tokens(
        TokenType::DefInt,
        0,
        vec![SValue::from_s(var_name)],
        val_tokens,
    );
    tok
}

fn read_playfrom(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    if cur.eq_char('(') {
        // time
        let tok = read_command_time(cur, song);
        let pf = Token::new_value(TokenType::PlayFrom, 0);
        return Token::new_tokens(TokenType::Tokens, 0, vec![tok, pf]);
    }
    Token::new_value(TokenType::PlayFrom, 0)
}

fn read_print(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    cur.skip_space();
    let tokens = read_args_tokens(cur, song);
    Token::new_tokens_lineno(TokenType::Print, 0, tokens, lineno)
}

fn read_play(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    let mut tokens: Vec<Token> = vec![
        Token::new_lineno(lineno), // set default lineno
    ];
    let mut track_no = 1;
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    cur.skip_space();
    if cur.eq_char('(') {
        cur.next();
    }
    loop {
        let tt = lex(song, &format!("TR={}", track_no), cur.line);
        for t in tt.into_iter() {
            tokens.push(t);
        }
        cur.skip_space();
        match cur.peek_n(0) {
            'A'..='Z' | '_' | '#' => {
                let name = cur.get_word();
                match song.variables_get(&name) {
                    None => {}
                    Some(sv) => {
                        let (src, lineno) = sv.get_str_and_tag();
                        let tt = lex(song, &src, lineno);
                        for t in tt.into_iter() {
                            tokens.push(t);
                        }
                    }
                }
            },
            '{' => {
                let src = cur.get_token_nest('{', '}');
                let tt = lex(song, &src, cur.line);
                for t in tt.into_iter() {
                    tokens.push(t);
                }
            },
            '"' => { // (option) for N88-BASIC users
                cur.next();
                let src = cur.get_token_to_double_quote();
                let tt = lex(song, &src, cur.line);
                for t in tt.into_iter() {
                    tokens.push(t);
                }
            },
            _ => break,
        }
        cur.skip_space();
        if cur.eq_char(',') {
            cur.next(); // skip ,
        } else {
            break;
        }
        track_no += 1;
    }
    if cur.eq_char(')') {
        cur.next();
    }
    let tokens_tok = Token::new_tokens_lineno(TokenType::Tokens, 0, tokens, lineno);
    tokens_tok
}

fn read_def_str(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let var_name = cur.get_word();
    if var_name == "" {
        song.add_log(format!(
            "[ERROR]({}): STR command should use Upper case like \"Test\"",
            cur.line
        ));
        return Token::new_empty("Failed to def STR", cur.line);
    }
    // check reserved words
    if song.reserved_words.contains_key(&var_name) {
        let msg = format!("{}: \"{}\"", song.get_message(MessageKind::ErrorDefineVariableIsReserved), var_name);
        read_error(cur, song, &msg);
        return Token::new_empty("Failed to def STR", cur.line);
    }
    cur.skip_space();
    let line_no = cur.line;
    let mut init_value_tokens = vec![];
    let mut data_str = "".to_string();
    if cur.eq_char('=') {
        cur.next();
        cur.skip_space();
        let ch = cur.peek().unwrap_or('\0');
        if ch == '"' && ch == '{' {
            // get normal string
            data_str = if ch == '{' { cur.get_token_nest('{', '}') } else { cur.next(); cur.get_token_ch(ch) };
            data_str = format!("{}{}{}", '{', data_str, '}');
            init_value_tokens = lex_calc(song, &data_str, line_no);
        } else {
            // get calc
            let tok = read_calc_token(cur, song);
            init_value_tokens = vec![tok];
        }
    }
    let var_value = SValue::from_str_and_tag(&data_str, line_no);
    let mut tok = Token::new_tokens(TokenType::DefStr, 0, init_value_tokens);
    tok.data = vec![SValue::from_s(var_name.clone())];
    song.variables_insert(&var_name, var_value);
    tok
}

fn read_command_sub(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let block = cur.get_token_nest('{', '}');
    let tokens = lex(song, &block, cur.line);
    let mut tok = Token::new(TokenType::Sub, 0, vec![]);
    tok.children = Some(tokens);
    tok
}

fn read_command_key(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    let arg_token = read_calc_token(cur, song);
    let tok = Token::new_tokens_lineno(TokenType::KeyShift, 0, vec![arg_token], lineno);
    tok
}
fn read_command_track_key(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    let arg_token = read_calc_token(cur, song);
    let tok = Token::new_tokens_lineno(TokenType::TrackKey, 0, vec![arg_token], lineno);
    tok
}

fn read_command_div(cur: &mut TokenCursor, song: &mut Song, need2back: bool) -> Token {
    // is 1char command
    if need2back {
        cur.prev();
    } else {
        cur.skip_space();
    }
    let block = cur.get_token_nest('{', '}');
    let len_s = cur.get_note_length();
    let tokens = lex(song, &block, cur.line);
    // count note
    let mut cnt = 0;
    for t in tokens.iter() {
        match t.ttype {
            TokenType::Note => {
                cnt += 1;
                cnt += scan_chars(&t.data[1].to_s(), '^');
            }
            TokenType::NoteN => {
                cnt += 1;
                cnt += scan_chars(&t.data[1].to_s(), '^');
            }
            TokenType::Div => {
                cnt += 1;
                cnt += scan_chars(&t.data[0].to_s(), '^');
            }
            TokenType::Rest => {
                cnt += 1;
                cnt += scan_chars(&t.data[0].to_s(), '^');
            }
            _ => {}
        }
    }
    let mut tok = Token::new(TokenType::Div, cnt, vec![SValue::from_s(len_s)]);
    tok.children = Some(tokens);
    tok
}

fn read_command_rhythm(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let mut result = String::new();
    cur.skip_space();
    let line_start = cur.line;
    let block = cur.get_token_nest('{', '}');
    // extract macro
    let mut macro_cur = TokenCursor::from(&block);
    macro_cur.line = line_start;
    while !macro_cur.is_eos() {
        if macro_cur.eq("Sub") || macro_cur.eq("SUB") {
            result.push_str("SUB");
            macro_cur.index += 3;
            continue;
        }
        let ch = macro_cur.get_char();
        match ch {
            '(' => {
                // 丸カッコの中は置換しない
                macro_cur.prev();
                let src = macro_cur.get_token_nest('(', ')');
                result.push_str(&src);
            }
            '\u{0040}'..='\u{007f}' => {
                let m = &song.rhthm_macro[ch as usize - 0x40];
                if m == "" {
                    result.push(ch);
                } else {
                    result.push_str(m);
                }
            }
            _ => {
                result.push(ch);
            }
        }
    }
    let mut t = Token::new_value(TokenType::Tokens, 0);
    t.children = Some(lex(song, &result, cur.line));
    t
}

fn read_def_rhythm_macro(cur: &mut TokenCursor, song: &mut Song) {
    let ch = cur.get_char(); // macro char
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    cur.skip_space();
    let s = cur.get_token_nest('{', '}');
    if 0x40 <= ch as u8 && ch as u8 <= 0x7F {
        song.rhthm_macro[ch as usize - 0x40] = s;
    } else {
        song.add_log(format!(
            "[ERROR]({}) could not define Rhythm macro '{}' ",
            cur.line, ch
        ));
    }
}

fn read_command_time(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    cur.skip_space();
    if cur.eq_char('(') {
        cur.next();
    }

    let v1 = read_arg_value(cur, song);
    cur.skip_space();
    if cur.eq_char(':') {
        cur.next();
    }
    let v2 = read_arg_value(cur, song);
    cur.skip_space();
    if cur.eq_char(':') {
        cur.next();
    }
    let v3 = read_arg_value(cur, song);
    cur.skip_space();
    if cur.eq_char(')') {
        cur.next();
    }

    return Token::new(TokenType::Time, 0, vec![v1, v2, v3]);
}

fn read_command_mes_shift(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let v = read_arg_value(cur, song);
    return Token::new(TokenType::MeasureShift, 0, vec![v]);
}

fn read_tempo_change(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let ia = read_arg_int_array(cur, song);
    return Token::new(TokenType::TempoChange, 0, vec![ia]);
}

fn read_fadein(cur: &mut TokenCursor, song: &mut Song, dir: isize) -> Token {
    let arg = read_arg_value(cur, song);
    let ia = if dir >= 1 {
        SValue::from_int_array(vec![0, 127, song.timebase * 4 * arg.to_i()])
    } else {
        SValue::from_int_array(vec![127, 0, song.timebase * 4 * arg.to_i()])
    };
    return Token::new(TokenType::CConTime, 11, vec![ia]);
}

fn read_decres(cur: &mut TokenCursor, song: &mut Song, dir: isize) -> Token {
    let mut v1 = SValue::from_i(if dir < 0 { 127 } else {  40 });
    let mut v2 = SValue::from_i(if dir < 0 {  40 } else { 127 });
    // skip =
    cur.skip_space();
    if cur.eq_char('=') { cur.next(); }
    // length
    let len_s = cur.get_note_length();
    cur.skip_space();
    if cur.eq_char(',') {
        cur.next(); cur.skip_space();
        v1 = read_arg_value(cur, song);
        cur.skip_space();
        if cur.eq_char(',') {
            cur.next(); cur.skip_space();
            v2 = read_arg_value(cur, song);
        }
    }
    return Token::new(TokenType::Decresc, 0, vec![
        SValue::from_s(len_s), v1, v2
    ]);
}

fn read_command_cc(cur: &mut TokenCursor, no: isize, song: &mut Song) -> Token {
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        if cmd == "onTime" || cmd == "T" {
            let ia = read_arg_int_array(cur, song);
            return Token::new(TokenType::CConTime, no, vec![ia]);
        } else if cmd == "Frequency" {
            let a = read_arg_value(cur, song);
            return Token::new(TokenType::CConTimeFreq, 0, vec![a]);
        } else if cmd == "onNoteWave" || cmd == "W" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            song.add_log(format!("[WARN]({}) not supported : onNoteWave", cur.line));
            return Token::new_empty("not supported : onNoteWave", cur.line);
        } else if cmd == "onNoteWaveEx" || cmd == "WE" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            song.add_log(format!("[WARN]({}) not supported : onNoteWaveEx", cur.line));
            return Token::new_empty("not supported : onNoteWave", cur.line);
        } else if cmd == "onCycle" || cmd == "C" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            song.add_log(format!("[WARN]({}) not supported : onCycle", cur.line));
            return Token::new_empty("not supported : onCycle", cur.line);
        }
    }
    if cur.eq_char('=') { cur.next(); }
    let arg_token1 = read_calc_token(cur, song);
    return Token::new_tokens(TokenType::CtrlChange, no, vec![arg_token1]);
}

fn read_command_rpn(cur: &mut TokenCursor, msb: isize, lsb: isize, song: &mut Song) -> Token {
    let val = read_arg_value(cur, song);
    let mut tokens = Token::new(TokenType::Tokens, 0, vec![]);
    tokens.children = Some(vec![
        Token::new_tokens(
            TokenType::CtrlChange,
            101,
            vec![Token::new(TokenType::Value, 0, vec![SValue::from_i(msb)])]
        ),
        Token::new_tokens(
            TokenType::CtrlChange,
            100,
            vec![Token::new(TokenType::Value, 0, vec![SValue::from_i(lsb)])]
        ),
        Token::new_tokens(
            TokenType::CtrlChange,
            6,
            vec![Token::new(TokenType::Value, 0, vec![val])]
        ),
    ]);
    tokens
}

fn read_command_rpn_n(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let a = read_args_vec(cur, song);
    if a.len() < 3 {
        song.add_log(format!("[ERROR]({}): RPN not enough arguments", cur.line));
        return Token::new_empty("RPN error", cur.line);
    }
    let msb = a[0].clone();
    let lsb = a[1].clone();
    let val = a[2].clone();
    let mut tokens = Token::new(TokenType::Tokens, 0, vec![]);
    tokens.children = Some(vec![
        Token::new_tokens(
            TokenType::CtrlChange,
            101,
            vec![Token::new(TokenType::Value, 0, vec![msb])]
        ),
        Token::new_tokens(
            TokenType::CtrlChange,
            100,
            vec![Token::new(TokenType::Value, 0, vec![lsb])]
        ),
        Token::new_tokens(
            TokenType::CtrlChange,
            6,
            vec![Token::new(TokenType::Value, 0, vec![val])]
        ),
    ]);
    tokens
}

fn read_command_nrpn(cur: &mut TokenCursor, msb: isize, lsb: isize, song: &mut Song) -> Token {
    let msb = SValue::from_i(msb);
    let lsb = SValue::from_i(lsb);
    let val = read_arg_value(cur, song);
    let mut tokens = Token::new(TokenType::Tokens, 0, vec![]);
    tokens.children = Some(vec![
        Token::new_tokens(
            TokenType::CtrlChange,
            99,
            vec![Token::new(TokenType::Value, 0, vec![msb])]
        ),
        Token::new_tokens(
            TokenType::CtrlChange,
            98,
            vec![Token::new(TokenType::Value, 0, vec![lsb])]
        ),
        Token::new_tokens(
            TokenType::CtrlChange,
            0,
            vec![Token::new(TokenType::Value, 0, vec![val])]
        ),
    ]);
    tokens
}

fn read_command_nrpn_n(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let a = read_arg_int_array(cur, song).to_array();
    if a.len() < 3 {
        song.add_log(format!("[ERROR]({}): NRPN not enough arguments", cur.line));
        return Token::new_empty("NRPN error", cur.line);
    }
    let msb = a[0].clone();
    let lsb = a[1].clone();
    let val = a[2].clone();
    let mut tokens = Token::new(TokenType::Tokens, 0, vec![]);
    tokens.children = Some(vec![
        Token::new_tokens(
            TokenType::CtrlChange,
            99,
            vec![Token::new(TokenType::Value, 0, vec![msb])]
        ),
        Token::new_tokens(
            TokenType::CtrlChange,
            98,
            vec![Token::new(TokenType::Value, 0, vec![lsb])]
        ),
        Token::new_tokens(
            TokenType::CtrlChange,
            0,
            vec![Token::new(TokenType::Value, 0, vec![val])]
        ),
    ]);
    tokens
}

fn read_voice(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let args = read_args_vec(cur, song);
    Token::new(TokenType::Voice, 0, args)
}

fn read_length(cur: &mut TokenCursor) -> Token {
    let s = cur.get_note_length();
    Token::new(TokenType::Length, 0, vec![SValue::from_s(s)])
}

fn read_octave(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Octave, value.to_i(), vec![])
}

fn read_qlen(cur: &mut TokenCursor, song: &mut Song) -> Token {
    if cur.eq("__") {
        // dummy
        cur.next();
        cur.next();
        cur.get_int(0);
    } else if cur.eq("_") {
        cur.next();
        cur.get_int(0);
    }
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        if cmd == "Random" {
            let r = read_arg_value(cur, song);
            return Token::new(TokenType::QLenRandom, 0, vec![r]);
        }
        if cmd == "onTime" || cmd == "T" {
            let _av = read_arg_int_array(cur, song);
            return Token::new_empty(&format!("[ERROR]({}) q.onTime not supported", cur.line), cur.line);
        }
        if cmd == "onNote" || cmd == "N" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::QLenOnNote, 0, vec![av]);
        }
        if cmd == "onCycle" || cmd == "C" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            return Token::new_empty("not supported : onCycle", cur.line);
        }
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::QLen, value.to_i(), vec![])
}

fn read_velocity(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let mut ino = -1;
    if cur.eq("__") {
        // sub velocity
        cur.next();
        cur.next();
        ino = cur.get_int(0);
    } else if cur.eq("_") {
        cur.next();
        cur.get_int(0);
        ino = 0;
    }
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        if cmd == "Random" {
            let r = read_arg_value(cur, song);
            return Token::new(TokenType::VelocityRandom, 0, vec![r]);
        }
        if cmd == "onTime" || cmd == "T" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::VelocityOnTime, 0, vec![av]);
        }
        if cmd == "onNote" || cmd == "N" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::VelocityOnNote, 0, vec![av]);
        }
        if cmd == "onCycle" || cmd == "C" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            return Token::new_empty("not supported : onCycle", cur.line);
        }
    }
    // v(no)
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Velocity, value.to_i(), vec![SValue::from_i(ino)])
}

fn read_timing(cur: &mut TokenCursor, song: &mut Song) -> Token {
    if cur.eq("__") {
        // dummy
        cur.next();
        cur.next();
        cur.get_int(0);
    } else if cur.eq_char('_') {
        cur.next();
    }
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        // t.Random ?
        if cmd == "Random" {
            cur.index += 7;
            let r = read_arg_value(cur, song);
            return Token::new(TokenType::TimingRandom, 0, vec![r]);
        }
        if cmd == "onNote" || cmd == "N" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::TimingOnNote, 0, vec![av]);
        }
        if cmd == "onCycle" || cmd == "C" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            return Token::new_empty("not supported : onCycle", cur.line);
        }
    }
    // t(no)
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Timing, value.to_i(), vec![])
}

fn read_command_pitch_bend_big(cur: &mut TokenCursor, song: &mut Song) -> Token {
    if cur.eq(".onTime") || cur.eq(".T") {
        if cur.eq(".onTime") {
            cur.index += ".onTime".len();
        } else {
            cur.index += ".T".len();
        }
        let ia = read_arg_int_array(cur, song);
        return Token::new(TokenType::PBonTime, 1, vec![ia]);
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::PitchBend, 1, vec![value])
}

fn read_pitch_bend_small(cur: &mut TokenCursor, song: &mut Song) -> Token {
    if cur.eq(".onTime") || cur.eq(".T") {
        if cur.eq(".onTime") {
            cur.index += ".onTime".len();
        } else {
            cur.index += ".T".len();
        }
        let ia = read_arg_int_array(cur, song);
        return Token::new(TokenType::PBonTime, 0, vec![ia]);
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::PitchBend, 0, vec![value])
}

fn read_cc(cur: &mut TokenCursor, song: &mut Song) -> Token {
    // red CC no
    let no = read_arg_value(cur, song);

    // .onTime
    if cur.eq_char('.') {
        cur.next();
        let cmd = cur.get_word();
        if cmd == "onTime" || cmd == "T" {
            let ia = read_arg_int_array(cur, song);
            return Token::new(TokenType::CConTime, no.to_i(), vec![ia]);
        }
        if cmd == "onNoteWave" || cmd == "W" {
            let _ia = read_arg_int_array(cur, song);
            return Token::new_empty("NOT SUPPORTED", cur.line);
        }
        if cmd == "onCycle" || cmd == "C" {
            let _ia = read_arg_int_array(cur, song);
            return Token::new_empty("NOT SUPPORTED", cur.line);
        }
    }

    cur.skip_space();
    if !cur.eq_char(',') && !cur.eq_char('(') {
        return Token::new(
            TokenType::Error,
            0,
            vec![SValue::from_s(format!(
                "[ERROR]({}): Faild to set Controll Change",
                cur.line + 1
            ))],
        );
    }
    cur.next();
    let val = read_arg_value(cur, song);
    Token::new_tokens(TokenType::CtrlChange, no.to_i(), vec![
        Token::new(TokenType::Value, 0, vec![val]),
    ])
}

fn read_loop(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let value = if cur.is_numeric() || cur.eq_char('=') || cur.eq_char('(') {
        read_arg_value(cur, song)
    } else {
        SValue::from_i(2)
    };
    Token::new(TokenType::LoopBegin, 0, vec![value])
}

fn read_rest(cur: &mut TokenCursor) -> Token {
    // '*'
    if cur.eq_char('*') {
        cur.next();
    }
    // length
    let mut dir = 1;
    if cur.eq_char('-') {
        cur.next();
        dir = -1;
    }
    let note_len = cur.get_note_length();
    cur.skip_space();
    Token::new(TokenType::Rest, dir, vec![SValue::from_s(note_len)])
}

fn read_note_n(cur: &mut TokenCursor, song: &mut Song) -> Token {
    // note no
    let note_no = read_arg_value(cur, song);
    cur.skip_space();
    if cur.eq_char(',') {
        cur.next();
    }
    // length
    let note_len = cur.get_note_length();
    cur.skip_space();
    // qlen
    let qlen = if !cur.eq_char(',') {
        0
    } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    cur.skip_space();
    // velocity
    let vel = if !cur.eq_char(',') {
        -1
    } else {
        cur.next();
        cur.skip_space();
        if cur.eq_char('+') {
            cur.next();
        } // 現状 +/- を無視する (TODO)
        cur.get_int(-1)
    };
    cur.skip_space();
    // timing
    let timing = if !cur.eq_char(',') {
        isize::MIN
    } else {
        cur.next();
        cur.skip_space();
        if cur.eq_char('+') {
            cur.next();
        }
        cur.get_int(isize::MIN)
    };
    // Slur or Tie
    let mut slur = SValue::None;
    if cur.eq_char('&') {
        cur.next(); // skip &
        cur.skip_space();
        slur = SValue::Int(1);
    }
    Token::new(
        TokenType::NoteN,
        0,
        vec![
            note_no,
            SValue::from_s(note_len),
            SValue::from_i(qlen),
            SValue::from_i(vel),
            SValue::from_i(timing),
            slur,
        ],
    )
}

fn read_note(cur: &mut TokenCursor, ch: char) -> Token {
    // flag
    let mut note_flag = 0;
    let mut flag_natual = false;
    loop {
        match cur.peek_n(0) {
            '+' | '#' => {
                note_flag += 1;
                cur.next();
            }
            '-' => {
                note_flag -= 1;
                cur.next();
            }
            '*' => {
                cur.next();
                flag_natual = true;
            }
            _ => break,
        }
    }
    // length
    let note_len = cur.get_note_length();
    cur.skip_space();
    // qlen
    let qlen = if !cur.eq_char(',') {
        0
    } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    cur.skip_space();
    // veolocity
    let vel = if !cur.eq_char(',') {
        -1
    } else {
        cur.next();
        cur.skip_space();
        if cur.eq_char('+') {
            cur.next();
        } // 現状 +/- を無視する (TODO)
        cur.get_int(0)
    };
    cur.skip_space();
    // timing
    let timing = if !cur.eq_char(',') {
        isize::MIN
    } else {
        cur.next();
        cur.skip_space();
        cur.get_int(isize::MIN)
    };
    // octave
    let octabe = if !cur.eq_char(',') {
        -1
    } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    // Slur or Tie
    let mut slur = SValue::None;
    if cur.eq_char('&') {
        cur.next(); // skip &
        cur.skip_space();
        slur = SValue::Int(1);
    }
    Token::new(
        TokenType::Note,
        match ch {
            'c' => 0,
            'd' => 2,
            'e' => 4,
            'f' => 5,
            'g' => 7,
            'a' => 9,
            'b' => 11,
            _ => 0,
        },
        vec![
            SValue::from_i(note_flag),
            SValue::from_i(if flag_natual { 1 } else { 0 }),
            SValue::from_s(note_len),
            SValue::from_i(qlen),
            SValue::from_i(vel),
            SValue::from_i(timing),
            SValue::from_i(octabe),
            slur,
        ],
    )
}

#[cfg(test)]
mod tests {
    use crate::token::tokens_to_str;

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
        assert_eq!(&tokens_to_str(&lex(&mut song, "P(10)", 0)), "[CtrlChange,10]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "M(10)", 0)), "[CtrlChange,1]");
    }
}
