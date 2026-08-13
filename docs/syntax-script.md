# スクリプト機能(変数・演算・制御構文・関数)

[← 目次に戻る](syntax.md)

サクラのMMLは、変数や条件分岐を使えるプログラミング言語としての側面も持っています。

## 変数

変数名は**大文字またはアンダースコアで始まる**必要があります。
予約語(コマンド名)と同じ名前は使えません。

```
Int A = 100         // 整数
Str S = {cde}       // 文字列(MMLの断片)
Array B = (1,2,3)   // 配列
```

| コマンド | 別名 | 内容 |
|---|---|---|
| `Int 名前 = 値` | `INT` | 整数変数を定義する |
| `Str 名前 = {...}` | `STR` | 文字列変数(マクロ)を定義する |
| `Array 名前 = (...)` | `ARRAY` | 配列を定義する |

定義済みの変数は、宣言なしで代入・参照できます。

```
Int A = 1
A = A + 2
A++                 // インクリメント
A--                 // デクリメント
Print(A)            // 3
```

### 文字列変数(マクロ)

`Str` で定義した文字列は、名前を書くだけでMMLとして展開されます。
`#名前 = {...}` と書いても同じです。詳しくは
[繰り返し・和音・連符・マクロ](syntax-macro.md#マクロ文字列変数)を参照してください。

```
Str Melody = {cde}
Melody              // cde が演奏される
```

文字列の一部を置換するメソッドもあります。

```
Str S = {cde}
S.s({c},{g})        // S の中の c を g に置き換える
```

### 配列

```
Array A = (10, 20, 30)
Print(A(0))         // 10
Print(SizeOf(A))    // 3
```

### 定義済みの変数

| 変数 | 内容 |
|---|---|
| `SAKURA_VERSION` | サクラのバージョン文字列 |
| `TRUE` / `FALSE` | 真偽値 |
| `SLUR_PORT` / `SLUR_BEND` / `SLUR_GATE` / `SLUR_ALPE` | スラーのモード定数 |
| 音色名(`GrandPiano` など) | 音色番号 → [voice.md](../voice.md) |
| `OctaveUnison` / `Unison5th` / `Unison3th` / `Unison` / `RndTiming` | 定義済みマクロ |

## 計算式で参照できる値

演奏中の状態を計算式の中で取得できます。

| 書き方 | 内容 |
|---|---|
| `TR` / `TRACK` / `Track` | 現在のトラック番号 |
| `CH` / `CHANNEL` | 現在のチャンネル番号 |
| `TIME` / `Time` / `TIMEPOS` / `TIMEPTR` | 現在のタイムポインタ値 |
| `TEMPO` / `Tempo` / `BPM` | 現在のテンポ |
| `KEY` / `KEY_SHIFT` | 現在のキーシフト値 |
| `TR_KEY` / `TrackKey` | トラックごとのキーシフト値 |
| `TIMEBASE` / `Timebase` | 現在のタイムベース |
| `l` | 現在の音長 |
| `v` | 現在のベロシティ |
| `q` | 現在のゲート |
| `o` | 現在のオクターブ |

```
Int Pos = TIME
Print(Pos)
```

変数名にコマンド名(`T` `S` `R` `P` `V` `M` `CC` など)は使えません。

## 演算子

優先順位の高い順に並べています。

| 優先順位 | 演算子 | 内容 |
|---|---|---|
| 1 | `( )` | カッコ |
| 2 | `*` `/` `%` | 乗算・除算・剰余 |
| 3 | `+` `-` | 加算・減算 |
| 4 | `=` `==` `<>` `!=` `>` `<` `>=` `<=` | 比較 |
| 5 | `&&` `\|\|` | 論理積・論理和 |

`>=` は `≧`、`<=` は `≦`、`<>` と `!=` は `≠` として扱われます。

```
IF (A > 10 && B < 5) { cde }
```

## 条件分岐 `IF`

```
IF(条件){ 真のとき }ELSE{ 偽のとき }
```

```
Int A = 3
IF(A == 3){ cde }ELSE{ efg }
```

`If` / `Else` と書いても同じです。

## 繰り返し `FOR` / `WHILE`

```
FOR(初期化; 条件; 更新){ ... }
WHILE(条件){ ... }
```

```
FOR(Int I = 0; I < 4; I++){ cde }
FOR(I = 0; I < 4; I++){ cde }       // Int を省略しても Int として扱われる

Int J = 0
WHILE(J < 4){ cde  J++ }
```

| コマンド | 内容 |
|---|---|
| `Break` / `BREAK` / `Exit` / `EXIT` | ループを抜ける |
| `Continue` / `CONTINUE` | 次の繰り返しへ進む |

なお、音符を単純に繰り返すだけなら `[4 cde]` のループ記法のほうが簡潔です
([繰り返し・和音・連符・マクロ](syntax-macro.md#繰り返し---)を参照)。

## ユーザー定義関数 `Function`

```
Function 名前(引数, ...){ 処理 }
```

引数には型と既定値を指定できます。型は `Int`(`I`) / `Str`(`S`) / `Array`(`A`) です。
型を省略すると `Int` になります。

```
Function Melody(Int Oct = 5, Str Body = {cde}){
    o(Oct)
    Body
}
Melody(4, {ceg})
Melody()
```

戻り値は `Return` で返します(内部的には `Result` 変数に格納されます)。

```
Function Add(Int A, Int B){
    Return(A + B)
}
Print(Add(1, 2))    // 3
```

関数はファイルの先頭で事前に走査されるため、定義より前で呼び出すこともできます。

## 組み込み関数

計算式の中で使える関数です。大文字だけの別名もあります。
単独の命令としてではなく、必ず式の中(`Print(...)` や代入の右辺など)で使います。

| 関数 | 書式 | 内容 |
|---|---|---|
| `Random` | `Random(N, M)` / `Random(N)` | N～Mの乱数を返す。別名 `RANDOM` `RandomInt` `RND` `Rnd` |
| `RandomSelect` | `RandomSelect(...)` | 引数の中から1つをランダムに選ぶ |
| `Chr` | `Chr(C)` | 文字コードを文字に変換する。別名 `CHR` |
| `Asc` | `Asc(S)` | 文字を文字コードに変換する。別名 `ASC` |
| `Mid` | `Mid(S, N, M)` | 文字列Sの位置NからM文字を取り出す。別名 `MID` |
| `Replace` | `Replace(S, A, B)` | 文字列Sの中のAをBに置き換える。別名 `REPLACE` |
| `SizeOf` | `SizeOf(A)` | 配列の要素数を返す。別名 `SIZEOF` |
| `StrLen` | `StrLen(S)` | 文字列の長さを返す。別名 `STRLEN` |
| `MML` | `MML(C)` | `l` `v` `o` `q` `t` `@` `BR` `p%` `Key` `TimeKey` `Port` の現在値を返す |
| `NoteNo` | `NoteNo(MML)` | MMLで書いた音符の音符番号を返す。別名 `NOTENO` |
| `Hex` | `Hex(V)` | 数値を16進文字列に変換する。別名 `HEX` |
| `Pos` | `Pos(N, M)` | 文字列Mの中でNが現れる位置(1始まり)を返す。別名 `POS` |

```
Print(Random(1,6))
Print(Mid({abc}, 1, 2))         // ab
Print(Replace({abc}, {a}, {b})) // bbc
Print(Hex(255))                 // FF
Print(Pos({b}, {abc}))          // 2
Print(RandomSelect({c}, {d}, {e}))
```

`MML` の引数には、現在値を調べるMML命令名をそのまま指定します。`l` は内部のステップ数を返すため、既定の `TimeBase=96` では `l4` の値は `96` です。`p%` は中央を0とする詳細なピッチベンド値を返します。なお、現在のRust版では `TimeKey` 命令が未実装のため、`MML(TimeKey)` は初期値の `0` を返します。

```
l4 v100 o4
Print(MML(l)) // 96
Print(MML(v)) // 100
Print(MML(o)) // 4
```

`NoteNo` の引数には、音符をMMLでそのまま書きます。オクターブ指定(`o5` や `>` `<`)や臨時記号(`+` `-`)、音符番号を指定する `n` 命令も使えます。オクターブを省略した場合は、その時点のトラックのオクターブを使います。`Key`(KeyShift)や `TrackKey` の指定も反映するため、実際に鳴る音の番号が返ります。

```
Int N = NoteNo(o5e)
Print(N)             // 64
Print(NoteNo(o4c))   // 48
Print(NoteNo(o5e+))  // 65
Print(NoteNo(n60))   // 60
o6 Print(NoteNo(c))  // 72
```

引数を変数で渡すこともできます。その場合は、変数にMMLの文字列を入れておきます。

```
Str MMLA = {o5e}
Print(NoteNo(MMLA)) // 64
```

### 乱数の種

```
RandomSeed(12345)
RANDOM_SEED(12345)
```

種を固定すると、毎回同じ乱数列になります。
コマンドライン版では、指定しない場合は実行のたびに変わります。

## デバッグ出力 `Print`

```
Print({hello})
Print(A)
PRINT(1 + 2)
```

コンパイル時にコンソールへ出力されます。

## 関連ページ

- [繰り返し・和音・連符・マクロ](syntax-macro.md)
- [トラック・チャンネル・タイム](syntax-track.md)
