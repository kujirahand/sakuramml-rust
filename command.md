# テキスト音楽 サクラ(Rust版) コマンド一覧

## ストトン表記

| コマンド | 説明    |
|---------|--------|
| 全音符 | "全音符"=> // @ 全音符を基本音符にする(="l1") |
| 二分音符 | "二分音符"=> // @ 二分音符を基本音符にする(="l2") |
| 四分音符 | "四分音符"=> // @ 四分音符を基本音符にする(="l4") |
| 八分音符 | "八分音符"=> // @ 八分音符を基本音符にする(="l8") |
| 十六音符 | "十六音符"=> // @ 十六音符を基本音符にする(="l16") |
| 付点四分音符 | "付点四分音符"=> // @ 付点四分音符を基本音符にする(="l4.") |
| 音源初期化 | "音源初期化"=> // @ 音源初期化//音源の初期化(GMリセット)を実行する。（例）音源初期化(="System.MeasureShift(1ResetGM;Time(1:1:0TrackSync;") |
| 音長 | "音長"=> // @ 基本音符を指定(="l") |
| 音量予約 | "音量予約"=> // @ 音量を予約指定する(="v.onTime=") |
| 「 | "「"=> // @ 和音はじめ(="'") |
| 」 | "」"=> // @ 和音おわり(="'") |
| 【 | "【"=> // @ 繰り返しはじめ(="[") |
| 】 | "】"=> // @ 繰り返しおわり(="]") |
| ↑ | "↑"=> // @ オクターブを1つ上げる(=">") |
| ↓ | "↓"=> // @ オクターブを1つ下げる(="<") |
| ♭ | "♭"=> // @ フラット(="-") |
| ♯ | "♯"=> // @ シャープ(="#") |
| − | "−"=> // @ マイナス(="-") |
| ‘ | "‘"=> // @ 次の音符をオクターブ1つ上げる(="`") |
| 調 | "調"=> // @ 調#(音符)//臨時記号を設定する。（例）調＃（ドファ）(="System.KeyFlag") |
| 音階 | "音階"=> // @ 音階(数値)//音階を数値で指定する。初期値は５。範囲は、0～10（例）音階５(="o") |
| 時間 | "時間"=> // @ 時間(小節数:拍数:ステップ数)//指定時間にポインタを移動する。範囲は、小節数・拍数が、１～。ステップ数は、０～。（例）時間（４：１：０）(="Time") |
| 読む | "読む"=> // @ 読む(ファイル名)//外部定義ファイルを読み込む。（例）読む(chord2.h)(="Include") |
| 予約 | "予約"=> // @ (コマンド)予約(v1,v2,v3...)//コマンドの値を予約しておく（例）音量予約120,50【ドレミファ】(=".onNote=") |
| 拍子 | "拍子"=> // @ 拍子 分子,分母//拍子を設定する。（例）拍子4,4(="System.TimeSignature=") |
| 音色 | "音色"=> // @ 音色（番号）//音色を設定する。(="@") |
| 音符 | "音符"=> // @ 音符（ｎ分音符指定）//基本となる音符の長さをｎ分音符で指定する。（例）音符16//１６分音符の意味(="l") |
| 音量 | "音量"=> // @ 音量（数値）//音量(実際は音の強さ)を設定する。初期値は、100。範囲は、0~127。（例）音量127(="v") |
| 連符 | "連符"=> // @ 連符{音名}[音長]//３連符や５連符などを表現する。（例）連符{ドレミ}4(="Div") |
| ゲート | "ゲート"=> // @ ゲート（割合）//音符の長さに対する実際の発音時間を割合（100分率）で指定する。範囲は、1～100～。（例）ゲート80(="q") |
| テンポ | "テンポ"=> // @ テンポ（数値）//テンポを設定する。初期値は、120。範囲は、20～240を推奨。（例）テンポ120(="Tempo=") |
| 曖昧さ | "曖昧さ"=> // @ (コマンド)曖昧さ（数値）//各属性の曖昧さを設定する。範囲は、0～。初期値は、0。（例）音量曖昧さ80 【ドレミソ】(=".Random=") |
| トラック | "トラック"=> // @ トラック（番号）//トラック番号を指定する。初期値は、０。範囲は、0～。（例）トラック３(="Track=") |
| チャンネル | "チャンネル"=> // @ チャンネル（番号）//現在のトラックにチャンネルを設定する。初期値は、トラック番号と同じ。範囲は、１～１６（例）トラック３チャンネル１０(="Channel=") |
| 曲名 | "曲名"=> // @ 曲名{"文字列"}//生成するMIDIファイルに曲名を埋め込む。（例）曲名{"テスト"}(="TrackName=") |
| 作者 | "作者"=> // @ 作者{"文字列"}//生成するMIDIファイルに著作権情報を埋め込む。（例）作者{"クジラ飛行机"}(="Copyright=") |
| コメント | "コメント"=> // @ コメント{"文字列"}//生成するMIDIファイルにコメントを埋め込む。（例）コメント{"テスト"}(="MetaText=") |
| 演奏位置 | "演奏位置"=> // @ 演奏位置(小節数:拍数:ステップ数))//長い曲の途中から演奏したい時、曲の演奏位置を指定する。（例）演奏位置（32:1:0）(="PlayFrom") |
| ー | "ー"=> // @ ー//タイ。音を伸ばす。（例）ドードレミミソーーー(="^") |
| 上 | "上"=> // @ 音階を相対的に１つ上げる(=">") |
| 下 | "下"=> // @ 音階を相対的に１つ下げる(="<") |
| ド | "ド"=> // @ 音名(="c") |
| レ | "レ"=> // @ 音名(="d") |
| ミ | "ミ"=> // @ 音名(="e") |
| フ | "フ"=> // @ 音名(="f") |
| ァ | "ァ"=> // @ 音名(="") |
| ソ | "ソ"=> // @ 音名(="g") |
| ラ | "ラ"=> // @ 音名(="a") |
| シ | "シ"=> // @ 音名(="b") |
| ン | "ン"=> // @ 休符。（例）ドーーン　レンレー(="r") |
| ッ | "ッ"=> // @ 休符。（例）ドーーッ　レッレー(="r") |
| ど | "ど"=> // @ 音名(="c") |
| れ | "れ"=> // @ 音名(="d") |
| み | "み"=> // @ 音名(="e") |
| ふ | "ふ"=> // @ 音名(="f") |
| ぁ | "ぁ"=> // @ 音名(="") |
| そ | "そ"=> // @ 音名(="g") |
| ら | "ら"=> // @ 音名(="a") |
| し | "し"=> // @ 音名(="b") |
| ん | "ん"=> // @ 休符(="r") |
| っ | "っ"=> // @ 休符(="r") |
| イ | "イ"=> // @ 音名(="a") |
| ロ | "ロ"=> // @ 音名(="b") |
| ハ | "ハ"=> // @ 音名(="c") |
| ニ | "ニ"=> // @ 音名(="d") |
| ホ | "ホ"=> // @ 音名(="e") |
| ヘ | "ヘ"=> // @ 音名(="f") |
| ト | "ト"=> // @ 音名(="g") |
| 変 | "変"=> // @ フラット（例）イ変(="-") |
| 嬰 | "嬰"=> // @ シャープ(="+") |
| リズム | "リズム"=> // @ リズムモード(="Rythm") |
| ず | "ず"=> // @ バスドラム(="n36) |
| た | "た"=> // @ スネアドラム(="n38) |
| つ | "つ"=> // @ ハイハット（クローズ）(="n42) |
| ち | "ち"=> // @ ハイハット（オープン）(="n46) |
| ぱ | "ぱ"=> // @ シンバル(="n49) |
| と | "と"=> // @ Lowタム(="n50) |
| む | "む"=> // @ Midタム(="n47) |
| ろ | "ろ"=> // @ Highタム(="n43) |
| く | "く"=> // @ ドラム(="n44) |
| 大きく | "大きく"=> // @ 大きく(音長),(最終値)//音量(EP)をだんだん大きくする(="Cresc=") |
| 小さく | "小さく"=> // @ 小さく(音長),(最終値)//音量(EP)をだんだん小さくする(="Decresc=") |
| クレッシェンド | "クレッシェンド"=> // @ 大きく(音長),(最終値)//音量(EP)をだんだん大きくする(="Cresc=") |
| デクレッシェンド | "デクレッシェンド"=> // @ 小さく(音長),(最終値)//音量(EP)をだんだん小さくする(="Cresc=") |
| 音量戻す | "音量戻す"=> // @ 音量(EP)を最大値に戻す(="EP(127)") |
| 方向左 | "方向左"=> // @ ステレオの左から音が出るようにする(="P(0)") |
| 方向左前 | "方向左前"=> // @ ステレオの左前から音が出るようにする(="P(32)") |
| 方向前 | "方向前"=> // @ ステレオの前から音が出るようにする(="P(64)") |
| 方向右前 | "方向右前"=> // @ ステレオの右前から音が出るようにする(="P(96)") |
| 方向右 | "方向右"=> // @ ステレオの右から音が出るようにする(="P(127)") |
| 方向回す | "方向回す"=> // @ ステレオの左右を回す(="P.onNoteWaveEx(0) |
| ビブラートオフ | "ビブラートオフ"=> // @ ビブラートをやめる(="M(0)") |
| ペダル | "ペダル"=> // @ ペダルを踏む(="y64) |
| 放す | "放す"=> // @ ペダルを放す(="y64) |
| テンポ改 | "テンポ改"=> // @ テンポ改([[[t1],t2],len])//テンポを推移的に変更する。lenを省略すると、全音符の間に推移し、t1を省略すると、以前の値からt2へ推移する。(="TempoChange=") |
| ビブラート | "ビブラート"=> // @ 推移的なビブラートをかける(="M.onNoteWaveEx(0) |
| ここから演奏 | "ここから演奏"=> // @ 途中から演奏したいときに書く(="PlayFrom(Time") |


## 1文字コマンド

| コマンド | 説明    |
|---------|--------|
| / \t / \r / ｜ / ; | ' ' | '\t' | '\r' | '|' | ';' => {}, // @ 空白文字 |
| c / d / e / f / g / a / b | 'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b' => result.push(read_note(&mut cur, ch)), // @ ドレミファソラシ c(音長),(ゲート),(音量),(タイミング),(音階) |
| n | 'n' => result.push(read_note_n(&mut cur, song)), // @ 番号を指定して発音(例: n36) n(番号),(音長),(ゲート),(音量),(タイミング) |
| r | 'r' => result.push(read_rest(&mut cur)),         // @ 休符 |
| l | 'l' => result.push(read_length(&mut cur, song)),       // @ 音長の指定(例 l4) |
| o | 'o' => result.push(read_octave(&mut cur, song)), // @ 音階の指定(例 o5) 範囲:0-10 |
| p | 'p' => result.push(read_pitch_bend_small(&mut cur, song)), // @ ピッチベンドの指定 範囲:0-127 (63が中央) |
| q | 'q' => result.push(read_qlen(&mut cur, song)), // @ ゲートの指定 (例 q90) 範囲:0-100 |
| v | 'v' => result.push(read_velocity(&mut cur, song)), // @ ベロシティ音量の指定 範囲:0-127 / v.Random=n |
| t | 't' => result.push(read_timing(&mut cur, song)), // @ 発音タイミングの指定 (例 t-1) / t.Random=n |
| y | 'y' => result.push(read_cc(&mut cur, song)), // @ コントロールチェンジの指定 (例 y1,100) 範囲:0-127 / y1.onTime(low,high,len) |
| @ | '@' => result.push(read_voice(&mut cur, song)), // @ 音色の指定 範囲:1-128 (書式) @(no),(Bank_LSB),(Bank_MSB) |
| > | '>' => result.push(Token::new_value(TokenType::OctaveRel, 1)), // @ 音階を1つ上げる |
| < | '<' => result.push(Token::new_value(TokenType::OctaveRel, -1)), // @ 音階を1つ下げる |
| ) | ')' => result.push(Token::new_value(TokenType::VelocityRel, song.v_add)), // @ 音量をvAddの値だけ上げる |
| ( | '(' => result.push(Token::new_value(TokenType::VelocityRel, -1 * song.v_add)), // @ 音量をvAddの値だけ下げる |
| // | "//" => // @ 一行コメント |
| /* .. */ | "/*" .. "*/" => // @ 範囲コメント |
| ## | "##" => // @ 一行コメント |
| # | "# " => // @ 一行コメント |
| #- | "#-" => // @ 一行コメント |
| [ | '[' => result.push(read_loop(&mut cur, song)), // @ ループ開始 (例 [4 cdeg]) |
| : | ':' => result.push(Token::new_value(TokenType::LoopBreak, 0)), // @ ループ最終回に脱出 (例　[4 cde:g]e) |
| ] | ']' => result.push(Token::new_value(TokenType::LoopEnd, 0)),   // @ ループ終了 |
| ’ | '\'' => result.push(read_harmony_flag(&mut cur, &mut flag_harmony)), // @ 和音 (例 'ceg') 'ceg'(音長),(ゲート) |
| $ | '$' => read_def_rhythm_macro(&mut cur, song), // @ リズムマクロ定義 $文字{定義内容} |
| { | '{' => result.push(read_command_div(&mut cur, song, true)), // @ 連符 (例 {ceg}4) {c^d}(音長) |
| ` | '`' => result.push(Token::new_value(TokenType::OctaveOnce, 1)), // @ 一度だけ音階を+1する |
|  | '"' => result.push(Token::new_value(TokenType::OctaveOnce, -1)), // @ 一度だけ音階を-1する |
| ? | '?' => result.push(Token::new_value(TokenType::PlayFromHere, 0)), // @ ここから演奏する (=PlayFromHere) |
| & | '&' => {}, // @ タイ・スラー(Slurコマンドで動作が変更できる) |


## 大文字コマンド

| コマンド | 説明    |
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
| Rythm | 互換性:綴りミス read Rhythm notes (ex) Rhythm{ bhsh bhsh } |
| RTTHM | 互換性:綴りミス read Rhythm notes (ex) Rhythm{ bhsh bhsh } |
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
| PB | Pitchbend range: -8192...0...8191 (ex) PB(10) |
| RPN | write RPN (ex) RPN(0,1,64) |
| NRPN | write NRPN (ex) NRPN(1,1,1) |
| BR | PitchBendSensitivity (ex) BR(10)  |
| PitchBendSensitivity | PitchBendSensitivity (ex) BR(10) |
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
| RANDOM_SEED | set random seed |
| RandomSeed | set random seed |
| FUNCTION | define user function |
| Function | define user function |


## 計算式で参照できる値

| コマンド | 説明    |
|---------|--------|
| TR /  TRACK /  Track  |現在のトラック番号を得る|
| CH /  CHANNEL  |現在のチャンネル番号を得る|
| TIME /  Time  |現在のタイムポインタ値を得る|



## マクロや音色など変数定義

- [🔗 Voice List and Macro](voice.md)

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