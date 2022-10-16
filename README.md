# sakuramml-rust

Sakuramml for Rust

## どこまで作ったか

- ストトンの変換
- MMLをMIDIに変換するところまで

## 未実装

- Rythm
- マクロ
- 和音 c0e0g
- Div
- Sub
- Play

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

