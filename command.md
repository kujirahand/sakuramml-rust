# Sakuramml command list - テキスト音楽 サクラ

## Single-character command

Single-character(lower case) command list. (1文字小文字コマンド)

| Command | Description |
|---------|--------|
| SPACE TAB CR LF ; CHR(0x7C) | space - 空白文字 / ';'や'|'も読み飛ばす |
| c   d   e   f   g   a   b | note - ドレミファソラシ c(音長),(ゲート),(音量),(タイミング),(音階) |
| n | note no - 番号を指定して発音(例: n36) n(番号),(音長),(ゲート),(音量),(タイミング) |
| r | rest - 休符 |
| l | length - 音長の指定(例 l4) |
| o | octave - 音階の指定(例 o5) 範囲:0-10 |
| p | pitch bend - ピッチベンドの指定 範囲:0-127 (63が中央) / (ref) PB(n) は -8192~0~8191 |
| q | gate - ゲートの指定 (例 q90) 範囲:0-100 |
| v | velocity - ベロシティ音量の指定 範囲:0-127 / v.Random=n |
| t | timing - 発音タイミングの指定 (例 t-1) / t.Random=n |
| y | Control change - コントロールチェンジの指定 (ex) y1,100) / range:0-127 / y1.onTime(low,high,len) |
| # | マクロ |
| @ | Voice select(音色の指定) range:1-128 (format) @(no),(Bank_LSB),(Bank_MSB) |
| > | Octave up (音階を1つ上げる) |
| < | Octave down (音階を1つ下げる) |
| ) | velocity up - 音量をvAddの値だけ上げる |
| ( | velocity down - 音量をvAddの値だけ下げる |
| "\/\*" ... "\*\/" | range comment (範囲コメント) |
| "///" | line comment for debug(デバッグ用一行コメント) |
| "//" | line comment (一行コメント) |
| "##" | line comment (一行コメント) |
| "# " | line comment (一行コメント) |
| "#-" | line comment (一行コメント) |
| [ | begin of loop - ループ開始 (ex) [4 cdeg] |
| : | break of loop - ループ最終回に脱出 (ex)　[4 cde:g]e |
| ] | end of loop - ループ終了 |
| \ | harmony - 和音 (ex) 'ceg' (format) 'ceg'(音長),(ゲート) |
| $ | define rhythm macro - リズムマクロ定義 (format) $char{ defined } |
| { | 連符 (例 {ceg}4) {c^d}(音長) |
| ` | Octave up once - 一度だけ音階を+1する |
| " | Octave down once - 一度だけ音階を-1する |
| ? | play from here - ここから演奏する (=PlayFromHere) |
| & | tie and slur - タイ・スラー(Slurコマンドで動作が変更できる) |


## Multiple-character command

Multiple-character(upper case) command list. (複数文字/大文字コマンド)

| Command | Description |
|---------|--------|
| End | end of song |
| END | end of song |
| Track | change current track [range:0 to 999] (ex) Track(1) |
| TRACK | change current track [range:0 to 999] (ex) TRACK(1) |
| TR | change current track [range:0 to 999] (ex) TR(1) |
| Channel | change channel no [range:1 to 16] (ex) Channel(1) |
| CHANNEL | change channel no [range:1 to 16] (ex) CHANNEL(1) |
| CH | change channel no [range:1 to 16] (ex) CH(1) |
| Time | change time position, Time(measure:beat:step) (ex) Time(1:1:0) Time(0) |
| TIME | change time position, TIME(measure:beat:step) (ex) Time(1:1:0) Time(0) |
| System.TimeBase | set system time base (ex) TimeBase(96) |
| Timebase | set system time base (ex) TimeBase(96) |
| TimeBase | set system time base (ex) TimeBase(96) |
| TIMEBASE | set system time base (ex) TimeBase(96) |
| Rhythm | read Rhythm notes (ex) Rhythm{ bhsh bhsh } |
| RHYTHM | read Rhythm notes (ex) Rhythm{ bhsh bhsh } |
| R | read Rhythm notes (ex) Rhythm{ bhsh bhsh } |
| Rythm | 互換性:綴りミス [typo] read Rhythm notes (ex) Rhythm{ bhsh bhsh } |
| RYTHM | 互換性:綴りミス [typo] read Rhythm notes (ex) Rhythm{ bhsh bhsh } |
| Div | tuplet(連符) (ex) Div{ ceg } |
| DIV | tuplet(連符) (ex) Div{ ceg } |
| Sub | sub track / rewind time position (ex) Sub{ceg} egb |
| SUB | sub track / rewind time position (ex) Sub{ceg} egb |
| S | sub track / rewind time position (ex) Sub{ceg} egb |
| System.KeyFlag | set key flag to note (ex) KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note) |
| KeyFlag | set key flag to note (ex) KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note) |
| KF | set key flag to note (ex) KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note) |
| KeyShift | set key-shift (ex) KeyShift(3) |
| Key | set key-shift (ex) Key(3) |
| KEY | set key-shift (ex) KEY(3) |
| UseKeyShift | set key shift mode value=on|off (ex) UseKeyShift(on) |
| TrackKey | set key-shift for track (ex) TrackKey(3) |
| TR_KEY | set key-shift for track (ex) TR_KEY(3) |
| Play | play multi track (ex) Play(AA,BB,CC) |
| PLAY | play multi track (ex) Play(AA,BB,CC) |
| SysEx | System Exclusive (ex) SysEx$=f0,43,10,4c,00,{00,00,30,f0},f7 |
| PlayFrom.SysEx | Unimplemented |
| PlayFrom.CtrlChg | Unimplemented |
| PlayFrom | play from time position (ex) PlayFrom(5:1:0) |
| PLAY_FROM | play from time position (ex) PLAY_FROM(5:1:0) |
| PlayFromHere | play from current time pos (ex) PlayFromHere |
| PLAY_FROM_HRER | play from current time pos (ex) PLAY_FROM_HERE |
| System.MeasureShift | set measure shift for time pointer (ex) System.MeasureShift(1) |
| MeasureShift | set measure shift for time pointer (ex) MeasureShift(1) |
| MEASURE_SHIFT | set measure shift for time pointer (ex) MeasureShift(1) |
| TrackSync | synchronize time pointers for all tracks (ex) TrackSync |
| TRACK_SYNC | synchronize time pointers for all tracks (ex) TrackSync |
| Slur | set slur/tie(&) mode (0:グリッサンド/1:ベンド/2:ゲート/3:アルペジオ) (ex) Slur(1) |
| SLUR | set slur/tie(&) mode (0:グリッサンド/1:ベンド/2:ゲート/3:アルペジオ) (ex) Slur(1) |
| System.vAdd | set relative velocity '(' or ')' or 'v++' or 'v--' command increment value (ex) vAdd(3) |
| vAdd | set relative velocity '(' or ')' or 'v++' or 'v--' command increment value (ex) vAdd(3) |
| System.qAdd | set "q++" command value (ex) qAdd(3) |
| qAdd | set "q++" command value (ex) qAdd(3) |
| System.q2Add | Unimplemented |
| q2Add | Unimplemented |
| SoundType | set sound type (ex) SoundType({pico}) |
| DeviceNumber | set Device Number (ex) DeviceNumber=$10 |
| Voice | set voice (=@) range: 1-128 Voice(n[,msb,lsb]) (ex) Voice(1) |
| VOICE | set voice (=@) range: 1-128 Voice(n[,msb,lsb]) (ex) Voice(1) |
| M | CC#1 Modulation (ex) M(10) |
| Modulation | CC#1 Modulation range:0-127 (ex) M(10) |
| PT | CC#5 Portamento Time range:0-127 (ex) PT(10) |
| PortamentoTime | CC#5 Portamento Time range:0-127 (ex) PT(10) |
| V | CC#7 Main Volume range:0-127 (ex) V(10) |
| MainVolume | CC#7 Main Volume range:0-127 (ex) V(10) |
| P | CC#10 Panpot range:0-63-127 (ex) P(63) |
| Panpot | CC#10 Panpot range:0-63-127 (ex) Panpot(63) |
| EP | CC#11 Expression range:0-127 (ex) EP(100) |
| Expression | CC#11 Expression range:0-127 (ex) EP(100) |
| PS | CC#65 Portament switch range:0-127 (ex) PS(1) |
| PortamentoSwitch | CC#65 Portament switch range:0-127 (ex) PS(1) |
| REV | CC#91 Reverb range:0-127 (ex) REV(100) |
| Reverb | CC#91 Reverb range:0-127 (ex) REV(100) |
| CHO | CC#93 Chorus range:0-127 (ex) CHO(100) |
| Chorus | CC#93 Chorus range:0-127 (ex) Chorus(100) |
| VAR | CC#94 Variation range:0-127 (ex) VAR(100) |
| Variation | CC#94 Variation range:0-127 (ex) Variation(100) |
| PitchBend | Pitchbend range: -8192~0~8191 (ex) PitchBend(10) / p(value) range: 0~63~127 |
| PB | Pitchbend range: -8192~0~8191 (ex) PB(10) |
| PitchBendSensitivity | PitchBendSensitivity (ex) BR(10) |
| BEND_RANGE | PitchBendSensitivity (ex) BEND_RANGE(10) |
| BendRange | PitchBendSensitivity (ex) BendRange(10) |
| BR | PitchBendSensitivity (ex) BR(10) |
| RPN | write RPN (ex) RPN(0,1,64) |
| NRPN | write NRPN (ex) NRPN(1,1,1) |
| FineTune | set fine tune range:0-63-127(-100 - 0 - +99.99セント）(ex) FineTune(63) |
| CoarseTune | set coarse tune 半音単位のチューニング 範囲:40-64-88 (-24 - 0 - 24半音) (ex) CoarseTune(63) |
| VibratoRate | set VibratoRate range: 0-127 |
| VibratoDepth | set VibratoRate range: 0-127 |
| VibratoDelay | set VibratoRate range: 0-127 |
| FilterCutoff | set FilterCutoff range: 0-127 |
| FilterResonance | set FilterResonance range: 0-127 |
| EGAttack | set EGAttack range: 0-127 |
| EGDecay | set EGDecay range: 0-127 |
| EGRelease | set EGRelease range: 0-127 |
| Fadein | fadein 小節数を指定 (ex) Fadein(1) |
| Fadeout | fadeout 小節数を指定 (ex) Fadeout(1) |
| Cresc | cresc 小節数を指定 Cresc([[[len],v1],v2]) v1からv2へ変更する。lenを省略すると全音符の長さに (ex) Cresc(1) |
| Decresc | cresc 小節数を指定 Decresc([[[len],v1],v2]) v1からv2へ変更する。lenを省略すると全音符の長さに (ex) Deresc(1) |
| CRESC | cresc 小節数を指定 Cresc([[[len],v1],v2]) v1からv2へ変更する。lenを省略すると全音符の長さに (ex) Cresc(1) |
| DECRESC | cresc 小節数を指定 Decresc([[[len],v1],v2]) v1からv2へ変更する。lenを省略すると全音符の長さに (ex) Deresc(1) |
| ResetGM | ResetGM |
| ResetGS | ResetGS |
| ResetXG | ResetXG |
| MasterVolume | master volume (range: 0-127) (ex) MasterVolume(100) |
| MasterBalance | master ballance (range: -8192 to 8191(ex) MasterBallance(0) |
| Tempo | set tempo (ex) Tempo(120) |
| TEMPO | set tempo (ex) TEMPO(120) |
| T | set tempo (ex) T(120) |
| BPM | set tempo (ex) BPM(120) |
| TempoChange | tempo change slowly TempoChange(start, end, !len) (ex) TempoChange(80,120,!1) |
| TimeSignature | set time signature (ex) TimeSignature(4, 4) |
| System.TimeSignature | set time signature (ex) TimeSignature(4, 4) |
| TimeSig | set time signature (ex) TimeSignature(4, 4) |
| TIMESIG | set time signature (ex) TimeSignature(4, 4) |
| Port | set Port No (ex) Port(0) |
| PORT | set Port No (ex) Port(0) |
| MetaText | write meta text (ex) MetaText{"hello"} |
| Text | write meta text (ex) MetaText{"hello"} |
| TEXT | write meta text (ex) MetaText{"hello"} |
| Copyright | write copyright text (ex) Copyright{"hello"} |
| COPYRIGHT | write copyright text (ex) COPYRIGHT{"hello"} |
| TrackName | write TrackName text (ex) TrackName{"hello"} |
| TRACK_NAME | write TrackName text (ex) TrackName{"hello"} |
| InstrumentName | write InstrumentName text (ex) InstrumentName{"hello"} |
| Lyric | write Lyric text (ex) Lyric{"hello"} |
| LYRIC | write Lyric text (ex) LYRIC{"hello"} |
| MAKER | write MAKER text (ex) MAKER{"hello"} |
| Maker | write Maker text (ex) Maker{"hello"} |
| CuePoint | write CuePoint text (ex) CuePoint{"hello"} |
| GSEffect | GSEffect(num, val) (ex) GSEffect($30, 0) |
| GSReverbMacro | GSReverbMacro(val) - 0:Room1 5:Hall 6:Delay (ex) GSReverbMacro(0) |
| GSReverbCharacter | GSReverbCharacter(val) - 0:Room1 5:Hall 6:Delay (ex) GSReverbMacro(0) |
| GSReverbPRE_LPE | GSReverbPRE_LPE(val) (ex) GSReverbPRE_LPE(0) |
| GSReverbLevel | GSReverbLevel(val) (ex) GSReverbLevel(0) |
| GSReverbTime | GSReverbTime(val) (ex) GSReverbTime(0) |
| GSReverbFeedback | GSReverbFeedback(val) (ex) GSReverbFeedback(0) |
| GSReverbSendToChorus | GSReverbSendToChorus(val) (ex) GSReverbSendToChorus(0) |
| GSChorusMacro | GSChorusMacro(val) (ex) GSChorusMacro(0) |
| GSChorusPRE_LPF | GSChorusPRE_LPF(val) (ex) GSChorusPRE_LPF(0) |
| GSChorusLevel | GSChorusLevel(val) (ex) GSChorusLevel(0) |
| GSChorusFeedback | GSChorusFeedback(val) (ex) GSChorusFeedback(0) |
| GSChorusDelay | GSChorusDelay(val) (ex) GSChorusDelay(0) |
| GSChorusRate | GSChorusRate(val) (ex) GSChorusRate(0) |
| GSChorusDepth | GSChorusDepth(val) (ex) GSChorusDepth(0) |
| GSChorusSendToReverb | GSChorusSendToReverb(val) (ex) GSChorusSendToReverb(0) |
| GSChorusSendToDelay | GSChorusSendToDelay(val) (ex) GSChorusSendToDelay(0) |
| GS_RHYTHM | Change to rhythm part val=0:instrument/1:drum1/2:drum2 (ex) GSChorusSendToDelay(0) |
| GSScaleTuning | GS Scale Tuning. GSScaleTuning(C,Cp,D,Dp,E,F,Fp,G,Gp,A,Ap,B) (ex) GSScaleTuning(0,0,0,0,0,0,0,0,0,0,0,0) |
| Int | define int variables (ex) Int A = 3 |
| INT | define int variables (ex) INT A = 3 |
| Str | define string variables (ex) Str A = {cde} |
| STR | define string variables (ex) STR A = {cde} |
| Array | define string variables (ex) Str A = {cde} |
| ARRAY | define string variables (ex) STR A = {cde} |
| Print | print value (ex) Print({hello}) |
| PRINT | print value (ex) PRINT({hello}) |
| System.Include | Unimplemented |
| Include | Unimplemented |
| INCLUDE | Unimplemented |
| IF | IF(cond){ true }ELSE{ false } |
| If | IF(cond){ true }ELSE{ false } |
| FOR | FOR(INT I = 0; I < 10; I++){ ... } |
| For | FOR(INT I = 0; I < 10; I++){ ... } |
| WHILE | WHILE(cond) { ... } |
| While | WHILE(cond) { ... } |
| BREAK | exit from loop |
| Break | exit from loop |
| EXIT | exit from loop |
| Exit | exit from loop |
| CONTINUE | exit from loop |
| Continue | exit from loop |
| RETURN | return from function |
| Return | return from function |
| Result | set function's result |
| RANDOM_SEED | set random seed |
| RandomSeed | set random seed |
| FUNCTION | define user function |
| Function | define user function |


## Values in a formula

Values that can be referenced in a formula (計算式で参照できる値)

| Command | Description |
|---------|--------|
| TR /  TRACK /  Track  |現在のトラック番号を得る|
| CH /  CHANNEL  |現在のチャンネル番号を得る|
| TIME /  Time  |現在のタイムポインタ値を得る|


## Macro and Voice List 

[🔗voice list - 日本語付きの音色一覧はこちら](voice.md)
Macros and Voice list (マクロや音色など変数定義):

| Voice | Description |
|-------|----|
| OctaveUnison |  オクターブユニゾンを演奏 (例 OctaveUnison{cde}) (値:"Sub{> #?1 <} #?1") |
| Unison5th |  5度のユニゾンを演奏 (例 Unison5th{cde}) (値:"Sub{ Key=7 #?1 Key=0 } #?1") |
| Unison3th |  3度のユニゾンを演奏 (例 Unison3th{cde}) (値:"Sub{ Key=4 #?1 Key=0 } #?1") |
| Unison |  N度のユニゾンを演奏 (例 Unison{cde},7) (値:"Sub{ Key=#?2 #?1 Key=0 } #?1") |
| SLUR_PORT |  スラー定数。グリッサンド。ノートオンを、ポルタメントでつなぐ (例 Slur(SlurPort, !8) のように指定) (値:0) |
| SLUR_BEND |  スラー定数。ベンド。異音程をベンドで表現。ギターのハンマリングに近い。 (例 Slur(SlurPort, !8) のように指定) (値:1) |
| SLUR_GATE |  スラー定数。＆のついた音符のゲートを、valueにする (値:2) |
| SLUR_ALPE |  スラー定数。＆でつないだ音符の終わりまでゲートを伸ばす (値:3) |
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


## Rhythm macro

Rhythm macro (リズムマクロ)

| Macro's name | Value |
|---------|--------|
| b | "n36," |
| s | "n38," |
| h | "n42," |
| H | "n44," |
| o | "n46," |
| c | "n49," |
| _ | "r" |


## Sutoton

日本語で指示できるストトン表記


| ストトン表記 | 説明 (=定義) |
| ---------|---------|
| 全音符 | 全音符を基本音符にする (="l1") |
| 二分音符 | 二分音符を基本音符にする (="l2") |
| 四分音符 | 四分音符を基本音符にする (="l4") |
| 八分音符 | 八分音符を基本音符にする (="l8") |
| 十六音符 | 十六音符を基本音符にする (="l16") |
| 付点四分音符 | 付点四分音符を基本音符にする (="l4.") |
| 音源初期化 | 音源初期化//音源の初期化(GMリセット)を実行する。（例）音源初期化 (="System.MeasureShift(1ResetGM;Time(1:1:0TrackSync;") |
| 音長 | 基本音符を指定 (="l") |
| 音量予約 | 音量を予約指定する (="v.onTime=") |
| 「 | 和音はじめ (="'") |
| 」 | 和音おわり (="'") |
| 【 | 繰り返しはじめ (="[") |
| 】 | 繰り返しおわり (="]") |
| ↑ | オクターブを1つ上げる (=">") |
| ↓ | オクターブを1つ下げる (="<") |
| ♭ | フラット (="-") |
| ♯ | シャープ (="#") |
| − | マイナス (="-") |
| ‘ | 次の音符をオクターブ1つ上げる (="`") |
| 調 | 調#(音符)//臨時記号を設定する。（例）調＃（ドファ） (="System.KeyFlag") |
| 音階 | 音階(数値)//音階を数値で指定する。初期値は５。範囲は、0～10（例）音階５ (="o") |
| 時間 | 時間(小節数:拍数:ステップ数)//指定時間にポインタを移動する。範囲は、小節数・拍数が、１～。ステップ数は、０～。（例）時間（４：１：０） (="Time") |
| 読む | 読む(ファイル名)//外部定義ファイルを読み込む。（例）読む(chord2.h) (="Include") |
| 予約 | (コマンド)予約(v1,v2,v3...)//コマンドの値を予約しておく（例）音量予約120,50【ドレミファ】 (=".onNote=") |
| 拍子 | 拍子 分子,分母//拍子を設定する。（例）拍子4,4 (="System.TimeSignature=") |
| 音色 | 音色（番号）//音色を設定する。 (="@") |
| 音符 | 音符（ｎ分音符指定）//基本となる音符の長さをｎ分音符で指定する。（例）音符16//１６分音符の意味 (="l") |
| 音量 | 音量（数値）//音量(実際は音の強さ)を設定する。初期値は、100。範囲は、0~127。（例）音量127 (="v") |
| 連符 | 連符{音名}[音長]//３連符や５連符などを表現する。（例）連符{ドレミ}4 (="Div") |
| ゲート | ゲート（割合）//音符の長さに対する実際の発音時間を割合（100分率）で指定する。範囲は、1～100～。（例）ゲート80 (="q") |
| テンポ | テンポ（数値）//テンポを設定する。初期値は、120。範囲は、20～240を推奨。（例）テンポ120 (="Tempo=") |
| 曖昧さ | (コマンド)曖昧さ（数値）//各属性の曖昧さを設定する。範囲は、0～。初期値は、0。（例）音量曖昧さ80 【ドレミソ】 (=".Random=") |
| トラック | トラック（番号）//トラック番号を指定する。初期値は、０。範囲は、0～。（例）トラック３ (="Track=") |
| チャンネル | チャンネル（番号）//現在のトラックにチャンネルを設定する。初期値は、トラック番号と同じ。範囲は、１～１６（例）トラック３チャンネル１０ (="Channel=") |
| 曲名 | 曲名{"文字列"}//生成するMIDIファイルに曲名を埋め込む。（例）曲名{"テスト"} (="TrackName=") |
| 作者 | 作者{"文字列"}//生成するMIDIファイルに著作権情報を埋め込む。（例）作者{"クジラ飛行机"} (="Copyright=") |
| コメント | コメント{"文字列"}//生成するMIDIファイルにコメントを埋め込む。（例）コメント{"テスト"} (="MetaText=") |
| 演奏位置 | 演奏位置(小節数:拍数:ステップ数))//長い曲の途中から演奏したい時、曲の演奏位置を指定する。（例）演奏位置（32:1:0） (="PlayFrom") |
| ー | ー//タイ。音を伸ばす。（例）ドードレミミソーーー (="^") |
| 上 | 音階を相対的に１つ上げる (=">") |
| 下 | 音階を相対的に１つ下げる (="<") |
| ド | 音名 (="c") |
| レ | 音名 (="d") |
| ミ | 音名 (="e") |
| フ | 音名 (="f") |
| ァ | 音名 (="") |
| ソ | 音名 (="g") |
| ラ | 音名 (="a") |
| シ | 音名 (="b") |
| ン | 休符。（例）ドーーン　レンレー (="r") |
| ッ | 休符。（例）ドーーッ　レッレー (="r") |
| ど | 音名 (="c") |
| れ | 音名 (="d") |
| み | 音名 (="e") |
| ふ | 音名 (="f") |
| ぁ | 音名 (="") |
| そ | 音名 (="g") |
| ら | 音名 (="a") |
| し | 音名 (="b") |
| ん | 休符 (="r") |
| っ | 休符 (="r") |
| イ | 音名 (="a") |
| ロ | 音名 (="b") |
| ハ | 音名 (="c") |
| ニ | 音名 (="d") |
| ホ | 音名 (="e") |
| ヘ | 音名 (="f") |
| ト | 音名 (="g") |
| 変 | フラット（例）イ変 (="-") |
| 嬰 | シャープ (="+") |
| リズム | リズムモード (="Rythm") |
| ず | バスドラム (="n36) |
| た | スネアドラム (="n38) |
| つ | ハイハット（クローズ） (="n42) |
| ち | ハイハット（オープン） (="n46) |
| ぱ | シンバル (="n49) |
| と | Lowタム (="n50) |
| む | Midタム (="n47) |
| ろ | Highタム (="n43) |
| く | ドラム (="n44) |
| 大きく | 大きく(音長),(最終値)//音量(EP)をだんだん大きくする (="Cresc=") |
| 小さく | 小さく(音長),(最終値)//音量(EP)をだんだん小さくする (="Decresc=") |
| クレッシェンド | 大きく(音長),(最終値)//音量(EP)をだんだん大きくする (="Cresc=") |
| デクレッシェンド | 小さく(音長),(最終値)//音量(EP)をだんだん小さくする (="Cresc=") |
| 音量戻す | 音量(EP)を最大値に戻す (="EP(127)") |
| 方向左 | ステレオの左から音が出るようにする (="P(0)") |
| 方向左前 | ステレオの左前から音が出るようにする (="P(32)") |
| 方向前 | ステレオの前から音が出るようにする (="P(64)") |
| 方向右前 | ステレオの右前から音が出るようにする (="P(96)") |
| 方向右 | ステレオの右から音が出るようにする (="P(127)") |
| 方向回す | ステレオの左右を回す (="P.onNoteWaveEx(0) |
| ビブラートオフ | ビブラートをやめる (="M(0)") |
| ペダル | ペダルを踏む (="y64) |
| 放す | ペダルを放す (="y64) |
| テンポ改 | テンポ改([[[t1],t2],len])//テンポを推移的に変更する。lenを省略すると、全音符の間に推移し、t1を省略すると、以前の値からt2へ推移する。 (="TempoChange=") |
| ビブラート | 推移的なビブラートをかける (="M.onNoteWaveEx(0) |
| ここから演奏 | 途中から演奏したいときに書く (="PlayFrom(Time") |