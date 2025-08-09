use crate::song::{Song};
use crate::svalue::{SValue};

/// Callback function
pub type CallbackCalcFn = fn (&mut Song, Vec<SValue>) -> SValue;

/// Random
pub fn calc_randomint(song: &mut Song, args: Vec<SValue>) -> SValue {
    let arg_count = args.len();
    if arg_count >= 2 {
        let min = args[0].to_i();
        let max = args[1].to_i();
        let rnd = (song.rand() & 0x7FFFFFFF) as isize % (max - min + 1) + min;
        SValue::from_i(rnd)
    } else if arg_count == 1 {
        let m = args[0].to_i();
        let v = ((song.rand() & 0x7FFFFFFF) as isize) % m;
        SValue::from_i(v)
    } else {
        let v = song.rand() as isize;
        SValue::from_i(v)
    }
}

/// RandomSelect
pub fn calc_random_select(song: &mut Song, args: Vec<SValue>) -> SValue {
    let arg_count = args.len();
    /*
    if arg_count == 1 {
        let a = args[0].to_array();
        let r = song.rand() as usize % a.len();
        return a[r].clone();
    }
    */
    let r = song.rand() as usize % arg_count;
    args[r as usize].clone()
}

/// Chr
pub fn calc_chr(_: &mut Song, args: Vec<SValue>) -> SValue {
    let arg_count = args.len();
    if arg_count >= 1 {
        let val = args[0].to_i();
        let mut s = String::new();
        s.push(std::char::from_u32(val as u32).unwrap_or(' '));
        SValue::from_s(s)
    } else {
        SValue::from_str(" ")
    }
}

// mid function
fn vb_mid(input: &str, start: usize, length: usize) -> Option<&str> {
    let input_len = input.len();
    let start = if start >= 1 { start - 1 } else { 0 };
    let mut end = start + length;
    if end >= input_len { end = input_len; }
    Some(&input[start..end])
}

/// Mid
pub fn calc_mid(_: &mut Song, args: Vec<SValue>) -> SValue {
    let arg_count = args.len();
    if arg_count >= 3 {
        let val = args[0].to_s();
        let i_from = args[1].to_i() as usize;
        let i_len = args[2].to_i() as usize;
        let s = vb_mid(&val, i_from, i_len).unwrap_or("");
        SValue::from_str(s)
    } else {
        SValue::from_str("(MID:ERROR)")
    }
}

/// Replace
pub fn calc_replace(_: &mut Song, args: Vec<SValue>) -> SValue {
    let arg_count = args.len();
    if arg_count >= 3 {
        let val = args[0].to_s();
        let s_from = args[1].to_s();
        let s_to = args[2].to_s();
        let s = val.replace(&s_from, &s_to);
        SValue::from_str(&s)
    } else {
        SValue::from_str("(REPLACE:ERROR)")
    }
}

/// SizeOf
pub fn calc_sizeof(_: &mut Song, args: Vec<SValue>) -> SValue {
    if args.len() >= 1 {
        let v = match &args[0] {
            SValue::Array(a) => a.len(),
            SValue::Str(s, _) => s.len(),
            SValue::IntArray(a) => a.len(),
            SValue::StrArray(a) => a.len(),
            _ => 0
        };
        return SValue::from_i(v as isize);
    }
    SValue::from_i(0)
}

/// StrLen
pub fn calc_strlen(_: &mut Song, args: Vec<SValue>) -> SValue {
    if args.len() >= 1 {
        let v = match &args[0] {
            SValue::Array(a) => a.len(),
            SValue::Str(s, _) => s.len(),
            SValue::IntArray(a) => a.len(),
            SValue::StrArray(a) => a.len(),
            _ => 0
        };
        return SValue::from_i(v as isize);
    }
    SValue::from_i(0)
}

/// Asc
pub fn calc_asc(_: &mut Song, args: Vec<SValue>) -> SValue {
    if args.len() == 0 {
        return SValue::from_i(0);
    }
    let s = args[0].to_s();
    let a = s.as_bytes().to_vec();
    if a.len() == 0 {
        return SValue::from_i(0);
    }
    SValue::from_i(a[0] as isize)
}

/// MML
pub fn calc_mml(song: &mut Song, args: Vec<SValue>) -> SValue {
    if args.len() == 0 {
        return SValue::from_i(0);
    }
    let arg = &args[0];
    let sa = arg.to_s();
    if sa == "o" {
        let o = song.tracks[song.cur_track].octave;
        return SValue::from_i(o);
    }
    if sa == "v" {
        let v = song.tracks[song.cur_track].velocity;
        return SValue::from_i(v);
    }
    if sa == "q" {
        let v = song.tracks[song.cur_track].qlen;
        return SValue::from_i(v);
    }
    if sa == "t" {
        let v = song.tracks[song.cur_track].timing;
        return SValue::from_i(v);
    }
    if sa == "@" {
        let v = song.tracks[song.cur_track].program_change;
        return SValue::from_i(v);
    }
    if sa == "BR" {
        let v = song.tracks[song.cur_track].bend_range;
        return SValue::from_i(v);
    }
    SValue::from_i(0)
}

/// Hex
pub fn calc_hex(_: &mut Song, args: Vec<SValue>) -> SValue {
    if args.len() == 0 {
        return SValue::from_s("00".to_string());
    }
    let v = args[0].to_i();
    SValue::from_s(format!("{:02X}", v))
}

/// Pos
pub fn calc_pos(_: &mut Song, args: Vec<SValue>) -> SValue {
    if args.len() < 2 {
        return SValue::from_i(0);
    }
    let sub = args[0].to_s();
    let str = args[1].to_s();
    if let Some(index) = str.find(&sub) {
        let prefix = &str[..index];
        return SValue::from_i((prefix.chars().count() + 1) as isize);
    }
    SValue::from_i(0)
}
