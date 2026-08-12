//! lexer: 大文字コマンドの読み取り
use super::*;

/// read Upper case commands
pub(super) fn read_upper_command(cur: &mut SourceCursor, song: &mut Song) -> Token {
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

    let lineno = cur.line;
    // SystemFunction ?
    if let Some(f) = song.system_functions.get(&cmd) {
        let arg_t = f.arg_type;
        let token_t = f.token_type;
        let tag1 = f.tag1;
        let tag2 = f.tag2;
        match arg_t {
            'I' | 'S' | 'A' => {
                cur.skip_space();
                if cur.eq_char('=') { cur.next(); }
                let args = read_args_tokens(cur, song);
                return Token::new_tokens_lineno(token_t, tag1, args, lineno);
            },
            '_' => { // no paramerter
                return Token::new_tokens_lineno(token_t, tag1, vec![], lineno);
            },
            _ => {
                // 例外的に読み取り処理が必要な特別コマンド
                match token_t {
                    TokenType::Rhythm => return read_command_rhythm(cur, song),
                    TokenType::Div => return read_command_div(cur, song, false),
                    TokenType::Sub => return read_command_sub(cur, song),
                    TokenType::KeyFlag => return read_key_flag(cur, song),
                    TokenType::DefInt => return read_def_var(cur, song, TokenValueType::INT),
                    TokenType::DefStr => return read_def_var(cur, song, TokenValueType::STR),
                    TokenType::DefArray => return read_def_var(cur, song, TokenValueType::ARRAY),
                    TokenType::Play => return read_play(cur, song),
                    TokenType::TimeBase => return read_timebase(cur, song),
                    TokenType::Include => return read_include(cur, song),
                    TokenType::ControlChange => return read_cc(cur, song, 'C'),
                    TokenType::ControlChangeCommand => return read_command_cc(cur, tag1, song),
                    TokenType::PitchBend => return read_command_pitch_bend_big(cur, song),
                    TokenType::RPNCommand => return read_rpn_command(cur, tag1, tag2, song),
                    TokenType::NRPNCommand => return read_nrpn_command(cur, tag1, tag2, song),
                    TokenType::FadeIO => return read_fadein(cur, song, tag1),
                    TokenType::Cresc => return read_decres(cur, song, tag1),
                    TokenType::If => return read_if(cur, song),
                    TokenType::For => return read_for(cur, song),
                    TokenType::While => return read_while(cur, song),
                    TokenType::SysEx => return read_sysex(cur, song),
                    TokenType::UseKeyShift => return read_use_key_shift(cur, song),
                    TokenType::Return => {
                        cur.skip_space();
                        let values = if cur.eq_char('(') {
                            read_args_tokens(cur, song)
                        } else {
                            vec![Token::new(TokenType::Value, LEX_VALUE, vec![SValue::from_i(0)])]
                        };
                        return Token::new_tokens(TokenType::Return, 0, values);
                    },
                    TokenType::SetRandomSeed => {
                        let v = read_arg_value(cur, song);
                        song.rand_seed = v.to_i() as u32;
                        return Token::new(TokenType::SetConfig, 0, vec![
                            SValue::from_str("RandomSeed"),
                            v
                        ]);
                    },
                    TokenType::DefUserFunction => return read_def_user_function(cur, song),
                    _ => {
                        println!("[SYSTEM_ERROR] FUNCTION NOT SET : {}", cmd);
                    },
                }
            }
        };
    }
    //
    // check variable
    //
    match check_variables(cur, song, cmd.clone()) {
        Some(res) => return res,
        None => {}
    }
    read_error_cmd(cur, song, &cmd);
    return Token::new_empty(&cmd, cur.line);
}

pub(super) fn read_include(cur: &mut SourceCursor, _song: &mut Song) -> Token {
    // @ 未実装
    cur.skip_space();
    let filename = if cur.eq_char('(') {
        cur.get_token_nest('(', ')')
    } else {
        "".to_string()
    };
    return Token::new_empty(&format!("Unimplemented Include({})", filename), cur.line);
}

pub(super) fn read_timebase(cur: &mut SourceCursor, song: &mut Song) -> Token {
    // タイムベースの変更は慎重さが求められるため音符書き込み後の変更は警告を出す
    if song.timebase_changed {
        let msg = song.get_message(MessageKind::WarningChangeTimebaseAfterNote);
        read_warning(cur, song, "TIMEBASE", &msg);
    }
    let v_opt = read_arg_const_int(cur);
    if v_opt.is_none() {
        let msg = song.get_message(MessageKind::ShouldBeConstant);
        return read_error(cur, song, msg);
    }
    song.timebase = v_opt.unwrap_or(96);
    if song.timebase <= 48 {
        song.timebase = 48;
    }
    song.timebase_changed = true;
    Token::new_comment(&format!("TIMEBASE={}", song.timebase), cur.line)
}

pub(super) fn read_key_flag(cur: &mut SourceCursor, _song: &mut Song) -> Token {
    let mut flag = 1;
    let mut key_flag = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // c, c#,d, d#,e, f, f#,g, g#,a, a#,b
    // --- key_flag means ---
    //                      0  1  2  3  4  5  6  7  8  9 10 11
    //                      c, c#,d, d#,e, f, f#,g, g#,a, a#,b
    // --- converter ---
    //                      a, b, c, d, e, f, g
    let key_flag_index_a = [9,11, 0, 2, 4, 5, 7];
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
        let mut plus_minus = 1;
        if cur.eq_char('+') {
            cur.next();
        } else if cur.eq_char('-') {
            cur.next();
            plus_minus = -1;
        }
        // number
        if cur.is_numeric() {
            let v = cur.get_int(0) * plus_minus;
            if key_flag_index_a.len() <= idx { continue; }
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

pub(super) fn read_play(cur: &mut SourceCursor, song: &mut Song) -> Token {
    let lineno = cur.line;
    let arg_tokens = read_args_tokens(cur, song);
    let play_tok = Token::new_tokens_lineno(TokenType::Play, 0, arg_tokens, lineno);
    play_tok
}

pub(super) fn read_use_key_shift(cur: &mut SourceCursor, song: &mut Song) -> Token {
    cur.skip_space();
    if cur.eq_char('=') || cur.eq_char('(') {
        cur.next();
        cur.skip_space();
    }
    let v = if cur.eq("on") || cur.eq("ON") {
        cur.next_n(2);
        1
    } else if cur.eq("off") || cur.eq("OFF") {
        cur.next_n(3);
        0
    } else {
        read_arg_value(cur, song).to_i()
    };
    if cur.eq_char(')') {
        cur.next();
    }
    Token::new(TokenType::UseKeyShift, v, vec![])
}

pub(super) fn read_command_sub(cur: &mut SourceCursor, song: &mut Song) -> Token {
    cur.skip_space();
    let lineno = cur.line; // ブロックを読む前の行番号が本体の先頭行
    let block = cur.get_token_nest('{', '}');
    let tokens = lex(song, &block, lineno);
    let mut tok = Token::new(TokenType::Sub, 0, vec![]);
    tok.children = Some(tokens);
    tok
}

pub(super) fn read_tie_error(cur: &mut SourceCursor, _: &mut Song) -> Token {
    Token::new_empty("[ERROR] tie", cur.line)
}

pub(super) fn read_command_div(cur: &mut SourceCursor, song: &mut Song, need2back: bool) -> Token {
    // is 1char command
    if need2back {
        cur.prev();
    } else {
        cur.skip_space();
    }
    let lineno = cur.line; // ブロックを読む前の行番号が本体の先頭行
    let block = cur.get_token_nest('{', '}');
    let len_s = cur.get_note_length();
    let tokens = lex(song, &block, lineno);
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

pub(super) fn read_command_rhythm(cur: &mut SourceCursor, song: &mut Song) -> Token {
    let mut result = String::new();
    cur.skip_space();
    let line_start = cur.line;
    let block = cur.get_token_nest('{', '}');
    // extract macro
    let mut macro_cur = SourceCursor::from(&block);
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

pub(super) fn read_def_rhythm_macro(cur: &mut SourceCursor, song: &mut Song) {
    let ch = cur.get_char(); // get macro char
    // println!("macro={}", ch);
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
