# メタイベント・テンポ・システムエクスクルーシブ

[← 目次に戻る](syntax.md)

曲全体に関わる情報(テンポ・拍子・曲名)と、音源に直接送るデータ(SysEx)のコマンドです。

## テンポ `Tempo`

```
Tempo(120)
Tempo=120
T(120)
BPM(120)
```

| コマンド | 別名 |
|---|---|
| `Tempo(n)` | `TEMPO` `T` `BPM` |

初期値は120です。20～240程度が推奨範囲です。

計算式の中で `Tempo` `TEMPO` `BPM` と書くと、現在のテンポ値を取得できます。

### テンポを徐々に変える `TempoChange`

```
TempoChange(開始テンポ, 終了テンポ, 長さ)
```

```
TempoChange(80, 120, 384)    // 384ステップ(タイムベース96なら全音符)かけて80から120へ
```

長さはステップ数で指定します(`!1` のような音長表記は使えません)。
長さを省略すると全音符ぶん、開始テンポを省略すると現在の値からの変化になります。

## 拍子 `TimeSignature`

```
TimeSignature(4, 4)
TimeSig(3, 4)
```

| コマンド | 別名 |
|---|---|
| `TimeSignature(分子, 分母)` | `System.TimeSignature` `TimeSig` `TIMESIG` |

初期値は 4/4 です。拍子は `Time(小節:拍:ステップ)` の計算にも使われます。

## メタテキスト

MIDIファイルに文字情報を埋め込みます。

```
コマンド{"文字列"}
```

| コマンド | 別名 | メタイベント |
|---|---|---|
| `MetaText{"..."}` | `Text` `TEXT` | テキスト |
| `Copyright{"..."}` | `COPYRIGHT` | 著作権表示 |
| `TrackName{"..."}` | `TRACK_NAME` | トラック名(曲名) |
| `InstrumentName{"..."}` | | 楽器名 |
| `Lyric{"..."}` | `LYRIC` | 歌詞 |
| `Maker{"..."}` | `MAKER` | マーカー |
| `CuePoint{"..."}` | | キューポイント |

```
TrackName{"サンプル曲"}
Copyright{"クジラ飛行机"}
Lyric{"ドレミの歌"}
```

## システムエクスクルーシブ `SysEx`

音源に固有のデータを直接送ります。

```
SysEx=$F0,$43,$10,$4C,$00,$00,$00,$30,$F0,$F7;
SysEx$=F0,43,10,4C,00,00,00,30,F0,60,F7;
```

`SysEx$=` の形式で書くと、値をすべて16進数として解釈します(`$` を省略できます)。

### チェックサムの自動計算

波カッコ `{ }` で囲んだ範囲は、チェックサムが自動計算されて末尾に挿入されます。

```
SysEx$=f0,43,10,4c,00,{00,00,30,f0},f7;
```

上の記述は次と同じ結果になります。

```
SysEx$=F0,43,10,4C,00,00,00,30,F0,60,F7;
```

## 音源のリセット

| コマンド | 内容 |
|---|---|
| `ResetGM` | GMリセットを送信する |
| `ResetGS` | GSリセットを送信する |
| `ResetXG` | XGリセットを送信する |

日本語の「音源初期化」は、GMリセットと小節ずらし・タイム同期をまとめて行います。

```
音源初期化
// = System.MeasureShift(1);ResetGM;Time(1:1:0);TrackSync;
```

## マスター設定

| コマンド | 内容 | 範囲 |
|---|---|---|
| `MasterVolume(n)` | マスターボリューム | 0-127 |
| `MasterBalance(n)` | マスターバランス | -8192～8191 |

```
MasterVolume(100)
MasterBalance(0)
```

互換性のため、従来の `MasterVolume(0, n)` / `MasterBalance(0, n)` 形式も利用できます。

## GS音源のエフェクト

ローランドGS音源向けのシステムエクスクルーシブを簡単に書けるコマンドです。

```
GSEffect(アドレス, 値)
```

```
GSEffect($30, 0)
GSReverbMacro(0)
```

| コマンド | アドレス | 内容 |
|---|---|---|
| `GSReverbMacro(v)` | $30 | リバーブの種類 (0:Room1 5:Hall 6:Delay) |
| `GSReverbCharacter(v)` | $31 | リバーブのキャラクター |
| `GSReverbPRE_LPE(v)` | $32 | リバーブのプリLPF |
| `GSReverbLevel(v)` | $33 | リバーブのレベル |
| `GSReverbTime(v)` | $34 | リバーブタイム |
| `GSReverbFeedback(v)` | $35 | リバーブのフィードバック |
| `GSReverbSendToChorus(v)` | $36 | リバーブからコーラスへの送り |
| `GSChorusMacro(v)` | $38 | コーラスの種類 |
| `GSChorusPRE_LPF(v)` | $39 | コーラスのプリLPF |
| `GSChorusLevel(v)` | $3A | コーラスのレベル |
| `GSChorusFeedback(v)` | $3B | コーラスのフィードバック |
| `GSChorusDelay(v)` | $3C | コーラスのディレイ |
| `GSChorusRate(v)` | $3D | コーラスのレート |
| `GSChorusDepth(v)` | $3E | コーラスのデプス |
| `GSChorusSendToReverb(v)` | $3F | コーラスからリバーブへの送り |
| `GSChorusSendToDelay(v)` | $40 | コーラスからディレイへの送り |
| `GS_RHYTHM(v)` | $15 | パートをリズムパートに変更 (0:楽器 1:ドラム1 2:ドラム2) |
| `GSScaleTuning(...)` | $11 | スケールチューニング(12個の値を指定) |

```
GSScaleTuning(0,0,0,0,0,0,0,0,0,0,0,0)
```

## 関連ページ

- [音色とMIDI制御](syntax-voice.md)
- [トラック・チャンネル・タイム](syntax-track.md)
