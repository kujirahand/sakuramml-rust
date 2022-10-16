use super::song::Song;
use super::cursor::TokenCursor;
use super::svalue::SValue;
use super::token::{Token, TokenType, zen2han};

/// split source code to tokens
pub fn lex(song: &mut Song, src: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut cur = TokenCursor::from(src);
    let mut flag_harmony = false;
    while !cur.is_eos() {
        let ch = zen2han(cur.get_char());
        match ch {
            // space
            ' ' | '\t' | '\r' | '|' => { },
            // ret
            '\n' => { cur.line += 1; },
            // lower command
            'a'..='g' => result.push(read_note(&mut cur, ch)),
            'n' => result.push(read_note_n(&mut cur)),
            'r' | '_' => result.push(read_rest(&mut cur)),
            'l' => result.push(read_length(&mut cur)),
            'o' => result.push(read_octave(&mut cur)),
            'p' => result.push(read_pitch_bend(&mut cur)),
            'q' => result.push(read_qlen(&mut cur)),
            'v' => result.push(read_velocity(&mut cur)),
            'y' => result.push(read_cc(&mut cur)),
            // uppwer command
            'A'..='Z' => result.push(read_upper_command(&mut cur, song)), 
            // flag
            '@' => result.push(read_voice(&mut cur)),
            '>' => result.push(Token::new_value(TokenType::OctaveRel, 1)),
            '<' => result.push(Token::new_value(TokenType::OctaveRel, -1)),
            '/' => {
                if cur.eq_char('/') {
                    cur.get_token_ch('\n');
                } else if cur.eq_char('*') {
                    cur.get_token_s("*/");
                }
            },
            '[' => result.push(read_loop(&mut cur)),
            ':' => result.push(Token::new_value(TokenType::LoopBreak, 0)),
            ']' => result.push(Token::new_value(TokenType::LoopEnd, 0)),
            '\'' => result.push(read_harmony_flag(&mut cur, &mut flag_harmony)),
            '$' => read_def_rhythm_macro(&mut cur, song),
            '{' => { // Div
                cur.prev();
                result.push(read_command_div(&mut cur, song));
            },
            '`' => result.push(Token::new_value(TokenType::OctaveOnce, 1)),
            '"' => result.push(Token::new_value(TokenType::OctaveOnce, -1)),
            _ => {
                song.logs.push(format!("[ERROR] {}", ch));
            }
        }
    }
    result
}

/// read Upper case commands
fn read_upper_command(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.prev(); // back 1char
    let cmd = cur.get_word();

    // Track & Channel
    if cmd == "TR" || cmd == "TRACK" || cmd == "Track" {
        let v = read_arg(cur);
        return Token::new(TokenType::Track, 0, vec![v]);
    }
    if cmd == "CH" || cmd == "Channel" {
        let v = read_arg(cur);
        return Token::new(TokenType::Channel, 0, vec![v]);
    }
    if cmd == "TIME" || cmd == "Time" { return read_command_time(cur); }
    if cmd == "RHYTHM" || cmd == "Rhythm" || cmd == "R" { return read_command_rhythm(cur, song) }
    if cmd == "RYTHM" || cmd == "Rythm" { return read_command_rhythm(cur, song) } // v1の綴りミス 😆
    if cmd == "DIV" || cmd == "Div" || cmd == "D" { return read_command_div(cur, song) }
    
    // controll change
    if cmd == "M" || cmd == "Modulation" { return read_command_cc(cur, 1); }
    if cmd == "PT" || cmd == "PortamentoTime" { return read_command_cc(cur, 5); }
    if cmd == "V" || cmd == "MainVolume" { return read_command_cc(cur, 7); }
    if cmd == "P" || cmd == "Panpot	" { return read_command_cc(cur, 10); }
    if cmd == "EP" || cmd == "Expression" { return read_command_cc(cur, 11); }
    if cmd == "PS" || cmd == "PortamentoSwitch" { return read_command_cc(cur, 65); }
    if cmd == "REV" || cmd == "Reverb" { return read_command_cc(cur, 91); }
    if cmd == "CHO" || cmd == "Chorus" { return read_command_cc(cur, 93); }

    // meta events
    if cmd == "TEMPO" || cmd == "Tempo" || cmd == "T" {
        let v = read_arg(cur);
        return Token::new(TokenType::Tempo, 0, vec![v]);
    }
    if cmd == "TimeSignature" || cmd == "TimeSig" || cmd == "TIMESIG" {
        let frac = read_arg(cur);
        cur.skip_space();
        let deno = if cur.eq_char(',') {
            cur.next();
            read_arg(cur)
        } else { frac.clone() };
        return Token::new(TokenType::TimeSignature, 0, vec![frac, deno]);
    }
    if cmd == "MetaText" || cmd == "TEXT" || cmd == "Text" {
        let v = read_arg(cur);
        return Token::new(TokenType::MetaText, 1, vec![v]);
    }
    if cmd == "COPYRIGHT" || cmd == "Copyright" {
        let v = read_arg(cur);
        return Token::new(TokenType::MetaText, 2, vec![v]);
    }
    if cmd == "LYRIC" || cmd == "Lyric" {
        let v = read_arg(cur);
        return Token::new(TokenType::MetaText, 5, vec![v]);
    }
    song.logs.push(format!("[ERROR] Unknown command: {}", cmd));
    return Token::new_unknown(&cmd);
}

fn read_arg(cur: &mut TokenCursor) -> SValue {
    cur.skip_space();
    let ch = cur.peek_n(0);
    match ch {
        '(' => {
            cur.next();
            let r = read_arg(cur);
            cur.skip_space();
            if cur.peek_n(0) == ')' { cur.next(); }
            return r;
        },
        '=' => {
            cur.next();
            read_arg(cur)
        },
        // number
        '0'..='9' => {
            let n = cur.get_int(0);
            SValue::from_i(n)
        },
        '{' => {
            SValue::from_s(cur.get_token_nest('{', '}'))
        }
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
        qlen = read_arg(cur);
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

fn read_command_div(cur: &mut TokenCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let block = cur.get_token_nest('{', '}');
    let len_s = cur.get_note_length();
    let tokens = lex(song, &block);
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
    let block = cur.get_token_nest('{', '}');
    // extract macro
    for ch in block.chars() {
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
    println!("rythm={:?}", result);
    t.children = Some(lex(song, &result));
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

fn read_command_time(cur: &mut TokenCursor) -> Token {
    cur.skip_space();
    if cur.eq_char('=') { cur.next(); }
    cur.skip_space();
    if cur.eq_char('(') { cur.next(); }
    
    let v1 = read_arg(cur);
    cur.skip_space();
    if cur.eq_char(':') { cur.next(); }
    let v2 = read_arg(cur);
    cur.skip_space();
    if cur.eq_char(':') { cur.next(); }
    let v3 = read_arg(cur);
    cur.skip_space();
    if cur.eq_char(')') { cur.next(); }

    return Token::new(TokenType::Time, 0, vec![v1, v2, v3]);
}

fn read_command_cc(cur: &mut TokenCursor, no: isize) -> Token {
    let v = read_arg(cur);
    return Token::new(TokenType::ControllChange, 0, vec![SValue::from_i(no), v]);
}

fn read_voice(cur: &mut TokenCursor) -> Token {
    let value = read_arg(cur);
    Token::new(TokenType::Voice, value.to_i(), vec![])
}

fn read_length(cur: &mut TokenCursor) -> Token {
    let s = cur.get_note_length();
    Token::new(TokenType::Length, 0, vec![SValue::from_s(s)])
}

fn read_octave(cur: &mut TokenCursor) -> Token {
    let value = read_arg(cur);
    Token::new(TokenType::Octave, value.to_i(), vec![])
}

fn read_qlen(cur: &mut TokenCursor) -> Token {
    let value = read_arg(cur);
    Token::new(TokenType::QLen, value.to_i(), vec![])
}

fn read_velocity(cur: &mut TokenCursor) -> Token {
    let value = read_arg(cur);
    Token::new(TokenType::Velocity, value.to_i(), vec![])
}

fn read_pitch_bend(cur: &mut TokenCursor) -> Token {
    let value = read_arg(cur);
    Token::new(TokenType::PitchBend, value.to_i(), vec![])
}

fn read_cc(cur: &mut TokenCursor) -> Token {
    let no = read_arg(cur);
    cur.skip_space();
    if !cur.eq_char(',') {
        return Token::new(TokenType::Error, 0, vec![
            SValue::from_s(format!("[ERROR]({}): Faild to set Controll Change", cur.line + 1))]);
    }
    cur.next();
    let val = read_arg(cur);
    Token::new(TokenType::ControllChange, 0, vec![no, val])
}

fn read_loop(cur: &mut TokenCursor) -> Token {
    let value = read_arg(cur);
    Token::new(TokenType::LoopBegin, value.to_i(), vec![])
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

fn read_note_n(cur: &mut TokenCursor) -> Token {
    // note no
    let note_no = read_arg(cur);
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
    let vel = if !cur.eq_char(',') { 0 } else {
        cur.next();
        cur.skip_space();
        cur.get_int(0)
    };
    Token::new(
        TokenType::NoteN,
        0,
        vec![
            note_no,
            SValue::from_s(note_len),
            SValue::from_i(qlen),
            SValue::from_i(vel),
        ]
    )
}

fn read_note(cur: &mut TokenCursor, ch: char) -> Token {
    // flag
    let note_flag = match cur.peek_n(0) {
        '+' | '#' => 1,
        '-' => -1,
        _ => 0,
    };
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
    let vel = if !cur.eq_char(',') { 0 } else {
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
        ]
    )
}

