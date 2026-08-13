//! song: トラックと演奏パラメータの管理
use super::*;
use std::collections::HashMap;

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

/// 先行指定(CC・ピッチベンド)の書き込み先
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteTarget {
    /// コントロールチェンジ(番号)
    CC(isize),
    /// ピッチベンド(1=PB:-8192〜8191 / 0=p:0〜127)
    PitchBend(isize),
}

impl WriteTarget {
    /// オプション(Delay/Random/Range/Repeat)を保存するときのキー
    /// ピッチベンドは `PB` と `p` で設定を共有する
    fn opt_key(&self) -> isize {
        match self {
            WriteTarget::CC(no) => *no,
            WriteTarget::PitchBend(_) => -1,
        }
    }
    /// 同じ書き込み先か(ピッチベンドは大小の書式を区別しない)
    fn is_same(&self, other: &WriteTarget) -> bool {
        self.opt_key() == other.opt_key()
    }
}

/// 音符ごとの波形の書き込み方法
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WaveMode {
    /// 指定した長さのまま書き込む (.onNoteWave)
    Normal,
    /// 音符の長さに合わせて伸縮させる (.onNoteWaveEx)
    Expand,
    /// 音符が鳴っている間くり返す (.onNoteWaveR)
    Repeat,
}

/// 正弦波の種類 (.Sine/.onNoteSine の第1引数)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SineType {
    /// 0: low→high→low と1周する正弦波
    Sine,
    /// 1: low→high へ上がる 1/4 周期の正弦波
    UpSine,
    /// 2: high→low へ下がる 1/4 周期の正弦波
    DownSine,
}

impl SineType {
    pub fn from_i(v: isize) -> SineType {
        match v {
            1 => SineType::UpSine,
            2 => SineType::DownSine,
            _ => SineType::Sine,
        }
    }
}

/// 先行指定の共通オプション (.Delay/.Random/.Range/.Repeat)
#[derive(Debug, Clone)]
pub struct WriteOption {
    /// 書き込み位置の遅延 (.Delay)
    pub delay: isize,
    /// 書き込む値に足すランダムの幅 (.Random)
    pub random: isize,
    /// 書き込む値の下限と上限 (.Range)
    pub range: Option<(isize, isize)>,
    /// .onNote などで値をくり返すか (.Repeat)
    pub repeat: bool,
}

impl WriteOption {
    pub fn new() -> Self {
        WriteOption { delay: 0, random: 0, range: None, repeat: false }
    }
}

/// 先行指定を書き込むときに必要な情報
/// (乱数は曲全体で1つの系列を使うので、呼び出し元から借りて使う)
pub struct WriteCtx<'a> {
    pub timebase: isize,
    pub rand_seed: &'a mut u32,
}

impl<'a> WriteCtx<'a> {
    /// Song::rand と同じ xorshift 乱数
    fn rand(&mut self) -> u32 {
        let mut y = *self.rand_seed;
        y ^= y << 13;
        y ^= y >> 17;
        y ^= y << 5;
        *self.rand_seed = y;
        y
    }
    /// Song::calc_rand_value と同じ計算
    fn calc_rand_value(&mut self, val: isize, rand_v: isize) -> isize {
        if rand_v <= 0 {
            return val;
        }
        let r = self.rand();
        let r = (r as isize) % rand_v - (rand_v / 2);
        val + r
    }
}

/// 音符ごとの値の先行指定 (.onNote/.onCycle)
#[derive(Debug, Clone)]
pub struct OnNoteValues {
    pub target: WriteTarget,
    pub data: Vec<isize>,
    pub index: isize,
    /// 末尾まで進んだら先頭に戻るか (.Repeat(on) / .onCycle)
    pub is_cycle: bool,
}

/// 音符ごとの波形の先行指定 (.onNoteWave/.onNoteWaveEx/.onNoteWaveR)
#[derive(Debug, Clone)]
pub struct OnNoteWave {
    pub target: WriteTarget,
    pub data: Vec<isize>,
    pub mode: WaveMode,
}

/// 音符ごとの正弦波の先行指定 (.onNoteSine)
#[derive(Debug, Clone)]
pub struct OnNoteSine {
    pub target: WriteTarget,
    pub stype: SineType,
    pub low: isize,
    pub high: isize,
    pub len: isize,
    pub times: isize,
}

/// 一定時間ごとの値の先行指定 (.onCycle) --- CC・ピッチベンド用
#[derive(Debug, Clone)]
pub struct OnCycleValues {
    pub target: WriteTarget,
    /// 書き込みの周期(ステップ数)
    pub len: isize,
    pub data: Vec<isize>,
    /// 次に書き込む時刻 (.Delay を含まない基準時刻)
    pub next_time: isize,
    pub index: usize,
}

/// 後方互換のための別名 (旧: ControlChangeOnNoteWave)
pub type ControlChangeOnNoteWave = OnNoteWave;

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

/// 一定時間ごとの値の先行指定 (.onCycle)
/// 解除するまで、値を周期的にくり返す
#[derive(Debug, Clone)]
pub struct OnCycleTime {
    /// 開始時刻
    pub start_time: isize,
    /// 周期(ステップ数)
    pub len: isize,
    pub values: Vec<isize>,
}

impl OnCycleTime {
    /// 指定した時刻の値を求める
    pub fn calc(&self, timepos: isize) -> Option<isize> {
        if self.len <= 0 || self.values.len() == 0 {
            return None;
        }
        if timepos < self.start_time {
            return None;
        }
        let index = ((timepos - self.start_time) / self.len) as usize;
        Some(self.values[index % self.values.len()])
    }
}

/// 音符属性(v/q/t/o/l)の先行指定の状態
#[derive(Debug, Clone)]
pub struct NoteParam {
    /// .onTime の開始時刻 (-1で未使用)
    pub on_time_start: isize,
    pub on_time: Option<Vec<isize>>,
    /// .onCycle --- 一定時間ごとに値を切り替える
    pub on_cycle: Option<OnCycleTime>,
    pub on_note: Option<Vec<isize>>,
    pub on_note_index: isize,
    pub on_note_is_cycle: bool,
    /// .Random の幅
    pub random: isize,
    /// .Range の下限と上限
    pub range: Option<(isize, isize)>,
    /// .Delay --- .onTime の開始位置をずらす
    pub delay: isize,
    /// .Max --- 値の上限 (isize::MAX で未設定)
    pub max: isize,
    /// .Repeat --- 次に指定する .onNote をくり返すか
    pub repeat: bool,
}

impl NoteParam {
    pub fn new() -> Self {
        NoteParam {
            on_time_start: -1,
            on_time: None,
            on_cycle: None,
            on_note: None,
            on_note_index: 0,
            on_note_is_cycle: false,
            random: 0,
            range: None,
            delay: 0,
            max: isize::MAX,
            repeat: false,
        }
    }
    /// 先行指定(.onTime/.onCycle/.onNote)を解除する --- 通常の値指定をしたとき
    pub fn clear_reserve(&mut self) {
        self.on_time = None;
        self.on_time_start = -1;
        self.on_cycle = None;
        self.on_note = None;
        self.on_note_index = 0;
    }
    /// .onTime を予約する
    pub fn set_on_time(&mut self, timepos: isize, values: Vec<isize>) {
        self.clear_reserve();
        self.on_time_start = timepos + self.delay;
        self.on_time = Some(values);
    }
    /// .onCycle を予約する --- (ステップ値, 値1, 値2, ...)
    pub fn set_on_cycle(&mut self, timepos: isize, args: Vec<isize>) {
        self.clear_reserve();
        if args.len() < 2 {
            return;
        }
        self.on_cycle = Some(OnCycleTime {
            start_time: timepos + self.delay,
            len: args[0],
            values: args[1..].to_vec(),
        });
    }
    /// .onNote を予約する
    pub fn set_on_note(&mut self, values: Vec<isize>) {
        self.clear_reserve();
        self.on_note = Some(values);
        self.on_note_index = 0;
        self.on_note_is_cycle = self.repeat;
    }
    /// 時間ベースの先行指定(.onTime/.onCycle)の現在時刻での値を求める
    pub fn calc_on_time(&mut self, timepos: isize, def: isize) -> isize {
        // .onCycle --- 解除するまで周期的にくり返す
        if let Some(cycle) = &self.on_cycle {
            if let Some(v) = cycle.calc(timepos) {
                return v;
            }
            return def;
        }
        let ia = match &self.on_time {
            None => return def,
            Some(pia) => pia.clone(),
        };
        let cur_time = timepos - self.on_time_start;
        let mut result = isize::MIN;
        let mut area_time = 0;
        for i in 0..ia.len() / 3 {
            let low = ia[i * 3 + 0];
            let high = ia[i * 3 + 1];
            let len = ia[i * 3 + 2];
            if len <= 0 {
                continue;
            }
            let area_time_to = area_time + len;
            if area_time <= cur_time && cur_time < area_time_to {
                let v = (high - low) as f32 * ((cur_time - area_time) as f32 / len as f32)
                    + low as f32;
                result = v as isize;
            }
            area_time = area_time_to;
        }
        // 指定した範囲を過ぎたら予約を解除する
        if area_time <= cur_time {
            self.on_time = None;
            self.on_time_start = -1;
        }
        if result == isize::MIN {
            result = def;
        }
        result
    }
    /// .onNote / .onCycle の次の値を求める
    pub fn calc_on_note(&mut self, def: isize) -> isize {
        let ia = match &self.on_note {
            None => return def,
            Some(pia) => pia.clone(),
        };
        if ia.len() == 0 {
            self.on_note = None;
            return def;
        }
        if self.on_note_index >= ia.len() as isize {
            if self.on_note_is_cycle {
                self.on_note_index = 0;
            } else {
                self.on_note = None;
                self.on_note_index = 0;
                return def;
            }
        }
        let v = ia[(self.on_note_index as usize) % ia.len()];
        self.on_note_index += 1;
        v
    }
    /// .Range と .Max を値に適用する (どちらも未設定なら値を変えない)
    pub fn apply_limit(&self, value: isize) -> isize {
        let mut v = value;
        if let Some((low, high)) = self.range {
            v = value_range(low, v, high);
        }
        if self.max != isize::MAX && v > self.max {
            v = self.max;
        }
        v
    }
    /// .Max が指定されていればその値を、なければ既定の上限を返す
    pub fn max_or(&self, def: isize) -> isize {
        if self.max == isize::MAX {
            def
        } else {
            self.max
        }
    }
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
    v_sub_on_cycle: Vec<Option<OnCycleTime>>,
    pub qlen: isize,
    pub timing: isize,
    pub port: isize,
    pub track_key: isize,
    pub tie_mode: TieMode, // Slur(#7)
    pub tie_value: isize,
    pub bend_range: isize,
    pub pitch_bend: isize,
    pub program_change: isize,
    /// 音符属性の先行指定 (v/q/t/o/l)
    pub v_opt: NoteParam,
    pub q_opt: NoteParam,
    pub t_opt: NoteParam,
    pub o_opt: NoteParam,
    pub l_opt: NoteParam,
    pub cc_on_time_freq: isize,
    /// ピッチベンドの書き込み頻度 (.Frequency)
    /// 0以下なら timebase/32 を使う
    pub pb_on_time_freq: isize,
    pub events: Vec<Event>,
    pub tie_notes: Vec<Event>,
    /// 音符ごとの値の先行指定 (CC・ピッチベンド)
    pub cc_on_note: Vec<OnNoteValues>,
    /// 音符ごとの波形の先行指定 (CC・ピッチベンド)
    pub cc_on_note_wave: Vec<OnNoteWave>,
    /// 音符ごとの正弦波の先行指定 (CC・ピッチベンド)
    pub cc_on_note_sine: Vec<OnNoteSine>,
    /// 一定時間ごとの値の先行指定 (CC・ピッチベンド)
    pub cc_on_cycle: Vec<OnCycleValues>,
    /// 先行指定の共通オプション (キー: CC番号 / -1=ピッチベンド)
    write_opt: HashMap<isize, WriteOption>,
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
            v_sub_on_cycle: vec![None],
            program_change: 0,
            cc_on_time_freq: 4,
            pb_on_time_freq: 0,
            v_opt: NoteParam::new(),
            q_opt: NoteParam::new(),
            t_opt: NoteParam::new(),
            o_opt: NoteParam::new(),
            l_opt: NoteParam::new(),
            channel,
            events: vec![],
            tie_notes: vec![],
            bend_range: -1,
            pitch_bend: 0,
            cc_on_note: vec![],
            cc_on_note_wave: vec![],
            cc_on_note_sine: vec![],
            cc_on_cycle: vec![],
            write_opt: HashMap::new(),
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
        let timepos = self.timepos;
        self.v_opt.calc_on_time(timepos, def)
    }
    pub fn calc_v_on_note(&mut self, def: isize) -> isize {
        let v = self.v_opt.calc_on_note(def);
        if self.v_opt.on_note.is_some() {
            self.velocity = v;
        }
        v
    }
    fn ensure_v_sub_index(&mut self, index: usize) {
        if self.v_sub.len() <= index {
            self.v_sub.resize(index + 1, 0);
            self.v_sub_rand.resize(index + 1, 0);
            self.v_sub_on_time.resize(index + 1, None);
            self.v_sub_on_note.resize(index + 1, None);
            self.v_sub_on_cycle.resize(index + 1, None);
        }
    }

    pub fn set_v_sub(&mut self, index: usize, velocity: isize) {
        self.ensure_v_sub_index(index);
        self.v_sub[index] = velocity;
        self.v_sub_on_time[index] = None;
        self.v_sub_on_note[index] = None;
        self.v_sub_on_cycle[index] = None;
    }

    pub fn set_v_sub_random(&mut self, index: usize, random: isize) {
        self.ensure_v_sub_index(index);
        self.v_sub_rand[index] = random;
    }

    pub fn set_v_sub_on_time(&mut self, index: usize, values: Vec<isize>) {
        self.ensure_v_sub_index(index);
        self.v_sub_on_note[index] = None;
        self.v_sub_on_cycle[index] = None;
        self.v_sub_on_time[index] = Some(VelocitySubOnTime {
            start_time: self.timepos + self.v_opt.delay,
            values,
        });
    }

    /// サブベロシティの .onCycle --- (ステップ値, 値1, 値2, ...)
    pub fn set_v_sub_on_cycle(&mut self, index: usize, args: Vec<isize>) {
        self.ensure_v_sub_index(index);
        self.v_sub_on_time[index] = None;
        self.v_sub_on_note[index] = None;
        self.v_sub_on_cycle[index] = None;
        if args.len() < 2 {
            return;
        }
        self.v_sub_on_cycle[index] = Some(OnCycleTime {
            start_time: self.timepos + self.v_opt.delay,
            len: args[0],
            values: args[1..].to_vec(),
        });
    }

    pub fn set_v_sub_on_note(&mut self, index: usize, values: Vec<isize>) {
        self.ensure_v_sub_index(index);
        self.v_sub_on_time[index] = None;
        self.v_sub_on_cycle[index] = None;
        self.v_sub_on_note[index] = Some(VelocitySubOnNote {
            values,
            index: 0,
            is_cycle: self.v_opt.repeat,
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
            // .onCycle --- 解除するまで周期的にくり返す
            if let Some(cycle) = &self.v_sub_on_cycle[index] {
                if let Some(v) = cycle.calc(self.timepos) {
                    sub_velocity = v;
                }
                total += sub_velocity;
                continue;
            }
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
    pub fn calc_t_on_time(&mut self, def: isize) -> isize {
        let timepos = self.timepos;
        self.t_opt.calc_on_time(timepos, def)
    }
    pub fn calc_t_on_note(&mut self, def: isize) -> isize {
        let t = self.t_opt.calc_on_note(def);
        if self.t_opt.on_note.is_some() {
            self.timing = t;
        }
        t
    }
    pub fn calc_qlen_on_time(&mut self, def: isize) -> isize {
        let timepos = self.timepos;
        self.q_opt.calc_on_time(timepos, def)
    }
    pub fn calc_qlen_on_note(&mut self, def: isize) -> isize {
        let qlen = self.q_opt.calc_on_note(def);
        if self.q_opt.on_note.is_some() {
            self.qlen = qlen;
        }
        qlen
    }
    pub fn calc_o_on_time(&mut self, def: isize) -> isize {
        let timepos = self.timepos;
        self.o_opt.calc_on_time(timepos, def)
    }
    pub fn calc_o_on_note(&mut self, def: isize) -> isize {
        let o = self.o_opt.calc_on_note(def);
        if self.o_opt.on_note.is_some() {
            self.octave = o;
        }
        o
    }
    pub fn calc_l_on_time(&mut self, def: isize) -> isize {
        let timepos = self.timepos;
        self.l_opt.calc_on_time(timepos, def)
    }
    pub fn calc_l_on_note(&mut self, def: isize) -> isize {
        self.l_opt.calc_on_note(def)
    }

    // ------------------------------------------------------------------
    // CC・ピッチベンドの先行指定
    // ------------------------------------------------------------------

    /// 書き込み先のオプションを取り出す(未設定なら既定値)
    pub fn get_write_opt(&self, target: WriteTarget) -> WriteOption {
        match self.write_opt.get(&target.opt_key()) {
            Some(opt) => opt.clone(),
            None => WriteOption::new(),
        }
    }
    /// 書き込み先のオプションを変更する
    pub fn update_write_opt<F: FnOnce(&mut WriteOption)>(&mut self, target: WriteTarget, f: F) {
        let opt = self
            .write_opt
            .entry(target.opt_key())
            .or_insert_with(WriteOption::new);
        f(opt);
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
    /// 書き込み先に応じたイベント種別
    fn target_event_type(target: WriteTarget) -> EventType {
        match target {
            WriteTarget::CC(_) => EventType::ControllChange,
            WriteTarget::PitchBend(_) => EventType::PitchBend,
        }
    }
    /// 書き込み先に応じた書き込み頻度(ステップ数)
    fn target_freq(&self, target: WriteTarget, timebase: isize) -> isize {
        match target {
            WriteTarget::CC(_) => self.cc_on_time_freq.max(1),
            WriteTarget::PitchBend(_) => {
                if self.pb_on_time_freq > 0 {
                    self.pb_on_time_freq
                } else {
                    (timebase / 32).max(1)
                }
            }
        }
    }
    /// 値を1つ書き込む (.Random/.Range/.Delay を適用する)
    fn push_value_event(
        &mut self,
        target: WriteTarget,
        time: isize,
        value: isize,
        ctx: &mut WriteCtx,
    ) {
        let opt = self.get_write_opt(target);
        let mut v = value;
        if opt.random > 0 {
            v = ctx.calc_rand_value(v, opt.random);
        }
        if let Some((low, high)) = opt.range {
            v = value_range(low, v, high);
        }
        let time = time + opt.delay;
        let ch = self.channel;
        match target {
            WriteTarget::CC(no) => {
                let v = value_range(0, v, 127);
                self.events.push(Event::cc(time, ch, no, v));
            }
            WriteTarget::PitchBend(is_big) => {
                let v = if is_big == 0 { v * 128 } else { v + 8192 };
                let v = value_range(0, v, 0x7f7f);
                self.events.push(Event::pitch_bend(time, ch, v));
            }
        }
    }
    /// 時間経過による値の変化を書き込む (.onTime の本体)
    pub fn write_on_time(&mut self, target: WriteTarget, ia: Vec<isize>, ctx: &mut WriteCtx) {
        let freq = self.target_freq(target, ctx.timebase);
        let opt = self.get_write_opt(target);
        let delay = opt.delay;
        // .Random を使うときは、同じ基準値でも書き込む値が変わるので重複を抑制しない
        let skip_same = opt.random <= 0;
        // 重なった古い書き込みを削除する (#78)
        let total = Self::calc_on_time_length(&ia);
        let cc_no = match target { WriteTarget::CC(no) => no, _ => 0 };
        let start = self.timepos + delay;
        self.remove_events_in_range(Self::target_event_type(target), cc_no, start, start + total);
        let mut elapsed = 0;
        let mut last_v: Option<isize> = None;
        for i in 0..ia.len() / 3 {
            let low = ia[i*3+0];
            let high = ia[i*3+1];
            let len = ia[i*3+2];
            if len <= 0 { continue; }
            for j in 0..len {
                if (j % freq) == 0 {
                    let v = (high - low) as f32 * (j as f32 / len as f32) + low as f32;
                    let v = v as isize;
                    // 直前と同じ値なら書き込まない (#78)
                    if skip_same && last_v == Some(v) { continue; }
                    last_v = Some(v);
                    let time = self.timepos + elapsed + j;
                    self.push_value_event(target, time, v, ctx);
                }
            }
            elapsed += len;
        }
    }
    /// 正弦波を書き込む (.Sine の本体)
    /// stype=Sine: low→high→low を1周期、UpSine: low→high、DownSine: high→low
    pub fn write_sine(
        &mut self,
        target: WriteTarget,
        stype: SineType,
        low: isize,
        high: isize,
        len: isize,
        times: isize,
        ctx: &mut WriteCtx,
    ) {
        if len <= 0 { return; }
        let times = if times <= 0 { 1 } else { times };
        let freq = self.target_freq(target, ctx.timebase);
        let opt = self.get_write_opt(target);
        let delay = opt.delay;
        // .Random を使うときは、同じ基準値でも書き込む値が変わるので重複を抑制しない
        let skip_same = opt.random <= 0;
        let total = len * times;
        let cc_no = match target { WriteTarget::CC(no) => no, _ => 0 };
        let start = self.timepos + delay;
        self.remove_events_in_range(Self::target_event_type(target), cc_no, start, start + total);
        let center = (low + high) as f32 / 2.0;
        let amp = (high - low) as f32 / 2.0;
        let mut last_v: Option<isize> = None;
        for j in 0..total {
            if (j % freq) != 0 { continue; }
            let rate = (j % len) as f32 / len as f32;
            let v = match stype {
                // 0: 1周期の正弦波 (low → high → low)
                SineType::Sine => {
                    center - amp * (rate * std::f32::consts::PI * 2.0).cos()
                }
                // 1: 1/4周期で low から high へ上がる
                SineType::UpSine => {
                    low as f32 + (high - low) as f32 * (rate * std::f32::consts::PI / 2.0).sin()
                }
                // 2: 1/4周期で high から low へ下がる
                SineType::DownSine => {
                    low as f32 + (high - low) as f32 * (rate * std::f32::consts::PI / 2.0).cos()
                }
            };
            let v = v.round() as isize;
            if skip_same && last_v == Some(v) { continue; }
            last_v = Some(v);
            self.push_value_event(target, self.timepos + j, v, ctx);
        }
    }
    /// (旧API) CCの時間変化を書き込む
    pub fn write_cc_on_time(&mut self, cc_no: isize, ia: Vec<isize>, ctx: &mut WriteCtx) {
        self.write_on_time(WriteTarget::CC(cc_no), ia, ctx);
    }
    /// (旧API) ピッチベンドの時間変化を書き込む
    pub fn write_pb_on_time(&mut self, is_big: isize, ia: Vec<isize>, ctx: &mut WriteCtx) {
        self.write_on_time(WriteTarget::PitchBend(is_big), ia, ctx);
    }

    /// 先行指定(.onNote/.onNoteWave/.onCycle など)をすべて解除する
    pub fn remove_reserve(&mut self, target: WriteTarget) {
        self.cc_on_note.retain(|it| !it.target.is_same(&target));
        self.cc_on_note_wave.retain(|it| !it.target.is_same(&target));
        self.cc_on_note_sine.retain(|it| !it.target.is_same(&target));
        self.cc_on_cycle.retain(|it| !it.target.is_same(&target));
    }
    /// .onNote の値をくり返すかどうかを設定する (.Repeat)
    /// すでに予約されている同じ書き込み先の .onNote にも反映する
    pub fn set_repeat(&mut self, target: WriteTarget, on: bool) {
        self.update_write_opt(target, |opt| opt.repeat = on);
        for it in self.cc_on_note.iter_mut() {
            if it.target.is_same(&target) {
                it.is_cycle = on;
            }
        }
    }
    /// 音符ごとの値の先行指定を予約する (.onNote/.N)
    pub fn set_on_note(&mut self, target: WriteTarget, ia: Vec<isize>) {
        self.remove_reserve(target);
        let is_cycle = self.get_write_opt(target).repeat;
        self.cc_on_note.push(OnNoteValues { target, data: ia, index: 0, is_cycle });
    }
    /// 音符ごとの波形の先行指定を予約する (.onNoteWave/.onNoteWaveEx/.onNoteWaveR)
    pub fn set_on_note_wave(&mut self, target: WriteTarget, ia: Vec<isize>, mode: WaveMode) {
        self.remove_reserve(target);
        self.cc_on_note_wave.push(OnNoteWave { target, data: ia, mode });
    }
    /// 音符ごとの正弦波の先行指定を予約する (.onNoteSine)
    pub fn set_on_note_sine(&mut self, target: WriteTarget, sine: OnNoteSine) {
        self.remove_reserve(target);
        self.cc_on_note_sine.push(sine);
    }
    /// 一定時間ごとの値の先行指定を予約する (.onCycle)
    pub fn set_on_cycle(&mut self, target: WriteTarget, len: isize, data: Vec<isize>) {
        self.remove_reserve(target);
        if len <= 0 || data.len() == 0 { return; }
        // .Delay は書き込み時に push_value_event が適用するので、ここでは足さない
        let next_time = self.timepos;
        self.cc_on_cycle.push(OnCycleValues { target, len, data, next_time, index: 0 });
    }

    /// 音符の発音開始時に、予約した値を書き出す (.onNote)
    pub fn write_cc_on_note(&mut self, start_pos: isize, ctx: &mut WriteCtx) {
        for item in self.cc_on_note.clone().iter_mut() {
            if item.data.len() == 0 { continue; }
            let mut index = item.index;
            if index >= item.data.len() as isize {
                if !item.is_cycle { continue; }
                index = 0;
            }
            let v = item.data[index as usize];
            self.push_value_event(item.target, start_pos, v, ctx);
            // 予約の状態を更新する
            for it in self.cc_on_note.iter_mut() {
                if it.target.is_same(&item.target) {
                    it.index = index + 1;
                }
            }
        }
        // くり返さない予約は、値を使い切ったら解除する
        self.cc_on_note
            .retain(|it| it.is_cycle || it.data.len() > it.index as usize);
    }
    /// 音符の発音開始時に、予約した波形を書き出す (.onNoteWave系)
    pub fn write_cc_on_note_wave(&mut self, start_pos: isize, ctx: &mut WriteCtx) {
        if self.cc_on_note_wave.len() == 0 && self.cc_on_note_sine.len() == 0 { return; }
        let end_pos = self.timepos;
        let note_len = (end_pos - start_pos).max(0);
        self.timepos = start_pos;
        for wave in self.cc_on_note_wave.clone().iter() {
            let data = match wave.mode {
                WaveMode::Normal => wave.data.clone(),
                // 音符の長さに合わせて各区間を伸縮させる
                WaveMode::Expand => Self::expand_wave(&wave.data, note_len),
                // 音符が鳴っている間くり返す
                WaveMode::Repeat => Self::repeat_wave(&wave.data, note_len),
            };
            self.write_on_time(wave.target, data, ctx);
        }
        for sine in self.cc_on_note_sine.clone().iter() {
            self.write_sine(
                sine.target, sine.stype, sine.low, sine.high, sine.len, sine.times, ctx,
            );
        }
        self.timepos = end_pos;
    }
    /// 波形(low,high,len...)を、合計が note_len になるように伸縮させる (.onNoteWaveEx)
    fn expand_wave(ia: &[isize], note_len: isize) -> Vec<isize> {
        let total = Self::calc_on_time_length(ia);
        if total <= 0 || note_len <= 0 { return ia.to_vec(); }
        let mut result: Vec<isize> = vec![];
        let mut wrote = 0; // 端数の誤差を最後の区間で吸収する
        let count = ia.len() / 3;
        for i in 0..count {
            let len = ia[i * 3 + 2];
            let new_len = if i == count - 1 {
                note_len - wrote
            } else {
                (len as f64 * note_len as f64 / total as f64) as isize
            };
            wrote += new_len;
            result.push(ia[i * 3 + 0]);
            result.push(ia[i * 3 + 1]);
            result.push(new_len.max(0));
        }
        result
    }
    /// 波形(low,high,len...)を note_len の長さになるまでくり返す (.onNoteWaveR)
    fn repeat_wave(ia: &[isize], note_len: isize) -> Vec<isize> {
        let total = Self::calc_on_time_length(ia);
        if total <= 0 || note_len <= 0 { return ia.to_vec(); }
        let mut result: Vec<isize> = vec![];
        let mut rest = note_len;
        while rest > 0 {
            for i in 0..ia.len() / 3 {
                let len = ia[i * 3 + 2];
                if len <= 0 { continue; }
                if rest <= 0 { break; }
                let len = if len > rest { rest } else { len };
                result.push(ia[i * 3 + 0]);
                result.push(ia[i * 3 + 1]);
                result.push(len);
                rest -= len;
            }
        }
        result
    }
    /// 音符の発音開始時に、周期的な先行指定を書き出す (.onCycle)
    pub fn write_cc_on_cycle(&mut self, start_pos: isize, ctx: &mut WriteCtx) {
        if self.cc_on_cycle.len() == 0 { return; }
        // 前回の書き込みから周期が経過していれば、その分だけ値を書き込む
        let mut writes: Vec<(WriteTarget, isize, isize)> = vec![];
        for item in self.cc_on_cycle.iter_mut() {
            // 無限ループを避けるため、1音符あたりの書き込み回数を制限する
            let mut count = 0;
            while item.next_time <= start_pos && count < 1000 {
                let v = item.data[item.index % item.data.len()];
                writes.push((item.target, item.next_time, v));
                item.index += 1;
                item.next_time += item.len;
                count += 1;
            }
        }
        for (target, time, v) in writes.into_iter() {
            self.push_value_event(target, time, v, ctx);
        }
    }

    // ------------------------------------------------------------------
    // 旧APIの互換ラッパー
    // ------------------------------------------------------------------
    pub fn remove_cc_on(&mut self, no: isize) {
        self.remove_reserve(WriteTarget::CC(no));
    }
    pub fn remove_cc_on_note_wave(&mut self, no: isize) {
        let target = WriteTarget::CC(no);
        self.cc_on_note_wave.retain(|it| !it.target.is_same(&target));
        self.cc_on_note_sine.retain(|it| !it.target.is_same(&target));
    }
    pub fn set_cc_on_note_wave(&mut self, no: isize, ia: Vec<isize>) {
        self.set_on_note_wave(WriteTarget::CC(no), ia, WaveMode::Normal);
    }
    pub fn remove_cc_on_note(&mut self, no: isize) {
        let target = WriteTarget::CC(no);
        self.cc_on_note.retain(|it| !it.target.is_same(&target));
    }
    pub fn set_cc_on_note(&mut self, no: isize, ia: Vec<isize>) {
        self.set_on_note(WriteTarget::CC(no), ia);
    }
    /// ピッチベンドの音符ごとの波形変化を予約する
    pub fn set_pb_on_note_wave(&mut self, is_big: isize, ia: Vec<isize>) {
        self.set_on_note_wave(WriteTarget::PitchBend(is_big), ia, WaveMode::Normal);
    }
    /// ピッチベンドの先行指定をすべて解除する
    pub fn remove_pb_on_note_wave(&mut self) {
        self.remove_reserve(WriteTarget::PitchBend(1));
    }
}
