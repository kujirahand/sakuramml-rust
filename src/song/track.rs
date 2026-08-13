//! song: トラックと演奏パラメータの管理
use super::*;

/// NoteInfo
#[derive(Debug)]
pub struct NoteInfo {
    pub no: isize,
    pub flag: isize,
    pub natural: isize,
    pub len_s: String,
    pub qlen: isize,
    pub vel: isize,
    pub t: isize,
    pub o: isize,
    pub slur: isize,
}

#[derive(Debug, Clone)]
pub struct ControlChangeOnNoteWave {
    pub no: isize,
    pub data: Vec<isize>,
    pub index: isize, // for ControlChangeOnNote
}

#[derive(Debug, Clone)]
struct VelocitySubOnTime {
    start_time: isize,
    values: Vec<isize>,
}

#[derive(Debug, Clone)]
struct VelocitySubOnNote {
    values: Vec<isize>,
    index: usize,
    is_cycle: bool,
}

/// Track
#[derive(Debug)]
pub struct Track {
    pub timepos: isize,
    pub channel: isize,
    pub length: isize,
    pub octave: isize,
    pub velocity: isize,
    pub v_sub: Vec<isize>,
    pub v_sub_rand: Vec<isize>,
    v_sub_on_time: Vec<Option<VelocitySubOnTime>>,
    v_sub_on_note: Vec<Option<VelocitySubOnNote>>,
    pub qlen: isize,
    pub timing: isize,
    pub v_rand: isize,
    pub q_rand: isize,
    pub t_rand: isize,
    pub o_rand: isize,
    pub port: isize,
    pub track_key: isize,
    pub tie_mode: TieMode, // Slur(#7)
    pub tie_value: isize,
    pub bend_range: isize,
    pub pitch_bend: isize,
    pub program_change: isize,
    pub v_on_time_start: isize,
    pub v_on_time: Option<Vec<isize>>,
    pub v_on_note_index: isize,
    pub v_on_note_is_cycle: bool,
    pub v_on_note: Option<Vec<isize>>,
    pub q_on_note_index: isize,
    pub q_on_note: Option<Vec<isize>>,
    pub q_on_note_is_cycle: bool,
    pub t_on_note_index: isize,
    pub t_on_note: Option<Vec<isize>>,
    pub t_on_note_is_cycle: bool,
    pub o_on_note_index: isize,
    pub o_on_note: Option<Vec<isize>>,
    pub o_on_note_is_cycle: bool,
    pub l_on_note_index: isize,
    pub l_on_note: Option<Vec<isize>>,
    pub l_on_note_is_cycle: bool,
    pub cc_on_time_freq: isize,
    pub events: Vec<Event>,
    pub tie_notes: Vec<Event>,
    pub cc_on_note: Vec<ControlChangeOnNoteWave>,
    pub cc_on_note_wave: Vec<ControlChangeOnNoteWave>,
    /// ピッチベンドの音符ごとの波形変化 (is_big, データ列)
    pub pb_on_note_wave: Option<(isize, Vec<isize>)>,
}

impl Track {
    pub fn new(timebase: isize, channel: isize) -> Self {
        let channel = if channel < 0 { 0 } else if channel > 15 { 15 } else { channel };
        Track {
            timepos: 0,
            length: timebase,
            velocity: 100,
            octave: 5,
            qlen: 90,
            timing: 0,
            track_key: 0,
            port: 0,
            tie_mode: TieMode::Port,
            tie_value: 0,
            v_sub: vec![0],
            v_sub_rand: vec![0],
            v_sub_on_time: vec![None],
            v_sub_on_note: vec![None],
            v_rand: 0,
            q_rand: 0,
            t_rand: 0,
            o_rand: 0,
            program_change: 0,
            cc_on_time_freq: 4,
            v_on_time_start: -1,
            v_on_time: None,
            v_on_note_index: 0,
            v_on_note_is_cycle: false,
            v_on_note: None,
            q_on_note_index: 0,
            q_on_note_is_cycle: false,
            q_on_note: None,
            t_on_note_index: 0,
            t_on_note: None,
            t_on_note_is_cycle: false,
            o_on_note_index: 0,
            o_on_note: None,
            o_on_note_is_cycle: false,
            l_on_note_index: 0,
            l_on_note: None,
            l_on_note_is_cycle: false,
            channel,
            events: vec![],
            tie_notes: vec![],
            bend_range: -1,
            pitch_bend: 0,
            cc_on_note: vec![],
            cc_on_note_wave: vec![],
            pb_on_note_wave: None,
        }
    }

    pub fn split_note_off(&self) -> Vec<Event> {
        let mut events: Vec<Event> = vec![];
        for i in 0..self.events.len() {
            let e = &self.events[i];
            match e.etype {
                EventType::NoteOn => {
                    events.push(e.clone());
                    let mut noteoff = e.clone();
                    noteoff.etype = EventType::NoteOff;
                    noteoff.time = e.time + e.v2;
                    events.push(noteoff);
                },
                _ => {
                    events.push(e.clone());
                }
            }
        }
        events
    }

    pub fn normalize(&mut self) {
        let events: Vec<Event> = self.split_note_off();
        self.events = events;
    }
    pub fn events_sort(&mut self) {
        // sort_byなら要素の順序は保持される
        self.events.sort_by(|a, b| a.time.cmp(&b.time));
    }
    pub fn play_from(&mut self, timepos: isize) {
        let mut events: Vec<Event> = vec![];
        let mut cc_values: Vec<isize> = vec![];
        let mut voice: isize = -1;
        let mut ch: isize = 0;
        for _ in 0..128 { cc_values.push(-1); }
        for e in self.events.iter() {
            match e.etype {
                EventType::Meta | EventType::SysEx => {
                    let mut e2 = e.clone();
                    e2.time -= timepos;
                    if e2.time < 0 { e2.time = 0; }
                    events.push(e2);
                },
                EventType::NoteOn => {
                    let mut e2 = e.clone();
                    e2.time -= timepos;
                    if e2.time < 0 { continue; }
                    events.push(e2);
                },
                EventType::Voice => {
                    let mut e2 = e.clone();
                    e2.time -= timepos;
                    if e2.time < 0 {
                        voice = e2.v1;
                        ch = e2.channel;
                        continue;
                    }
                    events.push(e2);
                },
                EventType::ControllChange => {
                    let mut e2 = e.clone();
                    e2.time -= timepos;
                    if e2.time < 0 {
                        cc_values[e2.v1 as usize] = e2.v2;
                        ch = e2.channel;
                        continue;
                    }
                    events.push(e2);
                },
                EventType::NoteOff => {},
                EventType::PitchBend => {}, // TODO: #8
                EventType::PitchBendRange => {}, // TODO: #8
                EventType::DirectSMF => {},
            }
        }
        // add cc
        for no in 0..128 {
            if cc_values[no] < 0 { continue; }
            events.push(Event::cc(0, ch, no as isize, cc_values[no as usize]));
        }
        // voice
        if voice >= 0 {
            events.push(Event::voice(0, ch, voice));
        }
        self.events = events;
    }
    pub fn calc_v_on_time(&mut self, def: isize) -> isize {
        let start_time = self.v_on_time_start;
        let cur_time = self.timepos - start_time;
        let mut result = isize::MIN;
        // on_time?
        let ia = match &self.v_on_time {
            None => return def,
            Some(pia) => pia.clone()
        };
        let mut area_time = 0;
        for i in 0..ia.len() / 3 {
            let low = ia[i*3+0];
            let high = ia[i*3+1];
            let len = ia[i*3+2];
            let area_time_to = area_time + len;
            if area_time <= cur_time && cur_time < area_time_to {
                let v = (high - low) as f32 * ((cur_time - area_time) as f32 / len as f32) + low as f32;
                result = v as isize;
            }
            area_time = area_time_to;
        }
        // over ?
        if area_time <= cur_time {
            self.v_on_time = None;
            self.v_on_time_start = -1;
        }
        if result == isize::MIN { result = def; }
        result
    }
    pub fn calc_v_on_note(&mut self, def: isize) -> isize {
        // on_note?
        let ia = match &self.v_on_note {
            None => return def,
            Some(pia) => pia.clone()
        };
        if ia.len() == 0 { return def; }
        if self.v_on_note_index >= ia.len() as isize {
            if self.v_on_note_is_cycle {
                self.v_on_note_index = 0;
            } else {
                self.v_on_note = None;
                self.v_on_note_index = 0;
                return def;
            }
        }
        let v = ia[(self.v_on_note_index as usize) % ia.len()];
        self.velocity = v;
        self.v_on_note_index += 1;
        return v;
    }
    fn ensure_v_sub_index(&mut self, index: usize) {
        if self.v_sub.len() <= index {
            self.v_sub.resize(index + 1, 0);
            self.v_sub_rand.resize(index + 1, 0);
            self.v_sub_on_time.resize(index + 1, None);
            self.v_sub_on_note.resize(index + 1, None);
        }
    }

    pub fn set_v_sub(&mut self, index: usize, velocity: isize) {
        self.ensure_v_sub_index(index);
        self.v_sub[index] = velocity;
        self.v_sub_on_time[index] = None;
        self.v_sub_on_note[index] = None;
    }

    pub fn set_v_sub_random(&mut self, index: usize, random: isize) {
        self.ensure_v_sub_index(index);
        self.v_sub_rand[index] = random;
    }

    pub fn set_v_sub_on_time(&mut self, index: usize, values: Vec<isize>) {
        self.ensure_v_sub_index(index);
        self.v_sub_on_note[index] = None;
        self.v_sub_on_time[index] = Some(VelocitySubOnTime {
            start_time: self.timepos,
            values,
        });
    }

    pub fn set_v_sub_on_note(&mut self, index: usize, values: Vec<isize>, is_cycle: bool) {
        self.ensure_v_sub_index(index);
        self.v_sub_on_time[index] = None;
        self.v_sub_on_note[index] = Some(VelocitySubOnNote {
            values,
            index: 0,
            is_cycle,
        });
    }

    /// サブベロシティの先行指定を進め、各レイヤーを基準値へ加算する
    pub fn apply_v_sub(&mut self, velocity: isize) -> isize {
        let mut total = 0;
        for index in 0..self.v_sub.len() {
            let mut clear_on_note = false;
            if let Some(state) = &mut self.v_sub_on_note[index] {
                if state.values.len() == 0 {
                    clear_on_note = true;
                } else {
                    if state.index >= state.values.len() {
                        if state.is_cycle {
                            state.index = 0;
                        } else {
                            clear_on_note = true;
                        }
                    }
                    if !clear_on_note {
                        self.v_sub[index] = state.values[state.index];
                        state.index += 1;
                    }
                }
            }
            if clear_on_note {
                self.v_sub_on_note[index] = None;
            }

            let mut sub_velocity = self.v_sub[index];
            let mut clear_on_time = false;
            if let Some(state) = &self.v_sub_on_time[index] {
                let cur_time = self.timepos - state.start_time;
                let mut area_time = 0;
                for values in state.values.chunks_exact(3) {
                    let low = values[0];
                    let high = values[1];
                    let len = values[2];
                    if len <= 0 {
                        continue;
                    }
                    let area_time_to = area_time + len;
                    if area_time <= cur_time && cur_time < area_time_to {
                        let velocity = (high - low) as f32
                            * ((cur_time - area_time) as f32 / len as f32)
                            + low as f32;
                        sub_velocity = velocity as isize;
                    }
                    area_time = area_time_to;
                }
                if area_time <= cur_time {
                    clear_on_time = true;
                    sub_velocity = self.v_sub[index];
                }
            }
            if clear_on_time {
                self.v_sub_on_time[index] = None;
            }
            total += sub_velocity;
        }
        velocity + total
    }
    pub fn calc_t_on_note(&mut self, def: isize) -> isize {
        // on_note?
        let ia = match &self.t_on_note {
            None => return def,
            Some(pia) => pia.clone()
        };
        if ia.len() == 0 { return def; }
        if self.t_on_note_index >= ia.len() as isize {
            if self.t_on_note_is_cycle {
                self.t_on_note_index = 0;
            } else {
                self.t_on_note = None;
                self.t_on_note_index = 0;
                return def;
            }
        }
        let t = ia[(self.t_on_note_index as usize) % ia.len()];
        self.timing = t;
        self.t_on_note_index += 1;
        return t;
    }
    pub fn calc_qlen_on_note(&mut self, def: isize) -> isize {
        // on_note?
        let ia = match &self.q_on_note {
            None => return def,
            Some(pia) => pia.clone()
        };
        if ia.len() == 0 { return def; }
        if self.q_on_note_index >= ia.len() as isize {
            if self.q_on_note_is_cycle {
                self.q_on_note_index = 0;
            } else {
                self.q_on_note = None;
                self.q_on_note_index = 0;
                return def;
            }
        }
        let qlen = ia[(self.q_on_note_index as usize) % ia.len()];
        self.qlen = qlen;
        self.q_on_note_index += 1;
        return qlen;
    }
    pub fn calc_o_on_note(&mut self, def: isize) -> isize {
        // on_note?
        let ia = match &self.o_on_note {
            None => return def,
            Some(pia) => pia.clone()
        };
        if ia.len() == 0 {
            self.o_on_note = None;
            return def;
        }
        if self.o_on_note_index >= ia.len() as isize {
            if self.o_on_note_is_cycle {
                self.o_on_note_index = 0;
            } else {
                self.o_on_note = None;
                self.o_on_note_index = 0;
                return def;
            }
        }
        let o = ia[(self.o_on_note_index as usize) % ia.len()];
        self.octave = o;
        self.o_on_note_index += 1;
        return o;
    }
    pub fn calc_l_on_note(&mut self, def: isize) -> isize {
        // on_note?
        let ia = match &self.l_on_note {
            None => return def,
            Some(pia) => pia.clone()
        };
        if ia.len() == 0 {
            self.l_on_note = None;
            return def;
        }
        if self.l_on_note_index >= ia.len() as isize {
            if self.l_on_note_is_cycle {
                self.l_on_note_index = 0;
            } else {
                self.l_on_note = None;
                self.l_on_note_index = 0;
                return def;
            }
        }
        let l = ia[(self.l_on_note_index as usize) % ia.len()];
        self.l_on_note_index += 1;
        return l;
    }
    /// 先行指定(low,high,len...)が書き込む長さの合計を求める
    /// len が0以下の組は書き込まれないので合計に含めない
    fn calc_on_time_length(ia: &[isize]) -> isize {
        let mut total = 0;
        for i in 0..ia.len() / 3 {
            let len = ia[i*3+2];
            if len <= 0 { continue; }
            total += len;
        }
        total
    }
    /// これから波形を書き込む時間範囲にある、古い書き込みを削除する (#78)
    /// 先行指定が重なったとき、あとから指定した波形を優先させるため
    fn remove_events_in_range(&mut self, etype: EventType, cc_no: isize, start: isize, end: isize) {
        if start >= end { return; }
        self.events.retain(|e| {
            if e.etype != etype || e.time < start || end <= e.time { return true; }
            // コントロールチェンジは同じ番号のときだけ削除する
            if etype == EventType::ControllChange && e.v1 != cc_no { return true; }
            false
        });
    }
    /// 指定した時間範囲にある古いピッチベンドを削除する (#78)
    /// タイ・スラーのグリッサンドを、先に書き込まれた波形より優先させるため
    /// (start以上end以下の範囲。書き込む前に呼ぶこと)
    pub fn remove_pitch_bend_in_range(&mut self, start: isize, end: isize) {
        self.remove_events_in_range(EventType::PitchBend, 0, start, end + 1);
    }
    pub fn write_cc_on_time(&mut self, cc_no: isize, ia: Vec<isize>) {
        let freq = self.cc_on_time_freq.max(1);
        // 重なった古い書き込みを削除する (#78)
        let total = Self::calc_on_time_length(&ia);
        self.remove_events_in_range(EventType::ControllChange, cc_no, self.timepos, self.timepos + total);
        let mut elapsed = 0;
        let mut last_v: Option<isize> = None;
        for i in 0..ia.len() / 3 {
            let low = ia[i*3+0];
            let high = ia[i*3+1];
            let len = ia[i*3+2];
            if len <= 0 { continue; }
            // println!("CC.T={},{},{}", low, high, len);
            for j in 0..len {
                if (j % freq) == 0 {
                    let v = (high - low) as f32 * (j as f32 / len as f32) + low as f32;
                    let v = value_range(0, v as isize, 127);
                    // 直前と同じ値なら書き込まない (#78)
                    if last_v == Some(v) { continue; }
                    last_v = Some(v);
                    let e = Event::cc(self.timepos + elapsed + j, self.channel, cc_no, v);
                    self.events.push(e);
                }
            }
            elapsed += len;
        }
    }
    pub fn write_pb_on_time(&mut self, is_big: isize, ia: Vec<isize>, timebase: isize) {
        let freq = timebase / 32;
        // 重なった古い書き込みを削除する (#78)
        let total = Self::calc_on_time_length(&ia);
        self.remove_events_in_range(EventType::PitchBend, 0, self.timepos, self.timepos + total);
        let mut elapsed = 0;
        let mut last_v: Option<isize> = None;
        for i in 0..ia.len() / 3 {
            let mut low = ia[i*3+0];
            let mut high = ia[i*3+1];
            if is_big == 0 { // small
                low = low * 128;
                high = high * 128;
            } else { // big
                low += 8192;
                high += 8192;
            }
            // println!("@@@PB.T={},{}", low,high);
            let len = ia[i*3+2];
            if len <= 0 { continue; }
            for j in 0..len {
                if (j % freq) == 0 {
                    let v = (high - low) as f32 * (j as f32 / len as f32) + low as f32;
                    let v = value_range(0, v as isize, 0x7f7f);
                    // 直前と同じ値なら書き込まない (#78)
                    if last_v == Some(v) { continue; }
                    last_v = Some(v);
                    let e = Event::pitch_bend(self.timepos + elapsed + j, self.channel, v);
                    self.events.push(e);
                }
            }
            elapsed += len;
        }
    }
    /// ピッチベンドの音符ごとの波形変化を予約する
    pub fn set_pb_on_note_wave(&mut self, is_big: isize, ia: Vec<isize>) {
        self.pb_on_note_wave = Some((is_big, ia));
    }
    /// ピッチベンドの音符ごとの波形変化を解除する
    pub fn remove_pb_on_note_wave(&mut self) {
        self.pb_on_note_wave = None;
    }
    /// 音符の先頭からピッチベンドの波形を書き出す
    pub fn write_pb_on_note_wave(&mut self, start_pos: isize, timebase: isize) {
        let (is_big, ia) = match &self.pb_on_note_wave {
            None => return,
            Some((is_big, ia)) => (*is_big, ia.clone()),
        };
        let end_pos = self.timepos;
        self.timepos = start_pos;
        self.write_pb_on_time(is_big, ia, timebase);
        self.timepos = end_pos;
    }
    pub fn remove_cc_on(&mut self, no: isize) {
        self.remove_cc_on_note(no);
        self.remove_cc_on_note_wave(no);
    }
    pub fn remove_cc_on_note_wave(&mut self, no: isize) {
        if self.cc_on_note_wave.len() == 0 { return; }
        let mut new_list: Vec<ControlChangeOnNoteWave> = vec![];
        for cow in self.cc_on_note_wave.iter() {
            if cow.no == no { continue; }
            new_list.push(cow.clone());
        }
        self.cc_on_note_wave = new_list;
    }
    pub fn set_cc_on_note_wave(&mut self, no: isize, ia: Vec<isize>) {
        self.remove_cc_on(no);
        let cc_new = ControlChangeOnNoteWave { no, data: ia, index: 0 };
        self.cc_on_note_wave.push(cc_new);
    }
    pub fn write_cc_on_note_wave(&mut self, start_pos: isize) {
        let end_pos = self.timepos;
        // let _wave_len = end_pos - start_pos;
        self.timepos = start_pos;
        for cow in self.cc_on_note_wave.clone().iter() {
            // println!("write_cc_on_note_wave:no={}", cow.no);
            self.write_cc_on_time(cow.no, cow.data.clone());
        }
        self.timepos = end_pos;
    }
    pub fn remove_cc_on_note(&mut self, no: isize) {
        if self.cc_on_note.len() == 0 { return; }
        let mut new_list: Vec<ControlChangeOnNoteWave> = vec![];
        for cow in self.cc_on_note.iter() {
            if cow.no == no { continue; }
            new_list.push(cow.clone());
        }
        self.cc_on_note = new_list;
    }
    pub fn set_cc_on_note(&mut self, no: isize, ia: Vec<isize>) {
        self.remove_cc_on(no);
        let cc_new = ControlChangeOnNoteWave { no, data: ia, index: 0 };
        self.cc_on_note.push(cc_new);
    }
    pub fn write_cc_on_note(&mut self, start_pos: isize) {
        for it in self.cc_on_note.iter_mut() {
            if it.data.len() <= it.index as usize {
                continue;
            }
            let v = it.data[it.index as usize];
            it.index += 1;
            let e = Event::cc(start_pos, self.channel, it.no, v);
            self.events.push(e);
        }
        self.cc_on_note.retain(|it| it.data.len() > it.index as usize);
    }
}
