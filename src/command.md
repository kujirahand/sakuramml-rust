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
| 作者 | 定義: "COPYRIGHT=" |
| 曲名 | 定義: "TrackName=" |
| コメント | 定義: "MetaText=" |
| 拍子 | 定義: "TimeSig=" |
| 音源初期化 | 定義: "ResetGM()" |
| 時間 | 定義: "Time=" |
| 音長 | 定義: "l" |
| 音符 | 定義: "l" |
| 音階 | 定義: "o" |
| 音色 | 定義: "@" |
| 音量 | 定義: "v" |
| 音量予約 | 定義: "v.onTime=" |
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
| っ | 定義: "r" |
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
| − | 定義: "-" |
| ‘ | 定義: "`" |


## 1文字コマンド

| コマンド | 説明    |
|---------|--------|
| / \t / \r / ｜ / ; | 空白文字 |
| c / d / e / f / g / a / b | ドレミファソラシ c(音長),(ゲート),(音量),(タイミング),(音階) |
| n | 番号を指定して発音(例: n36) n(番号),(音長),(ゲート),(音量),(タイミング) |
| r | 休符 |
| l | 音長の指定(例 l4) |
| o | 音階の指定(例 o5) 範囲:0-10 |
| p | ピッチベンドの指定 範囲:0-127 (63が中央) |
| q | ゲートの指定 (例 q90) 範囲:0-100 |
| v | ベロシティ音量の指定 範囲:0-127 / v.Random=n |
| t | 発音タイミングの指定 (例 t-1) / t.Random=n |
| y | コントロールチェンジの指定 (例 y1,100) 範囲:0-127 / y1.onTime(low,high,len) |
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
| ? | ここから演奏する (=PLAY_FROM) |
| & | タイ(todo: 現在未実装) |


## 大文字コマンド

| コマンド | 説明    |
|---------|--------|
| TR / TRACK / Track | トラック変更　TR=番号 範囲:0- |
| CH / Channel | チャンネル変更 CH=番号 範囲:1-16 |
| TIME / Time | タイム変更 TIME(節:拍:ステップ) |
| RHYTHM / Rhythm / R | リズムモード |
| RYTHM / Rythm | リズムモード(v1の綴りミス対処[^^;]) |
| DIV / Div | 連符 (例 DIV{ceg} ) |
| SUB / Sub | タイムポインタを戻す (例 SUB{ceg} egb) |
| KF / KeyFlag | 臨時記号を設定 - KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note) |
| KEY / Key / KeyShift | ノート(cdefgab)のキーをn半音シフトする (例 KEY=3 cde) |
| INT / Int | 変数を定義 (例 INT TestValue=30) |
| STR / Str | 文字列変数を定義 (例 STR A={cde}) |
| PLAY / Play | 複数トラックを１度に書き込む (例 PLAY={aa},{bb},{cc}) |
| PRINT / Print | 文字を出力する (例 PRINT{"cde"} )(例 INT AA=30;PRINT(AA)) |
| PLAY_FROM / PlayFrom | ここから演奏する　(?と同じ意味) |
| System.MeasureShift | 小節番号をシフトする (例 System.MeasureShift(1)) |
| System.KeyFlag | 臨時記号を設定 - KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note) |
| System.TimeBase / TIMEBASE / Timebase | タイムベースを設定 (例 TIMEBASE=96) |
| TRACK_SYNC / TrackSync | 全てのトラックのタイムポインタを同期する |
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
| TRACK_NAME / TrackName | 曲名 (例 TRACK_NAME{"aaa"}) |
| InstrumentName | 楽器名 (例 InstrumentName{"aaa"}) |
| LYRIC / Lyric | メタテキスト歌詞 (例 LYRIC{"aaa"}) |
| MAKER / Marker | マーカー (例 MAKER{"aaa"}) |
| CuePoint | キューポイント (例 CuePoint{"aaa"}) |


## マクロや音色など変数定義

| 変数名 | 値    |
|---------|--------|
| OctaveUnison |  オクターブユニゾンを演奏 (例 OctaveUnison{cde}) (値:"Sub{> #?1 <} #?1") |
| Unison5th |  5度のユニゾンを演奏 (例 Unison5th{cde}) (値:"Sub{ Key=7 #?1 Key=0 } #?1") |
| Unison3th |  3度のユニゾンを演奏 (例 Unison3th{cde}) (値:"Sub{ Key=4 #?1 Key=0 } #?1") |
| Unison |  N度のユニゾンを演奏 (例 Unison{cde},7) (値:"Sub{ Key=#?2 #?1 Key=0 } #?1") |
| GrandPiano |  音色:GrandPiano (値:1) |
| BrightPiano |  音色:BrightPiano (値:2) |
| ElectricGrandPiano |  音色:ElectricGrandPiano (値:3) |
| HonkyTonkPiano |  音色:HonkyTonkPiano (値:4) |
| ElectricPiano1 |  音色:ElectricPiano1 (値:5) |
| ElectricPiano2 |  音色:ElectricPiano2 (値:6) |
| Harpsichord |  音色:Harpsichord (値:7) |
| Clavi |  音色:Clavi (値:8) |
| CelestaStrings |  音色:CelestaStrings (値:9) |
| Glockenspiel |  音色:Glockenspiel (値:10) |
| MusicBox |  音色:MusicBox (値:11) |
| Vibraphone |  音色:Vibraphone (値:12) |
| Marimba |  音色:Marimba (値:13) |
| Xylophone |  音色:Xylophone (値:14) |
| TubularBells |  音色:TubularBells (値:15) |
| Dulcimer |  音色:Dulcimer (値:16) |
| DrawbarOrgan |  音色:DrawbarOrgan (値:17) |
| PercussiveOrgan |  音色:PercussiveOrgan (値:18) |
| RockOrgan |  音色:RockOrgan (値:19) |
| ChurchOrgan |  音色:ChurchOrgan (値:20) |
| ReedOrgan |  音色:ReedOrgan (値:21) |
| Accordion |  音色:Accordion (値:22) |
| Hamonica |  音色:Hamonica (値:23) |
| TangoAccordion |  音色:TangoAccordion (値:24) |
| NylonGuitar |  音色:NylonGuitar (値:25) |
| SteelcGuitar |  音色:SteelcGuitar (値:26) |
| JazzGuitar |  音色:JazzGuitar (値:27) |
| CleanGuitar |  音色:CleanGuitar (値:28) |
| MutedGuitar |  音色:MutedGuitar (値:29) |
| OverdrivenGuitar |  音色:OverdrivenGuitar (値:30) |
| DistortionGuitar |  音色:DistortionGuitar (値:31) |
| GuitarHarmonics |  音色:GuitarHarmonics (値:32) |
| AcousticBass |  音色:AcousticBass (値:33) |
| FingerBase |  音色:FingerBase (値:34) |
| FingerBass |  音色:FingerBass (値:34) |
| PickBass |  音色:PickBass (値:35) |
| FretlessBass |  音色:FretlessBass (値:36) |
| SlapBass1 |  音色:SlapBass1 (値:37) |
| SlapBass2 |  音色:SlapBass2 (値:38) |
| SynthBass1 |  音色:SynthBass1 (値:39) |
| SynthBass2 |  音色:SynthBass2 (値:40) |
| Violin |  音色:Violin (値:41) |
| Viola |  音色:Viola (値:42) |
| Cello |  音色:Cello (値:43) |
| Contrabass |  音色:Contrabass (値:44) |
| TremoloStrings |  音色:TremoloStrings (値:45) |
| PizzicatoStrings |  音色:PizzicatoStrings (値:46) |
| OrchestralHarp |  音色:OrchestralHarp (値:47) |
| Timpani |  音色:Timpani (値:48) |
| Strings1 |  音色:Strings1 (値:49) |
| Strings2 |  音色:Strings2 (値:50) |
| SynthStrings1 |  音色:SynthStrings1 (値:51) |
| SynthStrings2 |  音色:SynthStrings2 (値:52) |
| ChoirAahs |  音色:ChoirAahs (値:53) |
| VoiceOohs |  音色:VoiceOohs (値:54) |
| SynthVoice |  音色:SynthVoice (値:55) |
| OrchestraHit |  音色:OrchestraHit (値:56) |
| Trumpet |  音色:Trumpet (値:57) |
| Trombone |  音色:Trombone (値:58) |
| Tuba |  音色:Tuba (値:59) |
| MutedTrumpet |  音色:MutedTrumpet (値:60) |
| FrenchHorn |  音色:FrenchHorn (値:61) |
| BrassSection |  音色:BrassSection (値:62) |
| SynthBrass1 |  音色:SynthBrass1 (値:63) |
| SynthBrass2 |  音色:SynthBrass2 (値:64) |
| SopranoSax |  音色:SopranoSax (値:65) |
| AltoSax |  音色:AltoSax (値:66) |
| TenorSax |  音色:TenorSax (値:67) |
| BaritoneSax |  音色:BaritoneSax (値:68) |
| Oboe |  音色:Oboe (値:69) |
| EnglishHorn |  音色:EnglishHorn (値:70) |
| Bassoon |  音色:Bassoon (値:71) |
| Clarinet |  音色:Clarinet (値:72) |
| Piccolo |  音色:Piccolo (値:73) |
| Flute |  音色:Flute (値:74) |
| Recorder |  音色:Recorder (値:75) |
| PanFlute |  音色:PanFlute (値:76) |
| BlownBottle |  音色:BlownBottle (値:77) |
| Shakuhachi |  音色:Shakuhachi (値:78) |
| Whistle |  音色:Whistle (値:79) |
| Ocarina |  音色:Ocarina (値:80) |
| SquareLead |  音色:SquareLead (値:81) |
| SawtoothLead |  音色:SawtoothLead (値:82) |
| CalliopeLead |  音色:CalliopeLead (値:83) |
| ChiffLead |  音色:ChiffLead (値:84) |
| CharangLead |  音色:CharangLead (値:85) |
| VoiceLead |  音色:VoiceLead (値:86) |
| FifthsLead |  音色:FifthsLead (値:87) |
| BassLead |  音色:BassLead (値:88) |
| NewAgePad |  音色:NewAgePad (値:89) |
| WarmPad |  音色:WarmPad (値:90) |
| PolySynthPad |  音色:PolySynthPad (値:91) |
| ChoirPad |  音色:ChoirPad (値:92) |
| BowedPad |  音色:BowedPad (値:93) |
| MetallicPad |  音色:MetallicPad (値:94) |
| HaloPad |  音色:HaloPad (値:95) |
| SweepPad |  音色:SweepPad (値:96) |
| Rain |  音色:Rain (値:97) |
| SoundTrack |  音色:SoundTrack (値:98) |
| Crystal |  音色:Crystal (値:99) |
| Atmosphere |  音色:Atmosphere (値:100) |
| Brightness |  音色:Brightness (値:101) |
| Goblins |  音色:Goblins (値:102) |
| Echoes |  音色:Echoes (値:103) |
| Sci_Fi |  音色:Sci_Fi (値:104) |
| Sitar |  音色:Sitar (値:105) |
| Banjo |  音色:Banjo (値:106) |
| Shamisen |  音色:Shamisen (値:107) |
| Koto |  音色:Koto (値:108) |
| Kalimba |  音色:Kalimba (値:109) |
| Bagpipe |  音色:Bagpipe (値:110) |
| Fiddle |  音色:Fiddle (値:111) |
| Shanai |  音色:Shanai (値:112) |
| TibkleBell |  音色:TibkleBell (値:113) |
| TinkleBell |  音色:TinkleBell (値:113) |
| Agogo |  音色:Agogo (値:114) |
| SteelDrums |  音色:SteelDrums (値:115) |
| Woodblock |  音色:Woodblock (値:116) |
| TaikoDrum |  音色:TaikoDrum (値:117) |
| MelodicTom |  音色:MelodicTom (値:118) |
| SynthDrum |  音色:SynthDrum (値:119) |
| ReverseCymbal |  音色:ReverseCymbal (値:120) |
| FretNoise |  音色:FretNoise (値:121) |
| BreathNoise |  音色:BreathNoise (値:122) |
| Seashore |  音色:Seashore (値:123) |
| BirdTweet |  音色:BirdTweet (値:124) |
| TelephoneRing |  音色:TelephoneRing (値:125) |
| Helicopter |  音色:Helicopter (値:126) |
| Applause |  音色:Applause (値:127) |
| Gunshot |  音色:Gunshot (値:128) |
| StandardSet |  音色:StandardSet (値:1) |
| StandardSet2 |  音色:StandardSet2 (値:2) |
| RoomSet |  音色:RoomSet (値:9) |
| PowerSet |  音色:PowerSet (値:17) |
| ElectronicSet |  音色:ElectronicSet (値:25) |
| AnalogSet |  音色:AnalogSet (値:26) |
| DanceSet |  音色:DanceSet (値:27) |
| JazzSet |  音色:JazzSet (値:33) |
| BrushSet |  音色:BrushSet (値:41) |
| OrchestraSet |  音色:OrchestraSet (値:49) |
| SnareRoll |  音色:SnareRoll (値:25) |
| FingerSnap |  音色:FingerSnap (値:26) |
| HighQ |  音色:HighQ (値:27) |
| Slap |  音色:Slap (値:28) |
| ScratchPush |  音色:ScratchPush (値:29) |
| ScratchPull |  音色:ScratchPull (値:30) |
| Sticks |  音色:Sticks (値:31) |
| SquareClick |  音色:SquareClick (値:32) |
| MetronomeClick |  音色:MetronomeClick (値:33) |
| MetronomeBell |  音色:MetronomeBell (値:34) |
| Kick2 |  音色:Kick2 (値:35) |
| Kick1 |  音色:Kick1 (値:36) |
| SideStick |  音色:SideStick (値:37) |
| Snare1 |  音色:Snare1 (値:38) |
| HandClap |  音色:HandClap (値:39) |
| Snare2 |  音色:Snare2 (値:40) |
| LowTom2 |  音色:LowTom2 (値:41) |
| ClosedHiHat |  音色:ClosedHiHat (値:42) |
| LowTom1 |  音色:LowTom1 (値:43) |
| PedalHiHat |  音色:PedalHiHat (値:44) |
| MidTom2 |  音色:MidTom2 (値:45) |
| OpenHiHat |  音色:OpenHiHat (値:46) |
| MidTom1 |  音色:MidTom1 (値:47) |
| HighTom2 |  音色:HighTom2 (値:48) |
| CrashCymbal1 |  音色:CrashCymbal1 (値:49) |
| HighTom1 |  音色:HighTom1 (値:50) |
| RideCymbal1 |  音色:RideCymbal1 (値:51) |
| ChineseCymbal |  音色:ChineseCymbal (値:52) |
| RideBell |  音色:RideBell (値:53) |
| Tambourine |  音色:Tambourine (値:54) |
| SplashCymbal |  音色:SplashCymbal (値:55) |
| Cowbell |  音色:Cowbell (値:56) |
| CrashCymbal2 |  音色:CrashCymbal2 (値:57) |
| VibraSlap |  音色:VibraSlap (値:58) |
| RideCymbal2 |  音色:RideCymbal2 (値:59) |
| HighBongo |  音色:HighBongo (値:60) |
| LowBongo |  音色:LowBongo (値:61) |
| MuteHighConga |  音色:MuteHighConga (値:62) |
| OpenHighConga |  音色:OpenHighConga (値:63) |
| LowConga |  音色:LowConga (値:64) |
| HighTimbale |  音色:HighTimbale (値:65) |
| LowTimbale |  音色:LowTimbale (値:66) |
| HighAgogo |  音色:HighAgogo (値:67) |
| LowAgogo |  音色:LowAgogo (値:68) |
| Cabasa |  音色:Cabasa (値:69) |
| Maracas |  音色:Maracas (値:70) |
| ShortHiWhistle |  音色:ShortHiWhistle (値:71) |
| LongLowWhistle |  音色:LongLowWhistle (値:72) |
| ShortGuiro |  音色:ShortGuiro (値:73) |
| LongGuiro |  音色:LongGuiro (値:74) |
| Claves |  音色:Claves (値:75) |
| HighWoodBlock |  音色:HighWoodBlock (値:76) |
| LowWoodBlock |  音色:LowWoodBlock (値:77) |
| MuteCuica |  音色:MuteCuica (値:78) |
| OpenCuica |  音色:OpenCuica (値:79) |
| MuteTriangle |  音色:MuteTriangle (値:80) |
| OpenTriangle |  音色:OpenTriangle (値:81) |
| Shaker |  音色:Shaker (値:82) |
| JingleBell |  音色:JingleBell (値:83) |
| BellTree |  音色:BellTree (値:84) |
| Castanets |  音色:Castanets (値:85) |
| MuteSurdo |  音色:MuteSurdo (値:86) |
| OpenSurdo |  音色:OpenSurdo (値:87) |


## リズムマクロ

| 変数名 | 値    |
|---------|--------|
| b | "n36," |
| s | "n38," |
| h | "n42," |
| H | "n44," |
| o | "n46," |
| c | "n49," |