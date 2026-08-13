//! lexer: コントロールチェンジ系コマンドの読み取り
use super::*;

pub(super) fn read_sysex(cur: &mut SourceCursor, _song: &mut Song) -> Token {
    // read sysex
    let lineno = cur.line;
    let hex_mode = if cur.eq_char('$') {
        cur.next();
        true
    } else {
        false
    };
    if cur.eq_char('=') {
        cur.next();
    } // skip '='
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
                    let mut t = Token::new(
                        TokenType::Value,
                        0,
                        vec![SValue::from_s(format!("={}", var_name))],
                    );
                    t.value_type = TokenValueType::VARIABLE;
                    t.lineno = lineno;
                    data_vec.push(t);
                }
                _ => {}
            }
        }
        cur.skip_space();
        if cur.eq_char('}') {
            // 数値の後に'}'がある場合を考慮
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
    Token::new(TokenType::FadeIO, dir, vec![arg])
}

pub(super) fn read_decres(cur: &mut SourceCursor, song: &mut Song, dir: isize) -> Token {
    let mut v1 = SValue::from_i(if dir < 0 { 127 } else { 40 });
    let mut v2 = SValue::from_i(if dir < 0 { 40 } else { 127 });
    // skip =
    cur.skip_space();
    if cur.eq_char('=') {
        cur.next();
    }
    // length
    let len_s = cur.get_note_length();
    cur.skip_space();
    if cur.eq_char(',') {
        cur.next();
        cur.skip_space();
        v1 = read_arg_value(cur, song);
        cur.skip_space();
        if cur.eq_char(',') {
            cur.next();
            cur.skip_space();
            v2 = read_arg_value(cur, song);
        }
    }
    return Token::new(TokenType::Decresc, 0, vec![SValue::from_s(len_s), v1, v2]);
}

/// 先行指定の書き込み先(CC番号 / ピッチベンド)
#[derive(Clone, Copy)]
pub(super) enum CCTarget {
    /// コントロールチェンジ(番号)
    CC(isize),
    /// ピッチベンド (is_big: 1=PB / 0=p)
    PitchBend(isize),
}

impl CCTarget {
    /// 書き込み先を表す value_i
    fn value_i(&self) -> isize {
        match self {
            CCTarget::CC(no) => *no,
            CCTarget::PitchBend(is_big) => {
                if *is_big == 0 {
                    WRITE_TARGET_PB_SMALL
                } else {
                    WRITE_TARGET_PB_BIG
                }
            }
        }
    }
    /// エラーメッセージ用の名前
    fn name(&self) -> String {
        match self {
            CCTarget::CC(no) => format!("CC({})", no),
            CCTarget::PitchBend(is_big) => String::from(if *is_big == 0 { "p" } else { "PB" }),
        }
    }
}

/// 数値列を受け取るCCオプションを、実行時に評価する引数トークン付きで作る。
fn read_cc_int_args_token(
    cur: &mut SourceCursor,
    song: &mut Song,
    ttype: TokenType,
    value_i: isize,
) -> Token {
    Token::new_tokens(ttype, value_i, read_int_args_tokens(cur, song))
}

/// CC・ピッチベンドの先行指定を読み取る (#65)
/// 対応していないコマンドのときは None を返す
pub(super) fn read_cc_option(
    cur: &mut SourceCursor,
    song: &mut Song,
    target: CCTarget,
    cmd: &str,
) -> Option<Token> {
    let tv = target.value_i();
    match cmd {
        // 一度に low から high へ値を書き込む
        "onTime" | "T" => {
            let (ttype, value_i) = match target {
                CCTarget::CC(no) => (TokenType::CConTime, no),
                CCTarget::PitchBend(is_big) => (TokenType::PBonTime, is_big),
            };
            Some(read_cc_int_args_token(cur, song, ttype, value_i))
        }
        // ノートオン毎の値の先行指定
        "onNote" | "N" => {
            let (ttype, value_i) = match target {
                CCTarget::CC(no) => (TokenType::CConNote, no),
                CCTarget::PitchBend(is_big) => (TokenType::PBonNote, is_big),
            };
            Some(read_cc_int_args_token(cur, song, ttype, value_i))
        }
        // ノートオン毎に直線的な値の推移を書き込む
        "onNoteWave" | "W" => {
            let (ttype, value_i) = match target {
                CCTarget::CC(no) => (TokenType::CConNoteWave, no),
                CCTarget::PitchBend(is_big) => (TokenType::PBonNoteWave, is_big),
            };
            Some(read_cc_int_args_token(cur, song, ttype, value_i))
        }
        // ノートオン毎に、ノートの長さに応じた波形を書き込む
        "onNoteWaveEx" | "WE" => Some(read_cc_int_args_token(
            cur,
            song,
            TokenType::CConNoteWaveEx,
            tv,
        )),
        // ノートオンしている間、波形をくり返す
        "onNoteWaveR" | "WR" => Some(read_cc_int_args_token(
            cur,
            song,
            TokenType::CConNoteWaveR,
            tv,
        )),
        // 一定時間ごとの値の先行指定 (ステップ値, 値1, 値2, ...)
        "onCycle" | "C" => Some(read_cc_int_args_token(cur, song, TokenType::CConCycle, tv)),
        // 正弦波を1回だけ書き込む (type,low,high,len,times)
        "Sine" => Some(read_cc_int_args_token(cur, song, TokenType::CCSine, tv)),
        // ノートオン毎に正弦波を書き込む (type,low,high,len,times)
        "onNoteSine" => Some(read_cc_int_args_token(
            cur,
            song,
            TokenType::CConNoteSine,
            tv,
        )),
        // 書き込み頻度の指定
        "Frequency" => {
            let a = read_arg_value(cur, song);
            Some(Token::new(TokenType::CConTimeFreq, tv, vec![a]))
        }
        // 先行指定の効果の遅延時間
        "Delay" => {
            let a = read_arg_value(cur, song);
            Some(Token::new(TokenType::CCDelay, tv, vec![a]))
        }
        // 書き込まれる値にランダムな値を足す
        "Random" => {
            let a = read_arg_value(cur, song);
            Some(Token::new(TokenType::CCRandom, tv, vec![a]))
        }
        // 書き込まれる値に下限と上限を設定する
        "Range" => Some(read_cc_int_args_token(cur, song, TokenType::CCRange, tv)),
        // .onNote などで値をくり返すかどうか
        "Repeat" => {
            let on = read_arg_on_off(cur, song);
            Some(Token::new(TokenType::CCRepeat, tv, vec![on]))
        }
        _ => None,
    }
}

/// 未対応の先行指定コマンドを検出してエラーを出す (#65)
/// 引数を読み飛ばさないと `M.Delay(10)` が `M(10)` に化けてしまう
pub(super) fn read_cc_unknown_option(
    cur: &mut SourceCursor,
    song: &mut Song,
    target: CCTarget,
    cmd: &str,
) -> Token {
    let _ = read_int_args_tokens(cur, song); // 引数を読み飛ばす
    let msg = format!("not supported : {}.{}", target.name(), cmd);
    song.add_log(format!("[ERROR]({}) {}", cur.line, msg));
    Token::new_empty(&msg, cur.line)
}

/// read command CC
pub(super) fn read_command_cc(cur: &mut SourceCursor, no: isize, song: &mut Song) -> Token {
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        let target = CCTarget::CC(no);
        if let Some(t) = read_cc_option(cur, song, target, &cmd) {
            return t;
        }
        // 解釈できない指定を、無言で `CC(値)` に化けさせない
        return read_cc_unknown_option(cur, song, target, &cmd);
    }
    if cur.eq_char('=') {
        cur.next();
    }
    let value_tokens = read_args_tokens(cur, song);

    return Token::new_tokens(TokenType::ControlChange, no, value_tokens);
}

pub(super) fn read_rpn_command(
    cur: &mut SourceCursor,
    msb: isize,
    lsb: isize,
    song: &mut Song,
) -> Token {
    let args = read_args_tokens(cur, song);
    let token = Token::new_data_tokens(
        TokenType::RPNCommand,
        0,
        vec![SValue::Int(msb), SValue::Int(lsb)],
        args,
    );
    token
}

pub(super) fn read_nrpn_command(
    cur: &mut SourceCursor,
    msb: isize,
    lsb: isize,
    song: &mut Song,
) -> Token {
    let args = read_args_tokens(cur, song);
    let token = Token::new_data_tokens(
        TokenType::NRPNCommand,
        0,
        vec![SValue::Int(msb), SValue::Int(lsb)],
        args,
    );
    token
}

pub(super) fn read_voice(cur: &mut SourceCursor, song: &mut Song) -> Token {
    let args = read_args_tokens(cur, song);
    Token::new_tokens(TokenType::Voice, 0, args)
}

/// ピッチベンドの先行指定を読み取る
/// is_big: 1=PB(-8192〜8191) / 0=p(0〜127)
fn read_command_pitch_bend(
    cur: &mut SourceCursor,
    song: &mut Song,
    is_big: isize,
) -> Option<Token> {
    if !cur.eq_char('.') {
        return None;
    }
    cur.next(); // skip '.'
    let cmd = cur.get_word();
    let target = CCTarget::PitchBend(is_big);
    if let Some(t) = read_cc_option(cur, song, target, &cmd) {
        return Some(t);
    }
    // 解釈できない指定を、無言で値指定に化けさせない
    Some(read_cc_unknown_option(cur, song, target, &cmd))
}

pub(super) fn read_command_pitch_bend_big(cur: &mut SourceCursor, song: &mut Song) -> Token {
    if let Some(t) = read_command_pitch_bend(cur, song, 1) {
        return t;
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::PitchBend, 1, vec![value])
}

pub(super) fn read_pitch_bend_small(cur: &mut SourceCursor, song: &mut Song) -> Token {
    if let Some(t) = read_command_pitch_bend(cur, song, 0) {
        return t;
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
