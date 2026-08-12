//! runner: SysEx(システムエクスクルーシブ)の実行
use super::*;

/// デバイス番号の指定 (SysExの送信先に使う)
pub(super) fn exec_device_number(song: &mut Song, t: &Token) {
    let args_tokens = t.children.clone().unwrap_or(vec![]);
    let n = exec_args(song, &args_tokens);
    song.device_number = if n.len() >= 1 { n[0].to_i() as u8 } else { 0 };
}

/// 任意のSysExを送信する
pub(super) fn exec_sysex(song: &mut Song, t: &Token) {
    // check arguments
    let mut args: Vec<SValue> = exec_args(song, &t.children.clone().unwrap_or(vec![]));
    if args.len() == 0 {
        runtime_error(song, &format!("SysEx : {}", song.get_message(MessageKind::ErrorWrongArguments)));
        return;
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
}

/// 音源のリセット (GM/GS/XG)
pub(super) fn exec_sysex_reset(song: &mut Song, t: &Token) {
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
}

/// ユニバーサルSysEx (MasterVolume / MasterBalance)
pub(super) fn exec_sysex_command(song: &mut Song, t: &Token) {
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
}

/// GS音源用のエフェクト設定
pub(super) fn exec_gs_effect(song: &mut Song, t: &Token) {
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
}
