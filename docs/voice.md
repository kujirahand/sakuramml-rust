# Voice（音色番号）の仕様

この文書では、サクラMMLのVoice番号、内部状態、MIDI Program Change、Bank Select、MIDIダンプの間の変換規則を定めます。個々の音色名は[音色一覧](../voice.md)を参照してください。

## 結論

| 対象 | 表記範囲 | 内部・MIDI表現 | 変換 |
|---|---:|---:|---|
| サクラMMLの `Voice(n)` / `@n` | 1～128 | Program Change 0～127 | コンパイル時に `n - 1` |
| 音色名定数 | 1～128 | 通常の整数値 | `GrandPiano=1`、`Flute=74` など |
| MIDIダンプの `Voice(n)` | 1～128 | Program Change 0～127 | MIDI値へ1を加える |
| Bank Select MSB | 0～127 | CC#0 | 値をそのまま使う |
| Bank Select LSB | 0～127 | CC#32 | 値をそのまま使う |

Voice番号は1始まりですが、MIDI Program Changeのデータは0始まりです。

```text
Voice(1)    // MIDI Program Change 0: Grand Piano
@1          // Voice(1)と同じ
Voice(128)  // MIDI Program Change 127: Gunshot
```

## コマンド表記

次の表記を使用できます。

```text
@番号
@番号,バンクMSB,バンクLSB
Voice(番号)
Voice(番号,バンクMSB,バンクLSB)
VOICE(番号)
```

`@`、`Voice`、`VOICE` は同じ `TokenType::Voice` として処理されます。番号やバンクには計算式と音色名定数を指定できます。

```text
@GrandPiano
@(Flute)
Voice(SquareLead)
```

音色名定数は特別な型ではなく、1～128の整数値です。したがって、通常の音色名とドラムセット名で変換方法は変わりません。ドラムセットを選ぶ場合は、MIDI音源の仕様に合わせて通常は `CH(10)` と組み合わせます。

```text
CH(10) Voice(StandardSet)
```

## Program Changeへの変換

コンパイラはMMLのVoice番号を1～128に収めたあと、1を引いてMIDI Program Changeの0～127へ変換します。Program Changeイベントのチャンネルは、コマンド実行時のカレントチャンネルです。

カレントトラックには、MML表記と同じ1始まりの番号を `program_change` として保持します。`MML({@})` で参照する値もこの1始まりの現在値です。Voiceを一度も指定していないトラックでは、現行Rust実装の初期値として0を返します。

公開仕様上の入力範囲は1～128です。現行Rust実装は範囲外の番号をエラーにせず、1未満を1、128を超える値を128へ補正します。一方、オリジナル実装は範囲外をエラーにします。範囲外入力の補正は互換仕様として依存せず、必ず1～128を指定してください。

## Bank Select

バンクを指定した場合、同じ時刻・同じチャンネルへ次の順番でイベントを追加します。

1. CC#0（Bank Select MSB）
2. CC#32（Bank Select LSB）
3. Program Change

```text
Voice(1, 0, 0)
@1,0,0
```

MSBだけを指定した場合も、Rust版はLSBへ既定値0を設定し、CC#0とCC#32の両方を出力します。

```text
Voice(1, 8)  // CC#0=8、CC#32=0、Program Change=0
```

Bank Selectの公開仕様上の範囲はそれぞれ0～127です。現行Rust実装にはバンク値の範囲検査がないため、範囲外の値を指定してはいけません。

## MIDIダンプ

Program Changeイベントのデータは0～127なので、MIDIダンプでは1を加え、1始まりの `Voice(n)` として出力します。

```text
MIDI Program Change 0   -> Voice(1)
MIDI Program Change 73  -> Voice(74)
MIDI Program Change 127 -> Voice(128)
```

Program Changeはチャンネルイベントです。トラックの現在チャンネルと異なる場合は、[トラック・チャンネル番号の仕様](track-channel.md)に従って `CH(n)` を先に出力します。

現行のMIDIダンプは、直前のCC#0とCC#32をProgram Changeへ結合しません。Bank Select付きの音色指定は、次のように個別のイベントとして出力されます。

```text
CC($00,$08)
CC($20,$00)
Voice(1)
```

この出力を再コンパイルするとMIDIイベントとしての意味は保たれますが、元の短い `Voice(1,8,0)` 表記には戻りません。

## オリジナル実装との互換性

Pascal/Delphi版のオリジナル実装も、MMLのVoice番号から1を引いてMIDI Program Changeへ変換します。

- `sakuramml/mml_base.pas` の `scriptVoice` は `ary.IntItems[0] - 1` をProgram Change値に使う
- カレントトラックにはMML表記と同じ1始まりの番号を保持する
- MSBまたはLSBを指定した場合、未指定側を0としてCC#0とCC#32の両方をProgram Changeより前に出力する
- 範囲外のVoice番号はエラーにする

Rust版の基本的な番号変換とバンクイベントの並びはオリジナル実装と一致します。範囲外Voice番号を補正する点はRust版固有の差です。

## 実装上の対応箇所

- `src/lexer/cc.rs`: `@` の引数をVoiceトークンとして読み取る
- `src/lexer/command.rs`: `Voice`、`VOICE` の引数を読み取る
- `src/runner/cc.rs`: Voice番号を0～127へ変換し、Bank SelectとProgram Changeイベントを追加する
- `src/song/track.rs`: 1始まりの現在Voice番号を保持する
- `src/midi.rs`: Program Changeを書き込み、ダンプ時に1始まりへ戻す
- `src/sakura_functions.rs`: `MML({@})` で現在のVoice番号を返す
- `src/mml_def/variables.rs`: 音色名とドラムセット名を整数定数として定義する
- `sakuramml/mml_base.pas`: オリジナル実装のVoice番号変換とバンク指定を処理する
- `sakuramml/smf_types.pas`: オリジナル実装のBank SelectとProgram Changeを書き込む

基本的な利用方法は[音色とMIDI制御](syntax-voice.md)も参照してください。
