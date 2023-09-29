/// message calalogue

/// Message Kind
#[derive(Debug)]
pub enum MessageKind {
    UnknownChar,
    UnknownCommand,
    UnknownError,
    Near,
    TooManyErrorsInLexer,
    ScriptSyntaxError,
    ScriptSyntaxWarning,
    MissingParenthesis,
    LoopTooManyTimes,
    ErrorRedfineFnuction,
    RuntimeError,
    ErrorDefineVariableIsReserved,
}

/// Language
#[derive(Debug)]
pub enum MessageLang {
    /// English
    EN,
    /// Japanese
    JA,
}
impl MessageLang {
    pub fn from(code: &str) -> Self {
        match code {
            "en" => Self::EN,
            "ja" => Self::JA,
            _ => Self::EN,
        }
    }
}

#[derive(Debug)]
pub struct MessageData {
    pub lang: MessageLang,
}

impl MessageData {
    pub fn new(lang: MessageLang) -> Self {
        MessageData {
            lang,
        }
    }
    pub fn get(&self, kind: MessageKind) -> &'static str {
        get_message(&self.lang, kind)
    }
}

pub fn get_message(lang: &MessageLang, kind: MessageKind) -> &'static str {
    match kind {
        MessageKind::UnknownChar => {
            match lang {
                MessageLang::EN => "Unknown Character",
                MessageLang::JA => "未定義の文字",
            }
        },
        MessageKind::UnknownCommand => {
            match lang {
                MessageLang::EN => "Unknown Command",
                MessageLang::JA => "未定義のコマンド",
            }
        },
        MessageKind::UnknownError => {
            match lang {
                MessageLang::EN => "Unknown Error",
                MessageLang::JA => "未定義のエラー",
            }
        },
        MessageKind::Near => {
            match lang {
                MessageLang::EN => "near",
                MessageLang::JA => "続く部分",
            }
        },
        MessageKind::TooManyErrorsInLexer => {
            match lang {
                MessageLang::EN => "Too many errors in Lexer",
                MessageLang::JA => "字句解析でエラーが多いので省略します",
            }
        },
        MessageKind::ScriptSyntaxError => {
            match lang {
                MessageLang::EN => "Syntax Error",
                MessageLang::JA => "構文エラー",
            }
        },
        MessageKind::ScriptSyntaxWarning => {
            match lang {
                MessageLang::EN => "Script Syntax Warning",
                MessageLang::JA => "スクリプトの警告",
            }
        },
        MessageKind::MissingParenthesis => {
            match lang {
                MessageLang::EN => "Missing Parenthesis",
                MessageLang::JA => "括弧が閉じられていません",
            }
        },
        MessageKind::LoopTooManyTimes => {
            match lang {
                MessageLang::EN => "Loop too many times",
                MessageLang::JA => "ループが制限を超えました",
            }
        },
        MessageKind::ErrorRedfineFnuction => {
            match lang {
                MessageLang::EN => "Redefine Function",
                MessageLang::JA => "関数の再定義",
            }
        },
        MessageKind::RuntimeError => {
            match lang {
                MessageLang::EN => "Runtime Error",
                MessageLang::JA => "実行時エラー",
            }
        },
        MessageKind::ErrorDefineVariableIsReserved => {
            match lang {
                MessageLang::EN => "Define Variable is Reserved",
                MessageLang::JA => "変数の定義が予約語です",
            }
        }
    }
}