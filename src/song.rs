/// song & track
use std::collections::HashMap;
use super::svalue::SValue;
use super::sakura_version;

#[derive(Debug, Clone)]
pub enum EventType {
    NoteNo,
    NoteOff,
    ControllChange,
    PitchBend,
    Voice,
    Meta,
    SysEx,
}

#[allow(dead_code)]
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
        Self { etype: EventType::NoteNo, time, channel, v1: note_no, v2: len, v3: vel, data: None }
    }
    pub fn voice(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::Voice, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
    pub fn meta(time: isize, v1: isize, v2: isize, v3: isize, data_v: Vec<u8>) -> Self {
        Self { etype: EventType::Meta, time, channel: 0, v1, v2, v3, data: Some(data_v) }
    }
    pub fn sysex(time: isize, data_v: &Vec<SValue>) -> Self {
        let mut a: Vec<u8> = vec![];
        for v in data_v.iter() {
            a.push(v.to_i() as u8);
        }
        Self { etype: EventType::SysEx, time, channel: 0, v1: 0, v2: 0, v3: 0, data: Some(a) }
    }
    pub fn cc(time: isize, channel: isize, no: isize, value: isize) -> Self {
        Self { etype: EventType::ControllChange, time, channel, v1: no, v2: value, v3:0, data: None }
    }
    pub fn pitch_bend(time: isize, channel: isize, value: isize) -> Self {
        Self { etype: EventType::PitchBend, time, channel, v1: value, v2: 0, v3: 0, data: None }
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct Track {
    pub timepos: isize,
    pub channel: isize,
    pub length: isize,
    pub octave: isize,
    pub velocity: isize,
    pub qlen: isize,
    pub timing: isize,
    pub v_rand: isize,
    pub t_rand: isize,
    pub events: Vec<Event>,
}

impl Track {
    pub fn new(timebase: isize, channel: isize) -> Self {
        Track {
            timepos: 0,
            length: timebase,
            velocity: 100,
            octave: 5,
            qlen: 90,
            timing: 0,
            v_rand: 0,
            t_rand: 0,
            channel,
            events: vec![],
        }
    }
    pub fn normalize(&mut self) {
        let mut events: Vec<Event> = vec![];
        for i in 0..self.events.len() {
            let e = &self.events[i];
            match e.etype {
                EventType::NoteNo => {
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
        self.events = events;
    }
    pub fn events_sort(&mut self) {
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
                EventType::NoteNo => {
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
                EventType::PitchBend => {},
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
}

#[derive(Debug)]
pub struct Flags {
    pub harmony_flag: bool,
    pub harmony_time: isize,
    pub harmony_events: Vec<Event>,
    pub octave_once: isize,
}
impl Flags {
    pub fn new() -> Self {
        Flags {
            harmony_flag: false,
            harmony_time: 0,
            harmony_events: vec![],
            octave_once: 0,
        }
    }
}

#[derive(Debug)]
pub struct Song {
    pub debug: bool,
    pub tracks: Vec<Track>,
    pub timebase: isize,
    pub cur_track: usize,
    pub timesig_frac: isize, // 分子
    pub timesig_deno: isize, // 分母
    pub flags: Flags,
    pub rhthm_macro: Vec<String>,
    pub variables: HashMap<String, SValue>,
    pub key_flag: Vec<isize>,
    pub key_shift: isize,
    pub play_from: isize,
    pub logs: Vec<String>, // ログ
    rand_seed: u32,
}

impl Song {
    pub fn new() -> Self {
        let timebase = 96;
        let trk = Track::new(timebase, 0);
        Self {
            debug: false,
            timebase,
            tracks: vec![trk],
            cur_track: 0,
            timesig_frac: 4,
            timesig_deno: 4,
            flags: Flags::new(),
            rhthm_macro: init_rhythm_macro(),
            variables: init_variables(),
            key_flag: vec![0,0,0,0,0,0,0,0,0,0,0,0],
            key_shift: 0,
            play_from: -1,
            logs: vec![],
            rand_seed: 1110122942, // Random Seed
        }
    }
    pub fn add_event(&mut self, e: Event) {
        self.tracks[self.cur_track].events.push(e);
    }
    pub fn normalize_and_sort(&mut self) {
        for trk in self.tracks.iter_mut() {
            trk.normalize();
            trk.events_sort();
        }
    }
    pub fn play_from_all_track(&mut self) {
        if self.play_from < 0 { return; }
        if self.debug { println!("PLAY_FROM={}", self.play_from); }
        for trk in self.tracks.iter_mut() {
            trk.play_from(self.play_from);
        }
    }
    pub fn calc_rand_value(&mut self, val: isize, rand_v: isize) -> isize {
        let r = self.rand();
        let r = (r as isize) % rand_v - (rand_v / 2);
        val + r
    }
    pub fn rand(&mut self) -> u32 {
        let mut y = self.rand_seed;
        y ^= y << 13;
        y ^= y >> 17;
        y ^= y << 5;
        self.rand_seed = y;
        y
    }
}

fn init_rhythm_macro() -> Vec<String> {
    // Rhythm macro ... 1 char macro
    let mut rhthm_macro: Vec<String> = vec![];
    for _ in 0x40..=0x7F {
        rhthm_macro.push(String::new());
    }
    // set
    // <RHYTHM_MACRO>
    rhthm_macro['b' as usize - 0x40] = String::from("n36,");
    rhthm_macro['s' as usize - 0x40] = String::from("n38,");
    rhthm_macro['h' as usize - 0x40] = String::from("n42,");
    rhthm_macro['H' as usize - 0x40] = String::from("n44,");
    rhthm_macro['o' as usize - 0x40] = String::from("n46,");
    rhthm_macro['c' as usize - 0x40] = String::from("n49,");
    // </RHYTHM_MACRO>
    //
    rhthm_macro
}

fn init_variables() -> HashMap<String, SValue> {
    let mut var = HashMap::new();
    let version = sakura_version::version_str();
    //<VARIABLES>
    var.insert(String::from("SAKURA_VERSION"), SValue::from_s(version));
    //
    var.insert(String::from("GrandPiano"), SValue::from_i(1));
    var.insert(String::from("BrightPiano"), SValue::from_i(2));
    var.insert(String::from("ElectricGrandPiano"), SValue::from_i(3));
    var.insert(String::from("HonkyTonkPiano"), SValue::from_i(4));
    var.insert(String::from("ElectricPiano1"), SValue::from_i(5));
    var.insert(String::from("ElectricPiano2"), SValue::from_i(6));
    var.insert(String::from("Harpsichord"), SValue::from_i(7));
    var.insert(String::from("Clavi"), SValue::from_i(8));
    var.insert(String::from("CelestaStrings"), SValue::from_i(9));
    var.insert(String::from("Glockenspiel"), SValue::from_i(10));
    var.insert(String::from("MusicBox"), SValue::from_i(11));
    var.insert(String::from("Vibraphone"), SValue::from_i(12));
    var.insert(String::from("Marimba"), SValue::from_i(13));
    var.insert(String::from("Xylophone"), SValue::from_i(14));
    var.insert(String::from("TubularBells"), SValue::from_i(15));
    var.insert(String::from("Dulcimer"), SValue::from_i(16));
    var.insert(String::from("DrawbarOrgan"), SValue::from_i(17));
    var.insert(String::from("PercussiveOrgan"), SValue::from_i(18));
    var.insert(String::from("RockOrgan"), SValue::from_i(19));
    var.insert(String::from("ChurchOrgan"), SValue::from_i(20));
    var.insert(String::from("ReedOrgan"), SValue::from_i(21));
    var.insert(String::from("Accordion"), SValue::from_i(22));
    var.insert(String::from("Hamonica"), SValue::from_i(23));
    var.insert(String::from("TangoAccordion"), SValue::from_i(24));
    var.insert(String::from("NylonGuitar"), SValue::from_i(25));
    var.insert(String::from("SteelcGuitar"), SValue::from_i(26));
    var.insert(String::from("JazzGuitar"), SValue::from_i(27));
    var.insert(String::from("CleanGuitar"), SValue::from_i(28));
    var.insert(String::from("MutedGuitar"), SValue::from_i(29));
    var.insert(String::from("OverdrivenGuitar"), SValue::from_i(30));
    var.insert(String::from("DistortionGuitar"), SValue::from_i(31));
    var.insert(String::from("GuitarHarmonics"), SValue::from_i(32));
    var.insert(String::from("AcousticBass"), SValue::from_i(33));
    var.insert(String::from("FingerBase"), SValue::from_i(34));
    var.insert(String::from("FingerBass"), SValue::from_i(34));
    var.insert(String::from("PickBass"), SValue::from_i(35));
    var.insert(String::from("FretlessBass"), SValue::from_i(36));
    var.insert(String::from("SlapBass1"), SValue::from_i(37));
    var.insert(String::from("SlapBass2"), SValue::from_i(38));
    var.insert(String::from("SynthBass1"), SValue::from_i(39));
    var.insert(String::from("SynthBass2"), SValue::from_i(40));
    var.insert(String::from("Violin"), SValue::from_i(41));
    var.insert(String::from("Viola"), SValue::from_i(42));
    var.insert(String::from("Cello"), SValue::from_i(43));
    var.insert(String::from("Contrabass"), SValue::from_i(44));
    var.insert(String::from("TremoloStrings"), SValue::from_i(45));
    var.insert(String::from("PizzicatoStrings"), SValue::from_i(46));
    var.insert(String::from("OrchestralHarp"), SValue::from_i(47));
    var.insert(String::from("Timpani"), SValue::from_i(48));
    var.insert(String::from("Strings1"), SValue::from_i(49));
    var.insert(String::from("Strings2"), SValue::from_i(50));
    var.insert(String::from("SynthStrings1"), SValue::from_i(51));
    var.insert(String::from("SynthStrings2"), SValue::from_i(52));
    var.insert(String::from("ChoirAahs"), SValue::from_i(53));
    var.insert(String::from("VoiceOohs"), SValue::from_i(54));
    var.insert(String::from("SynthVoice"), SValue::from_i(55));
    var.insert(String::from("OrchestraHit"), SValue::from_i(56));
    var.insert(String::from("Trumpet"), SValue::from_i(57));
    var.insert(String::from("Trombone"), SValue::from_i(58));
    var.insert(String::from("Tuba"), SValue::from_i(59));
    var.insert(String::from("MutedTrumpet"), SValue::from_i(60));
    var.insert(String::from("FrenchHorn"), SValue::from_i(61));
    var.insert(String::from("BrassSection"), SValue::from_i(62));
    var.insert(String::from("SynthBrass1"), SValue::from_i(63));
    var.insert(String::from("SynthBrass2"), SValue::from_i(64));
    var.insert(String::from("SopranoSax"), SValue::from_i(65));
    var.insert(String::from("AltoSax"), SValue::from_i(66));
    var.insert(String::from("TenorSax"), SValue::from_i(67));
    var.insert(String::from("BaritoneSax"), SValue::from_i(68));
    var.insert(String::from("Oboe"), SValue::from_i(69));
    var.insert(String::from("EnglishHorn"), SValue::from_i(70));
    var.insert(String::from("Bassoon"), SValue::from_i(71));
    var.insert(String::from("Clarinet"), SValue::from_i(72));
    var.insert(String::from("Piccolo"), SValue::from_i(73));
    var.insert(String::from("Flute"), SValue::from_i(74));
    var.insert(String::from("Recorder"), SValue::from_i(75));
    var.insert(String::from("PanFlute"), SValue::from_i(76));
    var.insert(String::from("BlownBottle"), SValue::from_i(77));
    var.insert(String::from("Shakuhachi"), SValue::from_i(78));
    var.insert(String::from("Whistle"), SValue::from_i(79));
    var.insert(String::from("Ocarina"), SValue::from_i(80));
    var.insert(String::from("SquareLead"), SValue::from_i(81));
    var.insert(String::from("SawtoothLead"), SValue::from_i(82));
    var.insert(String::from("CalliopeLead"), SValue::from_i(83));
    var.insert(String::from("ChiffLead"), SValue::from_i(84));
    var.insert(String::from("CharangLead"), SValue::from_i(85));
    var.insert(String::from("VoiceLead"), SValue::from_i(86));
    var.insert(String::from("FifthsLead"), SValue::from_i(87));
    var.insert(String::from("BassLead"), SValue::from_i(88));
    var.insert(String::from("NewAgePad"), SValue::from_i(89));
    var.insert(String::from("WarmPad"), SValue::from_i(90));
    var.insert(String::from("PolySynthPad"), SValue::from_i(91));
    var.insert(String::from("ChoirPad"), SValue::from_i(92));
    var.insert(String::from("BowedPad"), SValue::from_i(93));
    var.insert(String::from("MetallicPad"), SValue::from_i(94));
    var.insert(String::from("HaloPad"), SValue::from_i(95));
    var.insert(String::from("SweepPad"), SValue::from_i(96));
    var.insert(String::from("Rain"), SValue::from_i(97));
    var.insert(String::from("SoundTrack"), SValue::from_i(98));
    var.insert(String::from("Crystal"), SValue::from_i(99));
    var.insert(String::from("Atmosphere"), SValue::from_i(100));
    var.insert(String::from("Brightness"), SValue::from_i(101));
    var.insert(String::from("Goblins"), SValue::from_i(102));
    var.insert(String::from("Echoes"), SValue::from_i(103));
    var.insert(String::from("Sci_Fi"), SValue::from_i(104));
    var.insert(String::from("Sitar"), SValue::from_i(105));
    var.insert(String::from("Banjo"), SValue::from_i(106));
    var.insert(String::from("Shamisen"), SValue::from_i(107));
    var.insert(String::from("Koto"), SValue::from_i(108));
    var.insert(String::from("Kalimba"), SValue::from_i(109));
    var.insert(String::from("Bagpipe"), SValue::from_i(110));
    var.insert(String::from("Fiddle"), SValue::from_i(111));
    var.insert(String::from("Shanai"), SValue::from_i(112));
    var.insert(String::from("TibkleBell"), SValue::from_i(113));
    var.insert(String::from("TinkleBell"), SValue::from_i(113));
    var.insert(String::from("Agogo"), SValue::from_i(114));
    var.insert(String::from("SteelDrums"), SValue::from_i(115));
    var.insert(String::from("Woodblock"), SValue::from_i(116));
    var.insert(String::from("TaikoDrum"), SValue::from_i(117));
    var.insert(String::from("MelodicTom"), SValue::from_i(118));
    var.insert(String::from("SynthDrum"), SValue::from_i(119));
    var.insert(String::from("ReverseCymbal"), SValue::from_i(120));
    var.insert(String::from("FretNoise"), SValue::from_i(121));
    var.insert(String::from("BreathNoise"), SValue::from_i(122));
    var.insert(String::from("Seashore"), SValue::from_i(123));
    var.insert(String::from("BirdTweet"), SValue::from_i(124));
    var.insert(String::from("TelephoneRing"), SValue::from_i(125));
    var.insert(String::from("Helicopter"), SValue::from_i(126));
    var.insert(String::from("Applause"), SValue::from_i(127));
    var.insert(String::from("Gunshot"), SValue::from_i(128));
    var.insert(String::from("StandardSet"), SValue::from_i(1));
    var.insert(String::from("StandardSet2"), SValue::from_i(2));
    var.insert(String::from("RoomSet"), SValue::from_i(9));
    var.insert(String::from("PowerSet"), SValue::from_i(17));
    var.insert(String::from("ElectronicSet"), SValue::from_i(25));
    var.insert(String::from("AnalogSet"), SValue::from_i(26));
    var.insert(String::from("DanceSet"), SValue::from_i(27));
    var.insert(String::from("JazzSet"), SValue::from_i(33));
    var.insert(String::from("BrushSet"), SValue::from_i(41));
    var.insert(String::from("OrchestraSet"), SValue::from_i(49));
    var.insert(String::from("SnareRoll"), SValue::from_i(25));
    var.insert(String::from("FingerSnap"), SValue::from_i(26));
    var.insert(String::from("HighQ"), SValue::from_i(27));
    var.insert(String::from("Slap"), SValue::from_i(28));
    var.insert(String::from("ScratchPush"), SValue::from_i(29));
    var.insert(String::from("ScratchPull"), SValue::from_i(30));
    var.insert(String::from("Sticks"), SValue::from_i(31));
    var.insert(String::from("SquareClick"), SValue::from_i(32));
    var.insert(String::from("MetronomeClick"), SValue::from_i(33));
    var.insert(String::from("MetronomeBell"), SValue::from_i(34));
    var.insert(String::from("Kick2"), SValue::from_i(35));
    var.insert(String::from("Kick1"), SValue::from_i(36));
    var.insert(String::from("SideStick"), SValue::from_i(37));
    var.insert(String::from("Snare1"), SValue::from_i(38));
    var.insert(String::from("HandClap"), SValue::from_i(39));
    var.insert(String::from("Snare2"), SValue::from_i(40));
    var.insert(String::from("LowTom2"), SValue::from_i(41));
    var.insert(String::from("ClosedHiHat"), SValue::from_i(42));
    var.insert(String::from("LowTom1"), SValue::from_i(43));
    var.insert(String::from("PedalHiHat"), SValue::from_i(44));
    var.insert(String::from("MidTom2"), SValue::from_i(45));
    var.insert(String::from("OpenHiHat"), SValue::from_i(46));
    var.insert(String::from("MidTom1"), SValue::from_i(47));
    var.insert(String::from("HighTom2"), SValue::from_i(48));
    var.insert(String::from("CrashCymbal1"), SValue::from_i(49));
    var.insert(String::from("HighTom1"), SValue::from_i(50));
    var.insert(String::from("RideCymbal1"), SValue::from_i(51));
    var.insert(String::from("ChineseCymbal"), SValue::from_i(52));
    var.insert(String::from("RideBell"), SValue::from_i(53));
    var.insert(String::from("Tambourine"), SValue::from_i(54));
    var.insert(String::from("SplashCymbal"), SValue::from_i(55));
    var.insert(String::from("Cowbell"), SValue::from_i(56));
    var.insert(String::from("CrashCymbal2"), SValue::from_i(57));
    var.insert(String::from("VibraSlap"), SValue::from_i(58));
    var.insert(String::from("RideCymbal2"), SValue::from_i(59));
    var.insert(String::from("HighBongo"), SValue::from_i(60));
    var.insert(String::from("LowBongo"), SValue::from_i(61));
    var.insert(String::from("MuteHighConga"), SValue::from_i(62));
    var.insert(String::from("OpenHighConga"), SValue::from_i(63));
    var.insert(String::from("LowConga"), SValue::from_i(64));
    var.insert(String::from("HighTimbale"), SValue::from_i(65));
    var.insert(String::from("LowTimbale"), SValue::from_i(66));
    var.insert(String::from("HighAgogo"), SValue::from_i(67));
    var.insert(String::from("LowAgogo"), SValue::from_i(68));
    var.insert(String::from("Cabasa"), SValue::from_i(69));
    var.insert(String::from("Maracas"), SValue::from_i(70));
    var.insert(String::from("ShortHiWhistle"), SValue::from_i(71));
    var.insert(String::from("LongLowWhistle"), SValue::from_i(72));
    var.insert(String::from("ShortGuiro"), SValue::from_i(73));
    var.insert(String::from("LongGuiro"), SValue::from_i(74));
    var.insert(String::from("Claves"), SValue::from_i(75));
    var.insert(String::from("HighWoodBlock"), SValue::from_i(76));
    var.insert(String::from("LowWoodBlock"), SValue::from_i(77));
    var.insert(String::from("MuteCuica"), SValue::from_i(78));
    var.insert(String::from("OpenCuica"), SValue::from_i(79));
    var.insert(String::from("MuteTriangle"), SValue::from_i(80));
    var.insert(String::from("OpenTriangle"), SValue::from_i(81));
    var.insert(String::from("Shaker"), SValue::from_i(82));
    var.insert(String::from("JingleBell"), SValue::from_i(83));
    var.insert(String::from("BellTree"), SValue::from_i(84));
    var.insert(String::from("Castanets"), SValue::from_i(85));
    var.insert(String::from("MuteSurdo"), SValue::from_i(86));
    var.insert(String::from("OpenSurdo"), SValue::from_i(87));
   //</VARIABLES>
    var
}