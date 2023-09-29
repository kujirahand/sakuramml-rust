# テキスト音楽 サクラ(Rust版) コマンド一覧

## ストトン表記

| コマンド | 説明    |
|---------|--------|
| 全音符 | 全音符を基本音符にする(="l1") |
| 二分音符 | 二分音符を基本音符にする(="l2") |
| 四分音符 | 四分音符を基本音符にする(="l4") |
| 八分音符 | 八分音符を基本音符にする(="l8") |
| 十六音符 | 十六音符を基本音符にする(="l16") |
| 付点四分音符 | 付点四分音符を基本音符にする(="l4.") |
| 音源初期化 | 音源初期化//音源の初期化(GMリセット)を実行する。（例）音源初期化(="System.MeasureShift(1ResetGM;Time(1:1:0TrackSync;") |
| 音長 | 基本音符を指定(="l") |
| 音量予約 | 音量を予約指定する(="v.onTime=") |
| 「 | 和音はじめ(="'") |
| 」 | 和音おわり(="'") |
| 【 | 繰り返しはじめ(="[") |
| 】 | 繰り返しおわり(="]") |
| ↑ | オクターブを1つ上げる(=">") |
| ↓ | オクターブを1つ下げる(="<") |
| ♭ | フラット(="-") |
| ♯ | シャープ(="#") |
| − | マイナス(="-") |
| ‘ | 次の音符をオクターブ1つ上げる(="`") |
| 調 | 調#(音符)//臨時記号を設定する。（例）調＃（ドファ）(="System.KeyFlag") |
| 音階 | 音階(数値)//音階を数値で指定する。初期値は５。範囲は、0～10（例）音階５(="o") |
| 時間 | 時間(小節数:拍数:ステップ数)//指定時間にポインタを移動する。範囲は、小節数・拍数が、１～。ステップ数は、０～。（例）時間（４：１：０）(="Time") |
| 読む | 読む(ファイル名)//外部定義ファイルを読み込む。（例）読む(chord2.h)(="Include") |
| 予約 | (コマンド)予約(v1,v2,v3...)//コマンドの値を予約しておく（例）音量予約120,50【ドレミファ】(=".onNote=") |
| 拍子 | 拍子 分子,分母//拍子を設定する。（例）拍子4,4(="System.TimeSignature=") |
| 音色 | 音色（番号）//音色を設定する。(="@") |
| 音符 | 音符（ｎ分音符指定）//基本となる音符の長さをｎ分音符で指定する。（例）音符16//１６分音符の意味(="l") |
| 音量 | 音量（数値）//音量(実際は音の強さ)を設定する。初期値は、100。範囲は、0~127。（例）音量127(="v") |
| 連符 | 連符{音名}[音長]//３連符や５連符などを表現する。（例）連符{ドレミ}4(="Div") |
| ゲート | ゲート（割合）//音符の長さに対する実際の発音時間を割合（100分率）で指定する。範囲は、1～100～。（例）ゲート80(="q") |
| テンポ | テンポ（数値）//テンポを設定する。初期値は、120。範囲は、20～240を推奨。（例）テンポ120(="Tempo=") |
| 曖昧さ | (コマンド)曖昧さ（数値）//各属性の曖昧さを設定する。範囲は、0～。初期値は、0。（例）音量曖昧さ80 【ドレミソ】(=".Random=") |
| トラック | トラック（番号）//トラック番号を指定する。初期値は、０。範囲は、0～。（例）トラック３(="Track=") |
| チャンネル | チャンネル（番号）//現在のトラックにチャンネルを設定する。初期値は、トラック番号と同じ。範囲は、１～１６（例）トラック３チャンネル１０(="Channel=") |
| 曲名 | 曲名{"文字列"}//生成するMIDIファイルに曲名を埋め込む。（例）曲名{"テスト"}(="TrackName=") |
| 作者 | 作者{"文字列"}//生成するMIDIファイルに著作権情報を埋め込む。（例）作者{"クジラ飛行机"}(="Copyright=") |
| コメント | コメント{"文字列"}//生成するMIDIファイルにコメントを埋め込む。（例）コメント{"テスト"}(="MetaText=") |
| 演奏位置 | 演奏位置(小節数:拍数:ステップ数))//長い曲の途中から演奏したい時、曲の演奏位置を指定する。（例）演奏位置（32:1:0）(="PlayFrom") |
| ー | ー//タイ。音を伸ばす。（例）ドードレミミソーーー(="^") |
| 上 | 音階を相対的に１つ上げる(=">") |
| 下 | 音階を相対的に１つ下げる(="<") |
| ド | 音名(="c") |
| レ | 音名(="d") |
| ミ | 音名(="e") |
| フ | 音名(="f") |
| ァ | 音名(="") |
| ソ | 音名(="g") |
| ラ | 音名(="a") |
| シ | 音名(="b") |
| ン | 休符。（例）ドーーン　レンレー(="r") |
| ッ | 休符。（例）ドーーッ　レッレー(="r") |
| ど | 音名(="c") |
| れ | 音名(="d") |
| み | 音名(="e") |
| ふ | 音名(="f") |
| ぁ | 音名(="") |
| そ | 音名(="g") |
| ら | 音名(="a") |
| し | 音名(="b") |
| ん | 休符(="r") |
| っ | 休符(="r") |
| イ | 音名(="a") |
| ロ | 音名(="b") |
| ハ | 音名(="c") |
| ニ | 音名(="d") |
| ホ | 音名(="e") |
| ヘ | 音名(="f") |
| ト | 音名(="g") |
| 変 | フラット（例）イ変(="-") |
| 嬰 | シャープ(="+") |
| リズム | リズムモード(="Rythm") |
| ず | バスドラム(="n36) |
| た | スネアドラム(="n38) |
| つ | ハイハット（クローズ）(="n42) |
| ち | ハイハット（オープン）(="n46) |
| ぱ | シンバル(="n49) |
| と | Lowタム(="n50) |
| む | Midタム(="n47) |
| ろ | Highタム(="n43) |
| く | ドラム(="n44) |
| 大きく | 大きく(音長),(最終値)//音量(EP)をだんだん大きくする(="Cresc=") |
| 小さく | 小さく(音長),(最終値)//音量(EP)をだんだん小さくする(="Decresc=") |
| クレッシェンド | 大きく(音長),(最終値)//音量(EP)をだんだん大きくする(="Cresc=") |
| デクレッシェンド | 小さく(音長),(最終値)//音量(EP)をだんだん小さくする(="Cresc=") |
| 音量戻す | 音量(EP)を最大値に戻す(="EP(127)") |
| 方向左 | ステレオの左から音が出るようにする(="P(0)") |
| 方向左前 | ステレオの左前から音が出るようにする(="P(32)") |
| 方向前 | ステレオの前から音が出るようにする(="P(64)") |
| 方向右前 | ステレオの右前から音が出るようにする(="P(96)") |
| 方向右 | ステレオの右から音が出るようにする(="P(127)") |
| 方向回す | ステレオの左右を回す(="P.onNoteWaveEx(0) |
| ビブラートオフ | ビブラートをやめる(="M(0)") |
| ペダル | ペダルを踏む(="y64) |
| 放す | ペダルを放す(="y64) |
| テンポ改 | テンポ改([[[t1],t2],len])//テンポを推移的に変更する。lenを省略すると、全音符の間に推移し、t1を省略すると、以前の値からt2へ推移する。(="TempoChange=") |
| ビブラート | 推移的なビブラートをかける(="M.onNoteWaveEx(0) |
| ここから演奏 | 途中から演奏したいときに書く(="PlayFrom(Time") |


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
| @ | 音色の指定 範囲:1-128 (書式) @(no),(Bank_LSB),(Bank_MSB) |
| > | 音階を1つ上げる |
| < | 音階を1つ下げる |
| ) | 音量を8つ上げる |
| ( | 音量を8つ下げる |
| // | 一行コメント |
| /* .. */ | 範囲コメント |
| ## | 一行コメント |
| # | 一行コメント |
| #- | 一行コメント |
| [ | ループ開始 (例 [4 cdeg]) |
| : | ループ最終回に脱出 (例　[4 cde:g]e) |
| ] | ループ終了 |
| ’ | 和音 (例 'ceg') 'ceg'(音長),(ゲート) |
| $ | リズムマクロ定義 $文字{定義内容} |
| { | 連符 (例 {ceg}4) {c^d}(音長) |
| ` | 一度だけ音階を+1する |
|  | 一度だけ音階を-1する |
| ? | ここから演奏する (=PLAY_FROM) |
| & | タイ・スラー(Slurコマンドで動作が変更できる) |


## 大文字コマンド

| コマンド | 説明    |
|---------|--------|
| End / END | それ移行をコンパイルしない |
| TR / TRACK / Track | トラック変更　TR=番号 範囲:0- |
| CH / Channel | チャンネル変更 CH=番号 範囲:1-16 |
| TIME / Time | タイム変更 TIME(節:拍:ステップ) |
| RHYTHM / Rhythm / R | リズムモード |
| RYTHM / Rythm | リズムモード(v1の綴りミス対処[^^;]) RHYTHM または R と同じ |
| DIV / Div | 連符 (例 DIV{ceg} ) |
| SUB / Sub / S | タイムポインタを戻す (例 SUB{ceg} egb) |
| KF / KeyFlag | 臨時記号を設定 - KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note) |
| KEY / Key / KeyShift | ノート(cdefgab)のキーをn半音シフトする (例 KEY=3 cde) |
| TR_KEY / TrackKey | トラック毎、ノート(cdefgab)のキーをn半音シフトする (例 TrackKey=3 cde) |
| INT / Int | 変数を定義 (例 INT TestValue=30) |
| STR / Str | 文字列変数を定義 (例 STR A={cde}) |
| PLAY / Play | 複数トラックを１度に書き込む (例 PLAY={aa},{bb},{cc}) |
| PRINT / Print | 文字を出力する (例 PRINT{"cde"} )(例 INT AA=30;PRINT(AA)) |
| PlayFrom.SysEx / PlayFrom.CtrlChg | 未実装 |
| PLAY_FROM / PlayFrom | ここから演奏する　(?と同じ意味) |
| System.MeasureShift | 小節番号をシフトする (例 System.MeasureShift(1)) |
| System.KeyFlag | 臨時記号を設定 - KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note) |
| System.TimeBase / TIMEBASE / Timebase / TimeBase | タイムベースを設定 (例 TIMEBASE=96) |
| TRACK_SYNC / TrackSync | 全てのトラックのタイムポインタを同期する |
| SLUR / Slur | タイ・スラー記号(&)の動作を変更する(0:グリッサンド/1:ベンド/2:ゲート/3:アルペジオ) |
| System.Include / Include / INCLUDE | 未実装 |
| System.vAdd / vAdd | ベロシティの相対変化(と)の変化値を指定する (例 System.vAdd(8)) |
| System.qAdd / qAdd | 未定義 |
| SoundType / SOUND_TYPE | 未実装 |
| VOICE / Voice | モジュレーション 範囲: 0-127 |
| M / Modulation | モジュレーション 範囲: 0-127 |
| PT / PortamentoTime | ポルタメント 範囲: 0-127 |
| V / MainVolume | メインボリューム 範囲: 0-127 |
| P / Panpot | パンポット 範囲: 0-63-127 |
| EP / Expression | エクスプレッション音量 範囲: 0-127 |
| PS / PortamentoSwitch | ポルタメントスイッチ |
| REV / Reverb | リバーブ 範囲: 0-127 |
| CHO / Chorus | コーラス 範囲: 0-127 |
| VAR / Variation | バリエーション 範囲: 0-127 |
| PB / PitchBend | ピッチベンドを指定 範囲: -8192~0~8191の範囲 |
| BR / PitchBendSensitivity | ピッチベンドの範囲を設定 範囲: 0-12半音 |
| RPN | RPNを書き込む (例 RPN=0,1,64) |
| NRPN | NRPNを書き込む (例 NRPN=1,0x64,10) |
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
| Fadein / FADEIN | 小節数を指定してフェードインする (例: Fadein(1)) |
| Fadeout / FADEOUT | 小節数を指定してフェードアウトする (例: Fadeout(1)) |
| Decresc / DECRESC | デクレッシェンドを表現 (書式) Decresc([[[len],v1],v2]) だんだん小さく。エクスプレッションをlen(ｎ分音符指定で)の間に、v1からv2へ変更する。lenを省略すると全音符の長さになる。 |
| Cresc / CRESC | クレッシェンドを表現 (書式) Cresc([[[len],v1],v2]) だんだん大きく。エクスプレッションをlen(ｎ分音符指定で)の間に、v1からv2へ変更する。lenを省略すると全音符の長さになる。 |
| ResetGM | GMリセットを送信 |
| ResetGS | GSリセットを送信 |
| ResetXG | XGリセットを送信 |
| TEMPO / Tempo / T | テンポの指定 |
| TempoChange | テンポを連続で変更する (書式) TempoChange(開始値,終了値, !長さ) |
| TimeSignature / TimeSig / TIMESIG / System.TimeSignature | 拍子の指定 |
| MetaText / TEXT / Text | メタテキスト (例 TEXT{"abcd"}) |
| COPYRIGHT / Copyright | メタテキスト著作権 (例 COPYRIGHT{"aaa"}) |
| TRACK_NAME / TrackName | 曲名 (例 TRACK_NAME{"aaa"}) |
| InstrumentName | 楽器名 (例 InstrumentName{"aaa"}) |
| LYRIC / Lyric | メタテキスト歌詞 (例 LYRIC{"aaa"}) |
| MAKER / Marker | マーカー (例 MAKER{"aaa"}) |
| CuePoint | キューポイント (例 CuePoint{"aaa"}) |
| IF / If | IF文 (書式) IF(条件){ … }ELSE{ … } |
| FOR / For | FOR文 (書式) FOR(初期化式; 条件; 増加式){ … } |
| WHILE / While | WHILE文 (書式) WHILE(条件){ … } |
| EXIT / Exit / BREAK / Break | BREAK文 FOR/WHILEを抜ける |
| CONTINUE / Continue | CONTINUE文 FOR/WHILEを続ける |
| RETURN / Return | RETURN(戻り値) 関数を抜ける |
| RandomSeed / RANDOM_SEED | 乱数の種を設定する (例 RandomSeed=1234) |
| FUNCTION / Function | 関数を定義する (未実装) |


## 計算式で参照できる値

| コマンド | 説明    |
|---------|--------|
| TR /  TRACK /  Track  |現在のトラック番号を得る|
| CH /  CHANNEL  |現在のチャンネル番号を得る|
| TIME /  Time  |現在のタイムポインタ値を得る|



## マクロや音色など変数定義

| 変数名 | 値    |
|---------|--------|
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


## リズムマクロ

| 変数名 | 値    |
|---------|--------|
| b | "n36," |
| s | "n38," |
| h | "n42," |
| H | "n44," |
| o | "n46," |
| c | "n49," |
| _ | "r" |