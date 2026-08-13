//! lexer: エラー・警告ログの生成
use super::*;

pub(super) const LEX_MAX_ERROR: usize = 30;

/// append error log for lex
pub(super) fn lex_error(cur: &mut SourceCursor, song: &mut Song, msg: &str) {
    // make error log
    let mut near = cur.peek_str_n(8).replace('\n', "↵");
    if near.len() == 0 {
        near = "[EOS]".to_string();
    }
    let log = format!(
        "[ERROR]({}) {}: \"{}\" {} \"{}\"",
        cur.line,
        song.get_message(MessageKind::UnknownChar),
        msg,
        song.get_message(MessageKind::Near),
        near
    );
    if song.debug {
        println!("{}", log);
    }
    // add to logs
    if song.get_logs_len() == LEX_MAX_ERROR {
        song.add_log(format!(
            "[ERROR]({}) {}",
            cur.line,
            song.get_message(MessageKind::TooManyErrorsInLexer)
        ));
    } else if song.get_logs_len() < LEX_MAX_ERROR {
        song.add_log(log);
    }
}

pub(super) fn read_error_cmd(cur: &mut SourceCursor, song: &mut Song, cmd: &str) -> Token {
    let near = cur.peek_str_n(8).replace('\n', "↵");
    let error_log = format!(
        "[ERROR]({}) {} \"{}\" {} \"{}\"",
        cur.line,
        song.get_message(MessageKind::ScriptSyntaxError),
        cmd,
        song.get_message(MessageKind::Near),
        near,
    );
    if song.debug {
        println!("{}", error_log);
    }
    song.add_log(error_log);
    return Token::new_empty("ERROR", cur.line);
}

pub(super) fn read_error(cur: &mut SourceCursor, song: &mut Song, msg: &str) -> Token {
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

pub(super) fn read_warning(
    cur: &mut SourceCursor,
    song: &mut Song,
    cmd: &str,
    reason: &str,
) -> Token {
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
