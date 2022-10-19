use super::runner::calc_length;
use super::song::Song;
use super::cursor::TokenCursor;
use super::svalue::SValue;
use super::token::{Token, TokenType, zen2han};

const LEX_MAX_ERROR: usize = 30;

/// split source code to tokens
pub fn lex(song: &mut Song, src: &str, lineno: isize) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut cur = TokenCursor::from(src);
    cur.line = lineno;
    let mut flag_harmony = false;
    while !cur.is_eos() {
        let ch = zen2han(cur.get_char());
        match ch {
            // <CHAR_COMMANDS>
            // space
            ' ' | '\t' | '\r' | '|' | ';' => { }, // @ 空白文字
            // ret
            '\n' => { cur.line += 1; },
            // lower command
            'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b' => result.push(read_note(&mut cur, ch)), // @ ドレミファソラシ c(音長),(ゲート),(音量),(タイミング),(音階)
            'n' => result.push(read_note_n(&mut cur, song)), // @ 番号を指定して発音(例: n36) n(番号),(音長),(ゲート),(音量),(タイミング)
            'r' => result.push(read_rest(&mut cur)), // @ 休符
            'l' => result.push(read_length(&mut cur)), // @ 音長の指定(例 l4)
            'o' => result.push(read_octave(&mut cur, song)), // @ 音階の指定(例 o5) 範囲:0-10
            'p' => result.push(read_pitch_bend_small(&mut cur, song)), // @ ピッチベンドの指定 範囲:0-127 (63が中央)
            'q' => result.push(read_qlen(&mut cur, song)), // @ ゲートの指定 (例 q90) 範囲:0-100
            'v' => result.push(read_velocity(&mut cur, song)), // @ ベロシティ音量の指定 範囲:0-127 / v.Random=n
            't' => result.push(read_timing(&mut cur, song)), // @ 発音タイミングの指定 (例 t-1) / t.Random=n
            'y' => result.push(read_cc(&mut cur, song)), // @ コントロールチェンジの指定 (例 y1,100) 範囲:0-127 / y1.onTime(low,high,len)
            // uppwer command
            'A'..='Z' | '_' => result.push(read_upper_command(&mut cur, song, ch)), 
            '#' => result.push(read_upper_command(&mut cur, song, ch)),
            // flag
            '@' => result.push(read_voice(&mut cur, song)), // @ 音色の指定 範囲:1-128
            '>' => result.push(Token::new_value(TokenType::OctaveRel, 1)), // @ 音階を1つ上げる
            '<' => result.push(Token::new_value(TokenType::OctaveRel, -1)), // @ 音階を1つ下げる
            ')' => result.push(Token::new_value(TokenType::VelocityRel, 8)), // @ 音量を8つ上げる
            '(' => result.push(Token::new_value(TokenType::VelocityRel, -8)), // @ 音量を8つ下げる
            // comment
            /*
                "//" => // @ 一行コメント
                "/*" .. "*/" => // @ 範囲コメント
             */
            '/' => {
                if cur.eq_char('/') {
                    cur.get_token_ch('\n');
                } else if cur.eq_char('*') {
                    cur.get_token_s("*/");
                }
            },
            '[' => result.push(read_loop(&mut cur, song)), // @ ループ開始 (例 [4 cdeg])
            ':' => result.push(Token::new_value(TokenType::LoopBreak, 0)), // @ ループ最終回に脱出 (例　[4 cde:g]e)
            ']' => result.push(Token::new_value(TokenType::LoopEnd, 0)), // @ ループ終了
            '\'' => result.push(read_harmony_flag(&mut cur, &mut flag_harmony)), // @ 和音 (例 'ceg') 'ceg'(音長),(ゲート)
            '$' => read_def_rhythm_macro(&mut cur, song), // @ リズムマクロ定義 $文字{定義内容}
            '{' => { // @ 連符 (例 {ceg}4) {c^d}(音長) 
                cur.prev();
                result.push(read_command_div(&mut cur, song));
            },
            '`' => result.push(Token::new_value(TokenType::OctaveOnce, 1)), // @ 一度だけ音階を+1する
            '"' => result.push(Token::new_value(TokenType::OctaveOnce, -1)), // @ 一度だけ音階を-1する
            '?' => result.push(Token::new_value(TokenType::PlayFrom, 0)), // @ ここから演奏する (=PLAY_FROM)
            // </CHAR_COMMANDS>
            _ => {
                if song.logs.len() == LEX_MAX_ERROR {
                    song.logs.push(format!("[ERROR]({}) Too many errors in Lexer ...", cur.line));
                } else if song.logs.len() < LEX_MAX_ERROR {
                    song.logs.push(format!("[ERROR]({}) Unknown char: '{}'", cur.line, ch));
                }
            }
        }
    }
    result
}

/// read Upper case commands
fn read_upper_command(cur: &mut TokenCursor, song: &mut Song, ch: char) -> Token {
    cur.prev(); // back 1char
    let cur_pos = cur.index;
    let cmd = cur.get_word();

    // macro define?
    if ch == '#' {
        cur.skip_space();
        if cur.eq_char('=') { // DEFINE MACRO
            cur.index = cur_pos;
            return read_def_str(cur, song);
        }
    }

    // variables?
    let sval: SValue = match song.variables.get(&cmd) {
        None => SValue::None,
        Some(sval) => sval.clone(),
    };
    match sval {
        SValue::Str(src, line_no) => {
            let tokens = lex(song, &src, line_no);
            return Token::new_tokens(TokenType::Tokens, line_no, tokens);    
        },
        _ => {},
    }

    // <UPPER_COMMANDS>
    // Track & Channel
    if cmd == "TR" || cmd == "TRACK" || cmd == "Track" { // @ トラック変更　TR=番号 範囲:1-
        let v = read_arg_int(cur, song);
        return Token::new(TokenType::Track, 0, vec![v]);
    }
    if cmd == "CH" || cmd == "Channel" { // @ チャンネル変更 CH=番号 範囲:1-16
        let v = read_arg_int(cur, song);
        return Token::new(TokenType::Channel, 0, vec![v]);
    }
    if cmd == "TIME" || cmd == "Time" { return read_command_time(cur, song); } // @ タイム変更 TIME(節:拍:ステップ)
    if cmd == "RHYTHM" || cmd == "Rhythm" || cmd == "R" { return read_command_rhythm(cur, song) } // @ リズムモード
    if cmd == "RYTHM" || cmd == "Rythm" { return read_command_rhythm(cur, song) } // @ リズムモード(v1の綴りミス対処[^^;])
    if cmd == "DIV" || cmd == "Div" { return read_command_div(cur, song) } // @ 連符 (例 DIV{ceg} )
    if cmd == "SUB" || cmd == "Sub" { return read_command_sub(cur, song) } // @ タイムポインタを戻す (例 SUB{ceg} egb)

    if cmd == "KF" || cmd == "KeyFlag" { return read_key_flag(cur, song); } // @ 臨時記号を設定 - KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note)
    if cmd == "KEY" || cmd == "Key" || cmd == "KeyShift" { return read_command_key(cur, song); } // @ ノート(cdefgab)のキーをn半音シフトする (例 KEY=3 cde)
    if cmd == "INT" || cmd == "Int" { return read_def_int(cur, song); } // @ 変数を定義 (例 INT TestValue=30)
    if cmd == "STR" || cmd == "Str" { return read_def_str(cur, song); } // @ 文字列変数を定義 (例 STR A={cde})
    if cmd == "PLAY" || cmd == "Play" { return read_play(cur, song); } // @ 複数トラックを１度に書き込む (例 PLAY={aa},{bb},{cc})
    if cmd == "PRINT" || cmd == "Print" { return read_print(cur, song); } // @ 文字を出力する (例 PRINT{"cde"} )(例 INT AA=30;PRINT(AA))
    if cmd == "PLAY_FROM" || cmd == "PlayFrom" { return Token::new_value(TokenType::PlayFrom, 0); } // @ ここから演奏する　(?と同じ意味)
    
    // controll change
    if cmd == "M" || cmd == "Modulation" { return read_command_cc(cur, 1, song); } // @ モジュレーション 範囲: 0-127
    if cmd == "PT" || cmd == "PortamentoTime" { return read_command_cc(cur, 5, song); } // @ ポルタメント 範囲: 0-127
    if cmd == "V" || cmd == "MainVolume" { return read_command_cc(cur, 7, song); } // @ メインボリューム 範囲: 0-127
    if cmd == "P" || cmd == "Panpot	" { return read_command_cc(cur, 10, song); } // @ パンポット 範囲: 0-63-127
    if cmd == "EP" || cmd == "Expression" { return read_command_cc(cur, 11, song); } // @ エクスプレッション音量 範囲: 0-127
    if cmd == "PS" || cmd == "PortamentoSwitch" { return read_command_cc(cur, 65, song); } // @ ポルタメントスイッチ
    if cmd == "REV" || cmd == "Reverb" { return read_command_cc(cur, 91, song); } // @ リバーブ 範囲: 0-127
    if cmd == "CHO" || cmd == "Chorus" { return read_command_cc(cur, 93, song); } // @ コーラス 範囲: 0-127
    
    if cmd == "PB" || cmd == "PitchBend" { return read_command_pitch_bend_big(cur, song); } // @ ピッチベンドを指定 範囲: -8192~0~8191の範囲
    if cmd == "BR" || cmd == "PitchBendSensitivity" { return read_command_rpn(cur, 0, 0, song); } // @ ピッチベンドの範囲を設定 範囲: 0-12半音
    if cmd == "FineTune" { return read_command_rpn(cur, 0, 1, song); } // @ チューニングの微調整 範囲:0-64-127 (-100 - 0 - +99.99セント）
    if cmd == "CoarseTune" { return read_command_rpn(cur, 0, 2, song); } // @ 半音単位のチューニング 範囲:40-64-88 (-24 - 0 - 24半音)
    if cmd == "VibratoRate" { return read_command_nrpn(cur, 1, 8, song); } // @ 音色の編集(GS/XG) 範囲: 0-127
    if cmd == "VibratoDepth" { return read_command_nrpn(cur, 1, 9, song); } // @ 音色の編集(GS/XG) 範囲: 0-127
    if cmd == "VibratoDelay" { return read_command_nrpn(cur, 1, 10, song); } // @ 音色の編集(GS/XG) 範囲: 0-127
    if cmd == "FilterCutoff" { return read_command_nrpn(cur, 1, 0x20, song); } // @ 音色の編集(GS/XG) 範囲: 0-127
    if cmd == "FilterResonance" { return read_command_nrpn(cur, 1, 0x21, song); } // @ 音色の編集(GS/XG) 範囲: 0-127
    if cmd == "EGAttack" { return read_command_nrpn(cur, 1, 0x63, song); } // @ 音色の編集(GS/XG) 範囲: 0-127
    if cmd == "EGDecay" { return read_command_nrpn(cur, 1, 0x64, song); } // @ 音色の編集(GS/XG) 範囲: 0-127
    if cmd == "EGRelease" { return read_command_nrpn(cur, 1, 0x66, song); } // @ 音色の編集(GS/XG) 範囲: 0-127

    // SysEx
    if cmd == "ResetGM" { return Token::new_sysex(vec![0x7E,0x7F,0x9,0x1,0xF7]) } // @ GMリセットを送信
    if cmd == "ResetGS" { return Token::new_sysex(vec![0x41,0x10,0x42,0x12,0x40,0x00,0x7F,0x00,0x41,0xF7]) } // @ GSリセットを送信
    if cmd == "ResetXG" { return Token::new_sysex(vec![0x43,0x10,0x4c,0x00,0x00,0x7e,0x00,0xf7]) } // @ XGリセットを送信

    // meta events
    if cmd == "TEMPO" || cmd == "Tempo" || cmd == "T" { // @ テンポの指定
        let v = read_arg_int(cur, song);
        return Token::new(TokenType::Tempo, 0, vec![v]);
    }
    if cmd == "TimeSignature" || cmd == "TimeSig" || cmd == "TIMESIG" { // @ 拍子の指定
        cur.skip_space();
        if cur.eq_char('=') { cur.next(); }
        cur.skip_space();
        if cur.eq_char('(') { cur.next(); }
        let frac = read_arg_value(cur, song);
        cur.skip_space();
        if cur.eq_char(',') { cur.next(); }
        let deno = read_arg_value(cur, song);
        cur.skip_space();
        if cur.eq_char(')') { cur.next(); }
        return Token::new(TokenType::TimeSignature, 0, vec![frac, deno]);
    }
    if cmd == "MetaText" || cmd == "TEXT" || cmd == "Text" { // @ メタテキスト (例 TEXT{"abcd"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 1, vec![v]);
    }
    if cmd == "COPYRIGHT" || cmd == "Copyright" { // @ メタテキスト著作権 (例 COPYRIGHT{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 2, vec![v]);
    }
    if cmd == "TRACK_NAME" || cmd == "TrackName" { // @ 曲名 (例 TRACK_NAME{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 3, vec![v]);
    }
    if cmd == "InstrumentName" { // @ 楽器名 (例 InstrumentName{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 4, vec![v]);
    }
    if cmd == "LYRIC" || cmd == "Lyric" { // @ メタテキスト歌詞 (例 LYRIC{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 5, vec![v]);
    }
    if cmd == "MAKER" || cmd == "Marker" { // @ マーカー (例 MAKER{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 6, vec![v]);
    }
    if cmd == "CuePoint" { // @ キューポイント (例 CuePoint{"aaa"})
        let v = read_arg_str(cur, song);
        return Token::new(TokenType::MetaText, 7, vec![v]);
    }
    // </UPPER_COMMANDS>
    song.logs.push(format!("[ERROR]({}) Unknown command: {}", cur.line, cmd));
    return Token::new_empty(&cmd);
}

fn read_arg_value(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        'A'..='Z' => {
            let vname = cur.get_word();
            match song.variables.get(&vname) {
                Some(v) =>  v.clone(),
                None => SValue::from_s(format!("={}", vname)), // 変数への参照
            }
        },
        '!' => { // timebase length
            cur.next(); // skip !
            let len_str = cur.get_note_length();
            SValue::from_i(calc_length(&len_str, song.timebase, song.timebase))
        },
        '-' | '0'..='9' => {
            let v = cur.get_int(0);
            SValue::from_i(v)
        },
        '=' => {
            cur.next(); // skip =
            read_arg_value(cur, song)
        },
        '(' => {
            cur.next(); // skip (
            let v = read_arg_value(cur, song);
            cur.skip_space();
            if cur.eq_char(')') { cur.next(); }
            v
        },
        _ => {
            SValue::None   
        }
    }
}

fn read_arg_value_str(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        'A'..='Z' => {
            let vname = cur.get_word();
            match song.variables.get(&vname) {
                Some(v) =>  v.clone(),
                None => SValue::from_s(format!("={}", vname)), // 変数への参照
            }
        },
        '-' | '0'..='9' => {
            let v = cur.get_int(0);
            SValue::from_i(v)
        },
        '{' => {
            let s = cur.get_token_nest('{', '}');
            SValue::from_s(s)
        },
        '=' => {
            cur.next(); // skip =
            read_arg_value_str(cur, song)
        },
        '(' => {
            cur.next(); // skip (
            let v = read_arg_value_str(cur, song);
            cur.skip_space();
            if cur.eq_char(')') { cur.next(); }
            v
        },
        _ => {
            SValue::None   
        }
    }
}

fn read_arg_int(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        '(' => {
            cur.next(); // skip '('
            let sv = read_arg_value(cur, song);
            cur.skip_space();
            if cur.peek_n(0) == ')' { cur.next(); }
            return sv;
        },
        '=' => {
            cur.next();
            read_arg_value(cur, song)
        },
        _ => {
            SValue::None
        }
    }
}

fn read_arg_value_int_array(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    let mut a: Vec<isize> = vec![];
    loop {
        cur.skip_space();
        let v = read_arg_value(cur, song);
        a.push(v.to_i());
        cur.skip_space();
        if ! cur.eq_char(',') { break; }
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
            if cur.peek_n(0) == ')' { cur.next(); }
            return sv;
        },
        '=' => {
            cur.next();
            read_arg_value_int_array(cur, song)
        },
        _ => {
            SValue::None
        }
    }
}

fn read_arg_str(cur: &mut TokenCursor, song: &mut Song) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        '(' => {
            cur.next(); // skip '('
            let sv = read_arg_value_str(cur, song);
            cur.skip_space();
            if cur.peek_n(0) == ')' { cur.next(); }
            return sv;
        },
        '=' => {
            cur.next();
            read_arg_value_str(cur, song)
        },
        '{' => {
            read_arg_value_str(cur, song)
        },
        _ => {
            SValue::None
        }
    }
}

fn read_harmony_flag(cur: &mut TokenCursor, flag_harmony: &mut bool) -> Token {
    // begin
    if !*flag_harmony {
        *flag_harmony = true;
        return Token::new(TokenType::HarmonyBegin, 0, vec![])
    }
    // end
    *flag_harmony = false;
    let mut len_s = SValue::None;
    let mut qlen = SValue::None;
    if cur.is_numeric() || cur.eq_char('^') {
        len_s = SValue::from_s(cur.get_note_length());
    }
    cur.skip_space();
    if cur.eq_char(',') {
        cur.next();
        qlen = SValue::from_i(cur.get_int(0));
    }
    Token::new(TokenType::HarmonyEnd, 0, vec![
        len_s,
        qlen,
    ])
}

fn scan_chars(s: &str, c: char) -> isize {
    let mut cnt = 0;
    for ch in s.chars() {
        if ch == c { cnt += 1; }
    }
    cnt
}

fn read_key_flag(cur: &mut TokenCursor, _song: &mut Song) -> Token {
    let mut flag = 1;
    let mut key_flag = vec![0,0,0,0,0,0,0,0,0,0,0,0];
    cur.skip_space();
    if cur.eq_char('=') { cur.next(); }
    cur.skip_space();
    // flag
    match cur.peek_n(0) {
        '+' | '#' => { cur.next(); flag = 1; },
        '-' => { cur.next(); flag = -1; }
        _ => {},
    }
    // check note
    cur.skip_space();
    if cur.eq_char('(') { cur.next(); }
    while !cur.is_eos() {
        cur.skip_space();
        match cur.peek_n(0) {
            'c' => { cur.next(); key_flag[0] = flag; },
            'd' => { cur.next(); key_flag[2] = flag; },
            'e' => { cur.next(); key_flag[4] = flag; },
            'f' => { cur.next(); key_flag[5] = flag; },
            'g' => { cur.next(); key_flag[7] = flag; },
            'a' => { cur.next(); key_flag[9] = flag; },
            'b' => { cur.next(); key_flag[11] = flag; },
            _ => break,
        }
    }
    cur.skip_space();
    if cur.eq_char(')') { cur.next(); }
    // token
    let tok = Token::new(TokenType::KeyFlag, 0, vec![SValue::from_int_array(key_flag)]);
    tok
}

fn read_def_int(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let var_name = cur.get_word();
    if var_name == "" {
        song.logs.push(format!("[ERROR]({}): INT command should use Upper case like \"Test\".", cur.line));
        return Token::new_empty("Failed to def INT");
    }
    cur.skip_space();
    if cur.eq_char('=') { cur.next(); }
    let var_value = read_arg_value(cur, song);
    let tok = Token::new(TokenType::DefInt, 0, vec![
        SValue::from_s(var_name),
        var_value,
    ]);
    tok
}

fn read_print(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    let val = read_arg_value_str(cur, song);
    Token::new(TokenType::Print, lineno, vec![val])
}

fn read_play(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let mut tokens: Vec<Token> = vec![];
    let mut track_no = 1;
    cur.skip_space();
    if cur.eq_char('=') { cur.next(); }
    cur.skip_space();
    if cur.eq_char('(') { cur.next(); }
    loop {
        let tt = lex(song, &format!("TR={}", track_no), cur.line);
        for t in tt.into_iter() { tokens.push(t); }
        cur.skip_space();
        match cur.peek_n(0) {
            'A'..='Z' | '_' | '#' => {
                let name = cur.get_word();
                match song.variables.get(&name) {
                    None => {},
                    Some(sv) => {
                        let (src, lineno) = sv.get_str_and_tag();
                        let tt = lex(song, &src, lineno);
                        for t in tt.into_iter() { tokens.push(t); }
                    },
                }
            },
            '{' .. => {
                let src = cur.get_token_nest('{', '}');
                let tt = lex(song, &src, cur.line);
                for t in tt.into_iter() { tokens.push(t); }
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
    if cur.eq_char(')') { cur.next(); }
    let tokens_tok = Token::new_tokens(TokenType::Tokens, 0, tokens);
    tokens_tok
}

fn read_def_str(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let var_name = cur.get_word();
    if var_name == "" {
        song.logs.push(format!("[ERROR]({}): STR command should use Upper case like \"Test\"", cur.line));
        return Token::new_empty("Failed to def STR");
    }
    cur.skip_space();
    if cur.eq_char('=') { cur.next(); }
    cur.skip_space();
    if !cur.eq_char('{') {
        song.logs.push(format!("[ERROR]({}): STR command should set string", cur.line));
        return Token::new_empty("Failed to def STR");
    }
    let line_no = cur.line;
    let data_str = cur.get_token_nest('{', '}');
    let var_value = SValue::from_str_and_tag(&data_str, line_no);
    let tok = Token::new_empty("STR");
    song.variables.insert(var_name, var_value);
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
    let v = read_arg_int(cur, song);
    let tok = Token::new(TokenType::KeyShift, 0, vec![v]);
    tok
}

fn read_command_div(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
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
            },
            TokenType::NoteN => {
                cnt += 1;
                cnt += scan_chars(&t.data[1].to_s(), '^');
            },
            TokenType::Div => {
                cnt += 1;
                cnt += scan_chars(&t.data[0].to_s(), '^');
            },
            TokenType::Rest => {
                cnt += 1;
                cnt += scan_chars(&t.data[0].to_s(), '^');
            },
            _ => {},
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
        let ch = macro_cur.get_char();
        if macro_cur.eq("Sub") || macro_cur.eq("SUB") {
            result.push_str("SUB");
            macro_cur.index += 3;
            continue;
        }
        match ch {
            '\u{0040}'..='\u{007f}' => {
                let m = &song.rhthm_macro[ch as usize - 0x40];
                if m == "" {
                    result.push(ch);
                } else {
                    result.push_str(m);
                }
            },
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
    if cur.eq_char('=') { cur.next(); }
    cur.skip_space();
    let s = cur.get_token_nest('{', '}');
    song.rhthm_macro[ch as usize - 0x40] = s;
}

fn read_command_time(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    if cur.eq_char('=') { cur.next(); }
    cur.skip_space();
    if cur.eq_char('(') { cur.next(); }
    
    let v1 = read_arg_value(cur, song);
    cur.skip_space();
    if cur.eq_char(':') { cur.next(); }
    let v2 = read_arg_value(cur, song);
    cur.skip_space();
    if cur.eq_char(':') { cur.next(); }
    let v3 = read_arg_value(cur, song);
    cur.skip_space();
    if cur.eq_char(')') { cur.next(); }

    return Token::new(TokenType::Time, 0, vec![v1, v2, v3]);
}

fn read_command_cc(cur: &mut TokenCursor, no: isize, song: &mut Song) -> Token {
    if cur.eq(".onTime") || cur.eq(".T") {
        if cur.eq(".onTime") {
            cur.index += ".onTime".len();
        } else {
            cur.index += ".T".len();
        }
        let ia = read_arg_int_array(cur, song);
        return Token::new(TokenType::CConTime, no, vec![ia]);
    }
    let v = read_arg_int(cur, song);
    return Token::new(TokenType::ControllChange, 0, vec![SValue::from_i(no), v]);
}

fn read_command_rpn(cur: &mut TokenCursor, msb: isize, lsb: isize, song: &mut Song) -> Token {
    let val = read_arg_int(cur, song);
    let mut tokens = Token::new(TokenType::Tokens, 0, vec![]);
    tokens.children = Some(vec![
        Token::new(TokenType::ControllChange, 0, vec![SValue::from_i(101), SValue::from_i(msb)]),
        Token::new(TokenType::ControllChange, 0, vec![SValue::from_i(100), SValue::from_i(lsb)]),
        Token::new(TokenType::ControllChange, 0, vec![SValue::from_i(6), val]),
    ]);
    tokens
}

fn read_command_nrpn(cur: &mut TokenCursor, msb: isize, lsb: isize, song: &mut Song) -> Token {
    let val = read_arg_int(cur, song);
    let mut tokens = Token::new(TokenType::Tokens, 0, vec![]);
    tokens.children = Some(vec![
        Token::new(TokenType::ControllChange, 0, vec![SValue::from_i(99), SValue::from_i(msb)]),
        Token::new(TokenType::ControllChange, 0, vec![SValue::from_i(98), SValue::from_i(lsb)]),
        Token::new(TokenType::ControllChange, 0, vec![SValue::from_i(6), val]),
    ]);
    tokens
}

fn read_voice(cur: &mut TokenCursor, song: &mut Song) -> Token {
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Voice, 0, vec![value])
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
    let value = read_arg_value(cur, song);
    Token::new(TokenType::QLen, value.to_i(), vec![])
}

fn read_velocity(cur: &mut TokenCursor, song: &mut Song) -> Token {
    // v.Random ?
    if cur.eq(".Random") {
        cur.index += ".Random".len();
        let r = read_arg_int(cur, song);
        return Token::new(TokenType::VelocityRandom, 0, vec![r])
    }
    // v.onTime
    if cur.eq(".onTime") {
        cur.index += ".onTime".len();
        let av = read_arg_int_array(cur, song);
        return Token::new(TokenType::VelocityOnTime, 0, vec![av])
    }
    // v(no)
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Velocity, value.to_i(), vec![])
}

fn read_timing(cur: &mut TokenCursor, song: &mut Song) -> Token {
    // t.Random ?
    if cur.eq(".Random") {
        cur.index += 7;
        let r = read_arg_int(cur, song);
        return Token::new(TokenType::TimingRandom, 0, vec![r])
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
    if cur.eq(".onTime") || cur.eq(".T") {
        if cur.eq(".onTime") {
            cur.index += ".onTime".len();
        } else {
            cur.index += ".T".len();
        }
        let ia = read_arg_int_array(cur, song);
        return Token::new(TokenType::CConTime, no.to_i(), vec![ia]);
    }

    cur.skip_space();
    if !cur.eq_char(',') {
        return Token::new(TokenType::Error, 0, vec![
            SValue::from_s(format!("[ERROR]({}): Faild to set Controll Change", cur.line + 1))]);
    }
    cur.next();
    let val = read_arg_value(cur, song);
    Token::new(TokenType::ControllChange, 0, vec![no, val])
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
    if cur.eq_char(',') { cur.next(); }
    // length
    let note_len = cur.get_note_length();
    cur.skip_space();
    // qlen
    let qlen = if !cur.eq_char(',') { 0 } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    // veolocity
    let vel = if !cur.eq_char(',') { -1 } else {
        cur.next();
        cur.skip_space();
        cur.get_int(-1)
    };
    // timing
    let timing = if !cur.eq_char(',') { isize::MIN } else {
        cur.next();
        cur.skip_space();
        cur.get_int(isize::MIN)
    };
    Token::new(
        TokenType::NoteN,
        0,
        vec![
            note_no,
            SValue::from_s(note_len),
            SValue::from_i(qlen),
            SValue::from_i(vel),
            SValue::from_i(timing),
        ]
    )
}

fn read_note(cur: &mut TokenCursor, ch: char) -> Token {
    // flag
    let mut note_flag = 0;
    loop {
        match cur.peek_n(0) {
            '+' | '#' => { note_flag += 1; cur.next(); },
            '-' => { note_flag -= 1; cur.next(); },
            _ => break,
        }
    }
    // length
    let note_len = cur.get_note_length();
    cur.skip_space();
    // qlen
    let qlen = if !cur.eq_char(',') { 0 } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    // veolocity
    let vel = if !cur.eq_char(',') { -1 } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    // timing
    let timing = if !cur.eq_char(',') { isize::MIN } else {
        cur.next();
        cur.skip_space();
        cur.get_int(isize::MIN)
    };
    // octave
    let octabe = if !cur.eq_char(',') { -1 } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
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
            SValue::from_s(note_len),
            SValue::from_i(qlen),
            SValue::from_i(vel),
            SValue::from_i(timing),
            SValue::from_i(octabe),
        ]
    )
}

#[cfg(test)]
mod tests {
    use crate::token::tokens_to_str;

    use super::*;
    #[test]
    fn test_lex1() {
        let mut song = Song::new();
        assert_eq!(&tokens_to_str(&lex(&mut song, "cdefgab", 0)), "[Note,0][Note,2][Note,4][Note,5][Note,7][Note,9][Note,11]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "l4c", 0)), "[Length,0][Note,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TR=1", 0)), "[Track,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TR(1)", 0)), "[Track,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "INT A=1;TR(A)", 0)), "[DefInt,0][Track,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "INT A=1;TR=A", 0)), "[DefInt,0][Track,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "COPYRIGHT{a}", 0)), "[MetaText,2]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "COPYRIGHT={a}", 0)), "[MetaText,2]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TimeSig=4,4", 0)), "[TimeSignature,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TimeSig=(4,4)", 0)), "[TimeSignature,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TimeSig(4,4)", 0)), "[TimeSignature,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TIME(1:1:0)", 0)), "[Time,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TIME=(1:1:0)", 0)), "[Time,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TIME(1:1:0)", 0)), "[Time,0]");
        assert_eq!(&tokens_to_str(&lex(&mut song, "TIME=1:1:0", 0)), "[Time,0]");
    }
}
