/// message calalogue

#[derive(Debug)]
pub enum MessageKind {
    UnknownChar,
    UnknownCommand,
    Near,
    TooManyErrorsInLexer,
}

#[derive(Debug)]
pub enum MessageLang {
    EN,
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
        }
    }
}