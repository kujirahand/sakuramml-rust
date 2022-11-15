/// Sutoton Mode Converter
use super::cursor::TokenCursor;
use super::token::zen2han;

#[derive(Debug)]
struct SutotonItem {
    name: String,
    value: String,
    length: usize,
}
impl SutotonItem {
    fn from(name: &str, value: &str) -> Self {
        Self {
            name: String::from(name),
            value: String::from(value),
            length: name.chars().count(),
        }
    }
}

struct SutotonList {
    items: Vec<SutotonItem>,
    sorted: bool,
}
impl SutotonList {
    fn new() -> Self {
        Self {
            items: vec![],
            sorted: false,
        }
    }
    fn push(&mut self, it: SutotonItem) {
        self.items.push(it);
        self.sorted = false;
    }
    fn sort_items(&mut self) {
        if self.sorted { return; }
        self.items.sort_by(|a, b| b.name.len().cmp(&a.name.len()));
        self.sorted = true;
    }
    fn set_item(&mut self, name: &str, value: &str) {
        let len = name.chars().count();
        for it in self.items.iter_mut() {
            if it.length != len { continue; }
            if it.name == name {
                it.value = value.to_string();
                return;
            }
        }
        self.items.push(SutotonItem::from(name, value));
        self.sorted = false;
    }
}

fn init_items() -> SutotonList {
    let mut items = SutotonList::new();
    // <SUTOTON>
    items.push(SutotonItem::from("ここから演奏", "?"));
    items.push(SutotonItem::from("テンポ", "TEMPO="));
    items.push(SutotonItem::from("トラック", "TR="));
    items.push(SutotonItem::from("チャンネル", "CH="));
    items.push(SutotonItem::from("全音符", "l1"));
    items.push(SutotonItem::from("二分音符", "l2"));
    items.push(SutotonItem::from("四分音符", "l4"));
    items.push(SutotonItem::from("八分音符", "l8"));
    items.push(SutotonItem::from("十六音符", "l16"));
    items.push(SutotonItem::from("付点四分音符", "l4."));
    items.push(SutotonItem::from("作者", "COPYRIGHT="));
    items.push(SutotonItem::from("曲名", "TrackName="));
    items.push(SutotonItem::from("コメント", "MetaText="));
    items.push(SutotonItem::from("拍子", "TimeSig="));
    items.push(SutotonItem::from("音源初期化", "ResetGM()"));
    items.push(SutotonItem::from("演奏位置", "PlayFrom"));
    items.push(SutotonItem::from("時間", "Time="));
    items.push(SutotonItem::from("音長", "l"));
    items.push(SutotonItem::from("音符", "l"));
    items.push(SutotonItem::from("音階", "o"));
    items.push(SutotonItem::from("音色", "@"));
    items.push(SutotonItem::from("音量", "v"));
    items.push(SutotonItem::from("音量予約", "v.onTime="));
    items.push(SutotonItem::from("ゲート", "q"));
    items.push(SutotonItem::from("連符", "Div"));
    items.push(SutotonItem::from("読む", "Include"));
    items.push(SutotonItem::from("ド", "c"));
    items.push(SutotonItem::from("レ", "d"));
    items.push(SutotonItem::from("ミ", "e"));
    items.push(SutotonItem::from("ファ", "f"));
    items.push(SutotonItem::from("フ", "f"));
    items.push(SutotonItem::from("ソ", "g"));
    items.push(SutotonItem::from("ラ", "a"));
    items.push(SutotonItem::from("シ", "b"));
    items.push(SutotonItem::from("ど", "c"));
    items.push(SutotonItem::from("れ", "d"));
    items.push(SutotonItem::from("み", "e"));
    items.push(SutotonItem::from("ふぁ", "f"));
    items.push(SutotonItem::from("ふ", "f"));
    items.push(SutotonItem::from("そ", "g"));
    items.push(SutotonItem::from("ら", "a"));
    items.push(SutotonItem::from("し", "b"));
    items.push(SutotonItem::from("ン", "r"));
    items.push(SutotonItem::from("ッ", "r"));
    items.push(SutotonItem::from("ー", "^"));
    items.push(SutotonItem::from("「", "'"));
    items.push(SutotonItem::from("」", "'"));
    items.push(SutotonItem::from("【", "["));
    items.push(SutotonItem::from("】", "]"));
    items.push(SutotonItem::from("↑", ">"));
    items.push(SutotonItem::from("↓", "<"));
    items.push(SutotonItem::from("っ", "r"));
    items.push(SutotonItem::from("ん", "r"));
    items.push(SutotonItem::from("♭", "-"));
    items.push(SutotonItem::from("♯", "#"));
    items.push(SutotonItem::from("調", "KeyFlag="));
    items.push(SutotonItem::from("ど", "n36,"));
    items.push(SutotonItem::from("た", "n38,"));
    items.push(SutotonItem::from("つ", "n42,"));
    items.push(SutotonItem::from("く", "n44,"));
    items.push(SutotonItem::from("ち", "n46,"));
    items.push(SutotonItem::from("ぱ", "n49,"));
    items.push(SutotonItem::from("−", "-"));
    items.push(SutotonItem::from("‘", "`"));
    items.push(SutotonItem::from("変", "-"));
    items.push(SutotonItem::from("嬰", "#"));
    items.push(SutotonItem::from("リズム", "RHYTHM"));
    items.push(SutotonItem::from("イ", "a"));
    items.push(SutotonItem::from("ロ", "b"));
    items.push(SutotonItem::from("ハ", "c"));
    items.push(SutotonItem::from("ニ", "d"));
    items.push(SutotonItem::from("ホ", "e"));
    items.push(SutotonItem::from("ヘ", "f"));
    items.push(SutotonItem::from("ト", "g"));
    // </SUTOTON>
    items.sort_items();
    items
}

pub fn convert(src: &str) -> String {
    let mut items = init_items();
    let mut res = String::new();
    let mut cur = TokenCursor::from(src);
    while !cur.is_eos() {
        let ch = zen2han(cur.peek_n(0));
        // string ?
        match ch {
            '{' => {
                if cur.eq("{\"") {
                    let s = cur.get_token_s("\"}");
                    res.push_str(&s);
                    res.push_str("\"}");
                    continue;
                }
                res.push(ch);
                cur.next();
                continue;
            },
            /*
            '\u{0020}'..='\u{007D}' => {
                res.push(ch);
                cur.next();
                continue;
            },
            */
            // add item
            '~' | '‾' => {
                cur.next(); // skip '~'
                cur.skip_space();
                if cur.peek_n(0) != '{' { continue; }
                let name = cur.get_token_nest('{', '}');
                cur.skip_space();
                if cur.eq_char('=') { cur.next(); } // skip '='
                cur.skip_space();
                if cur.peek_n(0) != '{' { continue; }
                let value = cur.get_token_nest('{', '}');
                items.set_item(&name, &value);
                items.sort_items();
                continue;
            }
            _ => {
            }
        }
        // check sutoton
        let mut found = false;
        for cmd in items.items.iter() {
            if cur.eq(&cmd.name) {
                res.push_str(&cmd.value);
                cur.index += cmd.length;
                found = true;
                break;
            }
        }
        if !found {
            res.push(ch);
            cur.index += 1;
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(convert("ドレミ"), String::from("cde"));
        assert_eq!(convert("トラック3ドレミ"), String::from("TR=3cde"));
    }
    #[test]
    fn test_ex() {
        assert_eq!(convert("~{ど}={c}ドレミどレミ"), String::from("cdecde"));
        assert_eq!(convert("~{じゅー}={c}ドレミじゅーレミ"), String::from("cdecde"));
    }
}

