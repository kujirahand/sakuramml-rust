//! lexer: コントロールチェンジ系コマンドの読み取り
use super::*;

pub(super) fn read_sysex(cur: &mut SourceCursor, _song: &mut Song) -> Token {
    // read sysex
    let lineno = cur.line;
    let hex_mode = if cur.eq_char('$') { cur.next(); true} else { false };
    if cur.eq_char('=') { cur.next(); } // skip '='
    let mut data_vec: Vec<Token> = vec![];
    let mut flag_calc_checksum = 0; // 0:none, 1:check_sum_mode
    loop {
        cur.skip_space();
        if cur.eq_char('{') {
            cur.next(); // skip '{'
            flag_calc_checksum = 1;
            let t = Token::new(TokenType::ConstInt, -1, vec![]); // start checksum
            data_vec.push(t);
        }
        if hex_mode {
            let hex = cur.get_hex(0, true);
            let mut v = Token::new(TokenType::ConstInt, hex, vec![]);
            v.value_type = TokenValueType::INT;
            v.lineno = lineno;
            data_vec.push(v);
        } else {
            let c = cur.peek_n(0);
            match c {
                '0'..='9' | '$' => {
                    let v = cur.get_int(0);
                    let mut t = Token::new(TokenType::ConstInt, v, vec![]);
                    t.value_type = TokenValueType::INT;
                    t.lineno = lineno;
                    data_vec.push(t);
                }
                'A'..='Z' | '_' => {
                    let var_name = cur.get_word();
                    let mut t = Token::new(TokenType::Value, 0, vec![SValue::from_s(format!("={}", var_name))]);
                    t.value_type = TokenValueType::VARIABLE;
                    t.lineno = lineno;
                    data_vec.push(t);
                }
                _ => {}
            }
        }
        cur.skip_space();
        if cur.eq_char('}') { // 数値の後に'}'がある場合を考慮
            cur.next(); // skip '}'
            let t = Token::new(TokenType::ConstInt, -2, vec![]); // end checksum
            data_vec.push(t);
        }
        // 続きのデータがあるか？
        if cur.eq_char(',') {
            cur.next();
        } else {
            break;
        }
    }
    let mut t = Token::new_tokens(TokenType::SysEx, flag_calc_checksum, data_vec);
    t.lineno = lineno;
    t
}

pub(super) fn read_fadein(cur: &mut SourceCursor, song: &mut Song, dir: isize) -> Token {
    let arg = read_arg_value(cur, song);
    let ia = if dir >= 1 {
        SValue::from_int_array(vec![0, 127, song.timebase * 4 * arg.to_i()])
    } else {
        SValue::from_int_array(vec![127, 0, song.timebase * 4 * arg.to_i()])
    };
    return Token::new(TokenType::CConTime, 11, vec![ia]);
}

pub(super) fn read_decres(cur: &mut SourceCursor, song: &mut Song, dir: isize) -> Token {
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

/// read command CC
pub(super) fn read_command_cc(cur: &mut SourceCursor, no: isize, song: &mut Song) -> Token {
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        if cmd == "onTime" || cmd == "T" {
            let ia = read_arg_int_array(cur, song);
            return Token::new(TokenType::CConTime, no, vec![ia]);
        } else if cmd == "onNote" || cmd == "N" {
            let ia = read_arg_int_array(cur, song);
            return Token::new(TokenType::CConNote, no, vec![ia]);
        } else if cmd == "Frequency" {
            let a = read_arg_value(cur, song);
            return Token::new(TokenType::CConTimeFreq, 0, vec![a]);
        } else if cmd == "onNoteWave" || cmd == "W" {
            let ia = read_arg_int_array(cur, song);
            return Token::new(TokenType::CConNoteWave, no, vec![ia]);
        } else if cmd == "onNoteWaveEx" || cmd == "WE" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            song.add_log(format!("[WARN]({}) not supported : onNoteWaveEx", cur.line));
            return Token::new_empty("not supported : onNoteWave", cur.line);
        } else if cmd == "onNoteWaveR" || cmd == "WR"{ // (命令).onNoteWaveR(low,high,len...) // ノートオンしている間、low,higi,len...を繰り返す
            // TODO: not supported
            let a = read_arg_int_array(cur, song);
            song.add_log(format!("[WARN]({}) not supported : onNoteWaveR : {:?}", cur.line, a));
            return Token::new_empty("not supported : onNoteWaveR", cur.line);
        } else if cmd == "onCycle" || cmd == "C" {
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            song.add_log(format!("[WARN]({}) not supported : onCycle", cur.line));
            return Token::new_empty("not supported : onCycle", cur.line);
        } else if cmd == "Sine" { // .Sine(type,low,high,len,times) // type=0:sine/1:up sine/2:down sine
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            song.add_log(format!("[WARN]({}) not supported : Sine", cur.line));
            return Token::new_empty("not supported : Sine", cur.line);
        } else if cmd == "onNoteSine" { // .onNoteSine(type,low,high,len,times) // type=0:sine/1:up sine/2:down sine
            // TODO: not supported
            let _ = read_arg_int_array(cur, song);
            song.add_log(format!("[WARN]({}) not supported : onNoteSine", cur.line));
            return Token::new_empty("not supported : onNoteSine", cur.line);
        }
        /*
        https://sakuramml.com/doc/reference/cc-option.htm
        Delay	先行指定の効果の遅延時間
        Repeat	予約指定で.onNoteなどで繰り返すかどうか
        Random	書き込まれる値に、vのランダムな値を足す
        Range	書き込まれる値に、上限と下限を設定する
        Frequency	コントロールチェンジの書き込み頻度を指定する
        */
    }
    if cur.eq_char('=') { cur.next(); }
    let value_tokens = read_args_tokens(cur, song);
    
    return Token::new_tokens(TokenType::ControlChange, no, value_tokens);
}

pub(super) fn read_rpn_command(cur: &mut SourceCursor, msb: isize, lsb: isize, song: &mut Song) -> Token {
    let args = read_args_tokens(cur, song);
    let token = Token::new_data_tokens(TokenType::RPNCommand, 0, vec![SValue::Int(msb), SValue::Int(lsb)], args);
    token
}

pub(super) fn read_nrpn_command(cur: &mut SourceCursor, msb: isize, lsb: isize, song: &mut Song) -> Token {
    let args = read_args_tokens(cur, song);
    let token = Token::new_data_tokens(TokenType::NRPNCommand, 0, vec![SValue::Int(msb), SValue::Int(lsb)], args);
    token
}

pub(super) fn read_voice(cur: &mut SourceCursor, song: &mut Song) -> Token {
    let args = read_args_tokens(cur, song);
    Token::new_tokens(TokenType::Voice, 0, args)
}

pub(super) fn read_command_pitch_bend_big(cur: &mut SourceCursor, song: &mut Song) -> Token {
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

pub(super) fn read_pitch_bend_small(cur: &mut SourceCursor, song: &mut Song) -> Token {
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

pub(super) fn read_cc(cur: &mut SourceCursor, song: &mut Song, ch: char) -> Token {
    // read CC no
    cur.skip_space();
    let mut no = 0;
    if ch == 'C' {
        if cur.eq_char('(') {
            cur.next(); // skip '('
            no = cur.get_int(0);
        }
    } else {
        no = cur.get_int(0);
    }
    // .onTime
    if cur.eq_char('.') {
        return read_command_cc(cur, no, song);
    }
    cur.skip_space();
    if !cur.eq_char(',') && !cur.eq_char('(') {
        return Token::new(
            TokenType::Error,
            0,
            vec![SValue::from_s(format!(
                "[ERROR]({}): Faild to set ControlChange[{}] ",
                cur.line + 1,
                ch
            ))],
        );
    }
    if cur.eq_char(',') {
        cur.next(); // skip ','
    }
    let val_token = match read_calc(cur, song) {
        Some(v) => v,
        None => {
            let msg = song.get_message(MessageKind::ScriptSyntaxError);
            read_error(cur, song, msg);
            return Token::new_empty("ERROR", cur.line);
        }
    };
    let cc_token = Token::new_tokens(TokenType::ControlChange, no, vec![val_token]);
    if ch == 'C' {
        cur.skip_space();
        if cur.eq_char(')') {
            cur.next(); // skip ')'
        }
    }
    cc_token
}
