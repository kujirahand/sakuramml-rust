# テキスト音楽 サクラ(Rust版) コマンド一覧

## ストトン表記

| コマンド | 説明    |
|---------|--------|
| テンポ | 定義: "TEMPO=" |
| トラック | 定義: "TR=" |
| チャンネル | 定義: "CH=" |
| 全音符 | 定義: "l1" |
| 二分音符 | 定義: "l2" |
| 四分音符 | 定義: "l4" |
| 八分音符 | 定義: "l8" |
| 十六音符 | 定義: "l16" |
| 付点四分音符 | 定義: "l4." |
| 音長 | 定義: "l" |
| 音符 | 定義: "l" |
| 音階 | 定義: "o" |
| 音色 | 定義: "@" |
| 音量 | 定義: "v" |
| ゲート | 定義: "q" |
| 連符 | 定義: "Div" |
| ド | 定義: "c" |
| レ | 定義: "d" |
| ミ | 定義: "e" |
| ファ | 定義: "f" |
| フ | 定義: "f" |
| ソ | 定義: "g" |
| ラ | 定義: "a" |
| シ | 定義: "b" |
| ン | 定義: "r" |
| ッ | 定義: "r" |
| ー | 定義: "^" |
| 「 | 定義: "'" |
| 」 | 定義: "'" |
| 【 | 定義: "[" |
| 】 | 定義: "]" |
| ↑ | 定義: ">" |
| ↓ | 定義: "<" |
| ん | 定義: "r" |
| ♭ | 定義: "-" |
| ♯ | 定義: "#" |
| 調 | 定義: "KeyFlag=" |
| ど | 定義: "n36 |
| た | 定義: "n38 |
| つ | 定義: "n42 |
| く | 定義: "n44 |
| ち | 定義: "n46 |
| ぱ | 定義: "n49 |


## 1文字コマンド

| コマンド | 説明    |
|---------|--------|
| / \t / \r / ｜ / ; | 空白文字 |
| c / d / e / f / g / a / b | ドレミファソラシ c(音長),(ゲート),(音量),(タイミング),(音階) |
| n | 番号を指定して発音(例: n36) n(番号),(音長),(ゲート),(音量),(タイミング) |
| r / _ | 休符 |
| l | 音長の指定(例 l4) |
| o | 音階の指定(例 o5) 範囲:0-10 |
| p | ピッチベンドの指定 範囲:0-127 (63が中央) |
| q | ゲートの指定 (例 q90) 範囲:0-100 |
| v | ベロシティ音量の指定 範囲:0-127 |
| y | コントロールチェンジの指定 (例 y1,100) 範囲:0-127 |
| @ | 音色の指定 範囲:1-128 |
| > | 音階を1つ上げる |
| < | 音階を1つ下げる |
| ) | 音量を8つ上げる |
| ( | 音量を8つ下げる |
| // | 一行コメント |
| /* .. */ | 範囲コメント |
| [ | ループ開始 (例 [4 cdeg]) |
| : | ループ最終回に脱出 (例　[4 cde:g]e) |
| ] | ループ終了 |
| ’ | 和音 (例 'ceg') 'ceg'(音長),(ゲート) |
| $ | リズムマクロ定義 $文字{定義内容} |
| { | 連符 (例 {ceg}4) {c^d}(音長) |
| ` | 一度だけ音階を+1する |
|  | 一度だけ音階を-1する |


## 大文字コマンド

| コマンド | 説明    |
|---------|--------|
| TR / TRACK / Track | トラック変更　TR=番号 範囲:1- |
| CH / Channel | チャンネル変更 CH=番号 範囲:1-16 |
| TIME / Time | タイム変更 TIME(節:拍:ステップ) |
| RHYTHM / Rhythm / R | リズムモード |
| RYTHM / Rythm | リズムモード(v1の綴りミス対処[^^;]) |
| DIV / Div | 連符 (例 DIV{ceg} ) |
| SUB / Sub | タイムポインタを戻す (例 SUB{ceg} egb) |
| KF / KeyFlag | 臨時記号を設定 - KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note) |
| INT / Int | 変数を定義 (例 INT TestValue=30) |
| STR / Str | 文字列変数を定義 (例 STR A={cde}) |
| M / Modulation | モジュレーション 範囲: 0-127 |
| PT / PortamentoTime | ポルタメント 範囲: 0-127 |
| V / MainVolume | メインボリューム 範囲: 0-127 |
| P / Panpot | パンポット 範囲: 0-63-127 |
| EP / Expression | エクスプレッション音量 範囲: 0-127 |
| PS / PortamentoSwitch | ポルタメントスイッチ |
| REV / Reverb | リバーブ 範囲: 0-127 |
| CHO / Chorus | コーラス 範囲: 0-127 |
| PB / PitchBend | ピッチベンドを指定 範囲: -8192~0~8191の範囲 |
| BR / PitchBendSensitivity | ピッチベンドの範囲を設定 範囲: 0-12半音 |
| FineTune | チューニングの微調整 範囲:0-64-127 (-100 - 0 - +99.99セント） |
| CoarseTune | 半音単位のチューニング 範囲:40-64-88 (-24 - 0 - 24半音) |
| VibratoRate | 音色の編集(GS/XG) 範囲: 0-127 |
| VibratoDepth | 音色の編集(GS/XG) 範囲: 0-127 |
| VibratoDelay | 音色の編集(GS/XG) 範囲: 0-127 |
| FilterCutoff | 音色の編集(GS/XG) 範囲: 0-127 |
| FilterResonance | 音色の編集(GS/XG) 範囲: 0-127 |
| EGAttack | 音色の編集(GS/XG) 範囲: 0-127 |
| EGDecay | 音色の編集(GS/XG) 範囲: 0-127 |
| EGRelease | 音色の編集(GS/XG) 範囲: 0-127 |
| ResetGM | GMリセットを送信 |
| ResetGS | GSリセットを送信 |
| ResetXG | XGリセットを送信 |
| TEMPO / Tempo / T | テンポの指定 |
| TimeSignature / TimeSig / TIMESIG | 拍子の指定 |
| MetaText / TEXT / Text | メタテキスト (例 TEXT{"abcd"}) |
| COPYRIGHT / Copyright | メタテキスト著作権 (例 COPYRIGHT{"aaa"}) |
| LYRIC / Lyric | メタテキスト歌詞 (例 LYRIC{"aaa"}) |


## 音色など変数定義

| 変数名 | 値    |
|---------|--------|
| GrandPiano | 1 |
| BrightPiano | 2 |
| ElectricGrandPiano | 3 |
| HonkyTonkPiano | 4 |
| ElectricPiano1 | 5 |
| ElectricPiano2 | 6 |
| Harpsichord | 7 |
| Clavi | 8 |
| CelestaStrings | 9 |
| Glockenspiel | 10 |
| MusicBox | 11 |
| Vibraphone | 12 |
| Marimba | 13 |
| Xylophone | 14 |
| TubularBells | 15 |
| Dulcimer | 16 |
| DrawbarOrgan | 17 |
| PercussiveOrgan | 18 |
| RockOrgan | 19 |
| ChurchOrgan | 20 |
| ReedOrgan | 21 |
| Accordion | 22 |
| Hamonica | 23 |
| TangoAccordion | 24 |
| NylonGuitar | 25 |
| SteelcGuitar | 26 |
| JazzGuitar | 27 |
| CleanGuitar | 28 |
| MutedGuitar | 29 |
| OverdrivenGuitar | 30 |
| DistortionGuitar | 31 |
| GuitarHarmonics | 32 |
| AcousticBass | 33 |
| FingerBase | 34 |
| FingerBass | 34 |
| PickBass | 35 |
| FretlessBass | 36 |
| SlapBass1 | 37 |
| SlapBass2 | 38 |
| SynthBass1 | 39 |
| SynthBass2 | 40 |
| Violin | 41 |
| Viola | 42 |
| Cello | 43 |
| Contrabass | 44 |
| TremoloStrings | 45 |
| PizzicatoStrings | 46 |
| OrchestralHarp | 47 |
| Timpani | 48 |
| Strings1 | 49 |
| Strings2 | 50 |
| SynthStrings1 | 51 |
| SynthStrings2 | 52 |
| ChoirAahs | 53 |
| VoiceOohs | 54 |
| SynthVoice | 55 |
| OrchestraHit | 56 |
| Trumpet | 57 |
| Trombone | 58 |
| Tuba | 59 |
| MutedTrumpet | 60 |
| FrenchHorn | 61 |
| BrassSection | 62 |
| SynthBrass1 | 63 |
| SynthBrass2 | 64 |
| SopranoSax | 65 |
| AltoSax | 66 |
| TenorSax | 67 |
| BaritoneSax | 68 |
| Oboe | 69 |
| EnglishHorn | 70 |
| Bassoon | 71 |
| Clarinet | 72 |
| Piccolo | 73 |
| Flute | 74 |
| Recorder | 75 |
| PanFlute | 76 |
| BlownBottle | 77 |
| Shakuhachi | 78 |
| Whistle | 79 |
| Ocarina | 80 |
| SquareLead | 81 |
| SawtoothLead | 82 |
| CalliopeLead | 83 |
| ChiffLead | 84 |
| CharangLead | 85 |
| VoiceLead | 86 |
| FifthsLead | 87 |
| BassLead | 88 |
| NewAgePad | 89 |
| WarmPad | 90 |
| PolySynthPad | 91 |
| ChoirPad | 92 |
| BowedPad | 93 |
| MetallicPad | 94 |
| HaloPad | 95 |
| SweepPad | 96 |
| Rain | 97 |
| SoundTrack | 98 |
| Crystal | 99 |
| Atmosphere | 100 |
| Brightness | 101 |
| Goblins | 102 |
| Echoes | 103 |
| Sci_Fi | 104 |
| Sitar | 105 |
| Banjo | 106 |
| Shamisen | 107 |
| Koto | 108 |
| Kalimba | 109 |
| Bagpipe | 110 |
| Fiddle | 111 |
| Shanai | 112 |
| TibkleBell | 113 |
| TinkleBell | 113 |
| Agogo | 114 |
| SteelDrums | 115 |
| Woodblock | 116 |
| TaikoDrum | 117 |
| MelodicTom | 118 |
| SynthDrum | 119 |
| ReverseCymbal | 120 |
| FretNoise | 121 |
| BreathNoise | 122 |
| Seashore | 123 |
| BirdTweet | 124 |
| TelephoneRing | 125 |
| Helicopter | 126 |
| Applause | 127 |
| Gunshot | 128 |
| StandardSet | 1 |
| StandardSet2 | 2 |
| RoomSet | 9 |
| PowerSet | 17 |
| ElectronicSet | 25 |
| AnalogSet | 26 |
| DanceSet | 27 |
| JazzSet | 33 |
| BrushSet | 41 |
| OrchestraSet | 49 |
| SnareRoll | 25 |
| FingerSnap | 26 |
| HighQ | 27 |
| Slap | 28 |
| ScratchPush | 29 |
| ScratchPull | 30 |
| Sticks | 31 |
| SquareClick | 32 |
| MetronomeClick | 33 |
| MetronomeBell | 34 |
| Kick2 | 35 |
| Kick1 | 36 |
| SideStick | 37 |
| Snare1 | 38 |
| HandClap | 39 |
| Snare2 | 40 |
| LowTom2 | 41 |
| ClosedHiHat | 42 |
| LowTom1 | 43 |
| PedalHiHat | 44 |
| MidTom2 | 45 |
| OpenHiHat | 46 |
| MidTom1 | 47 |
| HighTom2 | 48 |
| CrashCymbal1 | 49 |
| HighTom1 | 50 |
| RideCymbal1 | 51 |
| ChineseCymbal | 52 |
| RideBell | 53 |
| Tambourine | 54 |
| SplashCymbal | 55 |
| Cowbell | 56 |
| CrashCymbal2 | 57 |
| VibraSlap | 58 |
| RideCymbal2 | 59 |
| HighBongo | 60 |
| LowBongo | 61 |
| MuteHighConga | 62 |
| OpenHighConga | 63 |
| LowConga | 64 |
| HighTimbale | 65 |
| LowTimbale | 66 |
| HighAgogo | 67 |
| LowAgogo | 68 |
| Cabasa | 69 |
| Maracas | 70 |
| ShortHiWhistle | 71 |
| LongLowWhistle | 72 |
| ShortGuiro | 73 |
| LongGuiro | 74 |
| Claves | 75 |
| HighWoodBlock | 76 |
| LowWoodBlock | 77 |
| MuteCuica | 78 |
| OpenCuica | 79 |
| MuteTriangle | 80 |
| OpenTriangle | 81 |
| Shaker | 82 |
| JingleBell | 83 |
| BellTree | 84 |
| Castanets | 85 |
| MuteSurdo | 86 |
| OpenSurdo | 87 |


## リズムマクロ

| 変数名 | 値    |
|---------|--------|
| b | "n36," |
| s | "n38," |
| h | "n42," |
| H | "n44," |
| o | "n46," |
| c | "n49," |