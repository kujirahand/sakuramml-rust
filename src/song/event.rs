//! song: MIDIイベントの定義
use super::*;

/// Event Type
#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    NoteOn,
    NoteOff,
    ControllChange,
    PitchBend,
    PitchBendRange,
    Voice,
    Meta,
    SysEx,
    DirectSMF,
}

/// Event
#[derive(Debug, Clone)]
pub struct Event {
    pub etype: EventType,
    pub time: isize,
    pub channel: isize,
    pub v1: isize,
    pub v2: isize,
    pub v3: isize,
    pub data: Option<Vec<u8>>,
}

impl Event {
    pub fn note(time: isize, channel: isize, note_no: isize, len: isize, vel: isize) -> Self {
        Self { etype: EventType::NoteOn, time, channel, v1: note_no, v2: len, v3: vel, data: None }
    }
    pub fn voice(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::Voice, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
    pub fn meta(time: isize, v1: isize, v2: isize, v3: isize, data_v: Vec<u8>) -> Self {
        Self { etype: EventType::Meta, time, channel: 0, v1, v2, v3, data: Some(data_v) }
    }
    /// generate SMF event type
    pub fn direct_smf(time: isize, data_v: Vec<u8>) -> Self {
        Self { etype: EventType::DirectSMF, time, channel:0, v1: 0 , v2: 0, v3: 0, data: Some(data_v) }
    }
    pub fn sysex(time: isize, data_v: &Vec<SValue>, checksum_mode: bool) -> Self {
        // convert to u8 without checksum
        if checksum_mode == false {
            let mut a: Vec<u8> = vec![];
            for v in data_v.iter() {
                a.push(v.to_i() as u8);
            }
            return Self { etype: EventType::SysEx, time, channel: 0, v1: 0, v2: 0, v3: 0, data: Some(a) };
        }
        // calc checksum
        let mut a: Vec<u8> = vec![];
        let mut checksum: isize = 0;
        let mut flag_checksum = false;
        for v in data_v.iter() {
            let n = v.to_i();
            if flag_checksum {
                if n == -2 {
                    flag_checksum = false;
                    let checksum_v = ((128 - (checksum & 0x7F)) & 0x7F) as u8; // 7bit補正
                    a.push(checksum_v);
                    continue;
                } else {
                    checksum += n;
                }
            }
            if n == -1 {
                flag_checksum = true;
                continue;
            }
            a.push(n as u8);
        }
        Self { etype: EventType::SysEx, time, channel: 0, v1: 0, v2: 0, v3: 0, data: Some(a) }
    }
    
    pub fn sysex_raw(time: isize, data_v: Vec<u8>) -> Self {
        Self { etype: EventType::SysEx, time, channel: 0, v1: 0, v2: 0, v3: 0, data: Some(data_v) }
    }
    /// ControllChange
    pub fn cc(time: isize, channel: isize, no: isize, value: isize) -> Self {
        Self { etype: EventType::ControllChange, time, channel, v1: no, v2: value, v3:0, data: None }
    }
    /// pitch_bend : 0..16383 (-8192 .. 0 .. 8191)
    pub fn pitch_bend(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::PitchBend, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
    pub fn pitch_bend_range(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::PitchBendRange, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
    /// dump data
    pub fn dump_data_to_hexstr(&self) -> String {
        let mut r = vec![];
        match &self.data {
            None => return "".to_string(),
            Some(data) => {
                for v in data.iter() {
                    r.push(format!("{:02X}", v));
                }
            }
        }
        r.join(",")
    }
}
