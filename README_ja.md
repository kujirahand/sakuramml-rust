# テキスト音楽サクラ(Rust版) / ピコサクラ

サクラはMML(Music Macro Language)からMIDIファイルに変換するコンパイラです。
「ドレミ」や「cde」のテキストをMIDIファイルに変換するコンパイラです。
Rustで作られておりマルチプラットフォーム(macOS/Windows/Linux/WebAssembly)で動作します。

サクラは2000年以前に開発された歴史ある音楽制作ツールです。
MMLとは、「cde」のようなテキストで楽譜を表す記法です。
簡単に音楽を作れるツールです。Rustで作られており、macOS、Windows、Linux、WebAssemblyで動きます。
「オンラインソフト大賞2001」で入賞し、高校の情報の教科書にも載りました。

## サンプル一覧

- [mmlbbs6](https://sakuramml.com/mmlbbs6/index.php?action=pico) --- サクラ曲掲示板6にたくさんの曲が投稿されています。

## チュートリアル

オンラインチュートリアルを用意しています。ブラウザ上で音を聞きながらコマンドを覚えられます。

- [チュートリアル](https://sakuramml.com/index.php?%E7%B0%A1%E5%8D%98%E3%81%AA%E4%BD%BF%E3%81%84%E6%96%B9)

## ピコサクラ - インストールについて

Web版が「[こちら(ピコサクラ)](https://sakuramml.com/go.php?15)」です。
ブラウザ上で手軽にMIDIファイルを再生できます。

ダウンロードして使いたい場合、コマンドライン版が使えます。以下より各OSのバイナリをダウンロードしてください。

- [Command line(win/mac)](https://github.com/kujirahand/sakuramml-rust/releases/)

## 最新版のコンパイル (Rust)

最初に[Rust](https://www.rust-lang.org/tools/install)をインストールしておいてください。

```
$ git clone https://github.com/kujirahand/sakuramml-rust.git
$ cd sakuramml-rust
$ cargo build --release
```

すると、`target/release/sakuramml`が作成されます。

## 使い方

### コマンドライン版の使い方

楽譜情報をテキストに記述します。例えば「test.mml」というファイルに記述します。
それを、"test.mml"を"test.mid"に変換するには、コマンドラインで以下のようにコマンドを実行します。

```sh
$ sakuramml test.mml
```

安全のため、生成するMIDIイベントデータは既定で3MiBまでに制限されます。
コマンドライン版では、信頼できるMMLを変換するときに限り上限を変更できます。

```sh
$ sakuramml --max-event-bytes 10485760 test.mml
```

WebAssembly版の上限は3MiB固定です。詳しくは
[MIDIイベントデータの上限](docs/event-limit.md)を参照してください。
上限超過時はエラーを記録し、超過直前までの部分MIDIを返します。

### 基本的な使い方

```mml
音階4 ドレミファソラシ↑ド↓シラソファミレド
o4 cdefgab>c<bagfedc
```

```mml
トラック1 チャンネル1 音符1 ドミソー
TR=1 CH=1 l1 ceg^
```

## 和音

```mml
音符1「ドミソ」
l4 'ceg' 'dfa'8 'egb'8 'ceg'
```

和音はシングルクォートで囲みます。閉じたあとに `'ceg'4,90,120` のように音長・ゲート・ベロシティを指定できます。

## タイムの移動

TIME(小節:拍:ステップ)を使うと任意の小節に移動できます。

```mml
// 先頭に移動
TIME(1:1:0) cdef
TIME(1:1:0) efga

// 2小節目に移動
TIME(2:1:0) cdef
```

SUB{...}を使うと、タイムポインタをSUBの直前に戻すことが可能です。気軽に和音を演奏できます。

```mml
SUB{ cdef  c }
SUB{ efga  e }
     rrrr  g
```

## Rhythm macro

リズムマクロでは、大文字、小文字に関わらず、１文字１命令として扱われる。
リズムマクロの定義は「$文字{定義}」のように記述する。

```mml
// リズムマクロの定義(ただし以下のものはデフォルトで定義済み...再定義も可能)
$b{n36,}
$h{n42,}
$o{n46,}
// 新規でリズムマクロを定義
$S{n37,}
CH(10)
//Rhythm のサンプル
Rhythm{
　[4　l8
　　　brSr bbsr r-1
　　　hoho hoho
　]
}
```

## サクラv1/v2とサクラRust版の違い

サクラv1/v2と敢えて変更した点があります。

### ステップモードの指定

本バージョンでは、ステップモードの指定方法が異なります。v1/v2では、音長の指定を「l%96」のように書くと、その後ずっとステップ指定になっていました。
しかし、ステップモードで音符を指定することはほとんどないため、本バージョンでは、一時的にステップ指定ができますが、その後もステップ指定になるわけではありません。ステップ指定は一時的のみ使えます。

```mml
// 以下二行は同じ意味
l%96 cde
c4d4e4
```

### 連符の指定方法

従来、連符は『Div{...}』と記述していましたが、『Div』を省略して、『{ceg}』のように記述できます。

```mml
l4 Div{cde} f Div{gab} >c<
l4 {cde} f {gab} >c<
```

伸ばす記号「^」も1音と数えるので便利。

```mml
l4 {cde}c {gfe}d {c^d} e {d^e} f
```

連符はネストできます。

```mml
l1 { c d {efe} d } c
```

### 音量の相対指定記号

「(」でベロシティを8下げ、「)」でベロシティを8上げます。

```mml
v127 c ( c ( c (( c )) c ) c ) c  
```

### 和音の指定

「c0e0g」のように、0を指定した和音は、サポートしません。普通に以下のように指定してください。

```mml
'ceg' 'dfa' 'egb' 'ceg'
「ドミソ」「レファラ」「ミソシ」「ドミソ」
```

### 先行指定とCCやPBの連続書き込み

先行指定が使えます。

音符属性(`v` `q` `t` `o` `l`)とCC・ピッチベンドで、次の先行指定が使えます。

- .onNote(v1, v2, ...)　/ 省略形 .N(...)　音符ごとに値を切り替える
- .onTime(low, high, len, ...)　/ 省略形 .T(...)　時間経過に沿って値を推移させる
- .onCycle(step, v1, v2, ...)　/ 省略形 .C(...)　stepステップごとに値を切り替える
- .Random(n) / .Range(low, high) / .Delay(n) / .Repeat(on/off)
- v.Max(n) / q.Max(n)　値の上限を変更する（先行指定ではなく範囲の指定）
- CC・PB専用: .onNoteWave / .onNoteWaveEx / .onNoteWaveR / .Sine / .onNoteSine / .Frequency
  （.Frequency は書き込み間隔の指定。CCとピッチベンドで別々に設定できます）
- v__n.onTime/onNote/onCycle/Random(...)　通常のベロシティに加算するレイヤー別先行指定

```mml
v.onTime(0,127,!1)l8cccccccc
Array m=(10,10,!16); Array A=(100,100,!16); q.onTime(m,A) // 先行指定の配列引数展開
v70 v__1.onCycle(!4,10,-10) cdef // ベロシティは80,60,80,60
t.onCycle(!16,0,8) l16 cccc cccc // 16分音符ごとにハーフシャッフル
BR(2) PB.onTime(-8192,0,!4) l4c PB(0) efg^
M.onNoteWaveEx(0,0,!4,0,96,!8) l4 cdef // 音符の長さに合わせたビブラート
```

`v__n(value)`（nは0以上）は、通常のベロシティへ加算する独立した補正レイヤーです。
複数レイヤーは合算され、最終値は0～127に収められます。詳しい終了動作や解除方法は
[MML文法リファレンス](docs/syntax-note.md#サブベロシティ-v__n)を参照してください。

## マクロの機能

以下のようにしてマクロを定義できます。

```mml
// マクロの定義
STR P1 = {cdefg}
#P1 = {cdefg}
// マクロを展開
P1
#P1
```

マクロに引数を指定してマクロの内容を置換できます。マクロの中に「#?1」「#?2」「#?3」...を定義しておくと、その部分がマクロの直後に書いた引数で置換されます。

```mml
// マクロの定義
#Unison = { Key=#?2 Sub{ #?1 } Key=0 #?1 }
// マクロの展開
#Unison{cde},7
```

## リファレンス

- **MML文法リファレンス --- [docs/syntax.md](docs/syntax.md)**
- サクラ(Rust版)のコマンド一覧 --- [command.md](command.md)
  - サクラ(v2版)のコマンド一覧 --- https://sakuramml.com/doc/command/index.htm
- 音色一覧 --- [voice.md](voice.md)

## リポジトリ

- [GitHub](https://github.com/kujirahand/sakuramml-rust)
- [crate.io](https://crates.io/crates/sakuramml)
- [npm/sakuramml](https://www.npmjs.com/package/sakuramml)
