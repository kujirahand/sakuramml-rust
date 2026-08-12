//! runner from tokens
use crate::mml_def::TieMode;
use crate::token::TokenValueType;
use super::lexer::lex;
use super::song::{Event, NoteInfo, Song};
use super::svalue::SValue;
use super::token::{Token, TokenType, COMMENT_DEBUG};
use super::sakura_message::MessageKind;
use super::note_length::calc_length;

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
mod function;
mod note;
mod structure;
mod tie;

use cc::*;
use control::*;
use function::*;
use note::*;
use structure::*;
use tie::*;

/// run tokens and get arguments(=`Vec<Token>`)
pub fn exec_args(song: &mut Song, tokens: &Vec<Token>) -> Vec<SValue> {
    let mut args: Vec<SValue> = vec![];
    let tmp_needs_return_values = song.flags.function_needs_return_value;
    song.flags.function_needs_return_value = true;
    for t in tokens {
        exec(song, &vec![t.clone()]);
        let v = song.stack.pop().unwrap_or(SValue::None);
        args.push(v);
    }
    song.flags.function_needs_return_value = tmp_needs_return_values;
    args
}

/// run tokens and get value
pub fn exec_value(song: &mut Song, tokens: &Vec<Token>) -> SValue {
    let tmp_needs_return_values = song.flags.function_needs_return_value;
    song.flags.function_needs_return_value = true;
    exec(song, tokens);
    let return_value = song.stack.pop().unwrap_or(SValue::from_i(0));
    song.flags.function_needs_return_value = tmp_needs_return_values;
    return_value
}

/// run tokens and get int value
pub fn exec_value_int(song: &mut Song, tokens: &Vec<Token>) -> isize {
    exec_value(song, tokens).to_i()
}

/// run tokens and get int value
pub fn exec_value_int_by_token(song: &mut Song, tok: &Token) -> isize {
    let empty_tokens = vec![];
    let tokens = tok.children.as_ref().unwrap_or(&empty_tokens);
    exec_value_int(song, tokens)
}

/// MetaTextに書き込める文字列は127バイトまでなので、文字境界を保ったまま切り詰める
fn trim_meta_text(txt_raw: &str) -> String {
    let mut txt = String::from("");
    let mut cnt = 0;
    for c in txt_raw.chars() {
        cnt += c.len_utf8();
        if cnt < 128 {
            txt.push(c);
            continue;
        }
        break;
    }
    txt
}

/// run tokens
pub fn exec(song: &mut Song, tokens: &Vec<Token>) -> bool {
    let mut pos = 0;
    let mut loop_stack: Vec<LoopItem> = vec![];
    while pos < tokens.len() {
        if song.flags.break_flag != 0 { break; }
        let t = &tokens[pos];
        if song.debug {
            println!("- exec({:03})(line:{}) {}", pos, song.lineno, t.to_debug_str(0));
        }
        match t.ttype {
            TokenType::Unimplemented => {},
            TokenType::Empty => {},
            TokenType::Comment => {
                // 「/// xxx」形式のコメントは、行番号付きでMetaTextに埋め込む (デバッグ用) #79
                if t.value_i == COMMENT_DEBUG {
                    let body = t.value_s.clone().unwrap_or(String::from(""));
                    let txt = trim_meta_text(&format!("L{}: {}", t.lineno + 1, body));
                    let e = Event::meta(
                        trk!(song).timepos,
                        0xFF,
                        1, // Meta type = Text
                        txt.len() as isize,
                        txt.into_bytes(),
                    );
                    song.add_event(e);
                }
            },
            TokenType::LineNo => {
                song.lineno = t.lineno;
            },
            TokenType::Error => {
                if song.debug {
                    println!("[RUNTIME.ERROR]");
                }
            },
            TokenType::TimeBase => {}, // 構文解析の時に設定済み
            TokenType::Include => {}, // 構文解析時
            TokenType::SoundType => {}, // 現状意味なし
            TokenType::DeviceNumber => {
                let args_tokens = t.children.clone().unwrap_or(vec![]);
                let n = exec_args(song, &args_tokens);
                song.device_number = if n.len() >= 1 { n[0].to_i() as u8 } else { 0 };
            },
            TokenType::Print => {
                let args_tokens = t.children.clone().unwrap_or(vec![]);
                // println!("@@@print_args=:{:?}", args_tokens);
                let args = exec_args(song, &args_tokens);
                let mut disp: Vec<String> = vec![];
                for v in args {
                    disp.push(v.to_s());
                }
                let disp_s = disp.join(" ");
                let msg = format!("[PRINT]({}) {}", t.lineno, disp_s);
                if song.debug {
                    println!("{}", msg);
                }
                song.add_log(msg);
            },
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
            },
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
            },
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
            },
            TokenType::Track => {
                let no = exec_value_int_by_token(song, t) as usize;
                song.change_cur_track(no);
            },
            TokenType::Channel => {
                let no = exec_value_int_by_token(song, t);
                let v = value_range(1, no, 16) - 1; // CH(1 to 16)
                trk!(song).channel = v as isize;
            },
            TokenType::Voice => exec_voice(song, t),
            TokenType::Note => exec_note(song, t),
            TokenType::NoteN => exec_note_n(song, t),
            TokenType::Rest => exec_rest(song, t),
            TokenType::Length => {
                trk!(song).l_on_note = None;
                trk!(song).length = calc_length(&t.data[0].to_s(), song.timebase, song.timebase);
            },
            TokenType::Octave => {
                trk!(song).o_on_note = None;
                trk!(song).octave = value_range(0, t.value_i, 10);
            },
            TokenType::OctaveRel => {
                trk!(song).octave = value_range(0, trk!(song).octave + t.value_i, 10);
            },
            TokenType::VelocityRel => {
                trk!(song).velocity = value_range(0, trk!(song).velocity + (song.v_add * t.value_i), 127);
            },
            TokenType::QLenRel => {
                trk!(song).qlen = trk!(song).qlen + (song.q_add * t.value_i);
            },
            TokenType::OctaveOnce => {
                trk!(song).octave = value_range(0, trk!(song).octave + t.value_i, 10);
                song.flags.octave_once += t.value_i;
            },
            TokenType::QLen => {
                trk!(song).q_on_note = None;
                trk!(song).qlen = value_range(0, t.value_i, 100);
                trk!(song).q_on_note = None;
            },
            TokenType::Velocity => {
                trk!(song).v_on_note = None;
                trk!(song).v_on_time = None;
                let ino = t.data[0].to_i();
                if ino > 0 {
                    while trk!(song).v_sub.len() >= ino as usize {
                        trk!(song).v_sub.push(0);
                    }
                    trk!(song).v_sub[ino as usize] = value_range(0, t.value_i, 127);
                } else {
                    trk!(song).velocity = value_range(0, t.value_i, 127);
                }
                trk!(song).v_on_time = None;
                trk!(song).v_on_note = None;
            },
            TokenType::Timing => {
                trk!(song).t_on_note = None;
                trk!(song).timing = t.value_i;
                trk!(song).t_on_note = None;
            },
            TokenType::ControlChange => {
                let no = t.value_i;
                let val_tokens = t.children.clone().unwrap_or(vec![]);
                let val_v = exec_value(song, &val_tokens);
                let val = val_v.to_i();
                trk!(song).remove_cc_on_note_wave(no);
                song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, no, val));
            },
            TokenType::RPN => exec_cc_rpn_nrpn_direct(song, t, 101, 100, 6),
            TokenType::RPNCommand => exec_cc_rpn_nrpn(song, t, 101, 100, 6),
            TokenType::NRPN => exec_cc_rpn_nrpn_direct(song, t, 99, 98, 0),
            TokenType::NRPNCommand => exec_cc_rpn_nrpn(song, t, 99, 98, 0),
            TokenType::PitchBend => {
                let val = var_extract(&t.data[0], song).to_i();
                let val = if t.value_i == 0 { val * 128 } else { val + 8192 };
                song.add_event(Event::pitch_bend(
                    trk!(song).timepos,
                    trk!(song).channel,
                    val,
                ));
            },
            TokenType::Tempo => {
                let tempo = exec_value_int_by_token(song, t);
                let tempo = value_range(10, tempo, 300);
                tempo_change(song, tempo);
            },
            TokenType::TempoChange => {
                let data = exec_args(song, &t.children.clone().unwrap_or(vec![]));
                if data.len() == 3 {
                    tempo_change_a_to_b(song, data[0].to_i(), data[1].to_i(), data[2].to_i());
                } else if data.len() == 2 {
                    tempo_change_a_to_b(song, song.tempo, data[0].to_i(), data[1].to_i());
                } else {
                    tempo_change(song, data[0].to_i());
                }
            },
            TokenType::MetaText => {
                let txt_raw = exec_args(song, &t.children.clone().unwrap_or(vec![]))[0].to_s();
                let txt = trim_meta_text(&txt_raw);
                let e = Event::meta(
                    trk!(song).timepos,
                    0xFF,
                    t.value_i,
                    txt.len() as isize,
                    txt.into_bytes(),
                );
                song.add_event(e);
            },
            TokenType::Port => {
                let port = exec_args(song, &t.children.clone().unwrap_or(vec![]))[0].to_i();
                 trk!(song).port = port;
                let e = Event::meta(
                    trk!(song).timepos,
                    0xFF,
                    0x21,
                    0x01,
                    vec![port as u8],
                );
                song.add_event(e);
            },
            TokenType::TimeSignature => {
                let args = exec_args(song, &t.children.clone().unwrap_or(vec![]));
                if args.len() < 2 {
                    runtime_error(song, "[TimeSignature] argument must be 2");
                    pos += 1;
                    continue;
                }
                song.timesig_frac = value_range(2, args[0].to_i(), 64);
                song.timesig_deno = value_range(2, args[1].to_i(), 64);
                song.timesig_deno = match song.timesig_deno {
                    2 => 2,
                    4 => 4,
                    8 => 8,
                    16 => 16,
                    _ => {
                        runtime_error(song, "[TimeSignature] value must be 2/4/8/16,n");
                        4
                    }
                };
                let deno_v = match song.timesig_deno {
                    2 => 1,
                    4 => 2,
                    8 => 3,
                    16 => 4,
                    _ => 2,
                };
                let e = Event::meta(
                    trk!(song).timepos,
                    0xFF,
                    0x58,
                    0x04,
                    vec![song.timesig_frac as u8, deno_v as u8, 0x18, 0x08],
                );
                song.add_event(e);
            },
            TokenType::SysEx => {
                // check arguments
                let mut args: Vec<SValue> = exec_args(song, &t.children.clone().unwrap_or(vec![]));
                if args.len() == 0 {
                    runtime_error(song, &format!("SysEx : {}", song.get_message(MessageKind::ErrorWrongArguments)));
                    pos += 1;
                    continue;
                }
                // check leading 0xF0
                if args[0].to_i() != 0xF0 {
                    args.insert(0, SValue::from_i(0xF0));
                }
                // check trailing 0xF7
                if args[args.len()-1].to_i() != 0xF7 {
                    args.push(SValue::from_i(0xF7));
                }
                // sysex event
                let e = Event::sysex(trk!(song).timepos, &args, t.value_i == 1);
                song.add_event(e);
            },
            TokenType::SysexReset => {
                let time = trk!(song).timepos;
                let dev = song.device_number as u8;
                match t.value_i {
                    0 => { // GM
                        song.add_event(Event::sysex_raw(time, vec![0xF0, 0x7E, 0x7F, 0x9, 0x1, 0xF7]));
                    }
                    1 => { // GS
                        song.add_event(Event::sysex_raw(time, vec![0xF0, 0x41, dev, 0x42, 0x12, 0x40, 0x00, 0x7F, 0x00, 0x41, 0xF7]));
                    },
                    2 => { // XG
                        song.add_event(Event::sysex_raw(time, vec![0xF0, 0x43, dev, 0x4c, 0x00, 0x00, 0x7e, 0x00, 0xf7]));
                    },
                    _ => {},
                }
            },
            TokenType::SysExCommand => { // Universal SysEx
                let time = trk!(song).timepos;
                let mut event: Option<Event> = Option::None;
                let data = exec_args(song, &t.children.clone().unwrap_or(vec![]));
                let sub_id = t.value_i as u8 & 0x7F;
                match sub_id {
                    0x01 => { // Master Volume (0x01) 7bit
                        // The documented form is MasterVolume(value). Keep the
                        // legacy MasterVolume(dummy, value) form working too.
                        let val = if data.len() >= 2 { data[1].to_i() as u8 & 0x7F }
                            else if data.len() == 1 { data[0].to_i() as u8 & 0x7F }
                            else { 0 };
                        event = Some(Event::sysex(
                            time, &vec![
                                SValue::from_i(0xF0),
                                SValue::from_i(0x7F), // Universal SysEx
                                SValue::from_i(0x7F), // Braodcast
                                SValue::from_i(0x04), // Sub ID#1 (Device Control Messages) 
                                SValue::from_i(0x01), // Sub ID#2 (Master Volume)
                                SValue::from_i(0x00), // must be 0
                                SValue::from_i(val as isize),  // value
                                SValue::from_i(0xf7), // end of SysEx
                            ], false));
                    },
                    0x02 => { // Master Balance (0x02) 14bit
                        // The documented form is MasterBalance(value). Keep the
                        // legacy MasterBalance(dummy, value) form working too.
                        let mut val = if data.len() >= 2 { data[1].to_i() }
                            else if data.len() == 1 { data[0].to_i() }
                            else { 0 };
                        val += 8192;
                        let val_lsb = (val & 0x7F) as isize;
                        let val_msb = ((val >> 7) & 0x7F) as isize;
                        event = Some(Event::sysex(
                            time, &vec![
                                SValue::from_i(0xF0),
                                SValue::from_i(0x7F), // Universal SysEx
                                SValue::from_i(0x7F), // Braodcast
                                SValue::from_i(0x04), // Sub ID#1 (Device Control Messages) 
                                SValue::from_i(0x02), // Sub ID#2 (Master balance)
                                SValue::from_i(val_lsb), // value ll
                                SValue::from_i(val_msb),  // value mm
                                SValue::from_i(0xf7), // end of SysEx
                            ], false));
                    },
                    _ => {},
                }
                if let Some(e) = event {
                    song.add_event(e);
                }
            },
            TokenType::GSEffect => {
                let time = trk!(song).timepos;
                let dev = song.device_number;
                let mut event: Option<Event> = Option::None;
                let data = exec_args(song, &t.children.clone().unwrap_or(vec![]));
                match &t.value_i {
                    0x00 => { // basic
                        let num = if data.len() >= 1 { data[0].to_i() as u8 } else { 0 };
                        let val = if data.len() >= 2 { data[1].to_i() as u8 } else { 0 };
                        event = Some(Event::sysex(
                            time,
                            &vec![
                                SValue::from_i(0xF0), 
                                SValue::from_i(0x41),
                                SValue::from_i(dev as isize), 
                                SValue::from_i(0x42),
                                SValue::from_i(0x12),
                                SValue::from_i(-1), // checksum start
                                SValue::from_i(0x40),
                                SValue::from_i(0x01),
                                SValue::from_i(num as isize),
                                SValue::from_i(val as isize),
                                SValue::from_i(-2), // checksum end
                                SValue::from_i(0xf7)
                            ],
                            true));
                    },
                    0x11 => { // GSScaleTuning
                        if data.len() >= 12 {
                            let mut a = vec![];
                            for v in data.iter() {
                                a.push(v.to_i() as isize);
                            }
                            for ic in 0x11..=0x1F {
                                let e = Event::sysex(
                                    time,
                                    &vec![
                                        SValue::from_i(0xF0), 
                                        SValue::from_i(0x41),
                                        SValue::from_i(dev as isize), 
                                        SValue::from_i(0x42),
                                        SValue::from_i(0x12),
                                        SValue::from_i(-1), // checksum start
                                        SValue::from_i(0x40),
                                        SValue::from_i(ic as isize),
                                        SValue::from_i(0x40),
                                        SValue::from_i(a[0]), SValue::from_i(a[1]), SValue::from_i(a[2]),
                                        SValue::from_i(a[3]), SValue::from_i(a[4]), SValue::from_i(a[5]),
                                        SValue::from_i(a[6]), SValue::from_i(a[7]), SValue::from_i(a[8]),
                                        SValue::from_i(a[9]), SValue::from_i(a[10]), SValue::from_i(a[11]),
                                        SValue::from_i(-2), // checksum end
                                        SValue::from_i(0xf7)
                                    ],
                                    true);
                                song.add_event(e);
                            }
                        }
                    },
                    0x15 => { // change to the rhytm part
                        let val = if data.len() >= 1 { data[0].to_i() as u8 } else { 0 };
                        let ch = trk!(song).channel;
                        let sys_ch = if ch == 9 { 0 } else { if ch <= 9 { ch + 1 } else { ch } } as u8;
                        event = Some(Event::sysex(
                            time,
                            &vec![
                                SValue::from_i(0xF0), 
                                SValue::from_i(0x41),
                                SValue::from_i(dev as isize), 
                                SValue::from_i(0x42),
                                SValue::from_i(0x12),
                                SValue::from_i(-1), // checksum start
                                SValue::from_i(0x40),
                                SValue::from_i(sys_ch as isize),
                                SValue::from_i(0x15),
                                SValue::from_i(val as isize),
                                SValue::from_i(-2), // checksum end
                                SValue::from_i(0xf7)
                            ],
                            true));
                    }
                    // custom GS effect
                    0x30 ..= 0x40 => {
                        let num = (&t.value_i % 256) as u8;
                        let val = data[0].to_i() as u8;
                        event = Some(Event::sysex(
                            time,
                            &vec![
                                SValue::from_i(0xF0), 
                                SValue::from_i(0x41),
                                SValue::from_i(dev as isize), 
                                SValue::from_i(0x42),
                                SValue::from_i(0x12),
                                SValue::from_i(-1), // checksum start
                                SValue::from_i(0x40),
                                SValue::from_i(0x01),
                                SValue::from_i(num as isize),
                                SValue::from_i(val as isize),
                                SValue::from_i(-2), // checksum end
                                SValue::from_i(0xf7)
                            ],
                            true));
                    },
                    _ => {},
                }
                if let Some(e) = event {
                    song.add_event(e);
                }
            },
            TokenType::Time => trk!(song).timepos = exec_get_time(song, t, "TIME"),
            TokenType::PlayFrom => song.play_from = exec_get_time(song, t, "PlayFrom"),
            TokenType::HarmonyBegin => exec_harmony(song, t, true),
            TokenType::HarmonyEnd => exec_harmony(song, t, false),
            TokenType::Tokens => {
                let _ = match &t.children {
                    Some(tokens) => exec(song, tokens),
                    None => false,
                };
            },
            TokenType::Div => exec_div(song, t),
            TokenType::Sub => exec_sub(song, t),
            TokenType::KeyFlag => song.key_flag = t.data[0].to_int_array(),
            TokenType::KeyShift => song.key_shift = exec_value_int_by_token(song, t),
            TokenType::TrackKey => trk!(song).track_key = exec_value_int_by_token(song, t),
            TokenType::DefInt => {
                match &t.value_s {
                    None => { runtime_error(song, "[SYSTEM ERROR][DefInt] variable name is empty"); pos += 1; continue; },
                    Some(var_name) => {
                        let val = exec_value(song, &t.children.clone().unwrap_or(vec![]));
                        if val.is_array() {
                            let msg = format!("{}: {}",
                                song.get_message(MessageKind::ErrorTypeMismatch),
                                var_name);
                            runtime_error(song, &msg);
                        }
                        song.variables_insert(var_name, val);
                    }
                }
            },
            TokenType::DefStr => {
                match &t.value_s {
                    None => { runtime_error(song, "[SYSTEM ERROR][DefStr] variable name is empty"); pos += 1; continue; },
                    Some(var_name) => {
                        let val = exec_value(song, &t.children.clone().unwrap_or(vec![]));
                        song.variables_insert(var_name, val);
                    }
                }
            },
            TokenType::DefArray => {
                match &t.value_s {
                    None => { runtime_error(song, "[SYSTEM ERROR][DefArray] variable name is empty"); pos += 1; continue; },
                    Some(var_name) => {
                        let val = exec_value(song, &t.children.clone().unwrap_or(vec![]));
                        song.variables_insert(var_name, val);
                    }
                }
            },
            TokenType::GetVariable => {
                match &t.value_s {
                    None => {
                        runtime_error(song, "[SYSTEM ERROR][GetVariable] variable name is empty");
                        pos += 1;
                        continue;
                    },
                    Some(var_name) => {
                        // get variable's value
                        let val = song.variables_get(&var_name);
                        // println!("GetVariable: {}={:?}", var_name, vals);
                        let val = match val {
                            Some(v) => v.clone(),
                            None => {
                                match get_system_value(var_name, &song) {
                                    Some(v) => v,
                                    None => SValue::None,
                                }
                            }
                        };
                        song.stack.push(val);
                    }
                }
            },
            TokenType::LetVar => {
                let var_key = t.data[0].to_s();
                let val_tokens = t.children.clone().unwrap_or(vec![]);
                let val = exec_value(song, &val_tokens);
                song.variables_insert(&var_key, val);
            },
            TokenType::StrVarReplace => {
                let var_key = t.value_s.clone().unwrap_or(String::from("ERROR"));
                let args = exec_args(song, &t.children.clone().unwrap_or(vec![]));
                if args.len() >= 2 {
                    let mut val_s = song.variables_get(&var_key).unwrap_or(&SValue::None).to_s();
                    val_s = val_s.replace(&args[0].to_s(), &args[1].to_s());
                    song.variables_insert(&var_key, SValue::from_s(val_s));
                }
            },
            TokenType::PlayFromHere => song.play_from = trk!(song).timepos,
            TokenType::SongVelocityAdd => song.v_add = exec_value_int_by_token(song, t),
            TokenType::SongQAdd => song.q_add = exec_value_int_by_token(song, t),
            TokenType::OctaveRandom => {
                trk!(song).o_rand = var_extract(&t.data[0], song).to_i();
            },
            TokenType::VelocityRandom => {
                trk!(song).v_rand = var_extract(&t.data[0], song).to_i();
            },
            TokenType::TimingRandom => {
                trk!(song).t_rand = var_extract(&t.data[0], song).to_i();
            },
            TokenType::QLenRandom => {
                trk!(song).q_rand = var_extract(&t.data[0], song).to_i();
            },
            TokenType::VelocityOnTime => {
                trk!(song).v_on_note = None;
                trk!(song).v_on_time_start = trk!(song).timepos;
                trk!(song).v_on_time = Some(t.data[0].to_int_array());
            },
            TokenType::VelocityOnNote => {
                trk!(song).v_on_time = None;
                trk!(song).v_on_note_index = 0;
                trk!(song).v_on_note = Some(t.data[0].to_int_array());
                trk!(song).v_on_note_is_cycle = false;
            },
            TokenType::VelocityOnCycle => {
                trk!(song).v_on_time = None;
                trk!(song).v_on_note_index = 0;
                trk!(song).v_on_note = Some(t.data[0].to_int_array());
                trk!(song).v_on_note_is_cycle = true;
            },
            TokenType::TimingOnNote => {
                trk!(song).t_on_note_index = 0;
                trk!(song).t_on_note = Some(t.data[0].to_int_array());
                trk!(song).t_on_note_is_cycle = false;
            },
            TokenType::TimingOnCycle => {
                trk!(song).t_on_note_index = 0;
                trk!(song).t_on_note = Some(t.data[0].to_int_array());
                trk!(song).t_on_note_is_cycle = true;
            },
            TokenType::QLenOnNote => {
                trk!(song).q_on_note_index = 0;
                trk!(song).q_on_note = Some(t.data[0].to_int_array());
                trk!(song).q_on_note_is_cycle = false;
            },
            TokenType::QLenOnCycle => {
                trk!(song).q_on_note_index = 0;
                trk!(song).q_on_note = Some(t.data[0].to_int_array());
                trk!(song).q_on_note_is_cycle = true;
            },
            TokenType::OctaveOnNote => {
                trk!(song).o_on_note_index = 0;
                trk!(song).o_on_note = Some(t.data[0].to_int_array());
                trk!(song).o_on_note_is_cycle = false;
            },
            TokenType::OctaveOnCycle => {
                trk!(song).o_on_note_index = 0;
                trk!(song).o_on_note = Some(t.data[0].to_int_array());
                trk!(song).o_on_note_is_cycle = true;
            },
            TokenType::LengthOnNote => {
                trk!(song).l_on_note_index = 0;
                trk!(song).l_on_note = Some(t.data[0].to_int_array());
                trk!(song).l_on_note_is_cycle = false;
            },
            TokenType::LengthOnCycle => {
                trk!(song).l_on_note_index = 0;
                trk!(song).l_on_note = Some(t.data[0].to_int_array());
                trk!(song).l_on_note_is_cycle = true;
            },
            TokenType::CConTime => {
                let no = t.value_i;
                let ia = t.data[0].to_int_array();
                trk!(song).remove_cc_on(no);
                trk!(song).write_cc_on_time(no, ia);
            },
            TokenType::CConNote => {
                let no = t.value_i;
                let ia = t.data[0].to_int_array();
                trk!(song).set_cc_on_note(no, ia);
            },
            TokenType::CConNoteWave => {
                let no = t.value_i;
                let ia = t.data[0].to_int_array();
                trk!(song).set_cc_on_note_wave(no, ia);
            },
            TokenType::CConTimeFreq => {
                trk!(song).cc_on_time_freq = var_extract(&t.data[0], song).to_i();
            },
            TokenType::Decresc => {
                exec_decres(song, t);
            },
            TokenType::PBonTime => {
                trk!(song).write_pb_on_time(t.value_i, t.data[0].to_int_array(), song.timebase);
            },
            TokenType::MeasureShift => song.flags.measure_shift = exec_value_int_by_token(song, t),
            TokenType::TrackSync => song.track_sync(),
            TokenType::TieMode => {
                let args = exec_args(song, t.children.as_ref().unwrap_or(&vec![]));
                if args.len() >= 1 {
                    trk!(song).tie_mode = TieMode::from_i(var_extract(&args[0], song).to_i());
                }
                if args.len() >= 2 {
                    trk!(song).tie_value = var_extract(&args[1], song).to_i();
                }
            },
            TokenType::UseKeyShift => {
                song.use_key_shift = t.value_i != 0;
            },
            TokenType::If => {
                exec_if(song, t);
            },
            TokenType::For => {
                exec_for(song, t);
            },
            TokenType::While => {
                exec_while(song, t);
            },
            TokenType::Break => {
                song.flags.break_flag = 1;
                break;
            },
            TokenType::Continue => {
                song.flags.break_flag = 2;
                break;
            },
            TokenType::Return => {
                let val_tokens = t.children.clone().unwrap();
                let val = exec_value(song, &val_tokens);
                song.variables_insert("Result", val);
                // set return
                song.flags.break_flag = 3;
                break;
            },
            TokenType::DefUserFunction => {
                // nop
            },
            TokenType::CalcTree => {
                if t.operator_flag == '\0' { // dummy calc
                    match &t.children {
                        Some(tokens) => {
                            exec(song, tokens);
                        },
                        None => {},
                    }
                    pos += 1;
                    continue;
                }
                // get flag char
                let flag = t.operator_flag;
                let values = exec_args(song, t.children.as_ref().unwrap_or(&vec![]));
                // only 1 value
                if flag == '!' { // flag "!(val)"
                    let v = if values.len() >= 1 { values[0].to_b() } else { false };
                    song.stack.push(SValue::from_b(!v));
                    pos += 1;
                    continue;
                }
                // 2 values
                // println!("[calc_tree]{}({:?})", flag, values);
                let a = if values.len() >= 1 { values[0].clone() } else { SValue::None };
                let b = if values.len() >= 2 { values[1].clone() } else { SValue::None };
                let mut c = SValue::None;
                match flag {
                    '(' => c = a.clone(), // nop
                    '&' => c = SValue::from_b(a.to_b() && b.to_b()), // logical and
                    '|' => c = SValue::from_b(a.to_b() || b.to_b()), // logical or
                    '=' => c = SValue::from_b(a.eq(b)),
                    '≠' => c = SValue::from_b(a.ne(b)), // !=
                    '>' => c = SValue::from_b(a.gt(b)),
                    '≧' => c = SValue::from_b(a.gteq(b)),
                    '<' => c = SValue::from_b(a.lt(b)),
                    '≦' => c = SValue::from_b(a.lteq(b)),
                    '+' => c = a.add(b),
                    '-' => c = SValue::from_i(a.to_i() - b.to_i()),
                    '*' => c = SValue::from_i(a.to_i() * b.to_i()),
                    '/' => c = a.div(b),
                    '%' => c = SValue::from_i(a.to_i() % b.to_i()),
                    _ => {
                        song.add_log(String::from("[Calc] unknown flag"));
                    }
                }
                song.stack.push(c);
            },
            TokenType::ConstInt => {
                song.stack.push(SValue::from_i(t.value_i));
            },
            TokenType::ConstStr => {
                song.stack.push(SValue::from_s(t.value_s.clone().unwrap_or(String::new())));
            },
            TokenType::Value => {
                // extract value
                // t.value_i ... (ex) LEX_VALUE (lexer.rs) 計算の時に使う
                // t.data ... (ex) [SValue::S("=A")]
                // t.tag ... 関数管理に使う (0: 値 / 1以上: 関数)
                // t.value_type ... 値の種類 tokens::VALUE_XXXX
                // check is variable?
                let val = match t.value_type {
                    TokenValueType::VARIABLE => var_extract(&t.data[0], song),
                    _ => {
                        if t.tag == 0 && t.data.len() > 0 {
                            // exec value
                            let v = var_extract(&t.data[0], song);
                            let vs = v.to_s().clone();
                            // println!("lex={:?}", vs);
                            let tokens = lex(song, &vs, t.lineno);
                            exec(song, &tokens);
                            song.stack.pop().unwrap_or(SValue::None)
                        } else {
                            // user function or system function ref
                            exec_sys_function(song, t);
                            song.stack.pop().unwrap_or(SValue::None)
                        }
                    },
                };
                if song.flags.function_needs_return_value {
                    song.stack.push(val);
                }
            },
            TokenType::ValueInc => {
                let varname = t.value_s.clone().unwrap_or(String::new());
                let val_inc = t.value_i;
                let val = song.variables_get(&varname).unwrap_or(&SValue::Int(0));
                song.variables_insert(&varname, SValue::from_i(val.to_i() + val_inc));
                // let val = song.variables_get(&varname).unwrap_or(&SValue::Int(0));
                // println!("inc={}={}", varname, val.to_i());
            },
            TokenType::MakeArray => {
                match &t.children {
                    None => {
                        song.stack.push(SValue::Array(vec![]));
                        pos += 1;
                        continue;
                    },
                    Some(tokens) => {
                        let mut a: Vec<SValue> = vec![];
                        for tok in tokens {
                            let v = exec_value(song, &vec![tok.clone()]);
                            a.push(v);
                        }
                        song.stack.push(SValue::Array(a));
                    }
                }
            },
            TokenType::SetConfig => {
                let key = t.data[0].to_s();
                let val = &t.data[1];
                if key == "RandomSeed" {
                    song.rand_seed = val.to_i() as u32;
                }
            },
            TokenType::CallUserFunction => {
                exec_userfunc_or_array_or_macro(song, t);
            },
            TokenType::Play => {
                exec_play(song, t);
            },
            TokenType::Rhythm => {},
            TokenType::ControlChangeCommand => {},
            TokenType::FadeIO => {}, // replaced CConTime 
            TokenType::Cresc => {}, // replaced CConTime
            TokenType::SetRandomSeed => {}, // replace SetConfig
            TokenType::DirectSMF => {
                let args = exec_args(song, &t.children.clone().unwrap_or(vec![]));
                if args.len() >= 1 {
                    let timepos = trk!(song).timepos;
                    let args_u8 = args.iter().map(|v| v.to_i() as u8).collect();
                    trk!(song).events.push(Event::direct_smf(timepos, args_u8));
                }
            },
            TokenType::NoteOn => {
                let args = exec_args(song, &t.children.clone().unwrap_or(vec![]));
                if args.len() >= 2 {
                    let timepos = trk!(song).timepos;
                    let mut args_u8: Vec<u8> = args.iter().map(|v| v.to_i() as u8).collect();
                    args_u8.insert(0, 0x90 | trk!(song).channel as u8);
                    trk!(song).events.push(Event::direct_smf(timepos, args_u8));
                }
            },
            TokenType::NoteOff => {
                let args = exec_args(song, &t.children.clone().unwrap_or(vec![]));
                if args.len() >= 2 {
                    let timepos = trk!(song).timepos;
                    let mut args_u8: Vec<u8> = args.iter().map(|v| v.to_i() as u8).collect();
                    args_u8.insert(0, 0x80 | trk!(song).channel as u8);
                    trk!(song).events.push(Event::direct_smf(timepos, args_u8));
                }
            },
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
