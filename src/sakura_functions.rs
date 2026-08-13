use crate::song::{Song};
use crate::svalue::{SValue};

/// Callback function
pub type CallbackCalcFn = fn (&mut Song, Vec<SValue>) -> SValue;

/// Random
pub fn calc_randomint(song: &mut Song, args: Vec<SValue>) -> SValue {
    let arg_count = args.len();
    if arg_count >= 2 {
        let a = args[0].to_i();
        let b = args[1].to_i();
        let min = a.min(b);
        let max = a.max(b);
        let rnd = (song.rand() & 0x7FFFFFFF) as isize % (max - min + 1) + min;
        SValue::from_i(rnd)
    } else if arg_count == 1 {
        let m = args[0].to_i();
        if m <= 0 {
            return SValue::from_i(0);
        }
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
    if arg_count == 0 {
        return SValue::None;
    }
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
fn vb_mid(input: &str, start: isize, length: isize) -> String {
    if length <= 0 {
        return String::new();
    }
    let start = start.max(1) as usize - 1;
    input.chars().skip(start).take(length as usize).collect()
}

/// Mid
pub fn calc_mid(_: &mut Song, args: Vec<SValue>) -> SValue {
    let arg_count = args.len();
    if arg_count >= 3 {
        let val = args[0].to_s();
        let i_from = args[1].to_i();
        let i_len = args[2].to_i();
        SValue::from_s(vb_mid(&val, i_from, i_len))
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
    if sa == "l" {
        let v = song.tracks[song.cur_track].length;
        return SValue::from_i(v);
    }
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
    if sa == "p%" {
        let v = song.tracks[song.cur_track].pitch_bend;
        return SValue::from_i(v);
    }
    if sa == "Key" {
        return SValue::from_i(song.key_shift);
    }
    if sa == "TimeKey" {
        // TimeKey命令は未実装のため、現在の時間キーは初期値の0。
        return SValue::from_i(0);
    }
    if sa == "Port" {
        let v = song.tracks[song.cur_track].port;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_functions_handle_empty_or_invalid_ranges() {
        let mut song = Song::new();
        assert_eq!(calc_randomint(&mut song, vec![SValue::from_i(0)]).to_i(), 0);
        assert!(calc_random_select(&mut song, vec![]).is_none());

        for _ in 0..8 {
            let value = calc_randomint(
                &mut song,
                vec![SValue::from_i(5), SValue::from_i(3)],
            ).to_i();
            assert!((3..=5).contains(&value));
        }
    }

    #[test]
    fn string_functions_support_unicode_character_positions() {
        let mut song = Song::new();
        let mid = calc_mid(
            &mut song,
            vec![SValue::from_str("あいうえ"), SValue::from_i(2), SValue::from_i(2)],
        );
        assert_eq!(mid.to_s(), "いう");
        assert_eq!(
            calc_pos(
                &mut song,
                vec![SValue::from_str("う"), SValue::from_str("あいうえ")],
            ).to_i(),
            3,
        );
    }
}
