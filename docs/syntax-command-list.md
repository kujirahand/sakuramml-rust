# コマンド索引

[← 目次に戻る](syntax.md)

コマンドから該当する解説ページを探すための索引です。
全コマンドの機械的な一覧は、ソースコードから自動生成される
[command.md](../command.md) と [voice.md](../voice.md) を参照してください。

## 1文字コマンド(小文字・記号)

| コマンド | 内容 | 解説 |
|---|---|---|
| `c d e f g a b` | 音名 | [音符](syntax-note.md#音名-c-d-e-f-g-a-b) |
| `n` | 番号指定の音符 | [音符](syntax-note.md#番号指定の音符-n) |
| `r` | 休符 | [音符](syntax-note.md#休符-r) |
| `l` | 音長 | [音符](syntax-note.md#音長-l) |
| `o` `>` `<` `` ` `` `"` | オクターブ | [音符](syntax-note.md#オクターブ-o------) |
| `v` `(` `)` | ベロシティ | [音符](syntax-note.md#音量ベロシティ-v--) |
| `q` | ゲートタイム | [音符](syntax-note.md#ゲートタイム-q) |
| `t` | 発音タイミング | [音符](syntax-note.md#発音タイミング-t) |
| `&` | タイ・スラー | [音符](syntax-note.md#タイスラー--) |
| `p` | ピッチベンド(0-127) | [音色とMIDI制御](syntax-voice.md#ピッチベンド) |
| `y` | コントロールチェンジ | [音色とMIDI制御](syntax-voice.md#コントロールチェンジ) |
| `@` | 音色 | [音色とMIDI制御](syntax-voice.md#音色---voice) |
| `[` `:` `]` | 繰り返し | [マクロ](syntax-macro.md#繰り返し---) |
| `'` | 和音 | [マクロ](syntax-macro.md#和音--) |
| `{` `}` | 連符 | [マクロ](syntax-macro.md#連符----div) |
| `#` | マクロ定義 | [マクロ](syntax-macro.md#マクロ文字列変数) |
| `$` | リズムマクロ定義 | [マクロ](syntax-macro.md#リズムマクロ---rhythm) |
| `?` | ここから演奏 | [トラック](syntax-track.md#途中から演奏する-playfrom) |
| `;` `\|` 空白 | 区切り(読み飛ばし) | [目次](syntax.md#コメント) |
| `//` `/* */` `##` `///` | コメント | [目次](syntax.md#コメント) |

## 大文字コマンド

### トラック・タイム

| コマンド | 別名 | 解説 |
|---|---|---|
| `Track` | `TRACK` `TR` | [トラック](syntax-track.md#トラックとチャンネル) |
| `Channel` | `CHANNEL` `CH` | [トラック](syntax-track.md#トラックとチャンネル) |
| `Port` | `PORT` | [トラック](syntax-track.md#トラックとチャンネル) |
| `Time` | `TIME` | [トラック](syntax-track.md#タイムポインタ-time) |
| `TimeBase` | `Timebase` `TIMEBASE` `System.TimeBase` | [トラック](syntax-track.md#タイムベース-timebase) |
| `MeasureShift` | `MEASURE_SHIFT` `System.MeasureShift` | [トラック](syntax-track.md#小節番号のずらし-measureshift) |
| `TrackSync` | `TRACK_SYNC` | [トラック](syntax-track.md#タイムポインタの同期-tracksync) |
| `Sub` | `SUB` `S` | [トラック](syntax-track.md#サブ演奏-sub) |
| `Play` | `PLAY` | [トラック](syntax-track.md#複数トラックの同時演奏-play) |
| `PlayFrom` | `PLAY_FROM` | [トラック](syntax-track.md#途中から演奏する-playfrom) |
| `PlayFromHere` | `PLAY_FROM_HRER` | [トラック](syntax-track.md#途中から演奏する-playfrom) |
| `KeyShift` | `Key` `KEY` | [トラック](syntax-track.md#移調) |
| `TrackKey` | `TR_KEY` | [トラック](syntax-track.md#移調) |
| `UseKeyShift` | | [トラック](syntax-track.md#移調) |
| `End` | `END` | [トラック](syntax-track.md#曲の終わり-end) |

### 音符・演奏

| コマンド | 別名 | 解説 |
|---|---|---|
| `KeyFlag` | `KF` `System.KeyFlag` | [音符](syntax-note.md#調号-keyflag) |
| `Slur` | `SLUR` | [音符](syntax-note.md#スラー-) |
| `System.vAdd` | (`vAdd` は小文字始まりのため使用不可) | [音符](syntax-note.md#音量ベロシティ-v--) |
| `System.qAdd` | (`qAdd` は小文字始まりのため使用不可) | [音符](syntax-note.md#ゲートタイム-q) |
| `Div` | `DIV` | [マクロ](syntax-macro.md#連符----div) |
| `Rhythm` | `RHYTHM` `R` `Rythm` `RYTHM` | [マクロ](syntax-macro.md#リズムマクロ---rhythm) |

### 音色・MIDI制御

| コマンド | 別名 | 解説 |
|---|---|---|
| `Voice` | `VOICE` | [音色とMIDI制御](syntax-voice.md#音色---voice) |
| `ControlChange` | `CONTROL_CHANGE` `CC` | [音色とMIDI制御](syntax-voice.md#コントロールチェンジ) |
| `Modulation` `PortamentoTime` `MainVolume` `Panpot` `Expression` `PortamentoSwitch` `Reverb` `Chorus` `Variation` | `M` `PT` `V` `P` `EP` `PS` `REV` `CHO` `VAR` | [音色とMIDI制御](syntax-voice.md#名前付きコマンド) |
| `PitchBend` | `PB` | [音色とMIDI制御](syntax-voice.md#ピッチベンド) |
| `BendRange` | `BR` `BEND_RANGE` `PitchBendSensitivity` | [音色とMIDI制御](syntax-voice.md#ピッチベンド) |
| `RPN` `NRPN` | | [音色とMIDI制御](syntax-voice.md#rpn--nrpn) |
| `FineTune` `CoarseTune` `VibratoRate` `VibratoDepth` `VibratoDelay` `FilterCutoff` `FilterResonance` `EGAttack` `EGDecay` `EGRelease` | | [音色とMIDI制御](syntax-voice.md#rpn--nrpn) |
| `Fadein` `Fadeout` `Cresc` `Decresc` | `CRESC` `DECRESC` | [音色とMIDI制御](syntax-voice.md#フェードクレッシェンド) |
| `NoteOn` `NoteOff` `DirectSMF` | | [音色とMIDI制御](syntax-voice.md#直接的なmidi出力) |
| `SoundType` `DeviceNumber` | | [音色とMIDI制御](syntax-voice.md#その他) |

### メタイベント・SysEx

| コマンド | 別名 | 解説 |
|---|---|---|
| `Tempo` | `TEMPO` `T` `BPM` | [メタ](syntax-meta.md#テンポ-tempo) |
| `TempoChange` | | [メタ](syntax-meta.md#テンポを徐々に変える-tempochange) |
| `TimeSignature` | `TimeSig` `TIMESIG` `System.TimeSignature` | [メタ](syntax-meta.md#拍子-timesignature) |
| `MetaText` `Copyright` `TrackName` `InstrumentName` `Lyric` `Maker` `CuePoint` | `Text` `TEXT` `COPYRIGHT` `TRACK_NAME` `LYRIC` `MAKER` | [メタ](syntax-meta.md#メタテキスト) |
| `SysEx` | | [メタ](syntax-meta.md#システムエクスクルーシブ-sysex) |
| `ResetGM` `ResetGS` `ResetXG` | | [メタ](syntax-meta.md#音源のリセット) |
| `MasterVolume` `MasterBalance` | | [メタ](syntax-meta.md#マスター設定) |
| `GSEffect` ほか `GS*` | | [メタ](syntax-meta.md#gs音源のエフェクト) |

### スクリプト

| コマンド | 別名 | 解説 |
|---|---|---|
| `Int` `Str` `Array` | `INT` `STR` `ARRAY` | [スクリプト](syntax-script.md#変数) |
| `IF` / `ELSE` | `If` `Else` | [スクリプト](syntax-script.md#条件分岐-if) |
| `FOR` `WHILE` | `For` `While` | [スクリプト](syntax-script.md#繰り返し-for--while) |
| `Break` `Exit` `Continue` | `BREAK` `EXIT` `CONTINUE` | [スクリプト](syntax-script.md#繰り返し-for--while) |
| `Function` `Return` | `FUNCTION` `RETURN` | [スクリプト](syntax-script.md#ユーザー定義関数-function) |
| `Print` | `PRINT` | [スクリプト](syntax-script.md#デバッグ出力-print) |
| `RandomSeed` | `RANDOM_SEED` | [スクリプト](syntax-script.md#乱数の種) |
| `Random` `RandomSelect` `Chr` `Asc` `Mid` `Replace` `SizeOf` `StrLen` `MML` `Hex` `Pos` | | [スクリプト](syntax-script.md#組み込み関数) |

## 未実装のコマンド

| コマンド | 状態 |
|---|---|
| `Include` / `System.Include` / `INCLUDE` | 未実装 |
| `System.q2Add` / `q2Add` | 未実装 |
| `l.Random` `l.onTime` `o.onTime` `q.onTime` | 未対応(エラー) |
| CC系の `.onNoteWaveEx` `.onNoteWaveR` `.onCycle` `.Sine` `.onNoteSine` | 未対応(警告を出して無視) |

## 自動生成ドキュメント

| ファイル | 内容 |
|---|---|
| [command.md](../command.md) | 全コマンド・全ストトン表記・全マクロの一覧 |
| [voice.md](../voice.md) | 音色一覧(日本語名つき) |

これらは `src/` のコメントから `build_doc.sh` で生成されます。
