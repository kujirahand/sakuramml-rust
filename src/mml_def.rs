//! Define MML Commands and Macros

use std::collections::HashMap;
use crate::sakura_version;
use crate::svalue::SValue;
use crate::token::TokenType;

/// Tie & Slur Mode
/// 0: グリッサンド : ノートオンを、ポルタメントでつなぐ
/// 1: 異音程をベンドで表現、ギターのハンマリングに近い : ノートオンを、ベンドでつなぐ
/// 2: ノートオンのゲートを100%にする ( ＆のついた音符のゲートを、valueにする ... )
/// 3: ＆でつないだ音符の終わりまでゲートを伸ばす。どんどん重なる。
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TieMode {
    Port = 0,
    Bend = 1,
    Gate = 2,
    Alpe = 3,
}
impl TieMode {
    pub fn from_i(i: isize) -> Self {
        match i {
            0 => Self::Port,
            1 => Self::Bend,
            2 => Self::Gate,
            3 => Self::Alpe,
            _ => Self::Port,
        }
    }
}

pub fn init_rhythm_macro() -> Vec<String> {
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
    rhthm_macro['_' as usize - 0x40] = String::from("r");
    // </RHYTHM_MACRO>
    //
    rhthm_macro
}

pub fn init_variables() -> HashMap<String, SValue> {
    let mut var = HashMap::new();
    //<VARIABLES>
    var.insert(String::from("SAKURA_VERSION"), SValue::from_s(sakura_version::SAKURA_VERSION.to_string())); // @ サクラのバージョン情報を得る
    var.insert(String::from("FALSE"), SValue::from_b(false)); // @ bool型(false)
    var.insert(String::from("TRUE"), SValue::from_b(true)); // @ bool型(true)
    var.insert(String::from("OctaveUnison"), SValue::from_str("Sub{> #?1 <} #?1")); // @ オクターブユニゾンを演奏 (例 OctaveUnison{cde})
    var.insert(String::from("Unison5th"), SValue::from_str("Sub{ Key=7 #?1 Key=0 } #?1")); // @ 5度のユニゾンを演奏 (例 Unison5th{cde})
    var.insert(String::from("Unison3th"), SValue::from_str("Sub{ Key=4 #?1 Key=0 } #?1")); // @ 3度のユニゾンを演奏 (例 Unison3th{cde})
    var.insert(String::from("Unison"), SValue::from_str("Sub{ Key=#?2 #?1 Key=0 } #?1")); // @ N度のユニゾンを演奏 (例 Unison{cde},7)
    // tie/slur mode
    var.insert(String::from("SLUR_PORT"), SValue::from_i(0)); // @ スラー定数。グリッサンド。ノートオンを、ポルタメントでつなぐ (例 Slur(SlurPort, !8) のように指定)
    var.insert(String::from("SLUR_BEND"), SValue::from_i(1)); // @ スラー定数。ベンド。異音程をベンドで表現。ギターのハンマリングに近い。 (例 Slur(SlurPort, !8) のように指定)
    var.insert(String::from("SLUR_GATE"), SValue::from_i(2)); // @ スラー定数。＆のついた音符のゲートを、valueにする
    var.insert(String::from("SLUR_ALPE"), SValue::from_i(3)); // @ スラー定数。＆でつないだ音符の終わりまでゲートを伸ばす
    // Voice
    var.insert(String::from("GrandPiano"), SValue::from_i(1)); // @ 音色:GrandPiano
    var.insert(String::from("BrightPiano"), SValue::from_i(2)); // @ 音色:BrightPiano
    var.insert(String::from("ElectricGrandPiano"), SValue::from_i(3)); // @ 音色:ElectricGrandPiano
    var.insert(String::from("HonkyTonkPiano"), SValue::from_i(4)); // @ 音色:HonkyTonkPiano
    var.insert(String::from("ElectricPiano1"), SValue::from_i(5)); // @ 音色:ElectricPiano1
    var.insert(String::from("ElectricPiano2"), SValue::from_i(6)); // @ 音色:ElectricPiano2
    var.insert(String::from("Harpsichord"), SValue::from_i(7)); // @ 音色:Harpsichord
    var.insert(String::from("Clavi"), SValue::from_i(8)); // @ 音色:Clavi
    var.insert(String::from("CelestaStrings"), SValue::from_i(9)); // @ 音色:CelestaStrings
    var.insert(String::from("Glockenspiel"), SValue::from_i(10)); // @ 音色:Glockenspiel
    var.insert(String::from("MusicBox"), SValue::from_i(11)); // @ 音色:MusicBox
    var.insert(String::from("Vibraphone"), SValue::from_i(12)); // @ 音色:Vibraphone
    var.insert(String::from("Marimba"), SValue::from_i(13)); // @ 音色:Marimba
    var.insert(String::from("Xylophone"), SValue::from_i(14)); // @ 音色:Xylophone
    var.insert(String::from("TubularBells"), SValue::from_i(15)); // @ 音色:TubularBells
    var.insert(String::from("Dulcimer"), SValue::from_i(16)); // @ 音色:Dulcimer
    var.insert(String::from("DrawbarOrgan"), SValue::from_i(17)); // @ 音色:DrawbarOrgan
    var.insert(String::from("PercussiveOrgan"), SValue::from_i(18)); // @ 音色:PercussiveOrgan
    var.insert(String::from("RockOrgan"), SValue::from_i(19)); // @ 音色:RockOrgan
    var.insert(String::from("ChurchOrgan"), SValue::from_i(20)); // @ 音色:ChurchOrgan
    var.insert(String::from("ReedOrgan"), SValue::from_i(21)); // @ 音色:ReedOrgan
    var.insert(String::from("Accordion"), SValue::from_i(22)); // @ 音色:Accordion
    var.insert(String::from("Hamonica"), SValue::from_i(23)); // @ 音色:Hamonica
    var.insert(String::from("TangoAccordion"), SValue::from_i(24)); // @ 音色:TangoAccordion
    var.insert(String::from("NylonGuitar"), SValue::from_i(25)); // @ 音色:NylonGuitar
    var.insert(String::from("SteelcGuitar"), SValue::from_i(26)); // @ 音色:SteelcGuitar
    var.insert(String::from("JazzGuitar"), SValue::from_i(27)); // @ 音色:JazzGuitar
    var.insert(String::from("CleanGuitar"), SValue::from_i(28)); // @ 音色:CleanGuitar
    var.insert(String::from("MutedGuitar"), SValue::from_i(29)); // @ 音色:MutedGuitar
    var.insert(String::from("OverdrivenGuitar"), SValue::from_i(30)); // @ 音色:OverdrivenGuitar
    var.insert(String::from("DistortionGuitar"), SValue::from_i(31)); // @ 音色:DistortionGuitar
    var.insert(String::from("GuitarHarmonics"), SValue::from_i(32)); // @ 音色:GuitarHarmonics
    var.insert(String::from("AcousticBass"), SValue::from_i(33)); // @ 音色:AcousticBass
    var.insert(String::from("FingerBase"), SValue::from_i(34)); // @ 音色:FingerBase
    var.insert(String::from("FingerBass"), SValue::from_i(34)); // @ 音色:FingerBass
    var.insert(String::from("PickBass"), SValue::from_i(35)); // @ 音色:PickBass
    var.insert(String::from("FretlessBass"), SValue::from_i(36)); // @ 音色:FretlessBass
    var.insert(String::from("SlapBass1"), SValue::from_i(37)); // @ 音色:SlapBass1
    var.insert(String::from("SlapBass2"), SValue::from_i(38)); // @ 音色:SlapBass2
    var.insert(String::from("SynthBass1"), SValue::from_i(39)); // @ 音色:SynthBass1
    var.insert(String::from("SynthBass2"), SValue::from_i(40)); // @ 音色:SynthBass2
    var.insert(String::from("Violin"), SValue::from_i(41)); // @ 音色:Violin
    var.insert(String::from("Viola"), SValue::from_i(42)); // @ 音色:Viola
    var.insert(String::from("Cello"), SValue::from_i(43)); // @ 音色:Cello
    var.insert(String::from("Contrabass"), SValue::from_i(44)); // @ 音色:Contrabass
    var.insert(String::from("TremoloStrings"), SValue::from_i(45)); // @ 音色:TremoloStrings
    var.insert(String::from("PizzicatoStrings"), SValue::from_i(46)); // @ 音色:PizzicatoStrings
    var.insert(String::from("OrchestralHarp"), SValue::from_i(47)); // @ 音色:OrchestralHarp
    var.insert(String::from("Timpani"), SValue::from_i(48)); // @ 音色:Timpani
    var.insert(String::from("Strings1"), SValue::from_i(49)); // @ 音色:Strings1
    var.insert(String::from("Strings2"), SValue::from_i(50)); // @ 音色:Strings2
    var.insert(String::from("SynthStrings1"), SValue::from_i(51)); // @ 音色:SynthStrings1
    var.insert(String::from("SynthStrings2"), SValue::from_i(52)); // @ 音色:SynthStrings2
    var.insert(String::from("ChoirAahs"), SValue::from_i(53)); // @ 音色:ChoirAahs
    var.insert(String::from("VoiceOohs"), SValue::from_i(54)); // @ 音色:VoiceOohs
    var.insert(String::from("SynthVoice"), SValue::from_i(55)); // @ 音色:SynthVoice
    var.insert(String::from("OrchestraHit"), SValue::from_i(56)); // @ 音色:OrchestraHit
    var.insert(String::from("Trumpet"), SValue::from_i(57)); // @ 音色:Trumpet
    var.insert(String::from("Trombone"), SValue::from_i(58)); // @ 音色:Trombone
    var.insert(String::from("Tuba"), SValue::from_i(59)); // @ 音色:Tuba
    var.insert(String::from("MutedTrumpet"), SValue::from_i(60)); // @ 音色:MutedTrumpet
    var.insert(String::from("FrenchHorn"), SValue::from_i(61)); // @ 音色:FrenchHorn
    var.insert(String::from("BrassSection"), SValue::from_i(62)); // @ 音色:BrassSection
    var.insert(String::from("SynthBrass1"), SValue::from_i(63)); // @ 音色:SynthBrass1
    var.insert(String::from("SynthBrass2"), SValue::from_i(64)); // @ 音色:SynthBrass2
    var.insert(String::from("SopranoSax"), SValue::from_i(65)); // @ 音色:SopranoSax
    var.insert(String::from("AltoSax"), SValue::from_i(66)); // @ 音色:AltoSax
    var.insert(String::from("TenorSax"), SValue::from_i(67)); // @ 音色:TenorSax
    var.insert(String::from("BaritoneSax"), SValue::from_i(68)); // @ 音色:BaritoneSax
    var.insert(String::from("Oboe"), SValue::from_i(69)); // @ 音色:Oboe
    var.insert(String::from("EnglishHorn"), SValue::from_i(70)); // @ 音色:EnglishHorn
    var.insert(String::from("Bassoon"), SValue::from_i(71)); // @ 音色:Bassoon
    var.insert(String::from("Clarinet"), SValue::from_i(72)); // @ 音色:Clarinet
    var.insert(String::from("Piccolo"), SValue::from_i(73)); // @ 音色:Piccolo
    var.insert(String::from("Flute"), SValue::from_i(74)); // @ 音色:Flute
    var.insert(String::from("Recorder"), SValue::from_i(75)); // @ 音色:Recorder
    var.insert(String::from("PanFlute"), SValue::from_i(76)); // @ 音色:PanFlute
    var.insert(String::from("BlownBottle"), SValue::from_i(77)); // @ 音色:BlownBottle
    var.insert(String::from("Shakuhachi"), SValue::from_i(78)); // @ 音色:Shakuhachi
    var.insert(String::from("Whistle"), SValue::from_i(79)); // @ 音色:Whistle
    var.insert(String::from("Ocarina"), SValue::from_i(80)); // @ 音色:Ocarina
    var.insert(String::from("SquareLead"), SValue::from_i(81)); // @ 音色:SquareLead
    var.insert(String::from("SawtoothLead"), SValue::from_i(82)); // @ 音色:SawtoothLead
    var.insert(String::from("CalliopeLead"), SValue::from_i(83)); // @ 音色:CalliopeLead
    var.insert(String::from("ChiffLead"), SValue::from_i(84)); // @ 音色:ChiffLead
    var.insert(String::from("CharangLead"), SValue::from_i(85)); // @ 音色:CharangLead
    var.insert(String::from("VoiceLead"), SValue::from_i(86)); // @ 音色:VoiceLead
    var.insert(String::from("FifthsLead"), SValue::from_i(87)); // @ 音色:FifthsLead
    var.insert(String::from("BassLead"), SValue::from_i(88)); // @ 音色:BassLead
    var.insert(String::from("NewAgePad"), SValue::from_i(89)); // @ 音色:NewAgePad
    var.insert(String::from("WarmPad"), SValue::from_i(90)); // @ 音色:WarmPad
    var.insert(String::from("PolySynthPad"), SValue::from_i(91)); // @ 音色:PolySynthPad
    var.insert(String::from("ChoirPad"), SValue::from_i(92)); // @ 音色:ChoirPad
    var.insert(String::from("BowedPad"), SValue::from_i(93)); // @ 音色:BowedPad
    var.insert(String::from("MetallicPad"), SValue::from_i(94)); // @ 音色:MetallicPad
    var.insert(String::from("HaloPad"), SValue::from_i(95)); // @ 音色:HaloPad
    var.insert(String::from("SweepPad"), SValue::from_i(96)); // @ 音色:SweepPad
    var.insert(String::from("Rain"), SValue::from_i(97)); // @ 音色:Rain
    var.insert(String::from("SoundTrack"), SValue::from_i(98)); // @ 音色:SoundTrack
    var.insert(String::from("Crystal"), SValue::from_i(99)); // @ 音色:Crystal
    var.insert(String::from("Atmosphere"), SValue::from_i(100)); // @ 音色:Atmosphere
    var.insert(String::from("Brightness"), SValue::from_i(101)); // @ 音色:Brightness
    var.insert(String::from("Goblins"), SValue::from_i(102)); // @ 音色:Goblins
    var.insert(String::from("Echoes"), SValue::from_i(103)); // @ 音色:Echoes
    var.insert(String::from("Sci_Fi"), SValue::from_i(104)); // @ 音色:Sci_Fi
    var.insert(String::from("Sitar"), SValue::from_i(105)); // @ 音色:Sitar
    var.insert(String::from("Banjo"), SValue::from_i(106)); // @ 音色:Banjo
    var.insert(String::from("Shamisen"), SValue::from_i(107)); // @ 音色:Shamisen
    var.insert(String::from("Koto"), SValue::from_i(108)); // @ 音色:Koto
    var.insert(String::from("Kalimba"), SValue::from_i(109)); // @ 音色:Kalimba
    var.insert(String::from("Bagpipe"), SValue::from_i(110)); // @ 音色:Bagpipe
    var.insert(String::from("Fiddle"), SValue::from_i(111)); // @ 音色:Fiddle
    var.insert(String::from("Shanai"), SValue::from_i(112)); // @ 音色:Shanai
    var.insert(String::from("TibkleBell"), SValue::from_i(113)); // @ 音色:TibkleBell
    var.insert(String::from("TinkleBell"), SValue::from_i(113)); // @ 音色:TinkleBell
    var.insert(String::from("Agogo"), SValue::from_i(114)); // @ 音色:Agogo
    var.insert(String::from("SteelDrums"), SValue::from_i(115)); // @ 音色:SteelDrums
    var.insert(String::from("Woodblock"), SValue::from_i(116)); // @ 音色:Woodblock
    var.insert(String::from("TaikoDrum"), SValue::from_i(117)); // @ 音色:TaikoDrum
    var.insert(String::from("MelodicTom"), SValue::from_i(118)); // @ 音色:MelodicTom
    var.insert(String::from("SynthDrum"), SValue::from_i(119)); // @ 音色:SynthDrum
    var.insert(String::from("ReverseCymbal"), SValue::from_i(120)); // @ 音色:ReverseCymbal
    var.insert(String::from("FretNoise"), SValue::from_i(121)); // @ 音色:FretNoise
    var.insert(String::from("BreathNoise"), SValue::from_i(122)); // @ 音色:BreathNoise
    var.insert(String::from("Seashore"), SValue::from_i(123)); // @ 音色:Seashore
    var.insert(String::from("BirdTweet"), SValue::from_i(124)); // @ 音色:BirdTweet
    var.insert(String::from("TelephoneRing"), SValue::from_i(125)); // @ 音色:TelephoneRing
    var.insert(String::from("Helicopter"), SValue::from_i(126)); // @ 音色:Helicopter
    var.insert(String::from("Applause"), SValue::from_i(127)); // @ 音色:Applause
    var.insert(String::from("Gunshot"), SValue::from_i(128)); // @ 音色:Gunshot
    var.insert(String::from("StandardSet"), SValue::from_i(1)); // @ 音色:StandardSet
    var.insert(String::from("StandardSet2"), SValue::from_i(2)); // @ 音色:StandardSet2
    var.insert(String::from("RoomSet"), SValue::from_i(9)); // @ 音色:RoomSet
    var.insert(String::from("PowerSet"), SValue::from_i(17)); // @ 音色:PowerSet
    var.insert(String::from("ElectronicSet"), SValue::from_i(25)); // @ 音色:ElectronicSet
    var.insert(String::from("AnalogSet"), SValue::from_i(26)); // @ 音色:AnalogSet
    var.insert(String::from("DanceSet"), SValue::from_i(27)); // @ 音色:DanceSet
    var.insert(String::from("JazzSet"), SValue::from_i(33)); // @ 音色:JazzSet
    var.insert(String::from("BrushSet"), SValue::from_i(41)); // @ 音色:BrushSet
    var.insert(String::from("OrchestraSet"), SValue::from_i(49)); // @ 音色:OrchestraSet
    var.insert(String::from("SnareRoll"), SValue::from_i(25)); // @ 音色:SnareRoll
    var.insert(String::from("FingerSnap"), SValue::from_i(26)); // @ 音色:FingerSnap
    var.insert(String::from("HighQ"), SValue::from_i(27)); // @ 音色:HighQ
    var.insert(String::from("Slap"), SValue::from_i(28)); // @ 音色:Slap
    var.insert(String::from("ScratchPush"), SValue::from_i(29)); // @ 音色:ScratchPush
    var.insert(String::from("ScratchPull"), SValue::from_i(30)); // @ 音色:ScratchPull
    var.insert(String::from("Sticks"), SValue::from_i(31)); // @ 音色:Sticks
    var.insert(String::from("SquareClick"), SValue::from_i(32)); // @ 音色:SquareClick
    var.insert(String::from("MetronomeClick"), SValue::from_i(33)); // @ 音色:MetronomeClick
    var.insert(String::from("MetronomeBell"), SValue::from_i(34)); // @ 音色:MetronomeBell
    var.insert(String::from("Kick2"), SValue::from_i(35)); // @ 音色:Kick2
    var.insert(String::from("Kick1"), SValue::from_i(36)); // @ 音色:Kick1
    var.insert(String::from("SideStick"), SValue::from_i(37)); // @ 音色:SideStick
    var.insert(String::from("Snare1"), SValue::from_i(38)); // @ 音色:Snare1
    var.insert(String::from("HandClap"), SValue::from_i(39)); // @ 音色:HandClap
    var.insert(String::from("Snare2"), SValue::from_i(40)); // @ 音色:Snare2
    var.insert(String::from("LowTom2"), SValue::from_i(41)); // @ 音色:LowTom2
    var.insert(String::from("ClosedHiHat"), SValue::from_i(42)); // @ 音色:ClosedHiHat
    var.insert(String::from("LowTom1"), SValue::from_i(43)); // @ 音色:LowTom1
    var.insert(String::from("PedalHiHat"), SValue::from_i(44)); // @ 音色:PedalHiHat
    var.insert(String::from("MidTom2"), SValue::from_i(45)); // @ 音色:MidTom2
    var.insert(String::from("OpenHiHat"), SValue::from_i(46)); // @ 音色:OpenHiHat
    var.insert(String::from("MidTom1"), SValue::from_i(47)); // @ 音色:MidTom1
    var.insert(String::from("HighTom2"), SValue::from_i(48)); // @ 音色:HighTom2
    var.insert(String::from("CrashCymbal1"), SValue::from_i(49)); // @ 音色:CrashCymbal1
    var.insert(String::from("HighTom1"), SValue::from_i(50)); // @ 音色:HighTom1
    var.insert(String::from("RideCymbal1"), SValue::from_i(51)); // @ 音色:RideCymbal1
    var.insert(String::from("ChineseCymbal"), SValue::from_i(52)); // @ 音色:ChineseCymbal
    var.insert(String::from("RideBell"), SValue::from_i(53)); // @ 音色:RideBell
    var.insert(String::from("Tambourine"), SValue::from_i(54)); // @ 音色:Tambourine
    var.insert(String::from("SplashCymbal"), SValue::from_i(55)); // @ 音色:SplashCymbal
    var.insert(String::from("Cowbell"), SValue::from_i(56)); // @ 音色:Cowbell
    var.insert(String::from("CrashCymbal2"), SValue::from_i(57)); // @ 音色:CrashCymbal2
    var.insert(String::from("VibraSlap"), SValue::from_i(58)); // @ 音色:VibraSlap
    var.insert(String::from("RideCymbal2"), SValue::from_i(59)); // @ 音色:RideCymbal2
    var.insert(String::from("HighBongo"), SValue::from_i(60)); // @ 音色:HighBongo
    var.insert(String::from("LowBongo"), SValue::from_i(61)); // @ 音色:LowBongo
    var.insert(String::from("MuteHighConga"), SValue::from_i(62)); // @ 音色:MuteHighConga
    var.insert(String::from("OpenHighConga"), SValue::from_i(63)); // @ 音色:OpenHighConga
    var.insert(String::from("LowConga"), SValue::from_i(64)); // @ 音色:LowConga
    var.insert(String::from("HighTimbale"), SValue::from_i(65)); // @ 音色:HighTimbale
    var.insert(String::from("LowTimbale"), SValue::from_i(66)); // @ 音色:LowTimbale
    var.insert(String::from("HighAgogo"), SValue::from_i(67)); // @ 音色:HighAgogo
    var.insert(String::from("LowAgogo"), SValue::from_i(68)); // @ 音色:LowAgogo
    var.insert(String::from("Cabasa"), SValue::from_i(69)); // @ 音色:Cabasa
    var.insert(String::from("Maracas"), SValue::from_i(70)); // @ 音色:Maracas
    var.insert(String::from("ShortHiWhistle"), SValue::from_i(71)); // @ 音色:ShortHiWhistle
    var.insert(String::from("LongLowWhistle"), SValue::from_i(72)); // @ 音色:LongLowWhistle
    var.insert(String::from("ShortGuiro"), SValue::from_i(73)); // @ 音色:ShortGuiro
    var.insert(String::from("LongGuiro"), SValue::from_i(74)); // @ 音色:LongGuiro
    var.insert(String::from("Claves"), SValue::from_i(75)); // @ 音色:Claves
    var.insert(String::from("HighWoodBlock"), SValue::from_i(76)); // @ 音色:HighWoodBlock
    var.insert(String::from("LowWoodBlock"), SValue::from_i(77)); // @ 音色:LowWoodBlock
    var.insert(String::from("MuteCuica"), SValue::from_i(78)); // @ 音色:MuteCuica
    var.insert(String::from("OpenCuica"), SValue::from_i(79)); // @ 音色:OpenCuica
    var.insert(String::from("MuteTriangle"), SValue::from_i(80)); // @ 音色:MuteTriangle
    var.insert(String::from("OpenTriangle"), SValue::from_i(81)); // @ 音色:OpenTriangle
    var.insert(String::from("Shaker"), SValue::from_i(82)); // @ 音色:Shaker
    var.insert(String::from("JingleBell"), SValue::from_i(83)); // @ 音色:JingleBell
    var.insert(String::from("BellTree"), SValue::from_i(84)); // @ 音色:BellTree
    var.insert(String::from("Castanets"), SValue::from_i(85)); // @ 音色:Castanets
    var.insert(String::from("MuteSurdo"), SValue::from_i(86)); // @ 音色:MuteSurdo
    var.insert(String::from("OpenSurdo"), SValue::from_i(87)); // @ 音色:OpenSurdo
    //</VARIABLES>
    var
}
pub fn init_reserved_words(sys_funcs: &HashMap<String, SystemFunction>) -> HashMap<String, u8> {
    let mut var = HashMap::new();
    // Add system functions to reserved words
    for (i, (key, _value)) in sys_funcs.iter().enumerate() {
        var.insert(key.clone(), (100 + i) as u8);
    }
    //<RESERVED>
    var.insert(String::from("IF"), 0); // @ IF .. ELSE ..
    var.insert(String::from("If"), 0); // @ IF .. ELSE ..
    var.insert(String::from("ELSE"), 1); // @ IF .. ELSE ..
    var.insert(String::from("Else"), 1); // @ IF .. ELSE ..
    var.insert(String::from("FOR"), 2); // @ FOR
    var.insert(String::from("For"), 2); // @ FOR
    var.insert(String::from("WHILE"), 3); // @ WHILE
    var.insert(String::from("While"), 3); // @ WHILE
    var.insert(String::from("EXIT"), 4); // @ EXIT FOR/WHILE LOOP
    var.insert(String::from("Exit"), 4); // @ EXIT FOR/WHILE Loop
    var.insert(String::from("BREAK"), 4); // @ EXIT FOR/WHILE LOOP
    var.insert(String::from("Break"), 4); // @ EXIT FOR/WHILE Loop
    var.insert(String::from("CONTINUE"), 5); // @ CONTINUE FOR/WHILE LOOP
    var.insert(String::from("Continue"), 5); // @ CONTINUE FOR/WHILE Loop
    var.insert(String::from("FUNCTION"), 6); // @ FUNCTION
    var.insert(String::from("Function"), 6); // @ FUNCTION
    var.insert(String::from("RETURN"), 7); // @ RETURN
    var.insert(String::from("Return"), 7); // @ RETURN
    // var.insert(String::from("Result"), 8); // @ Set Function Result (代入は可能)
    var.insert(String::from("INT"), 9); // @ Define INT Variable
    var.insert(String::from("Int"), 9); // @ Define INT Variable
    var.insert(String::from("STR"), 10); // @ Define STR Variable
    var.insert(String::from("Str"), 10); // @ Define STR Variable

    var.insert(String::from("PRINT"), 20); // @ PRINT
    var.insert(String::from("Print"), 20); // @ PRINT

    var.insert(String::from("TRACK"), 100); // @ Track
    var.insert(String::from("TR"), 100); // @ Track
    var.insert(String::from("Track"), 100); // @ Track
    var.insert(String::from("CH"), 101); // @ Channel
    var.insert(String::from("Channel"), 101); // @ Channel
    var.insert(String::from("CHANNEL"), 101); // @ Channel
    var.insert(String::from("Time"), 102); // @ Time position
    var.insert(String::from("TIME"), 102); // @ Time position
    var.insert(String::from("Voice"), 103); // @ Voice
    var.insert(String::from("VOICE"), 103); // @ Voice
    var.insert(String::from("TEMPO"), 104); // @ Tempo
    var.insert(String::from("Tempo"), 104); // @ Tempo
    var.insert(String::from("T"), 104); // @ Tempo
    var.insert(String::from("TempoChange"), 105); // @ TempoChange
    var.insert(String::from("PLAY"), 106); // @ PLAY
    var.insert(String::from("Play"), 106); // @ PLAY
    var.insert(String::from("PLAY_FROM"), 107); // @ PLAY_FROM
    var.insert(String::from("PlayFrom"), 107); // @ PLAY_FROM
    var.insert(String::from("RHYTHM"), 108); // @ RHYTHM Mode
    var.insert(String::from("Rhythm"), 108); // @ RHYTHM Mode
    var.insert(String::from("R"), 108); // @ RHYTHM Mode
    var.insert(String::from("RYTHM"), 108); // @ RHYTHM Mode (v1 綴りミス)
    var.insert(String::from("Rythm"), 108); // @ RHYTHM Mode (v1 綴りミス)
    var.insert(String::from("DIV"), 109); // @ DIV (連符)
    var.insert(String::from("Div"), 109); // @ DIV (連符)
    var.insert(String::from("SUB"), 110); // @ SUB (Back Time pointer)
    var.insert(String::from("Sub"), 110); // @ Sub (Back Time pointer)
    var.insert(String::from("KeyFlag"), 111); // @ KeyFlag
    var.insert(String::from("KF"), 111); // @ KeyFlag
    var.insert(String::from("KEY"), 112); // @ KeyShift
    var.insert(String::from("Key"), 112); // @ KeyShift
    var.insert(String::from("KeyShift"), 112); // @ KeyShift
    var.insert(String::from("TR_KEY"), 113); // @ TrackKey
    var.insert(String::from("TrackKey"), 113); // @ TrackKey

    var.insert(String::from("SYSTEM"), 200); // @ SYSTEM
    var.insert(String::from("System"), 200); // @ SYSTEM

    var.insert(String::from("END"), 255); // @ End
    var.insert(String::from("End"), 255); // @ End
    //<RESERVED>
    var
}

macro_rules! sysfunc_add {
    ($obj:expr, $name:expr, $func_id:expr, $arg_type:expr) => {
        $obj.insert(String::from($name), SystemFunction{token_type: $func_id, arg_type: $arg_type, tag1: 0, tag2: 0});
    };
}
macro_rules! sysfunc_cc_add {
    ($obj:expr, $name:expr, $func_id:expr, $arg_type:expr, $tag:expr) => {
        $obj.insert(String::from($name), SystemFunction{token_type: $func_id, arg_type: $arg_type, tag1: $tag, tag2: 0});
    };
}
macro_rules! sysfunc_rpn_add {
    ($obj:expr, $name:expr, $func_id:expr, $arg_type:expr, $tag1:expr, $tag2:expr) => {
        $obj.insert(String::from($name), SystemFunction{token_type: $func_id, arg_type: $arg_type, tag1: $tag1, tag2: $tag2});
    };
}
#[derive(Debug, Clone, Copy)]
pub struct SystemFunction {
    pub token_type: TokenType,
    pub arg_type: char, // 'I' or 'S' or 'A' or '*'(special)
    pub tag1: isize,
    pub tag2: isize,
}
pub fn init_system_functions() -> HashMap<String, SystemFunction> {
    let mut sf = HashMap::new();
    //<SYSTEM_FUNCTION>
    //@ Basic command
    // sysfunc_add!(sf, "End", TokenType::End, '_'); // end of song
    // sysfunc_add!(sf, "END", TokenType::End, '_'); // end of song
    sysfunc_add!(sf, "Track", TokenType::Track, 'I'); // change current track [range:0 to 999] (ex) Track(1)
    sysfunc_add!(sf, "TRACK", TokenType::Track, 'I'); // change current track [range:0 to 999] (ex) TRACK(1)
    sysfunc_add!(sf, "TR", TokenType::Track, 'I'); // change current track [range:0 to 999] (ex) TR(1)
    sysfunc_add!(sf, "Channel", TokenType::Channel, 'I'); // change channel no [range:1 to 16] (ex) Channel(1)
    sysfunc_add!(sf, "CHANNEL", TokenType::Channel, 'I'); // change channel no [range:1 to 16] (ex) CHANNEL(1)
    sysfunc_add!(sf, "CH", TokenType::Channel, 'I'); // change channel no [range:1 to 16] (ex) CH(1)
    sysfunc_add!(sf, "Time", TokenType::Time, 'A'); // change time position, Time(measure:beat:step) (ex) Time(1:1:0) Time(0)
    sysfunc_add!(sf, "TIME", TokenType::Time, 'A'); // change time position, TIME(measure:beat:step) (ex) Time(1:1:0) Time(0)
    sysfunc_add!(sf, "System.TimeBase", TokenType::TimeBase, '*'); // set system time base (ex) TimeBase(96)
    sysfunc_add!(sf, "Timebase", TokenType::TimeBase, '*'); // set system time base (ex) TimeBase(96)
    sysfunc_add!(sf, "TimeBase", TokenType::TimeBase, '*'); // set system time base (ex) TimeBase(96)
    sysfunc_add!(sf, "TIMEBASE", TokenType::TimeBase, '*'); // set system time base (ex) TimeBase(96)
    sysfunc_add!(sf, "Rhythm", TokenType::Rhythm, '*'); // read Rhythm notes (ex) Rhythm{ bhsh bhsh }
    sysfunc_add!(sf, "RHYTHM", TokenType::Rhythm, '*'); // read Rhythm notes (ex) Rhythm{ bhsh bhsh }
    sysfunc_add!(sf, "R", TokenType::Rhythm, '*'); // read Rhythm notes (ex) Rhythm{ bhsh bhsh }
    sysfunc_add!(sf, "Rythm", TokenType::Rhythm, '*'); // 互換性:綴りミス read Rhythm notes (ex) Rhythm{ bhsh bhsh }
    sysfunc_add!(sf, "RTTHM", TokenType::Rhythm, '*'); // 互換性:綴りミス read Rhythm notes (ex) Rhythm{ bhsh bhsh }
    sysfunc_add!(sf, "Div", TokenType::Div, '*'); // tuplet(連符) (ex) Div{ ceg }
    sysfunc_add!(sf, "DIV", TokenType::Div, '*'); // tuplet(連符) (ex) Div{ ceg }
    sysfunc_add!(sf, "Sub", TokenType::Sub, '*'); // sub track / rewind time position (ex) Sub{ceg} egb
    sysfunc_add!(sf, "SUB", TokenType::Sub, '*'); // sub track / rewind time position (ex) Sub{ceg} egb
    sysfunc_add!(sf, "S", TokenType::Sub, '*'); // sub track / rewind time position (ex) Sub{ceg} egb
    sysfunc_add!(sf, "System.KeyFlag", TokenType::KeyFlag, '*'); // set key flag to note (ex) KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note)
    sysfunc_add!(sf, "KeyFlag", TokenType::KeyFlag, '*'); // set key flag to note (ex) KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note)
    sysfunc_add!(sf, "KF", TokenType::KeyFlag, '*'); // set key flag to note (ex) KeyFlag=(a,b,c,d,e,f,g) KeyFlag[=][+|-](note)
    sysfunc_add!(sf, "KeyShift", TokenType::KeyShift, 'I'); // set key-shift (ex) KeyShift(3)
    sysfunc_add!(sf, "Key", TokenType::KeyShift, 'I'); // set key-shift (ex) Key(3)
    sysfunc_add!(sf, "KEY", TokenType::KeyShift, 'I'); // set key-shift (ex) KEY(3)
    sysfunc_add!(sf, "TrackKey", TokenType::TrackKey, 'I'); // set key-shift for track (ex) TrackKey(3)
    sysfunc_add!(sf, "TR_KEY", TokenType::TrackKey, 'I'); // set key-shift for track (ex) TR_KEY(3)
    sysfunc_add!(sf, "Play", TokenType::Play, '*'); // play multi track (ex) Play(AA,BB,CC)
    sysfunc_add!(sf, "PLAY", TokenType::Play, '*'); // play multi track (ex) Play(AA,BB,CC)
    sysfunc_add!(sf, "PlayFrom.SysEx", TokenType::Unimplemented, 'A'); // Unimplemented
    sysfunc_add!(sf, "PlayFrom.CtrlChg", TokenType::Unimplemented, 'A'); // Unimplemented
    sysfunc_add!(sf, "PlayFrom", TokenType::PlayFrom, 'A'); // play from time position (ex) PlayFrom(5:1:0)
    sysfunc_add!(sf, "PLAY_FROM", TokenType::PlayFrom, 'A'); // play from time position (ex) PLAY_FROM(5:1:0)
    sysfunc_add!(sf, "PlayFromHere", TokenType::PlayFromHere, '_'); // play from current time pos (ex) PlayFromHere
    sysfunc_add!(sf, "PLAY_FROM_HRER", TokenType::PlayFromHere, '_'); // play from current time pos (ex) PLAY_FROM_HERE
    sysfunc_add!(sf, "System.MeasureShift", TokenType::MeasureShift, 'I'); // set measure shift for time pointer (ex) System.MeasureShift(1)
    sysfunc_add!(sf, "MeasureShift", TokenType::MeasureShift, 'I'); // set measure shift for time pointer (ex) MeasureShift(1)
    sysfunc_add!(sf, "MEASURE_SHIFT", TokenType::MeasureShift, 'I'); // set measure shift for time pointer (ex) MeasureShift(1)
    sysfunc_add!(sf, "TrackSync", TokenType::TrackSync, '_'); // synchronize time pointers for all tracks (ex) TrackSync
    sysfunc_add!(sf, "TRACK_SYNC", TokenType::TrackSync, '_'); // synchronize time pointers for all tracks (ex) TrackSync
    sysfunc_add!(sf, "Slur", TokenType::TieMode, 'A'); // set slur/tie(&) mode (0:グリッサンド/1:ベンド/2:ゲート/3:アルペジオ) (ex) Slur(1)
    sysfunc_add!(sf, "SLUR", TokenType::TieMode, 'A'); // set slur/tie(&) mode (0:グリッサンド/1:ベンド/2:ゲート/3:アルペジオ) (ex) Slur(1)
    sysfunc_add!(sf, "System.vAdd", TokenType::SongVelocityAdd, 'I'); // set relative velocity '(' or ')' or 'v++' or 'v--' command increment value (ex) vAdd(3)
    sysfunc_add!(sf, "vAdd", TokenType::SongVelocityAdd, 'I'); // set relative velocity '(' or ')' or 'v++' or 'v--' command increment value (ex) vAdd(3)
    sysfunc_add!(sf, "System.qAdd", TokenType::SongQAdd, 'I'); // set "q++" command value (ex) qAdd(3)
    sysfunc_add!(sf, "qAdd", TokenType::SongQAdd, 'I'); // set "q++" command value (ex) qAdd(3)
    sysfunc_add!(sf, "System.q2Add", TokenType::Unimplemented, 'I'); // Unimplemented
    sysfunc_add!(sf, "q2Add", TokenType::Unimplemented, 'I'); // Unimplemented
    sysfunc_add!(sf, "SoundType", TokenType::SoundType, 'S'); // set sound type (ex) SoundType({pico})
    sysfunc_add!(sf, "DeviceNumber", TokenType::DeviceNumber, 'I'); // set Device Number (ex) DeviceNumber=$10
    //@ Controll Change / Voice Change / RPN/NRPN
    sysfunc_add!(sf, "Voice", TokenType::Voice, 'A'); // set voice (=@) range: 1-128 Voice(n[,msb,lsb]) (ex) Voice(1)
    sysfunc_add!(sf, "VOICE", TokenType::Voice, 'A'); // set voice (=@) range: 1-128 Voice(n[,msb,lsb]) (ex) Voice(1)
    sysfunc_cc_add!(sf, "M", TokenType::ControllChangeCommand, '*', 1); // CC#1 Modulation (ex) M(10)
    sysfunc_cc_add!(sf, "Modulation", TokenType::ControllChangeCommand, '*', 1); // CC#1 Modulation range:0-127 (ex) M(10)
    sysfunc_cc_add!(sf, "PT", TokenType::ControllChangeCommand, '*', 5); // CC#5 Portamento Time range:0-127 (ex) PT(10)
    sysfunc_cc_add!(sf, "PortamentoTime", TokenType::ControllChangeCommand, '*', 5); // CC#5 Portamento Time range:0-127 (ex) PT(10)
    sysfunc_cc_add!(sf, "V", TokenType::ControllChangeCommand, '*', 7); // CC#7 Main Volume range:0-127 (ex) V(10)
    sysfunc_cc_add!(sf, "MainVolume", TokenType::ControllChangeCommand, '*', 7); // CC#7 Main Volume range:0-127 (ex) V(10)
    sysfunc_cc_add!(sf, "P", TokenType::ControllChangeCommand, '*', 10); // CC#10 Panpot range:0-63-127 (ex) P(63)
    sysfunc_cc_add!(sf, "Panpot", TokenType::ControllChangeCommand, '*', 10); // CC#10 Panpot range:0-63-127 (ex) Panpot(63)
    sysfunc_cc_add!(sf, "EP", TokenType::ControllChangeCommand, '*', 11); // CC#11 Expression range:0-127 (ex) EP(100)
    sysfunc_cc_add!(sf, "Expression", TokenType::ControllChangeCommand, '*', 11); // CC#11 Expression range:0-127 (ex) EP(100)
    sysfunc_cc_add!(sf, "PS", TokenType::ControllChangeCommand, '*', 65); // CC#65 Portament switch range:0-127 (ex) PS(1)
    sysfunc_cc_add!(sf, "PortamentoSwitch", TokenType::ControllChangeCommand, '*', 65); // CC#65 Portament switch range:0-127 (ex) PS(1)
    sysfunc_cc_add!(sf, "REV", TokenType::ControllChangeCommand, '*', 91); // CC#91 Reverb range:0-127 (ex) REV(100)
    sysfunc_cc_add!(sf, "Reverb", TokenType::ControllChangeCommand, '*', 91); // CC#91 Reverb range:0-127 (ex) REV(100)
    sysfunc_cc_add!(sf, "CHO", TokenType::ControllChangeCommand, '*', 93); // CC#93 Chorus range:0-127 (ex) CHO(100)
    sysfunc_cc_add!(sf, "Chorus", TokenType::ControllChangeCommand, '*', 93); // CC#93 Chorus range:0-127 (ex) Chorus(100)
    sysfunc_cc_add!(sf, "VAR", TokenType::ControllChangeCommand, '*', 94); // CC#94 Variation range:0-127 (ex) VAR(100)
    sysfunc_cc_add!(sf, "Variation", TokenType::ControllChangeCommand, '*', 94); // CC#94 Variation range:0-127 (ex) Variation(100)
    sysfunc_add!(sf, "PB", TokenType::PitchBend, '*'); // Pitchbend range: -8192...0...8191 (ex) PB(10)
    sysfunc_add!(sf, "RPN", TokenType::RPN, 'A'); // write RPN (ex) RPN(0,1,64)
    sysfunc_add!(sf, "NRPN", TokenType::NRPN, 'A'); // write NRPN (ex) NRPN(1,1,1)
    sysfunc_rpn_add!(sf, "BR", TokenType::RPNCommand, '*', 0, 0); // PitchBendSensitivity (ex) BR(10) 
    sysfunc_rpn_add!(sf, "PitchBendSensitivity", TokenType::RPNCommand, '*', 0, 0); // PitchBendSensitivity (ex) BR(10)
    sysfunc_rpn_add!(sf, "FineTune", TokenType::RPNCommand, '*', 0, 1); // set fine tune range:0-63-127(-100 - 0 - +99.99セント）(ex) FineTune(63)
    sysfunc_rpn_add!(sf, "CoarseTune", TokenType::RPNCommand, '*', 0, 2); // set coarse tune 半音単位のチューニング 範囲:40-64-88 (-24 - 0 - 24半音) (ex) CoarseTune(63)
    sysfunc_rpn_add!(sf, "VibratoRate", TokenType::NRPNCommand, '*', 1, 8); // set VibratoRate range: 0-127
    sysfunc_rpn_add!(sf, "VibratoDepth", TokenType::NRPNCommand, '*', 1, 9); // set VibratoRate range: 0-127
    sysfunc_rpn_add!(sf, "VibratoDelay", TokenType::NRPNCommand, '*', 1, 10); // set VibratoRate range: 0-127
    sysfunc_rpn_add!(sf, "FilterCutoff", TokenType::NRPNCommand, '*', 1, 0x20); // set FilterCutoff range: 0-127
    sysfunc_rpn_add!(sf, "FilterResonance", TokenType::NRPNCommand, '*', 1, 0x21); // set FilterResonance range: 0-127
    sysfunc_rpn_add!(sf, "EGAttack", TokenType::NRPNCommand, '*', 1, 0x63); // set EGAttack range: 0-127
    sysfunc_rpn_add!(sf, "EGDecay", TokenType::NRPNCommand, '*', 1, 0x64); // set EGDecay range: 0-127
    sysfunc_rpn_add!(sf, "EGRelease", TokenType::NRPNCommand, '*', 1, 0x66); // set EGRelease range: 0-127
    //@ fadein
    sysfunc_cc_add!(sf, "Fadein", TokenType::FadeIO, '*', 1); // fadein 小節数を指定 (ex) Fadein(1)
    sysfunc_cc_add!(sf, "Fadeout", TokenType::FadeIO, '*', -1); // fadeout 小節数を指定 (ex) Fadeout(1)
    sysfunc_cc_add!(sf, "Cresc", TokenType::Cresc, '*', 1); // cresc 小節数を指定 Cresc([[[len],v1],v2]) v1からv2へ変更する。lenを省略すると全音符の長さに (ex) Cresc(1)
    sysfunc_cc_add!(sf, "Decresc", TokenType::Cresc, '*', -1); // cresc 小節数を指定 Decresc([[[len],v1],v2]) v1からv2へ変更する。lenを省略すると全音符の長さに (ex) Deresc(1)
    sysfunc_cc_add!(sf, "CRESC", TokenType::Cresc, '*', 1); // cresc 小節数を指定 Cresc([[[len],v1],v2]) v1からv2へ変更する。lenを省略すると全音符の長さに (ex) Cresc(1)
    sysfunc_cc_add!(sf, "DECRESC", TokenType::Cresc, '*', -1); // cresc 小節数を指定 Decresc([[[len],v1],v2]) v1からv2へ変更する。lenを省略すると全音符の長さに (ex) Deresc(1)
    //@ SysEx / Meta
    sysfunc_cc_add!(sf, "ResetGM", TokenType::SysexReset, 'I', 0); // ResetGM
    sysfunc_cc_add!(sf, "ResetGS", TokenType::SysexReset, 'I', 1); // ResetGS
    sysfunc_cc_add!(sf, "ResetXG", TokenType::SysexReset, 'I', 2); // ResetXG
    sysfunc_add!(sf, "Tempo", TokenType::Tempo, 'I'); // set tempo (ex) Tempo(120)
    sysfunc_add!(sf, "TEMPO", TokenType::Tempo, 'I'); // set tempo (ex) TEMPO(120)
    sysfunc_add!(sf, "T", TokenType::Tempo, 'I'); // set tempo (ex) T(120)
    sysfunc_add!(sf, "BPM", TokenType::Tempo, 'I'); // set tempo (ex) BPM(120)
    sysfunc_add!(sf, "TempoChange", TokenType::TempoChange, 'A'); // tempo change slowly TempoChange(start, end, !len) (ex) TempoChange(80,120,!1)
    sysfunc_add!(sf, "TimeSignature", TokenType::TimeSignature, 'A'); // set time signature (ex) TimeSignature(4, 4)
    sysfunc_add!(sf, "System.TimeSignature", TokenType::TimeSignature, 'A'); // set time signature (ex) TimeSignature(4, 4)
    sysfunc_add!(sf, "TimeSig", TokenType::TimeSignature, 'A'); // set time signature (ex) TimeSignature(4, 4)
    sysfunc_add!(sf, "TIMESIG", TokenType::TimeSignature, 'A'); // set time signature (ex) TimeSignature(4, 4)
    sysfunc_add!(sf, "Port", TokenType::Port, 'I'); // set Port No (ex) Port(0)
    sysfunc_add!(sf, "PORT", TokenType::Port, 'I'); // set Port No (ex) Port(0)
    sysfunc_cc_add!(sf, "MetaText", TokenType::MetaText, 'S', 1); // write meta text (ex) MetaText{"hello"}
    sysfunc_cc_add!(sf, "Text", TokenType::MetaText, 'S', 1); // write meta text (ex) MetaText{"hello"}
    sysfunc_cc_add!(sf, "TEXT", TokenType::MetaText, 'S', 1); // write meta text (ex) MetaText{"hello"}
    sysfunc_cc_add!(sf, "Copyright", TokenType::MetaText, 'S', 2); // write copyright text (ex) Copyright{"hello"}
    sysfunc_cc_add!(sf, "COPYRIGHT", TokenType::MetaText, 'S', 2); // write copyright text (ex) COPYRIGHT{"hello"}
    sysfunc_cc_add!(sf, "TrackName", TokenType::MetaText, 'S', 3); // write TrackName text (ex) TrackName{"hello"}
    sysfunc_cc_add!(sf, "TRACK_NAME", TokenType::MetaText, 'S', 3); // write TrackName text (ex) TrackName{"hello"}
    sysfunc_cc_add!(sf, "InstrumentName", TokenType::MetaText, 'S', 4); // write InstrumentName text (ex) InstrumentName{"hello"}
    sysfunc_cc_add!(sf, "Lyric", TokenType::MetaText, 'S', 5); // write Lyric text (ex) Lyric{"hello"}
    sysfunc_cc_add!(sf, "LYRIC", TokenType::MetaText, 'S', 5); // write Lyric text (ex) LYRIC{"hello"}
    sysfunc_cc_add!(sf, "MAKER", TokenType::MetaText, 'S', 6); // write MAKER text (ex) MAKER{"hello"}
    sysfunc_cc_add!(sf, "Maker", TokenType::MetaText, 'S', 6); // write Maker text (ex) Maker{"hello"}
    sysfunc_cc_add!(sf, "CuePoint", TokenType::MetaText, 'S', 7); // write CuePoint text (ex) CuePoint{"hello"}
    //@ GS System Exclusive
    sysfunc_cc_add!(sf, "GSEffect", TokenType::GSEffect, 'A', 0); // GSEffect(num, val) (ex) GSEffect($30, 0)
    sysfunc_cc_add!(sf, "GSReverbMacro", TokenType::GSEffect, 'I', 0x30); // GSReverbMacro(val) - 0:Room1 5:Hall 6:Delay (ex) GSReverbMacro(0)
    sysfunc_cc_add!(sf, "GSReverbCharacter", TokenType::GSEffect, 'I', 0x31); // GSReverbCharacter(val) - 0:Room1 5:Hall 6:Delay (ex) GSReverbMacro(0)
    sysfunc_cc_add!(sf, "GSReverbPRE_LPE", TokenType::GSEffect, 'I', 0x32); // GSReverbPRE_LPE(val) (ex) GSReverbPRE_LPE(0)
    sysfunc_cc_add!(sf, "GSReverbLevel", TokenType::GSEffect, 'I', 0x33); // GSReverbLevel(val) (ex) GSReverbLevel(0)
    sysfunc_cc_add!(sf, "GSReverbTime", TokenType::GSEffect, 'I', 0x34); // GSReverbTime(val) (ex) GSReverbTime(0)
    sysfunc_cc_add!(sf, "GSReverbFeedback", TokenType::GSEffect, 'I', 0x35); // GSReverbFeedback(val) (ex) GSReverbFeedback(0)
    sysfunc_cc_add!(sf, "GSReverbSendToChorus", TokenType::GSEffect, 'I', 0x36); // GSReverbSendToChorus(val) (ex) GSReverbSendToChorus(0)
    sysfunc_cc_add!(sf, "GSChorusMacro", TokenType::GSEffect, 'I', 0x38); // GSChorusMacro(val) (ex) GSChorusMacro(0)
    sysfunc_cc_add!(sf, "GSChorusPRE_LPF", TokenType::GSEffect, 'I', 0x39); // GSChorusPRE_LPF(val) (ex) GSChorusPRE_LPF(0)
    sysfunc_cc_add!(sf, "GSChorusLevel", TokenType::GSEffect, 'I', 0x3A); // GSChorusLevel(val) (ex) GSChorusLevel(0)
    sysfunc_cc_add!(sf, "GSChorusFeedback", TokenType::GSEffect, 'I', 0x3B); // GSChorusFeedback(val) (ex) GSChorusFeedback(0)
    sysfunc_cc_add!(sf, "GSChorusDelay", TokenType::GSEffect, 'I', 0x3C); // GSChorusDelay(val) (ex) GSChorusDelay(0)
    sysfunc_cc_add!(sf, "GSChorusRate", TokenType::GSEffect, 'I', 0x3D); // GSChorusRate(val) (ex) GSChorusRate(0)
    sysfunc_cc_add!(sf, "GSChorusDepth", TokenType::GSEffect, 'I', 0x3E); // GSChorusDepth(val) (ex) GSChorusDepth(0)
    sysfunc_cc_add!(sf, "GSChorusSendToReverb", TokenType::GSEffect, 'I', 0x3F); // GSChorusSendToReverb(val) (ex) GSChorusSendToReverb(0)
    sysfunc_cc_add!(sf, "GSChorusSendToDelay", TokenType::GSEffect, 'I', 0x40); // GSChorusSendToDelay(val) (ex) GSChorusSendToDelay(0)
    sysfunc_cc_add!(sf, "GS_RHYTHM", TokenType::GSEffect, 'I', 0x15); // Change to rhythm part val=0:instrument/1:drum1/2:drum2 (ex) GSChorusSendToDelay(0)
    sysfunc_cc_add!(sf, "GSScaleTuning", TokenType::GSEffect, 'A', 0x11); // GS Scale Tuning. GSScaleTuning(C,Cp,D,Dp,E,F,Fp,G,Gp,A,Ap,B) (ex) GSScaleTuning(0,0,0,0,0,0,0,0,0,0,0,0)
    //@ Script command
    sysfunc_add!(sf, "Int", TokenType::DefInt, '*'); // define int variables (ex) Int A = 3
    sysfunc_add!(sf, "INT", TokenType::DefInt, '*'); // define int variables (ex) INT A = 3
    sysfunc_add!(sf, "Str", TokenType::DefStr, '*'); // define string variables (ex) Str A = {cde}
    sysfunc_add!(sf, "STR", TokenType::DefStr, '*'); // define string variables (ex) STR A = {cde}
    sysfunc_add!(sf, "Print", TokenType::Print, 'S'); // print value (ex) Print({hello})
    sysfunc_add!(sf, "PRINT", TokenType::Print, 'S'); // print value (ex) PRINT({hello})
    sysfunc_add!(sf, "System.Include", TokenType::Include, '*'); // Unimplemented
    sysfunc_add!(sf, "Include", TokenType::Include, '*'); // Unimplemented
    sysfunc_add!(sf, "INCLUDE", TokenType::Include, '*'); // Unimplemented
    sysfunc_add!(sf, "IF", TokenType::If, '*'); // IF(cond){ true }ELSE{ false }
    sysfunc_add!(sf, "If", TokenType::If, '*'); // IF(cond){ true }ELSE{ false }
    sysfunc_add!(sf, "FOR", TokenType::For, '*'); // FOR(INT I = 0; I < 10; I++){ ... }
    sysfunc_add!(sf, "For", TokenType::For, '*'); // FOR(INT I = 0; I < 10; I++){ ... }
    sysfunc_add!(sf, "WHILE", TokenType::While, '*'); // WHILE(cond) { ... }
    sysfunc_add!(sf, "While", TokenType::While, '*'); // WHILE(cond) { ... }
    sysfunc_add!(sf, "BREAK", TokenType::Break, '_'); // exit from loop
    sysfunc_add!(sf, "Break", TokenType::Break, '_'); // exit from loop
    sysfunc_add!(sf, "EXIT", TokenType::Break, '_'); // exit from loop
    sysfunc_add!(sf, "Exit", TokenType::Break, '_'); // exit from loop
    sysfunc_add!(sf, "CONTINUE", TokenType::Continue, '_'); // exit from loop
    sysfunc_add!(sf, "Continue", TokenType::Continue, '_'); // exit from loop
    sysfunc_add!(sf, "RETURN", TokenType::Return, '*'); // return from function
    sysfunc_add!(sf, "Return", TokenType::Return, '*'); // return from function
    sysfunc_add!(sf, "RANDOM_SEED", TokenType::SetRandomSeed, '*'); // set random seed
    sysfunc_add!(sf, "RandomSeed", TokenType::SetRandomSeed, '*'); // set random seed
    sysfunc_add!(sf, "FUNCTION", TokenType::DefUserFunction, '*'); // define user function
    sysfunc_add!(sf, "Function", TokenType::DefUserFunction, '*'); // define user function
    //</SYSTEM_FUNCTION>
    sf
}
