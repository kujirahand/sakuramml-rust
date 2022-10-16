# sakuramml-rust

Sakuramml for Rust

## どこまで作ったか

簡単な楽譜を再生できます。

## 未実装

- 和音 c0e0g ←　今回はサポートしない
- マクロ
- Div
- Sub
- Play
- Print
- FOR IF WHILE
- Key / KeyFlag / KeyShift
- PlayFrom

# 使い方

```
音階4 ドレミファソラシ↑ド↓シラソファミレド
o4 cdefgab>c<bagfedc
```

```
トラック1 チャンネル1 音符1 ドミソー
TR=1 CH=1 l1 ceg^
```

## 和音

```
音符1「ドミソ」
l4 `ceg` `dfa`8 `egb`8 `ceg`
```

## タイムの移動

```
// 先頭に移動
TIME(1:1:0) cdef
TIME(1:1:0) efga

// 2小節目に移動
TIME(2:1:0) cdef
```

## Rhythm macro

リズムマクロでは、大文字、小文字に関わらず、１文字１命令として扱われる。
リズムマクロの定義は「$文字{定義}」のように記述する。

```
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

## サクラv1/v2との違い

連符のDiv{...}でDivを省略できます。

```
l4 Div{cde} f Div{gab} >c<
l4 {cde} f {gab} >c<
```

伸ばす記号「^」も1音と数えるので便利。

```
l4 {cde}c {gfe}d {c^d} e {d^e} f
```

連符はネストできます。

```
l1 { c d {efe} d } c
```

