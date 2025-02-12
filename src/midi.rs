//! MIDI file generator and analizer

/// midi 
use super::song::{Song, Track, EventType};

/// MIDI Event
const MIDI_RPN_MSB: u8 = 0x65;
const MIDI_RPN_LSB: u8 = 0x64;
const MIDI_DATA_ENTRY_MSB: u8 = 0x06;
const _MIDI_DATA_ENTRY_LSB: u8 = 0x26;

fn array_push_str(res: &mut Vec<u8>, s: &str) {
    for b in s.as_bytes() {
        res.push(*b);
    }
}

fn array_push_u16(res: &mut Vec<u8>, v: isize) {
    res.push(((v >> 8) & 0xFF) as u8);
    res.push(((v >> 0) & 0xFF) as u8);
}

fn array_push_u32(res: &mut Vec<u8>, v: isize) {
    res.push(((v >> 24) & 0xFF) as u8);
    res.push(((v >> 16) & 0xFF) as u8);
    res.push(((v >>  8) & 0xFF) as u8);
    res.push(((v >>  0) & 0xFF) as u8);
}

fn array_push_delta(res: &mut Vec<u8>, time: isize) {
    let mut buf: Vec<u8> = vec![];
    let mut v = time;
    buf.push((v & 0x7F) as u8);
    v = v >> 7;
    while v > 0 {
        buf.push((0x80 | v & 0x7F) as u8);
        v = v >> 7;
    }
    // println!("time={},res={:?}", time, buf);
    buf.reverse();
    for b in buf.into_iter() {
        res.push(b);
    }
}

fn generate_track(track: &Track) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let mut timepos = 0;
    for e in &track.events {
        match e.etype {
            EventType::NoteOn => {
                let note_no = e.v1;
                // note_len = e.v2 // not use
                let note_vel = e.v3;
                // note on
                array_push_delta(&mut res, e.time - timepos);
                timepos = e.time;
                res.push(0x90 + e.channel as u8);
                res.push(note_no as u8); // note_no
                res.push(note_vel as u8); // velocity
            },
            EventType::NoteOff => {
                let note_no = e.v1;
                // note_len = e.v2 // not use
                let note_vel = e.v3;
                array_push_delta(&mut res, e.time - timepos);
                timepos = e.time;
                res.push(0x80 + e.channel as u8);
                res.push(note_no as u8);
                res.push(note_vel as u8);
            },
            EventType::Voice => {
                array_push_delta(&mut res, e.time - timepos);
                timepos = e.time;
                res.push(0xC0 + e.channel as u8);
                res.push(e.v1 as u8);
            },
            EventType::ControllChange => {
                array_push_delta(&mut res, e.time - timepos);
                timepos = e.time;
                res.push(0xB0 + e.channel as u8);
                res.push(e.v1 as u8);
                res.push(e.v2 as u8);
            },
            EventType::Meta => {
                array_push_delta(&mut res, e.time - timepos);
                timepos = e.time;
                res.push(e.v1 as u8);
                res.push(e.v2 as u8);
                res.push(e.v3 as u8);
                let data = e.data.clone().unwrap();
                for b in data.iter() {
                    res.push(*b);
                }
            },
            EventType::SysEx => {
                let data = e.data.clone().unwrap();
                if data.len() == 0 { continue; }
                let delta_time = e.time - timepos;
                array_push_delta(&mut res, delta_time);
                timepos = e.time;
                let size = data.len() - 1;
                // 1st byte must be 0xF0
                res.push(0xF0); // SysEx Event
                // 2nd byte must be length
                res.push(size as u8);
                // write data
                for (i, b) in data.iter().enumerate() {
                    if i == 0 && *b == 0xF0 { continue; }
                    res.push(*b);
                }
            },
            EventType::PitchBend => {
                let v = e.v1;
                let msb = ((v >> 7) & 0x7F) as u8;
                let lsb = ((v >> 0) & 0x7F) as u8;
                // println!("PB={}(0x{:02x}{:02x})", v, msb, lsb);
                array_push_delta(&mut res, e.time - timepos);
                timepos = e.time;
                res.push(0xE0 + e.channel as u8);
                res.push(lsb);
                res.push(msb);
            },
            EventType::PitchBendRange => { // RPN
                // Pitch Bend Sensitivity (3 events)
                let range = e.v1;
                let range = if range >= 0 && range <= 24 { range as u8 } else { 0 };
                // RPN MSB
                array_push_delta(&mut res, e.time - timepos);
                timepos = e.time;
                res.push(0xB0 + e.channel as u8);
                res.push(MIDI_RPN_MSB);
                res.push(0);
                // RPN LSB
                res.push(0);
                res.push(0xB0 + e.channel as u8);
                res.push(MIDI_RPN_LSB);
                res.push(0);
                // Data Entry MSB
                res.push(0);
                res.push(0xB0 + e.channel as u8);
                res.push(MIDI_DATA_ENTRY_MSB);
                res.push(range);
            },
            EventType::DirectSMF => {
                let data = e.data.clone().unwrap();
                if data.len() == 0 { continue; }
                let delta_time = e.time - timepos;
                array_push_delta(&mut res, delta_time);
                timepos = e.time;
                // write data
                for b in data.iter() {
                    res.push(*b);
                }
            },
        }
    }
    // end of track
    res.push(0x00);
    res.push(0xFF);
    res.push(0x2F);
    res.push(0x00);
    res
}

pub fn generate(song: &mut Song) -> Vec<u8> {
    let midi_format = 1;
    let mut res: Vec<u8> = vec![];
    song.play_from_all_track();
    song.normalize_and_sort();
    // header
    array_push_str(&mut res, "MThd");
    array_push_u32(&mut res, 6);
    array_push_u16(&mut res, midi_format);
    array_push_u16(&mut res, song.tracks.len() as isize);
    array_push_u16(&mut res, song.timebase);
    // tracks
    for track_no in 0..song.tracks.len() {
        let trk = &song.tracks[track_no];
        let block = generate_track(&trk);
        array_push_str(&mut res, "MTrk");
        array_push_u32(&mut res, block.len() as isize);
        for b in block { res.push(b); }
    }
    res
}


// midi reader
pub struct MidiReaderInfo {
    frac: usize,
    deno: usize,
    is_eot: bool,
}
impl MidiReaderInfo {
    fn new() -> Self {
        Self {
            frac: 4,
            deno: 4,
            is_eot: false,
        }
    }
}

pub fn array_read_str(a: &Vec<u8>, pos: usize, len: usize) -> String {
    let mut s = String::new();
    let sub_a = a[pos..pos+len].to_vec();
    match String::from_utf8(sub_a) {
        Ok(s) => s,
        Err(_) => {
            for i in 0..len {
                let idx = pos + i;
                if idx < a.len() {
                    s.push(a[idx] as char);
                }
            }
            s
        }
    }
}

pub fn array_read_u16(a: &Vec<u8>, pos: usize) ->u16 {
    let mut v: u16 = 0;
    if pos < a.len() {
        v = a[pos] as u16;
    }
    if (pos + 1) < a.len() {
        v = v << 8;
        v = v | a[pos+1] as u16;
    }
    v
}

pub fn array_read_u32(a: &Vec<u8>, pos: usize) ->u32 {
    let mut v: u32 = 0;
    if pos < a.len() { v = a[pos] as u32; }
    if (pos + 1) < a.len() { v = v << 8 | a[pos+1] as u32; }
    if (pos + 2) < a.len() { v = v << 8 | a[pos+2] as u32; }
    if (pos + 3) < a.len() { v = v << 8 | a[pos+3] as u32; }
    v
}

pub fn array_readl_delta_time(a: &Vec<u8>, pos: &mut usize) -> usize {
    let mut v: usize = 0;
    while *pos < a.len() {
        let cv = a[*pos] as usize;
        *pos += 1;
        if cv < 0x7F {
            v = v << 7 | cv;
            break;
        }
        v = v << 7 | (cv & 0x7F); 
    }
    v
}

pub fn dump_midi_event_meta(bin: &Vec<u8>, pos: &mut usize, info: &mut MidiReaderInfo) -> String {
    let p = *pos;
    let mtype = bin[p];
    let meta_type = bin[p+1] as usize;
    let meta_len = bin[p+2] as usize;
    match mtype {
        0xFF => {
            let msg = match meta_type {
                0x2F => { // end of track
                    info.is_eot = true;
                    String::from("/* __END_OF_TRACK__ */")
                },
                0x51 => { // tempo
                    // mpq = 60000000 / tempo || mpq * tempo = 60000000 || tempo = 60000000 / mpq
                    let mpq = (bin[p+3] as usize) << 16  | (bin[p+4] as usize) << 8 | bin[p+5] as usize;
                    let tempo = 60000000 / mpq;
                    format!("Tempo={}", tempo)
                },
                0x58 => { // TimeSig
                    let nn = bin[p + 3] as usize;
                    let dd = bin[p + 4] as usize;
                    info.frac = nn;
                    info.deno = (2i32.pow(dd as u32)) as usize;
                    format!("TimeSig={}/{}", info.frac, info.deno)
                },
                _ => { // text
                    let txt = array_read_str(bin, p+3, meta_len);
                    let meta_name = match meta_type {
                        0x01 => { "TEXT".to_string() },
                        0x02 => { "COPYRIGHT".to_string() },
                        0x03 => { "TRACK_NAME".to_string() },
                        0x04 => { "INSTRUMENT_NAME".to_string() },
                        0x05 => { "LYRIC".to_string() },
                        0x06 => { "MARKER".to_string() },
                        0x07 => { "CUE_POINT".to_string() },
                        _ => { format!("// Meta Type=${:02x} Length={} Text=", meta_type, meta_len) }
                    };
                    format!("{}{{{}}};", meta_name, txt)
                }
            };
            *pos += 3 + meta_len;
            msg
        },
        0xF0 => { // SysEx = 0xF0 ... 0xF7
            let mut m = String::new();
            let mut index = 0;
            while *pos < bin.len() {
                let b = bin[*pos];
                if index == 1 { // SysExの2バイト目はSysExの長さを表す
                    m.push_str(&format!("/*len:{:02X}*/", b));
                } else {
                    m.push_str(&format!("{:02X}", b));
                    if b != 0xf7 {
                        m.push(',');
                    }
                }
                if bin[*pos] == 0xf7 {
                    *pos += 1;
                    break;
                }
                *pos += 1;
                index += 1;
            }
            format!("SysEx$={};", m)
        },
        _ => {
            format!("[ERROR] Unknown meta event...={:02x}", meta_type)
        }
    }
}

pub fn note_no_dec(no: u8) -> String {
    format!("o{}{}",
        no / 12,
        match no % 12 {
            0 => "c",
            1 => "c#",
            2 => "d",
            3 => "d#",
            4 => "e",
            5 => "f",
            6 => "f#",
            7 => "g",
            8 => "g#",
            9 => "a",
            10 => "a#",
            11 => "b",
            _ => ""
        }
    )
}

pub fn dump_midi_event(bin: &Vec<u8>, pos: &mut usize, info: &mut MidiReaderInfo) -> String {
    let p = *pos;
    let event_type = bin[p] & 0xF0;
    match event_type {
        0x80 => { // note on
            let msg = format!("NoteOff(${:02x},${:02x}) // {}", bin[p+1], bin[p+2], note_no_dec(bin[p+1]));
            *pos += 3;
            msg
        },
        0x90 => { // note off
            let msg = format!("NoteOn(${:02x},${:02x})  // {},,{}", bin[p+1], bin[p+2], note_no_dec(bin[p+1]), bin[p+2]);
            *pos += 3;
            msg
        },
        0xA0 => {
            let msg = format!("DirectSMF(${:02x},${:02x},${:02x})", bin[p], bin[p+1], bin[p+2]);
            *pos += 3;
            msg
        },
        0xB0 => { // CC
            let msg = format!("CC(${:02x},${:02x})", bin[p+1], bin[p+2]);
            *pos += 3;
            msg
        },
        0xC0 => { // Voice
            let msg = format!("Voice({})", bin[p+1]);
            *pos += 2;
            msg
        },
        0xD0 => { // Channel after touch
            let msg = format!("DirectSMF(${:02x},${:02x}) // Channel after touch", bin[p], bin[p+1]);
            *pos += 2;
            msg
        },
        0xE0 => { // PitchBend
            // PichBend is Little Endian!!
            let vv: isize = (((bin[p+2] as isize) << 7) | bin[p+1] as isize) - 8192;
            let vv2 = vv + 8192;
            let pb: isize = ((vv + 8192) >> 7) & 0x7F;
            let msg = format!("PitchBend({}) /*vv={} vv2={}  p{}*/", vv, vv, vv2, pb);
            *pos += 3;
            msg
        },
        0xF0 => { // Meta
            dump_midi_event_meta(bin, pos, info)
        },
        _ => {
            format!("[ERROR] Unknown event...={:02x}", event_type)
        }
    }
}

pub fn dump_midi(bin: &Vec<u8>, flag_stdout: bool) -> String {
    let mut info = MidiReaderInfo::new();
    let mut res = String::new();
    let mut log = |s: &str| {
        res.push_str(s);
        res.push('\n');
        if flag_stdout { println!("{}", s); }
    };
    let mut pos = 0;
    let s = array_read_str(bin, pos, 4);
    if s != "MThd" {
        log("[ERROR] Not Midi file");
        return res;
    }
    pos += 4;
    let mthd_size = array_read_u32(bin, pos);
    if mthd_size != 6 {
        log(&format!("[ERROR] Midi MThd size error 6!={}", mthd_size));
        return res;
    }
    pos += 4;
    let smf_format = array_read_u16(bin, pos);
    if smf_format > 3 {
        log("[ERROR] Midi Format error");
        return res;
    }
    log("// ----- MIDI DUMP DATA -----");
    log(&format!("/// [MThd] midi format={}", smf_format));
    pos += 2;
    let track_count = array_read_u16(bin, pos);
    log(&format!("/// [MThd] track_count={}", track_count));
    pos += 2;
    let timebase = array_read_u16(bin, pos) as usize;
    log(&format!("TIMEBASE={}", timebase));
    pos += 2;
    // tracks
    for no in 0..track_count {
        log(&format!("// ----- TRACK -----"));
        log(&format!("TRACK({})", no + 1));
        let mtrk = array_read_str(bin, pos, 4);
        if mtrk != "MTrk" {
            log(&format!("[ERROR] Track header broken MTrk!={}", mtrk));
            return res;
        }
        pos += 4;
        let mtrk_size = array_read_u32(bin, pos);
        // log(&format!("// [MTrk] track_block_size={}B", mtrk_size));
        pos += 4;
        let mut time = 0;
        // loop track
        let end_pos = pos + mtrk_size as usize;
        while pos < end_pos || !info.is_eot {
            let delta_time = array_readl_delta_time(bin, &mut pos);
            time += delta_time;
            let beat_base = (timebase as f32 * 4.0 / info.deno as f32) as usize;
            let beat_base = if beat_base == 0 { timebase } else { beat_base }; // for divisor of zero
            let tick = time % beat_base;
            let base = time / beat_base;
            let beat = base %  info.frac + 1;
            let mes = base / info.frac + 1;
            //
            let desc = dump_midi_event(bin, &mut pos, &mut info);
            // log(&format!("{:5}|TIME({:03}:{:03}:{:03}) {}", time, mes, beat, tick, desc));
            log(&format!("TIME({:03}:{:03}:{:03}) {}", mes, beat, tick, desc));
        }
        info.is_eot = false;
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_delta() {
        //
        let mut res = vec![];
        array_push_delta(&mut res, 0);
        assert_eq!(res[0], 0);

        // 1111 1111 => 1000 0001 01111111
        let mut res = vec![];
        array_push_delta(&mut res, 0xFF);
        assert_eq!(res[0], 0x81);
        assert_eq!(res[1], 0x7F);

        // 1111 1111 1111 1111 => 1000 0011 11111111 01111111
        let mut res = vec![];
        array_push_delta(&mut res, 0xFFFF);
        println!("{:?}", res);
        assert_eq!(res[0], 0x83);
        assert_eq!(res[1], 0xFF);
        assert_eq!(res[2], 0x7F);
    }
}
