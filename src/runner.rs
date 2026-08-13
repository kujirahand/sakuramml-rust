//! runner from tokens
use super::lexer::lex;
use super::note_length::calc_length;
use super::sakura_message::MessageKind;
use super::song::{
    Event, NoteInfo, NoteParam, OnNoteSine, SineType, Song, Track, WaveMode, WriteCtx, WriteTarget,
};
use super::svalue::SValue;
use super::token::{
    Token, TokenType, COMMENT_DEBUG, NOTE_PARAM_L, NOTE_PARAM_O, NOTE_PARAM_Q, NOTE_PARAM_T,
    NOTE_PARAM_V, WRITE_TARGET_PB_BIG, WRITE_TARGET_PB_SMALL,
};
use crate::mml_def::TieMode;
use crate::token::TokenValueType;

#[derive(Debug)]
pub struct LoopItem {
    pub start_pos: usize,
    pub end_pos: usize,
    pub index: usize,
    pub count: usize,
}

impl LoopItem {
    fn new() -> Self {
        LoopItem {
            start_pos: 0,
            end_pos: 0,
            index: 0,
            count: 0,
        }
    }
}

macro_rules! trk {
    ($song:expr) => {
        $song.tracks[$song.cur_track]
    };
}

mod cc;
mod control;
pub(crate) mod function;
mod meta;
pub(crate) mod note;
mod structure;
mod sysex;
mod tie;
mod track_state;
mod variable;

use cc::*;
use control::*;
use function::*;
use meta::*;
use note::*;
use structure::*;
use sysex::*;
use tie::*;
use track_state::*;
use variable::*;

/// run tokens and get arguments(=`Vec<Token>`)
pub fn exec_args(song: &mut Song, tokens: &[Token]) -> Vec<SValue> {
    let mut args: Vec<SValue> = vec![];
    let tmp_needs_return_values = song.flags.function_needs_return_value;
    song.flags.function_needs_return_value = true;
    for t in tokens {
        exec(song, std::slice::from_ref(t));
        let v = song.stack.pop().unwrap_or(SValue::None);
        args.push(v);
    }
    song.flags.function_needs_return_value = tmp_needs_return_values;
    args
}

/// 数値列を必要とする命令の引数を実行し、配列を再帰的に平坦化する。
pub fn exec_int_args(song: &mut Song, t: &Token) -> Vec<isize> {
    exec_args(song, t.children.as_deref().unwrap_or(&[]))
        .iter()
        .flat_map(SValue::to_int_array)
        .collect()
}

/// run tokens and get value
pub fn exec_value(song: &mut Song, tokens: &[Token]) -> SValue {
    let tmp_needs_return_values = song.flags.function_needs_return_value;
    song.flags.function_needs_return_value = true;
    exec(song, tokens);
    let return_value = song.stack.pop().unwrap_or(SValue::from_i(0));
    song.flags.function_needs_return_value = tmp_needs_return_values;
    return_value
}

/// run tokens and get int value
pub fn exec_value_int(song: &mut Song, tokens: &[Token]) -> isize {
    exec_value(song, tokens).to_i()
}

/// run tokens and get int value
pub fn exec_value_int_by_token(song: &mut Song, tok: &Token) -> isize {
    let empty_tokens = vec![];
    let tokens = tok.children.as_ref().unwrap_or(&empty_tokens);
    exec_value_int(song, tokens)
}

/// run tokens
pub fn exec(song: &mut Song, tokens: &[Token]) -> bool {
    let mut pos = 0;
    let mut loop_stack: Vec<LoopItem> = vec![];
    while pos < tokens.len() {
        if song.event_limit_exceeded() {
            break;
        }
        if song.flags.break_flag != 0 {
            break;
        }
        let t = &tokens[pos];
        if song.debug {
            println!(
                "- exec({:03})(line:{}) {}",
                pos,
                song.lineno,
                t.to_debug_str(0)
            );
        }
        match t.ttype {
            TokenType::Unimplemented => {}
            TokenType::Empty => {}
            TokenType::Comment => exec_comment(song, t),
            TokenType::LineNo => song.lineno = t.lineno,
            TokenType::Error => {
                if song.debug {
                    println!("[RUNTIME.ERROR]");
                }
            }
            TokenType::TimeBase => {}  // 構文解析の時に設定済み
            TokenType::Include => {}   // 構文解析時
            TokenType::SoundType => {} // 現状意味なし
            TokenType::DeviceNumber => exec_device_number(song, t),
            TokenType::Print => exec_print(song, t),
            // Loop controll
            TokenType::LoopBegin => {
                let mut it = LoopItem::new();
                it.start_pos = pos + 1;
                it.count = var_extract(&t.data[0], song).to_i() as usize;
                // Avoid usize underflow/panic when loop count is 0.
                // Also keep behavior predictable (treat 0 as 1 iteration).
                if it.count == 0 {
                    song.add_log(format!(
                        "[WARN]({}) Loop count is 0; treated as 1.",
                        t.lineno
                    ));
                    it.count = 1;
                }
                // println!("loop={}", it.count);
                loop_stack.push(it);
            }
            TokenType::LoopBreak => {
                let mut it = match loop_stack.pop() {
                    None => {
                        pos += 1;
                        continue;
                    }
                    Some(i) => i,
                };
                if it.index.saturating_add(1) >= it.count {
                    if it.end_pos == 0 {
                        for i in pos..tokens.len() {
                            match &tokens[i].ttype {
                                TokenType::LoopEnd => {
                                    it.end_pos = i + 1;
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    if it.end_pos > 0 {
                        pos = it.end_pos;
                        continue;
                    }
                } else {
                    loop_stack.push(it);
                }
            }
            TokenType::LoopEnd => {
                if loop_stack.len() > 0 {
                    let mut it = match loop_stack.pop() {
                        Some(v) => v,
                        None => {
                            pos += 1;
                            continue;
                        }
                    };
                    it.end_pos = pos + 1;
                    it.index += 1;
                    if it.index < it.count {
                        pos = it.start_pos;
                        loop_stack.push(it);
                        continue;
                    }
                }
            }
            TokenType::Track => exec_track(song, t),
            TokenType::Channel => exec_channel(song, t),
            TokenType::Voice => exec_voice(song, t),
            TokenType::Note => exec_note(song, t),
            TokenType::NoteN => exec_note_n(song, t),
            TokenType::Rest => exec_rest(song, t),
            TokenType::Length => exec_length(song, t),
            TokenType::Octave => exec_octave(song, t),
            TokenType::OctaveRel => exec_octave_rel(song, t),
            TokenType::VelocityRel => exec_velocity_rel(song, t),
            TokenType::QLenRel => exec_qlen_rel(song, t),
            TokenType::OctaveOnce => exec_octave_once(song, t),
            TokenType::QLen => exec_qlen(song, t),
            TokenType::Velocity => exec_velocity(song, t),
            TokenType::Timing => exec_timing(song, t),
            TokenType::ControlChange => exec_control_change(song, t),
            TokenType::RPN => exec_cc_rpn_nrpn_direct(song, t, 101, 100, 6),
            TokenType::RPNCommand => exec_cc_rpn_nrpn(song, t, 101, 100, 6),
            TokenType::NRPN => exec_cc_rpn_nrpn_direct(song, t, 99, 98, 0),
            TokenType::NRPNCommand => exec_cc_rpn_nrpn(song, t, 99, 98, 0),
            TokenType::PitchBend => exec_pitch_bend(song, t),
            TokenType::Tempo => exec_tempo(song, t),
            TokenType::TempoChange => exec_tempo_change(song, t),
            TokenType::MetaText => exec_meta_text(song, t),
            TokenType::Port => exec_port(song, t),
            TokenType::TimeSignature => exec_time_signature(song, t),
            TokenType::SysEx => exec_sysex(song, t),
            TokenType::SysexReset => exec_sysex_reset(song, t),
            TokenType::SysExCommand => exec_sysex_command(song, t), // Universal SysEx
            TokenType::GSEffect => exec_gs_effect(song, t),
            TokenType::Time => trk!(song).timepos = exec_get_time(song, t, "TIME"),
            TokenType::PlayFrom => song.play_from = exec_get_time(song, t, "PlayFrom"),
            TokenType::HarmonyBegin => exec_harmony(song, t, true),
            TokenType::HarmonyEnd => exec_harmony(song, t, false),
            TokenType::Tokens => exec_tokens(song, t),
            TokenType::Div => exec_div(song, t),
            TokenType::Sub => exec_sub(song, t),
            TokenType::KeyFlag => exec_key_flag(song, t),
            TokenType::KeyShift => exec_key_shift(song, t),
            TokenType::TrackKey => exec_track_key(song, t),
            TokenType::DefInt => exec_def_int(song, t),
            TokenType::DefStr => exec_def_str(song, t),
            TokenType::DefArray => exec_def_array(song, t),
            TokenType::GetVariable => exec_get_variable(song, t),
            TokenType::LetVar => exec_let_var(song, t),
            TokenType::StrVarReplace => exec_str_var_replace(song, t),
            TokenType::PlayFromHere => exec_play_from_here(song),
            TokenType::SongVelocityAdd => exec_song_velocity_add(song, t),
            TokenType::SongQAdd => exec_song_q_add(song, t),
            // 音符属性(v/q/t/o/l)の先行指定
            TokenType::OctaveRandom => exec_note_param_random(song, t, NOTE_PARAM_O),
            TokenType::VelocityRandom => exec_note_param_random(song, t, NOTE_PARAM_V),
            TokenType::TimingRandom => exec_note_param_random(song, t, NOTE_PARAM_T),
            TokenType::QLenRandom => exec_note_param_random(song, t, NOTE_PARAM_Q),
            TokenType::LengthRandom => exec_note_param_random(song, t, NOTE_PARAM_L),
            TokenType::VelocityOnTime => exec_note_param_on_time(song, t, NOTE_PARAM_V),
            TokenType::QLenOnTime => exec_note_param_on_time(song, t, NOTE_PARAM_Q),
            TokenType::TimingOnTime => exec_note_param_on_time(song, t, NOTE_PARAM_T),
            TokenType::OctaveOnTime => exec_note_param_on_time(song, t, NOTE_PARAM_O),
            TokenType::LengthOnTime => exec_note_param_on_time(song, t, NOTE_PARAM_L),
            TokenType::VelocityOnNote => exec_note_param_on_note(song, t, NOTE_PARAM_V),
            TokenType::QLenOnNote => exec_note_param_on_note(song, t, NOTE_PARAM_Q),
            TokenType::TimingOnNote => exec_note_param_on_note(song, t, NOTE_PARAM_T),
            TokenType::OctaveOnNote => exec_note_param_on_note(song, t, NOTE_PARAM_O),
            TokenType::LengthOnNote => exec_note_param_on_note(song, t, NOTE_PARAM_L),
            TokenType::VelocityOnCycle => exec_note_param_on_cycle(song, t, NOTE_PARAM_V),
            TokenType::QLenOnCycle => exec_note_param_on_cycle(song, t, NOTE_PARAM_Q),
            TokenType::TimingOnCycle => exec_note_param_on_cycle(song, t, NOTE_PARAM_T),
            TokenType::OctaveOnCycle => exec_note_param_on_cycle(song, t, NOTE_PARAM_O),
            TokenType::LengthOnCycle => exec_note_param_on_cycle(song, t, NOTE_PARAM_L),
            TokenType::NoteParamRange => exec_note_param_range(song, t),
            TokenType::NoteParamDelay => exec_note_param_delay(song, t),
            TokenType::NoteParamRepeat => exec_note_param_repeat(song, t),
            TokenType::NoteParamMax => exec_note_param_max(song, t),
            // CC・ピッチベンドの先行指定
            TokenType::CConTime => exec_cc_on_time(song, t),
            TokenType::CConNote => exec_cc_on_note(song, t),
            TokenType::CConNoteWave => exec_cc_on_note_wave(song, t),
            TokenType::CConNoteWaveEx => exec_cc_on_note_wave_ex(song, t),
            TokenType::CConNoteWaveR => exec_cc_on_note_wave_r(song, t),
            TokenType::CConCycle => exec_cc_on_cycle(song, t),
            TokenType::CCSine => exec_cc_sine(song, t),
            TokenType::CConNoteSine => exec_cc_on_note_sine(song, t),
            TokenType::CCDelay => exec_cc_delay(song, t),
            TokenType::CCRandom => exec_cc_random(song, t),
            TokenType::CCRange => exec_cc_range(song, t),
            TokenType::CCRepeat => exec_cc_repeat(song, t),
            TokenType::CConTimeFreq => exec_cc_on_time_freq(song, t),
            TokenType::Decresc => exec_decres(song, t),
            TokenType::FadeIO => exec_fade_io(song, t),
            TokenType::PBonTime => exec_pb_on_time(song, t),
            TokenType::PBonNote => exec_pb_on_note(song, t),
            TokenType::PBonNoteWave => exec_pb_on_note_wave(song, t),
            TokenType::MeasureShift => exec_measure_shift(song, t),
            TokenType::TrackSync => song.track_sync(),
            TokenType::TieMode => exec_tie_mode(song, t),
            TokenType::UseKeyShift => exec_use_key_shift(song, t),
            TokenType::If => {
                exec_if(song, t);
            }
            TokenType::For => {
                exec_for(song, t);
            }
            TokenType::While => {
                exec_while(song, t);
            }
            TokenType::Break => {
                song.flags.break_flag = 1;
                break;
            }
            TokenType::Continue => {
                song.flags.break_flag = 2;
                break;
            }
            TokenType::Return => {
                exec_return(song, t);
                // set return
                song.flags.break_flag = 3;
                break;
            }
            TokenType::DefUserFunction => {
                // nop
            }
            TokenType::CalcTree => exec_calc_tree(song, t),
            TokenType::ConstInt => exec_const_int(song, t),
            TokenType::ConstStr => exec_const_str(song, t),
            TokenType::Value => exec_value_token(song, t),
            TokenType::ValueInc => exec_value_inc(song, t),
            TokenType::MakeArray => exec_make_array(song, t),
            TokenType::SetConfig => exec_set_config(song, t),
            TokenType::CallUserFunction => {
                exec_userfunc_or_array_or_macro(song, t);
            }
            TokenType::Play => {
                exec_play(song, t);
            }
            TokenType::Rhythm => {}
            TokenType::ControlChangeCommand => {}
            TokenType::Cresc => {}         // replaced CConTime
            TokenType::SetRandomSeed => {} // replace SetConfig
            TokenType::DirectSMF => exec_direct_smf(song, t),
            TokenType::NoteOn => exec_note_on(song, t),
            TokenType::NoteOff => exec_note_off(song, t),
        }
        pos += 1;
    }
    true
}

fn runtime_error(song: &mut Song, msg: &str) {
    song.add_log(format!(
        "[ERROR]({}) {}: {}",
        song.lineno,
        song.get_message(MessageKind::RuntimeError),
        msg
    ));
}

pub fn value_range(min_v: isize, value: isize, max_v: isize) -> isize {
    let mut v = value;
    if v < min_v {
        v = min_v;
    } else if v > max_v {
        v = max_v;
    }
    v
}

/// exec source (easy version)
pub fn exec_easy(src: &str) -> Song {
    let mut song = Song::new();
    let t = &lex(&mut song, src, 0);
    exec(&mut song, &t);
    song
}
