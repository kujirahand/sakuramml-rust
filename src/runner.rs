//! runner from tokens
use crate::mml_def::TieMode;
use crate::token::TokenValueType;
use super::source_cursor::SourceCursor;
use super::lexer::lex;
use super::song::{Event, NoteInfo, Song};
use super::svalue::SValue;
use super::token::{Token, TokenType};
use super::sakura_message::MessageKind;

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


/// run tokens
pub fn exec(song: &mut Song, tokens: &Vec<Token>) -> bool {
    let mut pos = 0;
    let mut loop_stack: Vec<LoopItem> = vec![];
    while pos < tokens.len() {
        if song.flags.break_flag != 0 { break; }
        let t = &tokens[pos];
        if song.debug {
            println!("- exec({:03})(line:{}) {}", pos, song.lineno, t.to_debug_str());
        }
        match t.ttype {
            TokenType::Unimplemented => {},
            TokenType::Empty => {},
            TokenType::Comment => {},
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
                if it.index == (it.count - 1) {
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
                    let mut it = loop_stack.pop().unwrap();
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
                let args: Vec<SValue> = exec_args(song, &t.children.clone().unwrap_or(vec![]));
                let e = Event::sysex(trk!(song).timepos, &args, t.value_i == 1);
                song.add_event(e);
            },
            TokenType::SysexReset => {
                let time = trk!(song).timepos;
                let dev = song.device_number as u8;
                match t.value_i {
                    0 => { // GM
                        song.add_event(Event::sysex_raw(time, vec![0x7E, 0x7F, 0x9, 0x1, 0xF7]));
                    }
                    1 => { // GS
                        song.add_event(Event::sysex_raw(time, vec![0x41, dev, 0x42, 0x12, 0x40, 0x00, 0x7F, 0x00, 0x41, 0xF7]));
                    },
                    2 => { // XG
                        song.add_event(Event::sysex_raw(time, vec![0x43, dev, 0x4c, 0x00, 0x00, 0x7e, 0x00, 0xf7]));
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
                        let val = if data.len() >= 1 { data[1].to_i() as u8 & 0x7F } else { 0 };
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
                        let mut val = if data.len() >= 1 { data[1].to_i() } else { 0 };
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
                    None => { runtime_error(song, "[SYSTEM ERROR][DefInt] variable name is empty"); continue; },
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
                    None => { runtime_error(song, "[SYSTEM ERROR][DefStr] variable name is empty"); continue; },
                    Some(var_name) => {
                        let val = exec_value(song, &t.children.clone().unwrap_or(vec![]));
                        song.variables_insert(var_name, val);
                    }
                }
            },
            TokenType::DefArray => {
                match &t.value_s {
                    None => { runtime_error(song, "[SYSTEM ERROR][DefArray] variable name is empty"); continue; },
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
                // tag == 0
                if t.tag == 0 { // dummy calc
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
                let flag = std::char::from_u32(t.tag as u32).unwrap_or('😔');
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
                    '&' => c = SValue::from_b(a.to_b() && b.to_b()), // and
                    '|' => c = SValue::from_b(a.to_b() || b.to_b()), // or
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

fn exec_play(song: &mut Song, t: &Token) -> bool {
    let tmp_cur_track = song.cur_track;
    let lineno = t.lineno;
    let start_pos = trk!(song).timepos;
    let mut time_ptr_last = start_pos;
    // play
    let arg_tokens = t.children.clone().unwrap_or(vec![]);
    for (index, arg) in arg_tokens.iter().enumerate() {
        song.change_cur_track(index + 1);
        trk!(song).timepos = start_pos;
        // eval calc
        let src = exec_value(song, &vec![arg.clone()]).to_s();
        // println!("play(TR={})({}):{}", index+1, lineno, src);
        // eval tokens
        let tokens = lex(song, &src, lineno);
        exec(song, &tokens);
        // check lastpos
        if trk!(song).timepos > time_ptr_last { time_ptr_last = trk!(song).timepos; }
    }
    song.track_sync();
    song.cur_track = tmp_cur_track;
    true
}

fn exec_cc_rpn_nrpn(song: &mut Song, t: &Token, cc1: isize, cc2: isize, cc3: isize) {
    let val = exec_value_int_by_token(song, t);
    let msb = t.data[0].to_i();
    let lsb = t.data[1].to_i();
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc1, msb));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc2, lsb));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc3, val)); 
}

fn exec_cc_rpn_nrpn_direct(song: &mut Song, t: &Token, cc1: isize, cc2: isize, cc3: isize) {
    let args = exec_args(song, t.children.as_ref().unwrap_or(&vec![]));
    if args.len() != 3 {
        runtime_error(song, "RPN/NRPN needs 3 arguments");
        return;
    }
    let msb = args[0].to_i();
    let lsb = args[1].to_i();
    let val = args[2].to_i();
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc1, msb));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc2, lsb));
    song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, cc3, val)); 
}

fn exec_userfunc_or_array_or_macro(song: &mut Song, t: &Token) -> bool {
    // check value is array?
    if t.data.len() > 0 {
        let name = t.data[0].to_s();
        let var = song.variables_get(&name).unwrap_or(&SValue::None).clone();
        match var {
            // is Array
            SValue::Array(a) => {
                // get arg
                let args_tokens = t.children.clone().unwrap();
                let args: Vec<SValue> = exec_args(song, &args_tokens);
                if args.len() == 0 {
                    runtime_error(song, &format!("get Array({}) element needs arguments", name));
                    return false;
                }
                let index = args[0].to_i() as usize;
                if a.len() <= index {
                    runtime_error(song, &format!("Array({}) index out of range", name));
                    return false;
                }
                let v = a[index].clone();
                song.stack.push(v);
                return true;
            },
            // is String macro >>> exec string
            SValue::Str(src, _) => {
                // get arg
                let args_tokens = t.children.clone().unwrap();
                let args: Vec<SValue> = exec_args(song, &args_tokens);
                if args.len() == 0 {
                    runtime_error(song, &format!("get String({}) element needs arguments", name));
                    return false;
                }
                // replace string
                let mut s = src.clone();
                for (i, v) in args.iter().enumerate() {
                    let varname = format!("#?{}", i + 1);
                    s = s.replace(&varname, &v.to_s());
                }
                let tokens = lex(song, &s, t.lineno);
                return exec(song, &tokens);
            },
            _ => {}
        }
    }
    // check func_id
    let func_id = t.tag as usize;
    if song.functions.len() <= func_id {
        runtime_error(song, &format!("broken func_id={} in exec_call_user_function", func_id));
        return false;
    }
    // println!("call_user_function={}::{:?}", song.functions[func_id].name, song.functions[func_id].arg_def_values);
    song.variables_stack_push();
    // eval args
    let args_tokens = t.children.clone().unwrap();
    let args: Vec<SValue> = exec_args(song, &args_tokens);
    // set local variables
    for i in 0..song.functions[func_id].arg_names.len() {
        let varname = &song.functions[func_id].arg_names[i].clone();
        let mut v: SValue = if i < args.len() { args[i].clone() } else { SValue::None };
        v = match v {
            SValue::None => song.functions[func_id].arg_def_values[i].clone(),
            _ => v,
        };
        song.variables_insert(varname, v);
    }
    // eval function
    let tokens = song.functions[func_id].tokens.clone();
    let tmp_break_flag = song.flags.break_flag;
    // println!("func_body={:?}", tokens);
    let eval_result = exec(song, &tokens);
    song.flags.break_flag = tmp_break_flag;
    let vars = song.variables_stack_pop();
    if song.flags.function_needs_return_value {
        let return_val = vars.get("Result");
        song.stack.push(return_val.unwrap_or(&SValue::None).clone());
    }
    eval_result
}

fn exec_sys_function(song: &mut Song, t: &Token) -> bool {
    let args_tokens = t.children.clone().unwrap_or(vec![]);
    let args:Vec<SValue> = exec_args(song, &args_tokens);
    let arg_count = args.len();
    let func_name = if t.data.len() > 0 { t.data[0].to_s() } else { "".to_string() };
    // is user function?
    let func_val = song.variables_get(&func_name).unwrap_or(&SValue::new()).clone();
    match func_val {
        SValue::UserFunc(_func_id) => {
            if exec_userfunc_or_array_or_macro(song, t) { return true; }
        },
        _ => {}, // maybe system function
    }
    //
    // todo: https://sakuramml.com/wiki/index.php?%E7%B5%84%E3%81%BF%E8%BE%BC%E3%81%BF%E9%96%A2%E6%95%B0
    //
    // 参照できるシステム関数
    if func_name == "Random" || func_name == "RANDOM" || func_name == "RandomInt" || func_name == "RND" || func_name == "Rnd" {
        // song.add_log(format!("[Random]({}) {:?}", t.lineno, arg_count, args));
        if arg_count >= 2 {
            let min = args[0].to_i();
            let max = args[1].to_i();
            let rnd = (song.rand() & 0x7FFFFFFF) as isize % (max - min + 1) + min;
            song.stack.push(SValue::from_i(rnd));
        } else if arg_count == 1 {
            let m = args[0].to_i();
            let v = ((song.rand() & 0x7FFFFFFF) as isize) % m;
            song.stack.push(SValue::from_i(v));
        } else if arg_count == 0 {
            let v = song.rand() as isize;
            song.stack.push(SValue::from_i(v));
        }
    }
    else if func_name == "RandomSelect" {
        let r = song.rand() as usize % arg_count;
        song.stack.push(args[r as usize].clone());
    }
    else if func_name == "CHR" || func_name == "Chr" {
        if arg_count >= 1 {
            let val = args[0].to_i();
            let mut s = String::new();
            s.push(std::char::from_u32(val as u32).unwrap_or(' '));
            song.stack.push(SValue::from_s(s));
        } else {
            song.stack.push(SValue::from_str(" "));
        }
    }
    else if func_name == "MID" || func_name == "Mid" {
        if arg_count >= 3 {
            let val = args[0].to_s();
            let i_from = args[1].to_i() as usize;
            let i_len = args[2].to_i() as usize;
            // println!("MID={},{},{}", val, i_from, i_len);
            let s = vb_mid(&val, i_from, i_len).unwrap_or("");
            // println!("MID={}", s);
            song.stack.push(SValue::from_str(s));
        } else {
            song.stack.push(SValue::from_str("(MID:ERROR)"));
        }
    }
    else if func_name == "REPLACE" || func_name == "Replace" {
        if arg_count >= 3 {
            let val = args[0].to_s();
            let s_from = args[1].to_s();
            let s_to = args[2].to_s();
            let s = val.replace(&s_from, &s_to);
            song.stack.push(SValue::from_str(&s));
        } else {
            song.stack.push(SValue::from_str("(REPLACE:ERROR)"));
        }
    }
    else if func_name == "SizeOf" || func_name == "SIZEOF" {
        if arg_count >= 1 {
            let v = match &args[0] {
                SValue::Array(a) => a.len(),
                SValue::Str(s, _) => s.len(),
                SValue::IntArray(a) => a.len(),
                SValue::StrArray(a) => a.len(),
                _ => 0
            };
            song.stack.push(SValue::from_i(v as isize));
        }
    }
    else {
        // macro ("=var_name")
        let func_name2 = if func_name.len() >= 2 { func_name[1..].to_string() } else { func_name };
        let args = t.children.clone().unwrap_or(vec![]);
        let args = exec_args(song, &args);
        let val = song.variables_get(&func_name2).unwrap_or(&SValue::new()).clone();
        let mut val_s = val.to_s();
        for (index, arg) in args.iter().enumerate() {
            let macro_n = format!("#?{}", index+1);
            let target = arg.clone().to_s();
            val_s = val_s.replace(&macro_n, &target);
        }
        // println!("macro={}//{:?}", val_s, t);
        if song.flags.function_needs_return_value {
            song.stack.push(SValue::from_s(val_s));
        } else {
            // exec macro
            let tokens = lex(song, &val_s, t.lineno);
            exec(song, &tokens);
        }
    }
    true
}

fn vb_mid(input: &str, start: usize, length: usize) -> Option<&str> {
    let input_len = input.len();
    let start = if start >= 1 { start - 1 } else { 0 };
    let mut end = start + length;
    if end >= input_len { end = input_len; }
    Some(&input[start..end])
}

fn exec_if(song: &mut Song, t: &Token) -> bool {
    let children = match &t.children {
        Some(tokens) => tokens,
        None => return false,
    };
    if children.len() < 3 {
        return false;
    }
    let cond_token = &children[0];
    let true_token = &children[1];
    let false_token = &children[2];
    // eval cond
    let cond = cond_token.children.clone().unwrap();
    let cond_val = exec_value(song, &cond);
    // exec true or false
    if cond_val.to_i() != 0 {
        let tokens = true_token.children.clone().unwrap();
        exec(song, &tokens);
    } else {
        let tokens = false_token.children.clone().unwrap();
        exec(song, &tokens);
    }
    true
}

fn exec_while(song: &mut Song, t: &Token) -> bool {
    let children = match &t.children {
        Some(tokens) => tokens,
        None => return false,
    };
    if children.len() < 2 {
        return false;
    }
    let cond_token = &children[0];
    let body_token = &children[1];
    let mut counter = 0;
    // loop
    loop {
        // eval cond
        let cond = cond_token.children.clone().unwrap();
        let cond_val = exec_value(song, &cond);
        if cond_val.to_b() == false {
            break;
        }
        // exec body
        let body = body_token.children.clone().unwrap();
        exec(song, &body);
        // check counter
        counter += 1;
        if counter > song.flags.max_loop {
            song.add_log(format!(
                "[ERROR]({}) {} WHILE(>{})", t.lineno, 
                song.get_message(MessageKind::LoopTooManyTimes),
                song.flags.max_loop
            ));
            break;
        }
        // check break flag
        match song.flags.break_flag {
            1 => {
                song.flags.break_flag = 0;
                break;
            },
            2 => {
                song.flags.break_flag = 0;
                continue;
            },
            3 => {
                break;
            }
            _ => {},
        }
    }
    true
}

fn exec_for(song: &mut Song, t: &Token) -> bool {
    let children = match &t.children {
        Some(tokens) => tokens,
        None => return false,
    };
    if children.len() < 4 {
        return false;
    }
    let init_token = &children[0];
    let cond_token = &children[1];
    let inc_token = &children[2];
    let body_token = &children[3];
    // eval init
    let init = init_token.children.clone().unwrap();
    exec(song, &init);
    let mut counter = 0;
    // loop
    loop {
        // eval cond
        let cond = cond_token.children.clone().unwrap();
        let cond_val = exec_value(song, &cond);
        if cond_val.to_b() == false {
            break;
        }
        // exec body
        let body = body_token.children.clone().unwrap();
        exec(song, &body);
        // check loop counter
        counter += 1;
        if counter > song.flags.max_loop {
            song.add_log(format!(
                "[ERROR]({}) {} FOR(>{})", t.lineno, 
                song.get_message(MessageKind::LoopTooManyTimes),
                song.flags.max_loop
            ));
            break;
        }
        // inc
        let inc_tokens = inc_token.children.clone().unwrap();
        // check break or continue
        if song.flags.break_flag == 1 { // break
            song.flags.break_flag = 0;
            break;
        }
        if song.flags.break_flag == 2 { // continue
            song.flags.break_flag = 0;
            exec(song, &inc_tokens); // eval inc
            continue;
        }
        // eval inc
        exec(song, &inc_tokens); // eval inc
    }
    true
}

fn get_system_value(cmd: &str, song: &Song) -> Option<SValue> {
    // <SYSTEM_REF>
    if cmd == "TR" || cmd == "TRACK" || cmd == "Track" { // @ get current track no - 現在のトラック番号を得る
        let tr = song.cur_track as isize;
        return Some(SValue::from_i(tr));
    }
    if cmd == "CH" || cmd == "CHANNEL" { // @ get current channel no - 現在のチャンネル番号を得る
        let ch = trk!(song).channel + 1; // range: 1-16
        return Some(SValue::from_i(ch));
    }
    if cmd == "TIME" || cmd == "Time" || cmd == "TIMEPOS" || cmd == "TIMEPTR" { // @ get time posision - 現在のタイムポインタ値を得る
        let v = trk!(song).timepos;
        return Some(SValue::from_i(v));
    }
    if cmd == "TEMPO" || cmd == "Tempo" || cmd == "BPM" { // @ get tempo - 現在のテンポ値を得る
        let v = song.tempo;
        return Some(SValue::from_i(v));
    }
    if cmd == "KEY" || cmd == "KEY_SHIFT" { // @ get key shift - 現在のキーシフト値を得る
        let v = song.key_shift;
        return Some(SValue::from_i(v));
    }
    if cmd == "TR_KEY" || cmd == "TrackKey" { // @ get track key shift - 現在のトラックごとのキーシフト値を得る
        let v = trk!(song).track_key;
        return Some(SValue::from_i(v));
    }
    if cmd == "TIMEBASE" || cmd == "Timebase" { // @ get timebase - 現在のタイムベース値を得る
        let v = song.timebase;
        return Some(SValue::from_i(v));
    }
    if cmd == "l" { // @ get length - 現在のlの値を得る
        let v = trk!(song).length;
        return Some(SValue::from_i(v));
    }
    if cmd == "v" { // @ get velocity - 現在のvの値を得る
        let v = trk!(song).velocity;
        return Some(SValue::from_i(v));
    }
    if cmd == "q" { // @ get gate rate - 現在のqの値を得る
        let v = trk!(song).qlen;
        return Some(SValue::from_i(v));
    }
    if cmd == "o" { // @ get octave rate - 現在のoの値を得る
        let v = trk!(song).octave;
        return Some(SValue::from_i(v));
    }
    // </SYSTEM_REF>
    None
}

fn var_extract(val: &SValue, song: &mut Song) -> SValue {
    match val {
        // String
        SValue::Str(s, _) => {
            if s.starts_with('=') && s.len() >= 2 {
                let key = &s[1..];
                match song.variables_get(key) {
                    Some(v) => v.clone(),
                    None => {
                        match get_system_value(key, song) {
                            Some(v) => return v,
                            None => {
                                let err_msg = format!("[WARN]({}) Undefined: {}", song.lineno, key);
                                song.add_log(err_msg);
                                SValue::None
                            },
                        }
                    }
                }
            } else {
                SValue::from_str(&s)
            }
        },
        // Other value
        _ => val.clone(),
    }
}

fn tempo_change_a_to_b(song: &mut Song, a: isize, b: isize, len: isize) {
    let step = (song.timebase * 4) / 16;
    let step_cnt = len / step;
    let width = b - a;
    let timepos = trk!(song).timepos;
    for i in 0..step_cnt {
        let v = (a as f32) + (width as f32) * (i as f32 / step_cnt as f32);
        tempo_change(song, v as isize);
        trk!(song).timepos += step;
    }
    trk!(song).timepos = timepos + len;
    tempo_change(song, b);
    trk!(song).timepos = timepos;
}

fn tempo_change(song: &mut Song, tempo: isize) {
    song.tempo = tempo;
    let mpq = if tempo > 0 { 60000000 / tempo } else { 120 };
    let e = Event::meta(
        trk!(song).timepos,
        0xFF,
        0x51,
        0x03,
        vec![
            (mpq >> 16 & 0xFF) as u8,
            (mpq >> 8 & 0xFF) as u8,
            (mpq >> 0 & 0xFF) as u8,
        ],
    );
    song.add_event(e);
}

fn exec_sub(song: &mut Song, t: &Token) {
    let timepos_tmp: isize;
    {
        let trk = &song.tracks[song.cur_track];
        timepos_tmp = trk.timepos;
    }
    {
        let _ = match &t.children {
            Some(tokens) => exec(song, tokens),
            None => false,
        };
    }
    {
        let trk = &mut song.tracks[song.cur_track];
        trk.timepos = timepos_tmp;
    }
}

fn exec_div(song: &mut Song, t: &Token) {
    let len_s = &t.data[0].to_s();
    let cnt = t.value_i;
    let length_org: isize;
    let timepos_end: isize;
    {
        let trk = &mut song.tracks[song.cur_track];
        let div_len = calc_length(len_s, song.timebase, trk.length);
        let note_len = if cnt > 0 { div_len / cnt } else { 0 };
        timepos_end = trk.timepos + div_len;
        length_org = trk.length;
        trk.length = note_len;
    }
    let _ = match &t.children {
        None => false,
        Some(tokens) => exec(song, tokens),
    };
    // clean
    {
        let trk = &mut song.tracks[song.cur_track];
        trk.timepos = timepos_end;
        trk.length = length_org;
    }
}

fn exec_harmony(song: &mut Song, t: &Token, flag_begin: bool) {
    // begin
    if flag_begin {
        song.flags.harmony_flag = true;
        song.flags.harmony_time = song.tracks[song.cur_track].timepos;
        return;
    }
    // end
    if song.flags.harmony_flag {
        song.flags.harmony_flag = false;
        // get harmony length
        let note_len_s = t.data[0].to_s();
        let mut note_qlen = t.data[1].to_i();
        let note_vel = t.data[2].clone();
        // parameters
        if note_qlen < 0 {
            note_qlen = trk!(song).qlen;
        }
        let note_len = calc_length(&note_len_s, song.timebase, trk!(song).length);
        // change event length
        while song.flags.harmony_events.len() > 0 {
            let mut e = song.flags.harmony_events.pop().unwrap();
            e.time = song.flags.harmony_time;
            if note_qlen != 0 {
                e.v2 = note_len * note_qlen / 100;
            }
            if !note_vel.is_none() {
                e.v3 = note_vel.to_i();
            }
            trk!(song).events.push(e);
        }
        trk!(song).timepos = song.flags.harmony_time + note_len;
        return;
    }
}

fn exec_get_time(song: &mut Song, t: &Token, cmd: &str) -> isize{
    // Calc Time (SakuraObj_time2step)
    // (ref) https://github.com/kujirahand/sakuramml-c/blob/68b62cbc101669211c511258ae1cf830616f238e/src/k_main.c#L473
    let args = exec_args(song, t.children.as_ref().unwrap_or(&Vec::new()));
    if args.len() == 0 {
        runtime_error(song, &format!("[{}] no arguments", cmd));
        return 0;
    }
    if args.len() == 1 {
        return args[0].to_i();
    }
    if args.len() < 3 {
        runtime_error(song, &format!("[{}] needs 1 or 3 arguments", cmd));
        return 0;
    }
    let mes = args[0].to_i() + song.flags.measure_shift;
    let beat = args[1].to_i();
    let tick = args[2].to_i();

    // calc
    let base = song.timebase * 4 / song.timesig_deno;
    let total = (mes - 1) * (base * song.timesig_frac) + (beat - 1) * base + tick;
    total
}

/// Calculate note length
pub fn calc_length(len_str: &str, timebase: isize, def_len: isize) -> isize {
    let mut res = def_len;
    if len_str == "" {
        return def_len;
    }
    let mut cur = SourceCursor::from(len_str);
    let mut step_mode = false;
    if cur.eq_char('%') {
        cur.next();
        step_mode = true;
    }
    if cur.is_numeric() || cur.eq_char('-') {
        if step_mode {
            res = cur.get_int(0);
        } else {
            let i = cur.get_int(4);
            res = if i > 0 { timebase * 4 / i } else { 0 };
        }
    }
    if cur.peek_n(0) == '.' {
        if cur.eq("....") {
            cur.next_n(4);
            res += (res as f32 / 2.0 + res as f32 / 4.0 + res as f32 / 8.0 + res as f32 / 16.0) as isize;
        } else if cur.eq("...") { // triple dotted note (三付点音符)
            cur.next_n(3);
            res += (res as f32 / 2.0 + res as f32 / 4.0 + res as f32 / 8.0) as isize;
        } else if cur.eq("..") { // double dotted note (複付点音符)
            cur.next_n(2);
            res += (res as f32 / 2.0 + res as f32 / 4.0) as isize;
        } else { // dotted note
            cur.next();
            res += (res as f32 / 2.0) as isize;
        }
    }
    while !cur.is_eos() {
        let c = cur.peek_n(0);
        if (c != '^') && (c != '+') {
            break;
        }
        cur.next(); // skip '^'
        if cur.eq_char('%') {
            step_mode = true;
            cur.next();
        }
        if cur.is_numeric() || cur.eq_char('-') {
            let mut n = if step_mode {
                cur.get_int(0)
            } else {
                let i = cur.get_int(4);
                if i == 0 {
                    def_len
                } else {
                    timebase * 4 / i
                }
            };
            if cur.eq("....") {
                cur.next_n(4);
                n += (n as f32 / 2.0 + n as f32 / 4.0 + n as f32 / 8.0 + n as f32 / 16.0) as isize;
            } else if cur.eq("...") {
                cur.next_n(3);
                n += (n as f32 / 2.0 + n as f32 / 4.0 + n as f32 / 8.0) as isize;
            } else if cur.eq("..") {
                cur.next_n(2);
                n += (n as f32 / 2.0 + n as f32 / 4.0) as isize;
            } else if cur.peek_n(0) == '.' {
                cur.next();
                n = (n as f32 * 1.5) as isize;
            }
            res += n;
        } else {
            res += def_len;
        }
    }
    res
}

fn get_note_info_from_token(t: &Token) -> NoteInfo {
    let data = &t.data;
    if data.len() < 8 { // broken note
        return NoteInfo {
            no: 0,
            flag: 0,
            natural: 0,
            len_s: "".to_string(),
            qlen: 0,
            vel: 0,
            t: 0,
            o: 0,
            slur: 0,
        };
    }
    let note_no = (t.value_i % 12) as isize;
    let data_note_flag = data[0].to_i();
    let data_note_natural = data[1].to_i();
    let data_note_len = data[2].to_s();
    let data_note_qlen = data[3].to_i(); // 0
    let data_note_vel = data[4].to_i(); // -1
    let data_note_t = data[5].to_i(); // isize::MIN
    let data_note_o = data[6].to_i(); // -1
    let data_slur = data[7].to_i(); // 0 or 1 --- TODO: #7
    NoteInfo {
        no: note_no,
        flag: data_note_flag,
        natural: data_note_natural,
        len_s: data_note_len,
        qlen: data_note_qlen,
        vel: data_note_vel,
        t: data_note_t,
        o: data_note_o,
        slur: data_slur,
    }
}

fn set_note_info_with_default_value(note: &mut NoteInfo, song: &mut Song) {
    // set note with track's default value
    if note.qlen == 0 {
        note.qlen = trk!(song).qlen;
    }
    if note.vel < 0 {
        note.vel = trk!(song).velocity;
    }
    if note.t == isize::MIN {
        note.t = trk!(song).timing;
    }
    if note.o < 0 {
        note.o = trk!(song).octave;
    }
    // calc note no
    let mut noteno = note.o * 12 + note.no + note.flag;
    // key_shift / key_flag / track_key
    if song.use_key_shift {
        noteno += if note.natural == 0 {
            song.key_flag[note.no as usize]
        } else {
            0
        };
        noteno += song.key_shift;
        noteno += trk!(song).track_key;
    }
    note.no = noteno;
}

fn exec_note(song: &mut Song, t: &Token) {
    // get note parameters
    let mut note = get_note_info_from_token(t);
    set_note_info_with_default_value(&mut note, song);
    // timepos
    let timepos = trk!(song).timepos;
    let start_pos = timepos;
    // onTime / onNote
    let v = trk!(song).calc_v_on_time(note.vel);
    let v = trk!(song).calc_v_on_note(v);
    let t = trk!(song).calc_t_on_note(note.t);
    let qlen = trk!(song).calc_qlen_on_note(note.qlen);
    let o_abs = trk!(song).calc_o_on_note(-1);
    if o_abs != -1 {// ノートはそのままでオクターブだけ変える
        note.no = note.no % 12 + o_abs * 12; // set absolute octave
    }
    // Random
    if trk!(song).o_rand > 0 { // octave randomize
        let r = song.calc_rand_value(0, trk!(song).o_rand);
        if r != 0 {
            note.no += r * 12;
        }
    }
    let v = if trk!(song).v_rand > 0 {
        song.calc_rand_value(v, trk!(song).v_rand)
    } else {
        v
    };
    let t = if trk!(song).t_rand > 0 {
        song.calc_rand_value(t, trk!(song).t_rand)
    } else {
        t
    };
    let qlen = if trk!(song).q_rand > 0 {
        song.calc_rand_value(qlen, trk!(song).q_rand)
    } else {
        qlen
    };
    // note len
    let mut notelen = calc_length(&note.len_s, song.timebase, trk!(song).length);
    // note len onNote / onCycle
    let notelen_on_note = trk!(song).calc_l_on_note(-1);
    if notelen_on_note != -1 { // onNote / onCycle の値があれば強制的に上書き
        notelen = notelen_on_note;
    }
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    // check range
    let v = value_range(0, v, 127);
    // event
    let event = Event::note(timepos + t, trk!(song).channel, note.no, notelen_real, v);
    // println!("- {}: note(no={},len={},qlen={},v={},t={},o={})", trk.timepos, noteno, notelen_real, qlen, v, t, o);
    trk!(song).timepos += notelen;

    // octave_once?
    if song.flags.octave_once != 0 {
        trk!(song).octave = trk!(song).octave - song.flags.octave_once;
        song.flags.octave_once = 0;
    }

    // harmony?
    if song.flags.harmony_flag {
        trk!(song).timepos = song.flags.harmony_time;
        song.flags.harmony_events.push(event);
        return;
    }
    // tie or slur?
    if note.slur == 1 {
        trk!(song).tie_notes.push(event);
        return;
    }
    if trk!(song).tie_notes.len() > 0 {
        trk!(song).tie_notes.push(event);
        check_tie_notes(song);
        return;
    }
    // onNote event
    trk!(song).write_cc_on_note(start_pos);
    // onNoteWave event
    trk!(song).write_cc_on_note_wave(start_pos);
    // write note event
    trk!(song).events.push(event);
}

/// TieMode::Port
fn tie_mode_port(song: &mut Song) {
    let mut last_note = trk!(song).tie_notes.remove(0);
    let mut tie_value = trk!(song).tie_value;
    loop {
        if trk!(song).tie_notes.len() == 0 {
            trk!(song).events.push(last_note);
            break;
        }
        let next_event = trk!(song).tie_notes.remove(0);
        // same note no
        if last_note.v1 == next_event.v1 {
            // add note length
            let time_pos = next_event.time + next_event.v2;
            last_note.v2 = time_pos - last_note.time;
            continue;
        }
        // check bend range in track
        let mut bend_range = trk!(song).bend_range;
        if bend_range <= 0 {
            // set bend range
            trk!(song).bend_range = 12;
            let timepos = if last_note.time <= 0 { 0 } else { last_note.time - 1 };
            let bend_range_event = Event::pitch_bend_range(timepos, trk!(song).channel, 12);
            trk!(song).events.push(bend_range_event);
            bend_range = 12;
        }
        // calc pitch range
        // bend value range: -8192 to 8191
        let note_diff: isize = next_event.v1 - last_note.v1;
        tie_value = if tie_value == 0 { (song.timebase * 4) / 8 } else { tie_value };
        let bend_from = (note_diff as f32 * (8192f32 / bend_range as f32)) as isize;
        let bend_to = 0;
        let mut last_v = 0;
        for i in 0..tie_value {
            let timepos = next_event.time - tie_value + i;
            let v = ((bend_from - bend_to) as f32 * (i as f32 / tie_value as f32)) as isize;
            if last_v == v { continue; }
            last_v = v;
            let bend_event = Event::pitch_bend(timepos, trk!(song).channel, v + 8192);
            trk!(song).events.push(bend_event);
        }
        last_note.v2 = next_event.time - last_note.time;
        trk!(song).events.push(last_note);
        let bend_event_end = Event::pitch_bend(next_event.time, trk!(song).channel, bend_to + 8192);
        trk!(song).events.push(bend_event_end);
        last_note = next_event;
    }
}

fn tie_mode_bend(song: &mut Song) {
    // first note
    let mut last_note = trk!(song).tie_notes.remove(0);
    let mut begin_note = last_note.clone();
    // set bend range
    let mut bend_range = trk!(song).bend_range;
    if bend_range <= 0 {
        trk!(song).bend_range = 12;
        let timepos = if last_note.time <= 0 { 0 } else { last_note.time - 1 };
        let bend_range_event = Event::pitch_bend_range(timepos, trk!(song).channel, 12);
        trk!(song).events.push(bend_range_event);
        bend_range = 12;
    }
    // set bend 0
    let bend0 = Event::pitch_bend(last_note.time, trk!(song).channel, 8192);
    trk!(song).events.push(bend0);
    let mut lastpos = last_note.time + last_note.v2;
    while trk!(song).tie_notes.len() > 0 {
        let next_event = trk!(song).tie_notes.remove(0);
        lastpos = next_event.time + next_event.v2;
        // same note no
        if last_note.v1 == next_event.v1 {
            // add note length
            let time_pos = next_event.time + next_event.v2;
            last_note.v2 = time_pos - last_note.time;
            continue;
        }
        // calc pitch range
        // bend value range: -8192 to 8191
        let note_diff: isize = next_event.v1 - last_note.v1;
        let bend_event = Event::pitch_bend(
            next_event.time,
            trk!(song).channel,
            (note_diff as f32 * 8192f32 / bend_range as f32) as isize + 8192,
        );
        trk!(song).events.push(bend_event);
    }
    // write begin note
    begin_note.v2 = lastpos - begin_note.time;
    trk!(song).events.push(begin_note);
    // reset bend
    let bend_end = Event::pitch_bend(lastpos, trk!(song).channel, 8192);
    trk!(song).events.push(bend_end);
}

fn tie_mode_gate(song: &mut Song) {
    let mut last_note = trk!(song).tie_notes.remove(0);
    let tie_value = trk!(song).tie_value;
    loop {
        if trk!(song).tie_notes.len() == 0 {
            trk!(song).events.push(last_note);
            break;
        }
        let next_event = trk!(song).tie_notes.remove(0);
        // same note no
        if last_note.v1 == next_event.v1 {
            // add note length
            let time_pos = next_event.time + next_event.v2;
            last_note.v2 = time_pos - last_note.time;
            continue;
        }
        // different note no
        if tie_value == 0 {
            last_note.v2 = next_event.time - last_note.time;
        } else {
            last_note.v2 = tie_value;
        }
        trk!(song).events.push(last_note);
        last_note = next_event;
    }
}

/// alpeggio mode
fn tie_mode_alpe(song: &mut Song) {
    let last_note = &trk!(song).tie_notes[trk!(song).tie_notes.len() - 1];
    let last_pos = last_note.time + last_note.v2;
    let tie_notes = trk!(song).tie_notes.clone();
    for mut event in tie_notes.into_iter() {
        event.v2 = last_pos - event.time;
        trk!(song).events.push(event);
    }
}

fn check_tie_notes(song: &mut Song) {
    // Tie/Slur mode (https://sakuramml.com/doc/command/11.htm)
    //
    // "Slur(type, value)"で、タイ記号"&(value)"の異音程(スラー)の動作を変更する。
    // type	typeの概略	動作	valueの意味	rangeの意味	使い方例
    // 0	グリッサンド	異音程をベンドで滑らかにつなぐ(※1)(※2)（※3）	グリッサンドの長さ	ベンドレンジを指定。省略可。	@81 l4 Slur(0,!8) c&e&g
    // 1	ベンド	異音程をベンドで表現。ギターのハンマリングに近い。(※1)	無効	無効	@25 l8 Slur(1,0) cdc c&d&c g&f&e&d
    // 2	ゲート	＆のついた音符のゲートを、valueにする	ゲートの長さ	無効	@81 l8 Slur(2,100) q50 c&d e&f g&f e&d
    // 3	アルペジオ	＆でつないだ音符の終わりまでゲートを伸ばす。そのとき、value に、ノートの最大発音音数を指定できる。	最大発音音数	無効	l16 Slur(3,100) c&e&g d&f&a
    if trk!(song).tie_notes.len() == 0 {
        return;
    }
    match trk!(song).tie_mode {
        TieMode::Port => tie_mode_port(song),
        TieMode::Bend => tie_mode_bend(song),
        TieMode::Gate => tie_mode_gate(song),
        TieMode::Alpe => tie_mode_alpe(song),
    };
}

fn exec_note_n(song: &mut Song, t: &Token) {
    // parameters
    let data_note_no = var_extract(&t.data[0], song).to_i();
    let data_note_len = var_extract(&t.data[1], song).to_s();
    let data_note_qlen = var_extract(&t.data[2], song).to_i(); // 0
    let data_note_vel = var_extract(&t.data[3], song).to_i(); // -1
    let data_note_t = var_extract(&t.data[4], song).to_i(); // isize::MIN
    let start_pos = trk!(song).timepos;

    // check parameters
    let notelen = calc_length(&data_note_len, song.timebase, trk!(song).length);
    let qlen = if data_note_qlen != 0 {
        data_note_qlen
    } else {
        trk!(song).qlen
    };
    let v = if data_note_vel >= 0 {
        data_note_vel
    } else {
        trk!(song).velocity
    };
    let t = if data_note_t != isize::MIN {
        data_note_t
    } else {
        trk!(song).timing
    };
    // onTime / onNote
    let v = trk!(song).calc_v_on_time(v);
    let v = trk!(song).calc_v_on_note(v);
    let t = trk!(song).calc_t_on_note(t);
    let qlen = trk!(song).calc_qlen_on_note(qlen);
    // Random
    let v = if trk!(song).v_rand > 0 {
        song.calc_rand_value(v, trk!(song).v_rand)
    } else {
        v
    };
    let t = if trk!(song).t_rand > 0 {
        song.calc_rand_value(t, trk!(song).t_rand)
    } else {
        t
    };
    let qlen = if trk!(song).q_rand > 0 {
        song.calc_rand_value(qlen, trk!(song).q_rand)
    } else {
        qlen
    };
    // calc
    let notelen_real = (notelen as f32 * qlen as f32 / 100.0) as isize;
    // range
    let v = value_range(0, v, 127);
    let event = Event::note(
        trk!(song).timepos + t,
        trk!(song).channel,
        data_note_no,
        notelen_real,
        v,
    );
    // println!("- {}: note(no={},len={},qlen={},v={},t={})", trk!(song).timepos, notelen_real, notelen, qlen, v, t);
    // onNoteWave event
    trk!(song).write_cc_on_note_wave(start_pos);
    // write event
    trk!(song).events.push(event);
    trk!(song).timepos += notelen;
}

fn exec_rest(song: &mut Song, t: &Token) {
    let trk = &mut song.tracks[song.cur_track];
    let data_note_len = t.data[0].to_s();
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    trk.timepos += notelen * t.value_i;
}

fn exec_voice(song: &mut Song, t: &Token) {
    // voice no
    let args = exec_args(song, t.children.as_ref().unwrap_or(&vec![]));
    let no = if args.len() >= 1 { args[0].to_i() } else { 1 };
    let no = value_range(1, no, 128) - 1;
    let bank_msb = if args.len() >= 2 { args[1].to_i() } else { 0 };
    let bank_lsb = if args.len() >= 3 { args[2].to_i() } else { 0 };
    // bank ?
    if args.len() == 1 {
        song.add_event(Event::voice(trk!(song).timepos, trk!(song).channel, no));
    } else {
        song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, 0x00, bank_msb)); // msb
        song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, 0x20, bank_lsb)); // lsb
        song.add_event(Event::voice(trk!(song).timepos, trk!(song).channel, no));
        // println!("voice: no={}, bank_msb={}, bank_lsb={}", no, bank_msb, bank_lsb);
    }
}

fn exec_decres(song: &mut Song, t: &Token) {
    let mut len_s = t.data[0].to_s();
    if len_s == "" { len_s = "1".to_string(); }
    let v1 = t.data[1].to_i();
    let v2 = t.data[2].to_i();
    let len = calc_length(&len_s, song.timebase, trk!(song).length);
    let ia = vec![v1, v2, len];
    // write EP
    trk!(song).write_cc_on_time(11, ia);
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

#[cfg(test)]
mod tests {
    use crate::song::EventType;

    use super::*;
    #[test]
    fn test_calc_len() {
        assert_eq!(calc_length("4", 480, 480), 480);
        assert_eq!(calc_length("", 480, 480), 480);
        assert_eq!(calc_length("8", 480, 480), 240);
        assert_eq!(calc_length("8^", 480, 240), 480);
        assert_eq!(calc_length("^^^", 480, 240), 480 * 2);
        assert_eq!(calc_length("4.", 480, 480), 480 + 240);
        assert_eq!(calc_length("4.^", 480, 240), 240 * 4);
    }
    #[test]
    fn test_calc_len2() {
        assert_eq!(calc_length("4", 96, 48), 96);
        assert_eq!(calc_length("", 96, 48), 48);
        assert_eq!(calc_length("^", 96, 48), 96);
        assert_eq!(calc_length("^4", 96, 48), 48 + 96);
    }
    #[test]
    fn test_calc_len3() {
        assert_eq!(calc_length("2", 48, 48), 96);
        assert_eq!(calc_length("4^4", 48, 48), 96);
        assert_eq!(calc_length("4.^8", 48, 48), 96);
        assert_eq!(calc_length("8^4.", 48, 48), 96);
    }
    #[test]
    fn test_calc_len_step() {
        assert_eq!(calc_length("%96", 96, 96), 96);
        assert_eq!(calc_length("4^%1", 96, 96), 97);
        assert_eq!(calc_length("^%2", 96, 96), 98);
        assert_eq!(calc_length("^%-1", 96, 48), 47);
    }
    #[test]
    fn test_calc_len_plus() {
        assert_eq!(calc_length("4+4", 96, 96), 96 * 2);
        assert_eq!(calc_length("8+", 96, 48), 96);
    }
    #[test]
    fn test_calc_len_dot() {
        assert_eq!(calc_length(".", 96, 96), 96 + 48);
    }
    #[test]
    fn test_exec1() {
        assert_eq!(exec_easy("PRINT({1})").get_logs_str(), "[PRINT](0) 1");
        assert_eq!(exec_easy("PRINT({abc})").get_logs_str(), "[PRINT](0) abc");
        assert_eq!(
            exec_easy("STR A={ddd} PRINT(A)").get_logs_str(),
            "[PRINT](0) ddd"
        );
    }
   #[test]
    fn test_def_var() {
        // define variable
        let song = exec_easy("INT N=333;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 333");
        // define variable
        let song = exec_easy("INT N; N=333; PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 333");
    }
    #[test]
    fn test_exec_harmony() {
        let song = exec_easy("q100 l8 'dg'^^^");
        let e = &song.tracks[0].events[0];
        assert_eq!(e.etype, EventType::NoteOn);
        assert_eq!(e.v2, 96 * 2);
    }
    #[test]
    fn test_exec_track_sync() {
        //
        let song = exec_easy("TR=1 l4 cdef TR=2 c TrackSync;");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96);
        //
        let song = exec_easy("TR=0 l4 c TR=2 cdef TrackSync;");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96 * 4);
    }
    #[test]
    fn test_exec_mes_shift() {
        //
        let song = exec_easy("System.MeasureShift=1;TR=0 TIME(1:1:0)");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, 96 * 4);
    }
    #[test]
    fn test_lex_macro_str() {
        //
        let song = exec_easy("#A={o#?1} #A(0) c");
        assert_eq!(song.tracks[0].events[0].v1, 0);
        //
        let song = exec_easy("STR AAA={o#?1} AAA(0) d");
        assert_eq!(song.tracks[0].events[0].v1, 2);
        //
        let song = exec_easy("STR BBB={o0 #?1 #?2 #?3} BBB({c},{d},{e})");
        assert_eq!(song.tracks[0].events[0].v1, 0);
        assert_eq!(song.tracks[0].events[1].v1, 2);
        assert_eq!(song.tracks[0].events[2].v1, 4);
    }
    #[test]
    fn test_exec_for() {
        let song = exec_easy("INT N=0;FOR(I=1;I<=10;I++){N=N+I;} PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 55");
        // break
        let song = exec_easy("INT N=0;FOR(I=1;I<=10;I++){IF(I==3){BREAK} N=N+I;} PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 3");
        // continue
        let song = exec_easy("INT N=0;FOR(I=1;I<=10;I++){IF(I>=3){CONTINUE} N=N+I;} PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 3");
    }
    #[test]
    fn test_exec_while() {
        let song = exec_easy("INT N=0;INT I=1;WHILE(I<=10){N=N+I;I++;} PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 55");
        // break
        let song = exec_easy("INT N=0;INT I=1;WHILE(I<=10){IF(I=3){BREAK}N=N+I;I++;} PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 3");
    }
   #[test]
    fn test_exec_calc() {
        // 1+2*3
        let song = exec_easy("INT N=1+2*3;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 7");
        // (1+2)*3
        let song = exec_easy("INT N=(1+2)*3;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 9");
        // 1>2 false(0)
        let song = exec_easy("PRINT(1>2)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) FALSE");
        // 6/3
        let song = exec_easy("INT N=6/3;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 2");
        // 4/0
        let song = exec_easy("INT N=4/0;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 0");
    }
   #[test]
    fn test_exec_function() {
        // simple call
        let song = exec_easy(&format!("{}\n{}\n{}\n{}\n{}",
            "FUNCTION FOO(A,B){",
            "  INT C=A+B;",
            "  PRINT(C);",
            "}",
            "FOO(3,5)"
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](2) 8");
        // with return
        let song = exec_easy(&format!("{}\n{}\n{}\n{}\n",
            "FUNCTION FOO(A,B){",
            "  RETURN(A+B);",
            "}",
            "PRINT(FOO(3,8));"
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](3) 11");
        // use global variable
        let song = exec_easy(&format!("{}\n{}\n{}\n{}\n{}\n{}\n",
            "INT C=100",
            "FUNCTION FOO(TMP){",
            "  INT C=TMP;",
            "  PRINT(C);",
            "}",
            "FOO(1); PRINT(C);"
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](3) 1\n[PRINT](5) 100");
        // use global variable
        let song = exec_easy(&format!("{}\n{}\n{}\n{}\n",
            "INT C=123",
            "FUNCTION FOO(TMP){ INT C=TMP; Result=TMP; }",
            "FUNCTION BAA(TMP){ INT C=TMP; RETURN(C);  }",
            "PRINT(FOO(100)); PRINT(BAA(200)); PRINT(C);",
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](3) 100\n[PRINT](3) 200\n[PRINT](3) 123");
        // use global variable and return into for-loop
        let song = exec_easy(&format!("{}\n{}\n{}\n{}\n{}\n",
            "PRINT(FOO());",
            "FUNCTION FOO(){",
            "  INT C=0; FOR(INT I=0; I<=3; I++){ IF(I==2){ RETURN(C); } ELSE { C=I; } }",
            "  RETURN(100);",
            "}",
        ));
        assert_eq!(song.get_logs_str(), "[PRINT](0) 1");
    }
   #[test]
    fn test_exec_sys_func_mid() {
        // mid
        let song = exec_easy("STR A={abcd};PRINT(MID(A,1,2))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) ab");
    }
   #[test]
    fn test_exec_sys_func_replace() {
        // mid
        let song = exec_easy("STR A={abcd};PRINT(REPLACE(A,{ab},{rr}))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) rrcd");
    }
   #[test]
    fn test_lex_macro_extract() {
        let song = exec_easy("STR A={c} PRINT(A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) c");
        let song = exec_easy("#A={c} PRINT(#A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) c");
        // let song = exec_easy("STR A={#?1} A{e}");
        // assert_eq!(song.get_logs_str(), "[PRINT](0) c");
    }
    #[test]
    fn test_array() {
        let song = exec_easy("ARRAY A=(1,2,3) PRINT(A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) (1,2,3)");
        // SizeOf
        let song = exec_easy("ARRAY A=(1,2,3) PRINT(SizeOf(A))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 3");
        // combine
        let song = exec_easy("ARRAY A=(1,1);ARRAY B=(2,2);ARRAY C=(3,3);PRINT((A,B,C))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) ((1,1),(2,2),(3,3))");
        let song = exec_easy("ARRAY A=(1,);ARRAY B=(2,);ARRAY C=(3,);PRINT((A,B,C))");
        assert_eq!(song.get_logs_str(), "[PRINT](0) ((1),(2),(3))");
    }
    #[test]
    fn test_lex_neg_number() {
        let song = exec_easy("PRINT(-1)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) -1");
        let song = exec_easy("PRINT(-50)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) -50");
        let song = exec_easy("INT A=30; PRINT(-A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) -30");
    }
    #[test]
    fn extract_function_args() { // 関数の引数で与えた文字列を関数の中で展開できない #27
        let song = exec_easy("Function EXT_MML(STR AA){ AA }; EXT_MML{ l4cdeg }");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, song.timebase * 4);
        //
        let song = exec_easy("Function EXT_MML(STR AA){ AA }; EXT_MML{ l8 [8c] }");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, song.timebase * 4);
    }
    #[test]
    fn func_def_value() { // 関数の引数に省略値が指定できないでエラーになる #37
        let song = exec_easy("Function EXT_MML(STR AA={l4cdef}){ AA }; EXT_MML");
        let pos = song.tracks[0].timepos;
        assert_eq!(pos, song.timebase * 4);
        //
        let song = exec_easy("Function EXT_MML(STR AA={cdef}){ PRINT(AA) }; EXT_MML ");
        assert_eq!(song.get_logs_str(), "[PRINT](0) cdef");
        //
        let song = exec_easy("Function DEF_TEST(AA=1){ PRINT(AA) }; DEF_TEST ");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 1");
    }
   #[test]
    fn test_read_value_hex() { // v1互換の16進数を読めない問題 #48
        let song = exec_easy("INT A=$10; PRINT(A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 16");
        let song = exec_easy("INT A=0x10; PRINT(A)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 16");
    }
   #[test]
    fn test_loop() {
        // loop simple
        let song = exec_easy("[4 c4]");
        assert_eq!(trk!(song).timepos, song.timebase * 4);
        // loop break
        let song = exec_easy("[4 c4 : c4] c4");
        assert_eq!(trk!(song).timepos, song.timebase * 8);
        // loop nested
        let song = exec_easy("[4 [2 c4] ]");
        assert_eq!(trk!(song).timepos, song.timebase * 8);
        // loop nested with break
        let song = exec_easy("[4 [2 c4 : c4] ]");
        assert_eq!(trk!(song).timepos, song.timebase * 12);
    }
   #[test]
    fn test_read_system_value() {
        // timebase test
        let song = exec_easy("TIMEBASE(96); c4; PRINT(TIME)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 96");
        let song = exec_easy("TIMEBASE(48); c4; PRINT(TIME)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 48");
        // v
        let song = exec_easy("v120 c4; PRINT(v)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 120");
        // o
        let song = exec_easy("o6 c4; PRINT(o)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 6");
    }
 }
