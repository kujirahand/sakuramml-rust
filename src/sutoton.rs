/// Sutoton Mode Converter
use super::cursor::TokenCursor;
use super::token::zen2han;

#[derive(Debug, Clone)]
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
    fn sort_items(&mut self) {
        if self.sorted {
            return;
        }
        self.items.sort_by(|a, b| b.name.len().cmp(&a.name.len()));
        self.sorted = true;
    }
    fn set_item(&mut self, name: &str, value: &str) {
        let len = name.chars().count();
        for it in self.items.iter_mut() {
            if it.length != len {
                continue;
            }
            if it.name == name {
                it.value = value.to_string();
                return;
            }
        }
        let item = SutotonItem::from(name, value);
        self.items.push(item);
        self.sorted = false;
    }
}

fn init_items() -> SutotonList {
    let mut items = SutotonList::new();
    // <SUTOTON>
    items.set_item("全音符", "l1"); // 全音符を基本音符にする
    items.set_item("二分音符", "l2"); // 二分音符を基本音符にする
    items.set_item("四分音符", "l4"); // 四分音符を基本音符にする
    items.set_item("八分音符", "l8"); // 八分音符を基本音符にする
    items.set_item("十六音符", "l16"); // 十六音符を基本音符にする
    items.set_item("付点四分音符", "l4."); // 付点四分音符を基本音符にする
    items.set_item("音源初期化", "System.MeasureShift(1);ResetGM;Time(1:1:0);TrackSync;"); // @ 音源初期化//音源の初期化(GMリセット)を実行する。（例）音源初期化
    items.set_item("音長", "l"); // 基本音符を指定
    items.set_item("音量予約", "v.onTime="); // 音量を予約指定する
    items.set_item("「", "'"); // @ 和音はじめ
    items.set_item("」", "'"); // @ 和音おわり
    items.set_item("【", "["); // @ 繰り返しはじめ
    items.set_item("】", "]"); // @ 繰り返しおわり
    items.set_item("↑", ">"); // @ オクターブを1つ上げる
    items.set_item("↓", "<"); // @ オクターブを1つ下げる
    items.set_item("♭", "-"); // @ フラット
    items.set_item("♯", "#"); // @ シャープ
    items.set_item("−", "-"); // @ マイナス
    items.set_item("‘", "`"); // @ 次の音符をオクターブ1つ上げる
    items.set_item("調", "System.KeyFlag"); // @ 調#(音符)//臨時記号を設定する。（例）調＃（ドファ）
    items.set_item("音階", "o"); // @ 音階(数値)//音階を数値で指定する。初期値は５。範囲は、0～10（例）音階５
    items.set_item("時間", "Time"); // @ 時間(小節数:拍数:ステップ数)//指定時間にポインタを移動する。範囲は、小節数・拍数が、１～。ステップ数は、０～。（例）時間（４：１：０）
    items.set_item("読む", "Include"); // @ 読む(ファイル名)//外部定義ファイルを読み込む。（例）読む(chord2.h)
    items.set_item("予約", ".onNote="); // @ (コマンド)予約(v1,v2,v3...)//コマンドの値を予約しておく（例）音量予約120,50【ドレミファ】
    items.set_item("拍子", "System.TimeSignature="); // @ 拍子 分子,分母//拍子を設定する。（例）拍子4,4
    items.set_item("音色", "@"); // @ 音色（番号）//音色を設定する。
    items.set_item("音符", "l"); // @ 音符（ｎ分音符指定）//基本となる音符の長さをｎ分音符で指定する。（例）音符16//１６分音符の意味
    items.set_item("音量", "v"); // @ 音量（数値）//音量(実際は音の強さ)を設定する。初期値は、100。範囲は、0~127。（例）音量127
    items.set_item("連符", "Div"); // @ 連符{音名}[音長]//３連符や５連符などを表現する。（例）連符{ドレミ}4
    items.set_item("ゲート", "q"); // @ ゲート（割合）//音符の長さに対する実際の発音時間を割合（100分率）で指定する。範囲は、1～100～。（例）ゲート80
    items.set_item("テンポ", "Tempo="); // @ テンポ（数値）//テンポを設定する。初期値は、120。範囲は、20～240を推奨。（例）テンポ120
    items.set_item("曖昧さ", ".Random="); // @ (コマンド)曖昧さ（数値）//各属性の曖昧さを設定する。範囲は、0～。初期値は、0。（例）音量曖昧さ80 【ドレミソ】
    items.set_item("トラック", "Track="); // @ トラック（番号）//トラック番号を指定する。初期値は、０。範囲は、0～。（例）トラック３
    items.set_item("チャンネル", "Channel="); // @ チャンネル（番号）//現在のトラックにチャンネルを設定する。初期値は、トラック番号と同じ。範囲は、１～１６（例）トラック３チャンネル１０
    items.set_item("曲名", "TrackName="); // @ 曲名{"文字列"}//生成するMIDIファイルに曲名を埋め込む。（例）曲名{"テスト"}
    items.set_item("作者", "Copyright="); // @ 作者{"文字列"}//生成するMIDIファイルに著作権情報を埋め込む。（例）作者{"クジラ飛行机"}
    items.set_item("コメント", "MetaText="); // @ コメント{"文字列"}//生成するMIDIファイルにコメントを埋め込む。（例）コメント{"テスト"}
    items.set_item("演奏位置", "PlayFrom"); // @ 演奏位置(小節数:拍数:ステップ数))//長い曲の途中から演奏したい時、曲の演奏位置を指定する。（例）演奏位置（32:1:0）
    items.set_item("ー", "^"); // @ ー//タイ。音を伸ばす。（例）ドードレミミソーーー
    items.set_item("上", ">"); // @ 音階を相対的に１つ上げる
    items.set_item("下", "<"); // @ 音階を相対的に１つ下げる
    items.set_item("ド", "c"); // @ 音名
    items.set_item("レ", "d"); // @ 音名
    items.set_item("ミ", "e"); // @ 音名
    items.set_item("フ", "f"); // @ 音名
    items.set_item("ァ", ""); // @ 音名
    items.set_item("ソ", "g"); // @ 音名
    items.set_item("ラ", "a"); // @ 音名
    items.set_item("シ", "b"); // @ 音名
    items.set_item("ン", "r"); // @ 休符。（例）ドーーン　レンレー
    items.set_item("ッ", "r"); // @ 休符。（例）ドーーッ　レッレー
    items.set_item("ど", "c"); // @ 音名
    items.set_item("れ", "d"); // @ 音名
    items.set_item("み", "e"); // @ 音名
    items.set_item("ふ", "f"); // @ 音名
    items.set_item("ぁ", ""); // @ 音名
    items.set_item("そ", "g"); // @ 音名
    items.set_item("ら", "a"); // @ 音名
    items.set_item("し", "b"); // @ 音名
    items.set_item("ん", "r"); // @ 休符
    items.set_item("っ", "r"); // @ 休符
    items.set_item("イ", "a"); // @ 音名
    items.set_item("ロ", "b"); // @ 音名
    items.set_item("ハ", "c"); // @ 音名
    items.set_item("ニ", "d"); // @ 音名
    items.set_item("ホ", "e"); // @ 音名
    items.set_item("ヘ", "f"); // @ 音名
    items.set_item("ト", "g"); // @ 音名
    items.set_item("変", "-"); // @ フラット（例）イ変
    items.set_item("嬰", "+"); // @ シャープ
    items.set_item("リズム", "Rythm"); // @ リズムモード
    items.set_item("ず", "n36,"); // @ バスドラム
    items.set_item("た", "n38,"); // @ スネアドラム
    items.set_item("つ", "n42,"); // @ ハイハット（クローズ）
    items.set_item("ち", "n46,"); // @ ハイハット（オープン）
    items.set_item("ぱ", "n49,"); // @ シンバル
    items.set_item("と", "n50,"); // @ Lowタム
    items.set_item("む", "n47,"); // @ Midタム
    items.set_item("ろ", "n43,"); // @ Highタム
    items.set_item("く", "n44,"); // @ ドラム
    items.set_item("大きく", "Cresc="); // @ 大きく(音長),(最終値)//音量(EP)をだんだん大きくする
    items.set_item("小さく", "Decresc="); // @ 小さく(音長),(最終値)//音量(EP)をだんだん小さくする
    items.set_item("クレッシェンド", "Cresc="); // @ 大きく(音長),(最終値)//音量(EP)をだんだん大きくする
    items.set_item("デクレッシェンド", "Cresc="); // @ 小さく(音長),(最終値)//音量(EP)をだんだん小さくする
    items.set_item("音量戻す", "EP(127)"); // @ 音量(EP)を最大値に戻す
    items.set_item("方向左", "P(0)"); // @ ステレオの左から音が出るようにする
    items.set_item("方向左前", "P(32)"); // @ ステレオの左前から音が出るようにする
    items.set_item("方向前", "P(64)"); // @ ステレオの前から音が出るようにする
    items.set_item("方向右前", "P(96)"); // @ ステレオの右前から音が出るようにする
    items.set_item("方向右", "P(127)"); // @ ステレオの右から音が出るようにする
    items.set_item("方向回す", "P.onNoteWaveEx(0,127,!1,127,0,!1);"); // @ ステレオの左右を回す
    items.set_item("ビブラートオフ", "M(0)"); // @ ビブラートをやめる
    items.set_item("ペダル", "y64,127;"); // @ ペダルを踏む
    items.set_item("放す", "y64,0;"); // @ ペダルを放す
    items.set_item("テンポ改", "TempoChange="); // @ テンポ改([[[t1],t2],len])//テンポを推移的に変更する。lenを省略すると、全音符の間に推移し、t1を省略すると、以前の値からt2へ推移する。
    items.set_item("ビブラート", "M.onNoteWaveEx(0,0,!4,0,96,!8);"); // @ 推移的なビブラートをかける
    items.set_item("ここから演奏", "PlayFrom(Time);"); // @ 途中から演奏したいときに書く
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
            }
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
                if cur.peek_n(0) != '{' {
                    continue;
                }
                let name = cur.get_token_nest('{', '}');
                cur.skip_space();
                if cur.eq_char('=') {
                    cur.next();
                } // skip '='
                cur.skip_space();
                if cur.peek_n(0) != '{' {
                    continue;
                }
                let value = cur.get_token_nest('{', '}');
                items.set_item(&name, &value);
                items.sort_items();
                continue;
            }
            _ => {}
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
        assert_eq!(convert("トラック3ドレミ"), String::from("Track=3cde"));
    }
    #[test]
    fn test_ex() {
        assert_eq!(convert("~{ど}={c}ドレミどレミ"), String::from("cdecde"));
        assert_eq!(
            convert("~{じゅー}={c}ドレミじゅーレミ"),
            String::from("cdecde")
        );
    }
}
