//! mml_def: システム関数の定義
use crate::sakura_functions;
use crate::token::TokenType;
use std::collections::HashMap;

macro_rules! sysfunc_add {
    ($obj:expr, $name:expr, $func_id:expr, $arg_type:expr) => {
        $obj.insert(
            String::from($name),
            SystemFunction {
                token_type: $func_id,
                arg_type: $arg_type,
                tag1: 0,
                tag2: 0,
            },
        );
    };
}

macro_rules! sysfunc_cc_add {
    ($obj:expr, $name:expr, $func_id:expr, $arg_type:expr, $tag:expr) => {
        $obj.insert(
            String::from($name),
            SystemFunction {
                token_type: $func_id,
                arg_type: $arg_type,
                tag1: $tag,
                tag2: 0,
            },
        );
    };
}

macro_rules! sysfunc_rpn_add {
    ($obj:expr, $name:expr, $func_id:expr, $arg_type:expr, $tag1:expr, $tag2:expr) => {
        $obj.insert(
            String::from($name),
            SystemFunction {
                token_type: $func_id,
                arg_type: $arg_type,
                tag1: $tag1,
                tag2: $tag2,
            },
        );
    };
}

#[derive(Debug, Clone, Copy)]
pub struct SystemFunction {
    pub token_type: TokenType,
    pub arg_type: char, // 'I' or 'S' or 'A' or '*'(special)
    pub tag1: isize,
    pub tag2: isize,
}

pub fn init_system_functions() -> HashMap<String, SystemFunction> {
    let mut sf = HashMap::new();
    //<SYSTEM_FUNCTION>
    //@ Basic command
    // sysfunc_add!(sf, "End", TokenType::End, '_'); // end of song
    // sysfunc_add!(sf, "END", TokenType::End, '_'); // end of song
    sysfunc_add!(sf, "Track", TokenType::Track, 'I'); // change current track [range:0 to 999] (ex) Track(1)
    sysfunc_add!(sf, "TRACK", TokenType::Track, 'I'); // change current track [range:0 to 999] (ex) TRACK(1)
    sysfunc_add!(sf, "TR", TokenType::Track, 'I'); // change current track [range:0 to 999] (ex) TR(1)
    sysfunc_add!(sf, "Channel", TokenType::Channel, 'I'); // change channel no [range:1 to 16] (ex) Channel(1)
    sysfunc_add!(sf, "CHANNEL", TokenType::Channel, 'I'); // change channel no [range:1 to 16] (ex) CHANNEL(1)
    sysfunc_add!(sf, "CH", TokenType::Channel, 'I'); // change channel no [range:1 to 16] (ex) CH(1)
    sysfunc_add!(sf, "Time", TokenType::Time, 'A'); // change time position, Time(measure:beat:step) (ex) Time(1:1:0) Time(0)
    sysfunc_add!(sf, "TIME", TokenType::Time, 'A'); // change time position, TIME(measure:beat:step) (ex) Time(1:1:0) Time(0)
    sysfunc_add!(sf, "System.TimeBase", TokenType::TimeBase, '*'); // set system time base (ex) TimeBase(96)
    sysfunc_add!(sf, "Timebase", TokenType::TimeBase, '*'); // set system time base (ex) TimeBase(96)
    sysfunc_add!(sf, "TimeBase", TokenType::TimeBase, '*'); // set system time base (ex) TimeBase(96)
    sysfunc_add!(sf, "TIMEBASE", TokenType::TimeBase, '*'); // set system time base (ex) TimeBase(96)
    sysfunc_add!(sf, "Rhythm", TokenType::Rhythm, '*'); // read Rhythm notes (ex) Rhythm{ bhsh bhsh }
    sysfunc_add!(sf, "RHYTHM", TokenType::Rhythm, '*'); // read Rhythm notes (ex) Rhythm{ bhsh bhsh }
    sysfunc_add!(sf, "R", TokenType::Rhythm, '*'); // read Rhythm notes (ex) Rhythm{ bhsh bhsh }
    sysfunc_add!(sf, "Rythm", TokenType::Rhythm, '*'); // 互換性:綴りミス [typo] read Rhythm notes (ex) Rhythm{ bhsh bhsh }
    sysfunc_add!(sf, "RYTHM", TokenType::Rhythm, '*'); // 互換性:綴りミス [typo] read Rhythm notes (ex) Rhythm{ bhsh bhsh }
    sysfunc_add!(sf, "Div", TokenType::Div, '*'); // tuplet(連符) (ex) Div{ ceg }
    sysfunc_add!(sf, "DIV", TokenType::Div, '*'); // tuplet(連符) (ex) Div{ ceg }
    sysfunc_add!(sf, "Sub", TokenType::Sub, '*'); // sub track / rewind time position (ex) Sub{ceg} egb
    sysfunc_add!(sf, "SUB", TokenType::Sub, '*'); // sub track / rewind time position (ex) Sub{ceg} egb
    sysfunc_add!(sf, "S", TokenType::Sub, '*'); // sub track / rewind time position (ex) Sub{ceg} egb
    sysfunc_add!(sf, "System.KeyFlag", TokenType::KeyFlag, '*'); // set key flag to note / 音名は区切らず並べる (ex) KeyFlag+(cf) / 数値指定は a,b,c,d,e,f,g の順 (ex) KeyFlag=(0,0,1,0,0,1,0)
    sysfunc_add!(sf, "KeyFlag", TokenType::KeyFlag, '*'); // set key flag to note / 音名は区切らず並べる (ex) KeyFlag+(cf) / 数値指定は a,b,c,d,e,f,g の順 (ex) KeyFlag=(0,0,1,0,0,1,0)
    sysfunc_add!(sf, "KF", TokenType::KeyFlag, '*'); // set key flag to note / 音名は区切らず並べる (ex) KeyFlag+(cf) / 数値指定は a,b,c,d,e,f,g の順 (ex) KeyFlag=(0,0,1,0,0,1,0)
    sysfunc_add!(sf, "KeyShift", TokenType::KeyShift, 'I'); // set key-shift (ex) KeyShift(3)
    sysfunc_add!(sf, "Key", TokenType::KeyShift, 'I'); // set key-shift (ex) Key(3)
    sysfunc_add!(sf, "KEY", TokenType::KeyShift, 'I'); // set key-shift (ex) KEY(3)
    sysfunc_add!(sf, "UseKeyShift", TokenType::UseKeyShift, '*'); // set key shift mode value=on|off (ex) UseKeyShift(on)
    sysfunc_add!(sf, "TrackKey", TokenType::TrackKey, 'I'); // set key-shift for track (ex) TrackKey(3)
    sysfunc_add!(sf, "TR_KEY", TokenType::TrackKey, 'I'); // set key-shift for track (ex) TR_KEY(3)
    sysfunc_add!(sf, "Play", TokenType::Play, '*'); // play multi track (ex) Play(AA,BB,CC)
    sysfunc_add!(sf, "PLAY", TokenType::Play, '*'); // play multi track (ex) Play(AA,BB,CC)
    sysfunc_add!(sf, "SysEx", TokenType::SysEx, '*'); // System Exclusive (ex) SysEx$=f0,43,10,4c,00,{00,00,30,f0},f7
    sysfunc_add!(sf, "PlayFrom.SysEx", TokenType::SysEx, '*'); // =SysEx
    sysfunc_add!(sf, "PlayFrom.CtrlChg", TokenType::ControlChange, 'A'); // =CONTROL_CHANGE
    sysfunc_add!(sf, "PlayFrom", TokenType::PlayFrom, 'A'); // play from time position (ex) PlayFrom(5:1:0)
    sysfunc_add!(sf, "PLAY_FROM", TokenType::PlayFrom, 'A'); // play from time position (ex) PLAY_FROM(5:1:0)
    sysfunc_add!(sf, "PlayFromHere", TokenType::PlayFromHere, '_'); // play from current time pos (ex) PlayFromHere
    sysfunc_add!(sf, "PLAY_FROM_HRER", TokenType::PlayFromHere, '_'); // play from current time pos / 綴りミスだが互換性のため維持 (ex) PLAY_FROM_HRER
    sysfunc_add!(sf, "System.MeasureShift", TokenType::MeasureShift, 'I'); // set measure shift for time pointer (ex) System.MeasureShift(1)
    sysfunc_add!(sf, "MeasureShift", TokenType::MeasureShift, 'I'); // set measure shift for time pointer (ex) MeasureShift(1)
    sysfunc_add!(sf, "MEASURE_SHIFT", TokenType::MeasureShift, 'I'); // set measure shift for time pointer (ex) MeasureShift(1)
    sysfunc_add!(sf, "TrackSync", TokenType::TrackSync, '_'); // synchronize time pointers for all tracks (ex) TrackSync
    sysfunc_add!(sf, "TRACK_SYNC", TokenType::TrackSync, '_'); // synchronize time pointers for all tracks (ex) TrackSync
    sysfunc_add!(sf, "Slur", TokenType::TieMode, 'A'); // set slur/tie(&) mode (0:グリッサンド/1:ベンド/2:ゲート/3:アルペジオ) (ex) Slur(1)
    sysfunc_add!(sf, "SLUR", TokenType::TieMode, 'A'); // set slur/tie(&) mode (0:グリッサンド/1:ベンド/2:ゲート/3:アルペジオ) (ex) Slur(1)
    sysfunc_add!(sf, "System.vAdd", TokenType::SongVelocityAdd, 'I'); // set relative velocity '(' or ')' or 'v++' or 'v--' command increment value / 小文字始まりの vAdd は v コマンドと解釈されるため System.vAdd と書く (ex) System.vAdd(3)
    sysfunc_add!(sf, "vAdd", TokenType::SongVelocityAdd, 'I'); // set relative velocity '(' or ')' or 'v++' or 'v--' command increment value / 小文字始まりの vAdd は v コマンドと解釈されるため System.vAdd と書く (ex) System.vAdd(3)
    sysfunc_add!(sf, "System.qAdd", TokenType::SongQAdd, 'I'); // set q++ command value / 小文字始まりの qAdd は q コマンドと解釈されるため System.qAdd と書く (ex) System.qAdd(3)
    sysfunc_add!(sf, "qAdd", TokenType::SongQAdd, 'I'); // set q++ command value / 小文字始まりの qAdd は q コマンドと解釈されるため System.qAdd と書く (ex) System.qAdd(3)
    sysfunc_add!(sf, "System.q2Add", TokenType::Unimplemented, 'I'); // Unimplemented
    sysfunc_add!(sf, "q2Add", TokenType::Unimplemented, 'I'); // Unimplemented
    sysfunc_add!(sf, "SoundType", TokenType::SoundType, 'S'); // set sound type (ex) SoundType({pico})
    sysfunc_add!(sf, "DeviceNumber", TokenType::DeviceNumber, 'I'); // set Device Number (ex) DeviceNumber=$10
                                                                    //@ Controll Change / Voice Change / RPN/NRPN / PitchBend
    sysfunc_add!(sf, "Voice", TokenType::Voice, 'A'); // set voice (=@) range: 1-128 Voice(n[,msb,lsb]) (ex) Voice(1)
    sysfunc_add!(sf, "VOICE", TokenType::Voice, 'A'); // set voice (=@) range: 1-128 Voice(n[,msb,lsb]) (ex) Voice(1)
    sysfunc_add!(sf, "CONTROL_CHANGE", TokenType::ControlChange, '*'); // write Control Change (ex) CC(1,100)
    sysfunc_add!(sf, "ControlChange", TokenType::ControlChange, '*'); // write Control Change (ex) CC(1,100)
    sysfunc_add!(sf, "CC", TokenType::ControlChange, '*'); // write Control Change (ex) CC(1,100)
    sysfunc_cc_add!(sf, "M", TokenType::ControlChangeCommand, '*', 1); // CC#1 Modulation (ex) M(10)
    sysfunc_cc_add!(sf, "Modulation", TokenType::ControlChangeCommand, '*', 1); // CC#1 Modulation range:0-127 (ex) M(10)
    sysfunc_cc_add!(sf, "PT", TokenType::ControlChangeCommand, '*', 5); // CC#5 Portamento Time range:0-127 (ex) PT(10)
    sysfunc_cc_add!(
        sf,
        "PortamentoTime",
        TokenType::ControlChangeCommand,
        '*',
        5
    ); // CC#5 Portamento Time range:0-127 (ex) PT(10)
    sysfunc_cc_add!(sf, "V", TokenType::ControlChangeCommand, '*', 7); // CC#7 Main Volume range:0-127 (ex) V(10)
    sysfunc_cc_add!(sf, "MainVolume", TokenType::ControlChangeCommand, '*', 7); // CC#7 Main Volume range:0-127 (ex) V(10)
    sysfunc_cc_add!(sf, "P", TokenType::ControlChangeCommand, '*', 10); // CC#10 Panpot range:0-64-127 (ex) P(64)
    sysfunc_cc_add!(sf, "Panpot", TokenType::ControlChangeCommand, '*', 10); // CC#10 Panpot range:0-64-127 (ex) Panpot(64)
    sysfunc_cc_add!(sf, "EP", TokenType::ControlChangeCommand, '*', 11); // CC#11 Expression range:0-127 (ex) EP(100)
    sysfunc_cc_add!(sf, "Expression", TokenType::ControlChangeCommand, '*', 11); // CC#11 Expression range:0-127 (ex) EP(100)
    sysfunc_cc_add!(sf, "PS", TokenType::ControlChangeCommand, '*', 65); // CC#65 Portament switch range:0-127 (ex) PS(1)
    sysfunc_cc_add!(
        sf,
        "PortamentoSwitch",
        TokenType::ControlChangeCommand,
        '*',
        65
    ); // CC#65 Portament switch range:0-127 (ex) PS(1)
    sysfunc_cc_add!(sf, "REV", TokenType::ControlChangeCommand, '*', 91); // CC#91 Reverb range:0-127 (ex) REV(100)
    sysfunc_cc_add!(sf, "Reverb", TokenType::ControlChangeCommand, '*', 91); // CC#91 Reverb range:0-127 (ex) REV(100)
    sysfunc_cc_add!(sf, "CHO", TokenType::ControlChangeCommand, '*', 93); // CC#93 Chorus range:0-127 (ex) CHO(100)
    sysfunc_cc_add!(sf, "Chorus", TokenType::ControlChangeCommand, '*', 93); // CC#93 Chorus range:0-127 (ex) Chorus(100)
    sysfunc_cc_add!(sf, "VAR", TokenType::ControlChangeCommand, '*', 94); // CC#94 Variation range:0-127 (ex) VAR(100)
    sysfunc_cc_add!(sf, "Variation", TokenType::ControlChangeCommand, '*', 94); // CC#94 Variation range:0-127 (ex) Variation(100)
    sysfunc_add!(sf, "PitchBend", TokenType::PitchBend, '*'); // Pitchbend range: -8192~0~8191 (ex) PitchBend(10) / p(value) range: 0~64~127 / PitchBend.onTime(low,high,len) / PitchBend.onNoteWave(low,high,len)
    sysfunc_add!(sf, "PB", TokenType::PitchBend, '*'); // Pitchbend range: -8192~0~8191 (ex) PB(10) / PB.onTime(low,high,len) / PB.onNoteWave(low,high,len)
    sysfunc_rpn_add!(sf, "PitchBendSensitivity", TokenType::RPNCommand, '*', 0, 0); // PitchBendSensitivity (ex) BR(10)
    sysfunc_rpn_add!(sf, "BEND_RANGE", TokenType::RPNCommand, '*', 0, 0); // PitchBendSensitivity (ex) BEND_RANGE(10)
    sysfunc_rpn_add!(sf, "BendRange", TokenType::RPNCommand, '*', 0, 0); // PitchBendSensitivity (ex) BendRange(10)
    sysfunc_rpn_add!(sf, "BR", TokenType::RPNCommand, '*', 0, 0); // PitchBendSensitivity (ex) BR(10)
    sysfunc_add!(sf, "RPN", TokenType::RPN, 'A'); // write RPN (ex) RPN(0,1,64)
    sysfunc_add!(sf, "NRPN", TokenType::NRPN, 'A'); // write NRPN (ex) NRPN(1,1,1)
    sysfunc_rpn_add!(sf, "FineTune", TokenType::RPNCommand, '*', 0, 1); // set fine tune range:0-64-127(-100 - 0 - +99.99セント）(ex) FineTune(64)
    sysfunc_rpn_add!(sf, "CoarseTune", TokenType::RPNCommand, '*', 0, 2); // set coarse tune 半音単位のチューニング 範囲:40-64-88 (-24 - 0 - 24半音) (ex) CoarseTune(64)
    sysfunc_rpn_add!(sf, "VibratoRate", TokenType::NRPNCommand, '*', 1, 8); // set VibratoRate range: 0-127
    sysfunc_rpn_add!(sf, "VibratoDepth", TokenType::NRPNCommand, '*', 1, 9); // set VibratoDepth range: 0-127
    sysfunc_rpn_add!(sf, "VibratoDelay", TokenType::NRPNCommand, '*', 1, 10); // set VibratoDelay range: 0-127
    sysfunc_rpn_add!(sf, "FilterCutoff", TokenType::NRPNCommand, '*', 1, 0x20); // set FilterCutoff range: 0-127
    sysfunc_rpn_add!(sf, "FilterResonance", TokenType::NRPNCommand, '*', 1, 0x21); // set FilterResonance range: 0-127
    sysfunc_rpn_add!(sf, "EGAttack", TokenType::NRPNCommand, '*', 1, 0x63); // set EGAttack range: 0-127
    sysfunc_rpn_add!(sf, "EGDecay", TokenType::NRPNCommand, '*', 1, 0x64); // set EGDecay range: 0-127
    sysfunc_rpn_add!(sf, "EGRelease", TokenType::NRPNCommand, '*', 1, 0x66); // set EGRelease range: 0-127
                                                                             //@ fadein
    sysfunc_cc_add!(sf, "Fadein", TokenType::FadeIO, '*', 1); // fadein 小節数を指定 (ex) Fadein(1)
    sysfunc_cc_add!(sf, "Fadeout", TokenType::FadeIO, '*', -1); // fadeout 小節数を指定 (ex) Fadeout(1)
    sysfunc_cc_add!(sf, "Cresc", TokenType::Cresc, '*', 1); // だんだん大きくする Cresc=[len][,v1][,v2] v1からv2へ変更する。lenを省略すると全音符の長さに。カッコは使えない (ex) Cresc=1,40,127
    sysfunc_cc_add!(sf, "Decresc", TokenType::Cresc, '*', -1); // だんだん小さくする Decresc=[len][,v1][,v2] v1からv2へ変更する。lenを省略すると全音符の長さに。カッコは使えない (ex) Decresc=1,127,40
    sysfunc_cc_add!(sf, "CRESC", TokenType::Cresc, '*', 1); // だんだん大きくする Cresc=[len][,v1][,v2] v1からv2へ変更する。lenを省略すると全音符の長さに。カッコは使えない (ex) Cresc=1,40,127
    sysfunc_cc_add!(sf, "DECRESC", TokenType::Cresc, '*', -1); // だんだん小さくする Decresc=[len][,v1][,v2] v1からv2へ変更する。lenを省略すると全音符の長さに。カッコは使えない (ex) Decresc=1,127,40
                                                               //@ SysEx / Meta
    sysfunc_cc_add!(sf, "ResetGM", TokenType::SysexReset, 'I', 0); // ResetGM
    sysfunc_cc_add!(sf, "ResetGS", TokenType::SysexReset, 'I', 1); // ResetGS
    sysfunc_cc_add!(sf, "ResetXG", TokenType::SysexReset, 'I', 2); // ResetXG
    sysfunc_cc_add!(sf, "MasterVolume", TokenType::SysExCommand, 'I', 1); // master volume (range: 0-127) (ex) MasterVolume(100)
    sysfunc_cc_add!(sf, "MasterBalance", TokenType::SysExCommand, 'I', 2); // master balance (range: -8192 to 8191) (ex) MasterBalance(0)
    sysfunc_add!(sf, "Tempo", TokenType::Tempo, 'I'); // set tempo (ex) Tempo(120)
    sysfunc_add!(sf, "TEMPO", TokenType::Tempo, 'I'); // set tempo (ex) TEMPO(120)
    sysfunc_add!(sf, "T", TokenType::Tempo, 'I'); // set tempo (ex) T(120)
    sysfunc_add!(sf, "BPM", TokenType::Tempo, 'I'); // set tempo (ex) BPM(120)
    sysfunc_add!(sf, "TempoChange", TokenType::TempoChange, 'A'); // tempo change slowly TempoChange(start, end, len) / lenはステップ数で指定する (ex) TempoChange(80,120,384)
    sysfunc_add!(sf, "TimeSignature", TokenType::TimeSignature, 'A'); // set time signature (ex) TimeSignature(4, 4)
    sysfunc_add!(sf, "System.TimeSignature", TokenType::TimeSignature, 'A'); // set time signature (ex) TimeSignature(4, 4)
    sysfunc_add!(sf, "TimeSig", TokenType::TimeSignature, 'A'); // set time signature (ex) TimeSignature(4, 4)
    sysfunc_add!(sf, "TIMESIG", TokenType::TimeSignature, 'A'); // set time signature (ex) TimeSignature(4, 4)
    sysfunc_add!(sf, "Port", TokenType::Port, 'I'); // set Port No (ex) Port(0)
    sysfunc_add!(sf, "PORT", TokenType::Port, 'I'); // set Port No (ex) Port(0)
    sysfunc_cc_add!(sf, "MetaText", TokenType::MetaText, 'S', 1); // write meta text (ex) MetaText{"hello"}
    sysfunc_cc_add!(sf, "Text", TokenType::MetaText, 'S', 1); // write meta text (ex) MetaText{"hello"}
    sysfunc_cc_add!(sf, "TEXT", TokenType::MetaText, 'S', 1); // write meta text (ex) MetaText{"hello"}
    sysfunc_cc_add!(sf, "Copyright", TokenType::MetaText, 'S', 2); // write copyright text (ex) Copyright{"hello"}
    sysfunc_cc_add!(sf, "COPYRIGHT", TokenType::MetaText, 'S', 2); // write copyright text (ex) COPYRIGHT{"hello"}
    sysfunc_cc_add!(sf, "TrackName", TokenType::MetaText, 'S', 3); // write TrackName text (ex) TrackName{"hello"}
    sysfunc_cc_add!(sf, "TRACK_NAME", TokenType::MetaText, 'S', 3); // write TrackName text (ex) TrackName{"hello"}
    sysfunc_cc_add!(sf, "InstrumentName", TokenType::MetaText, 'S', 4); // write InstrumentName text (ex) InstrumentName{"hello"}
    sysfunc_cc_add!(sf, "Lyric", TokenType::MetaText, 'S', 5); // write Lyric text (ex) Lyric{"hello"}
    sysfunc_cc_add!(sf, "LYRIC", TokenType::MetaText, 'S', 5); // write Lyric text (ex) LYRIC{"hello"}
    sysfunc_cc_add!(sf, "MAKER", TokenType::MetaText, 'S', 6); // write MAKER text (ex) MAKER{"hello"}
    sysfunc_cc_add!(sf, "Maker", TokenType::MetaText, 'S', 6); // write Maker text (ex) Maker{"hello"}
    sysfunc_cc_add!(sf, "CuePoint", TokenType::MetaText, 'S', 7); // write CuePoint text (ex) CuePoint{"hello"}
                                                                  //@ GS System Exclusive
    sysfunc_cc_add!(sf, "GSEffect", TokenType::GSEffect, 'A', 0); // GSEffect(num, val) (ex) GSEffect($30, 0)
    sysfunc_cc_add!(sf, "GSReverbMacro", TokenType::GSEffect, 'I', 0x30); // GSReverbMacro(val) - 0:Room1 5:Hall 6:Delay (ex) GSReverbMacro(0)
    sysfunc_cc_add!(sf, "GSReverbCharacter", TokenType::GSEffect, 'I', 0x31); // GSReverbCharacter(val) - リバーブのキャラクター (ex) GSReverbCharacter(0)
    sysfunc_cc_add!(sf, "GSReverbPRE_LPE", TokenType::GSEffect, 'I', 0x32); // GSReverbPRE_LPE(val) (ex) GSReverbPRE_LPE(0)
    sysfunc_cc_add!(sf, "GSReverbLevel", TokenType::GSEffect, 'I', 0x33); // GSReverbLevel(val) (ex) GSReverbLevel(0)
    sysfunc_cc_add!(sf, "GSReverbTime", TokenType::GSEffect, 'I', 0x34); // GSReverbTime(val) (ex) GSReverbTime(0)
    sysfunc_cc_add!(sf, "GSReverbFeedback", TokenType::GSEffect, 'I', 0x35); // GSReverbFeedback(val) (ex) GSReverbFeedback(0)
    sysfunc_cc_add!(sf, "GSReverbSendToChorus", TokenType::GSEffect, 'I', 0x36); // GSReverbSendToChorus(val) (ex) GSReverbSendToChorus(0)
    sysfunc_cc_add!(sf, "GSChorusMacro", TokenType::GSEffect, 'I', 0x38); // GSChorusMacro(val) (ex) GSChorusMacro(0)
    sysfunc_cc_add!(sf, "GSChorusPRE_LPF", TokenType::GSEffect, 'I', 0x39); // GSChorusPRE_LPF(val) (ex) GSChorusPRE_LPF(0)
    sysfunc_cc_add!(sf, "GSChorusLevel", TokenType::GSEffect, 'I', 0x3A); // GSChorusLevel(val) (ex) GSChorusLevel(0)
    sysfunc_cc_add!(sf, "GSChorusFeedback", TokenType::GSEffect, 'I', 0x3B); // GSChorusFeedback(val) (ex) GSChorusFeedback(0)
    sysfunc_cc_add!(sf, "GSChorusDelay", TokenType::GSEffect, 'I', 0x3C); // GSChorusDelay(val) (ex) GSChorusDelay(0)
    sysfunc_cc_add!(sf, "GSChorusRate", TokenType::GSEffect, 'I', 0x3D); // GSChorusRate(val) (ex) GSChorusRate(0)
    sysfunc_cc_add!(sf, "GSChorusDepth", TokenType::GSEffect, 'I', 0x3E); // GSChorusDepth(val) (ex) GSChorusDepth(0)
    sysfunc_cc_add!(sf, "GSChorusSendToReverb", TokenType::GSEffect, 'I', 0x3F); // GSChorusSendToReverb(val) (ex) GSChorusSendToReverb(0)
    sysfunc_cc_add!(sf, "GSChorusSendToDelay", TokenType::GSEffect, 'I', 0x40); // GSChorusSendToDelay(val) (ex) GSChorusSendToDelay(0)
    sysfunc_cc_add!(sf, "GS_RHYTHM", TokenType::GSEffect, 'I', 0x15); // Change to rhythm part val=0:instrument/1:drum1/2:drum2 (ex) GS_RHYTHM(1)
    sysfunc_cc_add!(sf, "GSScaleTuning", TokenType::GSEffect, 'A', 0x11); // GS Scale Tuning. GSScaleTuning(C,Cp,D,Dp,E,F,Fp,G,Gp,A,Ap,B) (ex) GSScaleTuning(0,0,0,0,0,0,0,0,0,0,0,0)
                                                                          //@ Script command
    sysfunc_add!(sf, "Int", TokenType::DefInt, '*'); // define int variables (ex) Int A = 3
    sysfunc_add!(sf, "INT", TokenType::DefInt, '*'); // define int variables (ex) INT A = 3
    sysfunc_add!(sf, "Str", TokenType::DefStr, '*'); // define string variables (ex) Str A = {cde}
    sysfunc_add!(sf, "STR", TokenType::DefStr, '*'); // define string variables (ex) STR A = {cde}
    sysfunc_add!(sf, "Array", TokenType::DefArray, '*'); // define array variables (ex) Array A = (1,2,3)
    sysfunc_add!(sf, "ARRAY", TokenType::DefArray, '*'); // define array variables (ex) ARRAY A = (1,2,3)
    sysfunc_add!(sf, "Print", TokenType::Print, 'S'); // print value (ex) Print({hello})
    sysfunc_add!(sf, "PRINT", TokenType::Print, 'S'); // print value (ex) PRINT({hello})
    sysfunc_add!(sf, "System.Include", TokenType::Include, '*'); // Unimplemented
    sysfunc_add!(sf, "Include", TokenType::Include, '*'); // Unimplemented
    sysfunc_add!(sf, "INCLUDE", TokenType::Include, '*'); // Unimplemented
    sysfunc_add!(sf, "IF", TokenType::If, '*'); // IF(cond){ true }ELSE{ false }
    sysfunc_add!(sf, "If", TokenType::If, '*'); // IF(cond){ true }ELSE{ false }
    sysfunc_add!(sf, "FOR", TokenType::For, '*'); // FOR(INT I = 0; I < 10; I++){ ... }
    sysfunc_add!(sf, "For", TokenType::For, '*'); // FOR(INT I = 0; I < 10; I++){ ... }
    sysfunc_add!(sf, "WHILE", TokenType::While, '*'); // WHILE(cond) { ... }
    sysfunc_add!(sf, "While", TokenType::While, '*'); // WHILE(cond) { ... }
    sysfunc_add!(sf, "BREAK", TokenType::Break, '_'); // exit from loop
    sysfunc_add!(sf, "Break", TokenType::Break, '_'); // exit from loop
    sysfunc_add!(sf, "EXIT", TokenType::Break, '_'); // exit from loop
    sysfunc_add!(sf, "Exit", TokenType::Break, '_'); // exit from loop
    sysfunc_add!(sf, "CONTINUE", TokenType::Continue, '_'); // exit from loop
    sysfunc_add!(sf, "Continue", TokenType::Continue, '_'); // exit from loop
    sysfunc_add!(sf, "RETURN", TokenType::Return, '*'); // return from function
    sysfunc_add!(sf, "Return", TokenType::Return, '*'); // return from function
                                                        // sysfunc_add!(sf, "Result", TokenType::Return, '*'); // set function's result
    sysfunc_add!(sf, "RANDOM_SEED", TokenType::SetRandomSeed, '*'); // set random seed
    sysfunc_add!(sf, "RandomSeed", TokenType::SetRandomSeed, '*'); // set random seed
    sysfunc_add!(sf, "FUNCTION", TokenType::DefUserFunction, '*'); // define user function
    sysfunc_add!(sf, "Function", TokenType::DefUserFunction, '*'); // define user function
                                                                   //@ MIDI command
    sysfunc_add!(sf, "DirectSMF", TokenType::DirectSMF, 'A'); // direct smf event / DirectSMF(b1, b2, b3, ...)
    sysfunc_add!(sf, "NoteOn", TokenType::NoteOn, 'A'); // note no / NoteOn(noteno, velocity)
    sysfunc_add!(sf, "NoteOff", TokenType::NoteOff, 'A'); // note off / NoteOn(noteno, velocity)
                                                          //</SYSTEM_FUNCTION>
    sf
}

macro_rules! syscalc_add {
    ($obj:expr, $name:expr, $callback:expr) => {
        $obj.insert(String::from($name), $callback)
    };
}

pub fn init_system_calc_functions() -> HashMap<String, sakura_functions::CallbackCalcFn> {
    let mut sf: HashMap<String, sakura_functions::CallbackCalcFn> = HashMap::new();
    // <SYSTEM_CALC_FUNCTION>
    syscalc_add!(sf, "Random", sakura_functions::calc_randomint); // Random(N, M) | Random(N) // return random number from n to m (ex) Random(1,6)
    syscalc_add!(sf, "RANDOM", sakura_functions::calc_randomint); // RANDOM(N, M) | RANDOM(N) // return random number from n to m (ex) RANDOM(1,6)
    syscalc_add!(sf, "RandomInt", sakura_functions::calc_randomint); // RandomInt(N, M) | RandomInt(N) // return random number from n to m (ex) RandomInt(1,6)
    syscalc_add!(sf, "RND", sakura_functions::calc_randomint); // RND(N, M) | RND(N) // return random number from n to m (ex) RND(1,6)
    syscalc_add!(sf, "Rnd", sakura_functions::calc_randomint); // Rnd(N, M) | Rnd(N) // return random number from n to m (ex) Rnd(1,6)
    syscalc_add!(sf, "RandomSelect", sakura_functions::calc_random_select); // RandomSelect(...) // return one item selected from the arguments (ex) RandomSelect({a}, {b}, {c})
    syscalc_add!(sf, "Chr", sakura_functions::calc_chr); // Chr(C) // convert code to char (ex) Chr(49)
    syscalc_add!(sf, "CHR", sakura_functions::calc_chr); // CHR(C) // convert code to char (ex) CHR(49)
    syscalc_add!(sf, "Asc", sakura_functions::calc_asc); // Asc(S) // return character code of S (ex) Asc({A}) // => 65
    syscalc_add!(sf, "ASC", sakura_functions::calc_asc); // ASC(S) // return character code of S (ex) ASC({A}) // => 65
    syscalc_add!(sf, "Mid", sakura_functions::calc_mid); // Mid(S, N, M) // extract M characters from string S starting at position N and return them (ex) Mid({abc}, 1,2) // => ab
    syscalc_add!(sf, "MID", sakura_functions::calc_mid); // MID(S, N, M) // extract M characters from string S starting at position N and return them (ex) MID({abc}, 1,2) // => ab
    syscalc_add!(sf, "Replace", sakura_functions::calc_replace); // Replace(S, A, B) // Replace A in string S with B (ex) Replace({abc}, {a}, {b}) // =>  bbc
    syscalc_add!(sf, "REPLACE", sakura_functions::calc_replace); // REPLACE(S, A, B) // Replace A in string S with B (ex) REPLACE({abc}, {a}, {b}) // =>  bbc
    syscalc_add!(sf, "SizeOf", sakura_functions::calc_sizeof); // SizeOf(A) // return size of A
    syscalc_add!(sf, "SIZEOF", sakura_functions::calc_sizeof); // SIZEOF(A) // return size of A
    syscalc_add!(sf, "ArrayFlatten", sakura_functions::calc_array_flatten); // ArrayFlatten(A) // flatten nested arrays
    syscalc_add!(sf, "ARRAYFLATTEN", sakura_functions::calc_array_flatten); // ARRAYFLATTEN(A) // flatten nested arrays
    syscalc_add!(sf, "StrLen", sakura_functions::calc_strlen); // StrLen(S) // return length of S (ex) StrLen({abc}) // => 3
    syscalc_add!(sf, "STRLEN", sakura_functions::calc_strlen); // STRLEN(S) // return length of S (ex) STRLEN({abc}) // => 3
    syscalc_add!(sf, "MML", sakura_functions::calc_mml); // MML(C) // return C(o/v/q/t/@/BR) value (ex) MML({o})
    syscalc_add!(sf, "NoteNo", sakura_functions::calc_noteno); // NoteNo(MML) // return note no of the note written in MML (ex) NoteNo(o5e) // => 64
    syscalc_add!(sf, "NOTENO", sakura_functions::calc_noteno); // NOTENO(MML) // return note no of the note written in MML (ex) NOTENO(o5e) // => 64
    syscalc_add!(sf, "Hex", sakura_functions::calc_hex); // Hex(V) // return Hex value (ex) Hex(255) // => FF
    syscalc_add!(sf, "HEX", sakura_functions::calc_hex); // HEX(V) // return Hex value (ex) Hex(255) // => FF
    syscalc_add!(sf, "Pos", sakura_functions::calc_pos); // Pos(N, M) // Return the 1-based index of substring N in M (ex) Pos({b}, {abc}) // => 2
    syscalc_add!(sf, "POS", sakura_functions::calc_pos); // POS(N, M) // Return the 1-based index of substring N in M (ex) Pos({b}, {abc}) // => 2
                                                         // </SYSTEM_CALC_FUNCTION>
    sf
}
