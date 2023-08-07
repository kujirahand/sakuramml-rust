//! runner from tokens

use crate::mml_def::TieMode;

use super::cursor::TokenCursor;
use super::lexer::lex;
use super::song::{Event, NoteInfo, Song, Track};
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

pub fn exec_args(song: &mut Song, tokens: &Vec<Token>) -> Vec<SValue> {
    let mut args: Vec<SValue> = vec![];
    for t in tokens {
        exec(song, &vec![t.clone()]);
        let v = song.stack.pop().unwrap_or(SValue::None);
        args.push(v);
    }
    args
}

/// run tokens
pub fn exec(song: &mut Song, tokens: &Vec<Token>) -> bool {
    let mut pos = 0;
    let mut loop_stack: Vec<LoopItem> = vec![];
    while pos < tokens.len() {
        if song.flags.break_flag != 0 { break; }
        let t = &tokens[pos];
        if song.debug {
            println!("- exec({:03})(line:{}) {:?} {}", pos, song.lineno, t.ttype, t.to_debug_str());
        }
        match t.ttype {
            TokenType::Empty => {},
            TokenType::LineNo => {
                song.lineno = t.lineno;
            },
            TokenType::Error => {
                if song.debug {
                    println!("[RUNTIME.ERROR]");
                }
            },
            TokenType::Print => {
                let args_tokens = t.children.clone().unwrap_or(vec![]);
                let args = exec_args(song, &args_tokens);
                let mut disp: Vec<String> = vec![];
                for v in args.into_iter() {
                    disp.push(v.to_s());
                }
                let disp_s = disp.join(" ");
                let msg = format!("[PRINT]({}) {}", t.lineno, disp_s);
                if song.debug {
                    println!("{}", msg);
                }
                song.logs.push(msg);
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
            TokenType::Track => exec_track(song, t),
            TokenType::Channel => {
                let v = value_range(1, data_get_int(&t.data, song), 16) - 1; // CH(1 to 16)
                trk!(song).channel = v as isize;
            },
            TokenType::Voice => exec_voice(song, t),
            TokenType::Note => exec_note(song, t),
            TokenType::NoteN => exec_note_n(song, t),
            TokenType::Rest => exec_rest(song, t),
            TokenType::Length => {
                trk!(song).length = calc_length(&t.data[0].to_s(), song.timebase, song.timebase);
            },
            TokenType::Octave => {
                trk!(song).octave = value_range(0, t.value, 10);
            },
            TokenType::OctaveRel => {
                trk!(song).octave = value_range(0, trk!(song).octave + t.value, 10);
            },
            TokenType::VelocityRel => {
                trk!(song).velocity = value_range(0, trk!(song).velocity + t.value, 127);
            },
            TokenType::OctaveOnce => {
                trk!(song).octave = value_range(0, trk!(song).octave + t.value, 10);
                song.flags.octave_once += t.value;
            },
            TokenType::QLen => {
                trk!(song).qlen = value_range(0, t.value, 100);
                trk!(song).q_on_note = None;
            },
            TokenType::Velocity => {
                let ino = t.data[0].to_i();
                if ino > 0 {
                    while trk!(song).v_sub.len() >= ino as usize {
                        trk!(song).v_sub.push(0);
                    }
                    trk!(song).v_sub[ino as usize] = value_range(0, t.value, 127);
                } else {
                    trk!(song).velocity = value_range(0, t.value, 127);
                }
                trk!(song).v_on_time = None;
                trk!(song).v_on_note = None;
            },
            TokenType::Timing => {
                trk!(song).timing = t.value;
                trk!(song).t_on_note = None;
            },
            TokenType::CtrlChange => {
                let no = t.value;
                let val_tokens = t.children.clone().unwrap_or(vec![]);
                exec(song, &val_tokens);
                let val_v = song.stack.pop().unwrap_or(SValue::None);
                let val = val_v.to_i();
                song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, no, val));
            },
            TokenType::PitchBend => {
                let val = var_extract(&t.data[0], song).to_i();
                let val = if t.value == 0 { val * 128 } else { val + 8192 };
                song.add_event(Event::pitch_bend(
                    trk!(song).timepos,
                    trk!(song).channel,
                    val,
                ));
            },
            TokenType::Tempo => {
                let tempo = data_get_int(&t.data, song);
                let tempo = if tempo > 300 { 300 } else { tempo };
                tempo_change(song, tempo);
            },
            TokenType::TempoChange => {
                let data: Vec<isize> = (&t.data[0]).to_int_array();
                if data.len() == 3 {
                    tempo_change_a_to_b(song, data[0], data[1], data[2]);
                } else if data.len() == 2 {
                    tempo_change_a_to_b(song, song.tempo, data[0], data[1]);
                } else {
                    tempo_change(song, data[0]);
                }
            },
            TokenType::MetaText => {
                let mut txt = data_get_str(&t.data, song);
                if txt.len() > 128 {
                    txt = txt.chars().take(40).collect();
                }
                let e = Event::meta(
                    trk!(song).timepos,
                    0xFF,
                    t.value,
                    txt.len() as isize,
                    txt.into_bytes(),
                );
                song.add_event(e);
            },
            TokenType::TimeSignature => {
                song.timesig_frac = t.data[0].to_i();
                song.timesig_deno = t.data[1].to_i();
                song.timesig_deno = match song.timesig_deno {
                    2 => 2,
                    4 => 4,
                    8 => 8,
                    16 => 16,
                    _ => {
                        song.logs
                            .push(String::from("[TimeSignature] value must be 2/4/8/16,n"));
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
                let e = Event::sysex(trk!(song).timepos, &t.data);
                song.add_event(e);
            },
            TokenType::Time => exec_time(song, t),
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
            TokenType::KeyShift => {
                let args_tokens = t.children.clone().unwrap_or(vec![]);
                let args = exec_args(song, &args_tokens);
                song.key_shift = args[0].to_i();
            },
            TokenType::TrackKey => {
                let args_tokens = t.children.clone().unwrap_or(vec![]);
                let args = exec_args(song, &args_tokens);
                trk!(song).track_key = args[0].to_i();
            },
            TokenType::DefInt => {
                let var_key = t.data[0].to_s().clone();
                let val_tokens = t.children.clone().unwrap_or(vec![]);
                exec(song, &val_tokens);
                let val = song.stack.pop().unwrap_or(SValue::from_i(0));
                song.variables.insert(var_key, val);
            },
            TokenType::LetVar => {
                let var_key = t.data[0].to_s().clone();
                let val_tokens = t.children.clone().unwrap_or(vec![]);
                exec(song, &val_tokens);
                let val = song.stack.pop().unwrap_or(SValue::from_i(0));
                song.variables.insert(var_key, val);
            },
            TokenType::DefStr => {
                let var_key = t.data[0].to_s().clone();
                let var_val = var_extract(&t.data[1], song);
                song.variables.insert(var_key, var_val);
            },
            TokenType::PlayFrom => {
                song.play_from = trk!(song).timepos;
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
                trk!(song).v_on_time_start = trk!(song).timepos;
                trk!(song).v_on_time = Some(t.data[0].to_int_array());
            },
            TokenType::VelocityOnNote => {
                trk!(song).v_on_note_index = 0;
                trk!(song).v_on_note = Some(t.data[0].to_int_array());
            },
            TokenType::TimingOnNote => {
                trk!(song).t_on_note_index = 0;
                trk!(song).t_on_note = Some(t.data[0].to_int_array());
            },
            TokenType::QLenOnNote => {
                trk!(song).q_on_note_index = 0;
                trk!(song).q_on_note = Some(t.data[0].to_int_array());
            },
            TokenType::CConTime => {
                trk!(song).write_cc_on_time(t.value, t.data[0].to_int_array());
            },
            TokenType::CConTimeFreq => {
                trk!(song).cc_on_time_freq = var_extract(&t.data[0], song).to_i();
            },
            TokenType::PBonTime => {
                trk!(song).write_pb_on_time(t.value, t.data[0].to_int_array(), song.timebase);
            },
            TokenType::MeasureShift => {
                song.flags.measure_shift = var_extract(&t.data[0], song).to_i();
            },
            TokenType::TrackSync => song.track_sync(),
            TokenType::TieMode => {
                let args = &t.data[0].to_array();
                if args.len() >= 1 {
                    trk!(song).tie_mode = TieMode::from_i(var_extract(&args[0], song).to_i());
                }
                if args.len() >= 2 {
                    trk!(song).tie_value = var_extract(&args[1], song).to_i();
                }
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
            TokenType::Calc => {
                // get flag char
                let flag = std::char::from_u32(t.tag as u32).unwrap_or('😔');
                // only 1 value
                if flag == '!' { // flag "!(val)"
                    let v = song.stack.pop().unwrap_or(SValue::None);
                    song.stack.push(SValue::from_b(!v.to_b()));
                    pos += 1;
                    continue;
                }
                // 2 values
                let b = song.stack.pop().unwrap_or(SValue::None);
                let a = song.stack.pop().unwrap_or(SValue::None);
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
                        song.logs.push(String::from("[Calc] unknown flag"));
                    }
                }
                song.stack.push(c);
            },
            TokenType::Value => {
                // extract value
                // println!("@@@Value=>{:?}", t);
                // check is variable?
                if t.tag == 0 {
                    let v = var_extract(&t.data[0], song);
                    // println!("push={}", v.to_s());
                    song.stack.push(v);
                } else {
                    // function
                    exec_function(song, t);
                }
            },
            TokenType::ValueInc => {
                let varname = t.data[0].to_s();
                let val_inc = t.value;
                song.variables.get_mut(&varname).map(|v| {
                    *v = SValue::from_i(v.to_i() + val_inc);
                });
            },
            TokenType::SetConfig => {
                let key = t.data[0].to_s();
                let val = &t.data[1];
                if key == "RandomSeed" {
                    song.rand_seed = val.to_i() as u32;
                }
            },
        }
        pos += 1;
    }
    true
}

fn exec_function(song: &mut Song, t: &Token) -> bool {
    let stack_size1 = song.stack.len();
    let args_tokens = t.children.clone().unwrap_or(vec![]);
    exec(song, &args_tokens);
    let stack_size2 = song.stack.len();
    let args: Vec<SValue> = song.stack.splice(stack_size1..stack_size2, vec![]).collect();
    // println!("@@@function_args={:?}", args);
    let arg_count = args.len();
    let func_name = t.data[0].to_s();
    //
    // todo: https://sakuramml.com/wiki/index.php?%E7%B5%84%E3%81%BF%E8%BE%BC%E3%81%BF%E9%96%A2%E6%95%B0
    //
    if func_name == "Random" || func_name == "RANDOM" || func_name == "RandomInt" || func_name == "RND" || func_name == "Rnd" {
        if arg_count >= 2 {
            let min = args[0].to_i();
            let max = args[1].to_i();
            let rnd = song.rand() as isize % (max - min + 1) + min;
            song.stack.push(SValue::from_i(rnd));
        } else if arg_count == 1 {
            let m = args[0].to_i();
            let v = (song.rand() as isize) % m;
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
    else {
        song.stack.push(SValue::from_s(t.data[0].to_s().clone()));
    }
    true
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
    exec(song, &cond);
    let cond_val = song.stack.pop().unwrap_or(SValue::from_i(0));
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
        exec(song, &cond);
        let cond_val = song.stack.pop().unwrap_or(SValue::from_i(0));
        if cond_val.to_b() == false {
            break;
        }
        // exec body
        let body = body_token.children.clone().unwrap();
        exec(song, &body);
        // check counter
        counter += 1;
        if counter > song.flags.max_loop {
            song.logs.push(format!(
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
        exec(song, &cond);
        let cond_val = song.stack.pop().unwrap_or(SValue::from_i(0));
        if cond_val.to_b() == false {
            break;
        }
        // exec body
        let body = body_token.children.clone().unwrap();
        exec(song, &body);
        // check loop counter
        counter += 1;
        if counter > song.flags.max_loop {
            song.logs.push(format!(
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
    if cmd == "TR" || cmd == "TRACK" || cmd == "Track" {
        let tr = song.cur_track as isize;
        return Some(SValue::from_i(tr));
    }
    else if cmd == "CH" || cmd == "CHANNEL" {
        let ch = trk!(song).channel;
        return Some(SValue::from_i(ch));
    }
    else if cmd == "TIME" || cmd == "Time" {
        let v = trk!(song).timepos;
        return Some(SValue::from_i(v));
    }
    None
}

fn var_extract(val: &SValue, song: &mut Song) -> SValue {
    match val {
        // String
        SValue::Str(s, _) => {
            if s.starts_with('=') && s.len() >= 2 {
                let key = &s[1..];
                match song.variables.get(key) {
                    Some(v) => v.clone(),
                    None => {
                        match get_system_value(key, song) {
                            Some(v) => return v,
                            None => {
                                let err_msg = format!("[WARN]({}) Undefined: {}", song.lineno, key);
                                song.logs.push(err_msg);
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
    let cnt = t.value;
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

fn exec_time(song: &mut Song, t: &Token) {
    // Calc Time (SakuraObj_time2step)
    // (ref) https://github.com/kujirahand/sakuramml-c/blob/68b62cbc101669211c511258ae1cf830616f238e/src/k_main.c#L473
    let mes = t.data[0].to_i() + song.flags.measure_shift;
    let beat = t.data[1].to_i();
    let tick = t.data[2].to_i();
    // calc
    let base = song.timebase * 4 / song.timesig_deno;
    let total = (mes - 1) * (base * song.timesig_frac) + (beat - 1) * base + tick;
    song.tracks[song.cur_track].timepos = total;
}

fn data_get_int(data: &Vec<SValue>, song: &mut Song) -> isize {
    if data.len() == 0 {
        return 0;
    }
    var_extract(&data[0], song).to_i()
}

fn data_get_str(data: &Vec<SValue>, song: &mut Song) -> String {
    if data.len() == 0 {
        return String::new();
    }
    var_extract(&data[0], song).to_s()
}

pub fn calc_length(len_str: &str, timebase: isize, def_len: isize) -> isize {
    let mut res = def_len;
    if len_str == "" {
        return def_len;
    }
    let mut cur = TokenCursor::from(len_str);
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
        cur.next();
        res = (res as f32 * 1.5) as isize;
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
            if cur.peek_n(0) == '.' {
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
    let note_no = (t.value % 12) as isize;
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
    noteno += if note.natural == 0 {
        song.key_flag[note.no as usize]
    } else {
        0
    };
    noteno += song.key_shift;
    noteno += trk!(song).track_key;
    note.no = noteno;
}

fn exec_note(song: &mut Song, t: &Token) {
    // get note parameters
    let mut note = get_note_info_from_token(t);
    set_note_info_with_default_value(&mut note, song);
    // timepos
    let timepos = trk!(song).timepos;
    // onTime / onNote
    let v = trk!(song).calc_v_on_time(note.vel);
    let v = trk!(song).calc_v_on_note(v);
    let t = trk!(song).calc_t_on_note(note.t);
    let qlen = trk!(song).calc_qlen_on_note(note.qlen);
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
    // note len
    let notelen = calc_length(&note.len_s, song.timebase, trk!(song).length);
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
    // write event
    trk!(song).events.push(event);
    trk!(song).timepos += notelen;
}

fn exec_rest(song: &mut Song, t: &Token) {
    let trk = &mut song.tracks[song.cur_track];
    let data_note_len = t.data[0].to_s();
    let notelen = calc_length(&data_note_len, song.timebase, trk.length);
    trk.timepos += notelen * t.value;
}

fn exec_voice(song: &mut Song, t: &Token) {
    // voice no
    let no = var_extract(&t.data[0], song).to_i() - 1;
    let mut bank_msb = 0;
    let mut bank_lsb = 0;
    // bank ?
    if t.data.len() == 1 {
        song.add_event(Event::voice(trk!(song).timepos, trk!(song).channel, no));
    } else {
        if t.data.len() == 2 {
            bank_lsb = var_extract(&t.data[1], song).to_i();
        }
        if t.data.len() == 3 {
            bank_lsb = var_extract(&t.data[1], song).to_i();
            bank_msb = var_extract(&t.data[2], song).to_i();
        }
        song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, 0x00, bank_msb)); // msb
        song.add_event(Event::cc(trk!(song).timepos, trk!(song).channel, 0x20, bank_lsb)); // lsb
        song.add_event(Event::voice(trk!(song).timepos, trk!(song).channel, no));
        // println!("voice: no={}, bank_msb={}, bank_lsb={}", no, bank_msb, bank_lsb);
    }
}

fn exec_track(song: &mut Song, t: &Token) {
    let mut v = data_get_int(&t.data, song); // TR=0..
    if v < 0 {
        v = 0;
    }
    song.cur_track = v as usize;
    // new track ?
    while song.tracks.len() <= song.cur_track {
        // println!("{:?}", v);
        let trk = Track::new(song.timebase, v - 1);
        song.tracks.push(trk);
    }
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
        let song = exec_easy("INT N=1>2;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) FALSE");
        // 6/3
        let song = exec_easy("INT N=6/3;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 2");
        // 4/0
        let song = exec_easy("INT N=4/0;PRINT(N)");
        assert_eq!(song.get_logs_str(), "[PRINT](0) 0");
    }
}
