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

/// 数値列を受け取る音符属性オプションを、実行時に評価する引数トークン付きで作る。
fn read_note_int_args_token(
    cur: &mut SourceCursor,
    song: &mut Song,
    ttype: TokenType,
    value_i: isize,
    ino: isize,
) -> Token {
    let mut token = Token::new_tokens(ttype, value_i, read_int_args_tokens(cur, song));
    token.data.push(SValue::from_i(ino));
    token
}

/// 音符属性(v/q/t/o/l)に共通の先行指定を読み取る (#65)
/// target: NOTE_PARAM_V〜NOTE_PARAM_L / ino: サブベロシティの番号(-1で通常のv)
/// 対応していないコマンドのときは None を返す
pub(super) fn read_note_param_option(
    cur: &mut SourceCursor,
    song: &mut Song,
    target: isize,
    cmd: &str,
    ino: isize,
) -> Option<Token> {
    let ino_v = SValue::from_i(ino);
    // 対象ごとのトークン種別を選ぶ
    let pick = |v, q, t, o, l| -> TokenType {
        match target {
            NOTE_PARAM_V => v,
            NOTE_PARAM_Q => q,
            NOTE_PARAM_T => t,
            NOTE_PARAM_O => o,
            _ => l,
        }
    };
    match cmd {
        // 書き込む値にランダムな値を足す
        "Random" => {
            let r = read_arg_value(cur, song);
            let ttype = pick(
                TokenType::VelocityRandom,
                TokenType::QLenRandom,
                TokenType::TimingRandom,
                TokenType::OctaveRandom,
                TokenType::LengthRandom,
            );
            Some(Token::new(ttype, 0, vec![r, ino_v]))
        }
        // 時間ごとの推移的な先行指定
        "onTime" | "T" => {
            let ttype = pick(
                TokenType::VelocityOnTime,
                TokenType::QLenOnTime,
                TokenType::TimingOnTime,
                TokenType::OctaveOnTime,
                TokenType::LengthOnTime,
            );
            Some(read_note_int_args_token(cur, song, ttype, 0, ino))
        }
        // 音符ごとの先行指定
        "onNote" | "N" => {
            let ttype = pick(
                TokenType::VelocityOnNote,
                TokenType::QLenOnNote,
                TokenType::TimingOnNote,
                TokenType::OctaveOnNote,
                TokenType::LengthOnNote,
            );
            Some(read_note_int_args_token(cur, song, ttype, 0, ino))
        }
        // 一定時間ごとの先行指定 (ステップ値, 値1, 値2, ...)
        "onCycle" | "C" => {
            let ttype = pick(
                TokenType::VelocityOnCycle,
                TokenType::QLenOnCycle,
                TokenType::TimingOnCycle,
                TokenType::OctaveOnCycle,
                TokenType::LengthOnCycle,
            );
            Some(read_note_int_args_token(cur, song, ttype, 0, ino))
        }
        // 値の下限と上限を設定する
        "Range" => Some(read_note_int_args_token(
            cur,
            song,
            TokenType::NoteParamRange,
            target,
            ino,
        )),
        // 先行指定の効果の遅延時間
        "Delay" => {
            let v = read_arg_value(cur, song);
            Some(Token::new(TokenType::NoteParamDelay, target, vec![v]))
        }
        // .onNote などで値をくり返すかどうか
        "Repeat" => {
            let on = read_arg_on_off(cur, song);
            Some(Token::new(TokenType::NoteParamRepeat, target, vec![on]))
        }
        // 値の上限を変更する (v/q のみ)
        "Max" => {
            if target != NOTE_PARAM_V && target != NOTE_PARAM_Q {
                return None;
            }
            let v = read_arg_value(cur, song);
            Some(Token::new(TokenType::NoteParamMax, target, vec![v]))
        }
        _ => None,
    }
}

/// 未対応の先行指定コマンドを検出してエラーを出す (#78)
/// 例: `v.onNoteWave(...)` のように解釈できないコマンドを、無言で無視して
/// `v0` などに化けさせないようにする
pub(super) fn read_unknown_on_command(
    cur: &mut SourceCursor,
    song: &mut Song,
    target: &str,
    cmd: &str,
) -> Option<Token> {
    if cmd.len() == 0 {
        return None;
    }
    let _ = read_int_args_tokens(cur, song); // 引数を読み飛ばす
    let mut msg = format!("not supported : {}.{}", target, cmd);
    // 音符の中で音量を波形状に変化させたい場合は、ベロシティではなく
    // エクスプレッション(CC#11)を使う必要があるので、代替手段を案内する
    if cmd == "onNoteWave" || cmd == "W" {
        if target == "v" {
            msg = format!("{} (hint: EP.onNoteWave / y11.onNoteWave)", msg);
        } else {
            msg = format!("{} (hint: CC.onNoteWave / PB.onNoteWave)", msg);
        }
    }
    song.add_log(format!("[ERROR]({}) {}", cur.line, msg));
    Some(Token::new_empty(&msg, cur.line))
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
        if let Some(t) = read_note_param_option(cur, song, NOTE_PARAM_L, &cmd, -1) {
            return t;
        }
        if let Some(t) = read_unknown_on_command(cur, song, "l", &cmd) {
            return t;
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
        if let Some(t) = read_note_param_option(cur, song, NOTE_PARAM_O, &cmd, -1) {
            return t;
        }
        if let Some(t) = read_unknown_on_command(cur, song, "o", &cmd) {
            return t;
        }
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Octave, value.to_i(), vec![value])
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
        if let Some(t) = read_note_param_option(cur, song, NOTE_PARAM_Q, &cmd, -1) {
            return t;
        }
        if let Some(t) = read_unknown_on_command(cur, song, "q", &cmd) {
            return t;
        }
    }
    let value = read_arg_value(cur, song);
    Token::new(TokenType::QLen, value.to_i(), vec![value])
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
    }
    if cur.eq_char('.') {
        cur.next(); // skip '.'
        let cmd = cur.get_word();
        if let Some(t) = read_note_param_option(cur, song, NOTE_PARAM_V, &cmd, ino) {
            return t;
        }
        if let Some(t) = read_unknown_on_command(cur, song, "v", &cmd) {
            return t;
        }
    }
    // v(no)
    let value = read_arg_value(cur, song);
    Token::new(
        TokenType::Velocity,
        value.to_i(),
        vec![value, SValue::from_i(ino)],
    )
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
        if let Some(t) = read_note_param_option(cur, song, NOTE_PARAM_T, &cmd, -1) {
            return t;
        }
        if let Some(t) = read_unknown_on_command(cur, song, "t", &cmd) {
            return t;
        }
    }
    // t(no)
    let value = read_arg_value(cur, song);
    Token::new(TokenType::Timing, value.to_i(), vec![value])
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
