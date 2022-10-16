use super::song::{Song, Track, EventType};

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
            EventType::NoteNo => {
                let note_no = e.v1;
                let note_len = e.v2;
                let note_vel = e.v3;
                // note on
                array_push_delta(&mut res, e.time - timepos);
                res.push(0x90 + e.channel as u8);
                res.push(note_no as u8); // note_no
                res.push(note_vel as u8); // velocity
                // note off
                timepos = e.time + note_len;
                array_push_delta(&mut res, note_len);
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
            EventType::PitchBend => {
                array_push_delta(&mut res, e.time - timepos);
                timepos = e.time;
                let v = e.v1;
                res.push(0xE0 + e.channel as u8);
                res.push((v >> 7 | 0x7F) as u8);
                res.push((v >> 0 | 0x7F) as u8);
            }
        }
    }
    // end of track
    res.push(00);
    res.push(0xFF);
    res.push(0x2F);
    res.push(00);
    res
}

pub fn generate(song: &mut Song) -> Vec<u8> {
    let midi_format = 1;
    let mut res: Vec<u8> = vec![];
    song.sort_all_events();
    // header
    array_push_str(&mut res, "MThd");
    array_push_u32(&mut res, 6);
    array_push_u16(&mut res, midi_format);
    array_push_u16(&mut res, song.tracks.len() as isize);
    array_push_u16(&mut res, song.timebase);
    // tracks
    for track_no in 0..song.tracks.len() {
        if song.debug {
            println!("TR={}", track_no);
        }
        let trk = &song.tracks[track_no];
        let block = generate_track(&trk);
        array_push_str(&mut res, "MTrk");
        array_push_u32(&mut res, block.len() as isize);
        for b in block { res.push(b); }
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
