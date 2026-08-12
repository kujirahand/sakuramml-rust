//! lexer: 音符と演奏パラメータの読み取り
use super::*;

pub(super) fn read_harmony_flag(cur: &mut SourceCursor, flag_harmony: &mut bool) -> Token {
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

pub(super) fn scan_chars(s: &str, c: char) -> isize {
    let mut cnt = 0;
    for ch in s.chars() {
        if ch == c {
            cnt += 1;
        }
    }
    cnt
}

pub(super) fn read_length(cur: &mut SourceCursor, song: &mut Song) -> Token {
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        if cmd == "Random" {
            let _av = read_arg_int_array(cur, song);
            return Token::new_empty(&format!("[ERROR]({}) l.Random not supported", cur.line), cur.line);
        }
        if cmd == "onTime" || cmd == "T" {
            let _av = read_arg_int_array(cur, song);
            return Token::new_empty(&format!("[ERROR]({}) l.onTime not supported", cur.line), cur.line);
        }
        if cmd == "onNote" || cmd == "N" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::LengthOnNote, 0, vec![av]);
        }
        if cmd == "onCycle" || cmd == "C" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::LengthOnCycle, 0, vec![av]);
        }
    }
    let s = cur.get_note_length();
    Token::new(TokenType::Length, 0, vec![SValue::from_s(s)])
}

pub(super) fn read_octave(cur: &mut SourceCursor, song: &mut Song) -> Token {
    // 先行指定を行うか
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        if cmd == "Random" {
            let r = read_arg_value(cur, song);
            return Token::new(TokenType::OctaveRandom, 0, vec![r]);
        }
        if cmd == "onTime" || cmd == "T" {
            let _av = read_arg_int_array(cur, song);
            return Token::new_empty(&format!("[ERROR]({}) o.onTime not supported", cur.line), cur.line);
        }
        if cmd == "onNote" || cmd == "N" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::OctaveOnNote, 0, vec![av]);
        }
        if cmd == "onCycle" || cmd == "C" {
            let r = read_arg_int_array(cur, song);
            return Token::new(TokenType::OctaveOnCycle, 0, vec![r]);
        }
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Octave, value.to_i(), vec![])
}

pub(super) fn read_qlen(cur: &mut SourceCursor, song: &mut Song) -> Token {
    if cur.eq("++") {
        cur.next_n(2);
        return Token::new(TokenType::QLenRel, 1, vec![]);
    }
    if cur.eq("--") {
        cur.next_n(2);
        return Token::new(TokenType::QLenRel, -1, vec![]);
    }
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
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::QLenOnCycle, 0, vec![av]);
        }
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::QLen, value.to_i(), vec![])
}

pub(super) fn read_velocity(cur: &mut SourceCursor, song: &mut Song) -> Token {
    if cur.eq("++") {
        cur.next_n(2);
        return Token::new(TokenType::VelocityRel, 1, vec![]);
    }
    if cur.eq("--") {
        cur.next_n(2);
        return Token::new(TokenType::VelocityRel, -1, vec![]);
    }
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
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::VelocityOnCycle, 0, vec![av]);
        }
    }
    // v(no)
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Velocity, value.to_i(), vec![SValue::from_i(ino)])
}

pub(super) fn read_timing(cur: &mut SourceCursor, song: &mut Song) -> Token {
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
            let r = read_arg_value(cur, song);
            return Token::new(TokenType::TimingRandom, 0, vec![r]);
        }
        if cmd == "onNote" || cmd == "N" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::TimingOnNote, 0, vec![av]);
        }
        if cmd == "onCycle" || cmd == "C" {
            let av = read_arg_int_array(cur, song);
            return Token::new(TokenType::TimingOnCycle, 0, vec![av]);
        }
    }
    // t(no)
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Timing, value.to_i(), vec![])
}

pub(super) fn read_loop(cur: &mut SourceCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let value = if cur.is_numeric() || cur.eq_char('=') || cur.eq_char('(') {
        read_arg_value(cur, song)
    } else {
        SValue::from_i(2)
    };
    Token::new(TokenType::LoopBegin, 0, vec![value])
}

pub(super) fn read_rest(cur: &mut SourceCursor) -> Token {
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

pub(super) fn read_note_n(cur: &mut SourceCursor, song: &mut Song) -> Token {
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

pub(super) fn read_note(cur: &mut SourceCursor, ch: char) -> Token {
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
    // 例外的に改行を許す
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
        if cur.eq_char('$') || cur.is_numeric() {
            slur = SValue::Int(cur.get_int(0));
        } else {
            slur = SValue::Int(1);
        }
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
