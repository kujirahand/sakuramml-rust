//! Define MML Commands and Macros

use crate::sakura_version;
use crate::svalue::SValue;
use std::collections::HashMap;

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
    var.insert(
        String::from("SAKURA_VERSION"),
        SValue::from_s(sakura_version::SAKURA_VERSION.to_string()),
    ); // @ サクラのバージョン情報を得る
    var.insert(
        String::from("OctaveUnison"),
        SValue::from_str("Sub{> #?1 <} #?1"),
    ); // @ オクターブユニゾンを演奏 (例 OctaveUnison{cde})
    var.insert(
        String::from("Unison5th"),
        SValue::from_str("Sub{ Key=7 #?1 Key=0 } #?1"),
    ); // @ 5度のユニゾンを演奏 (例 Unison5th{cde})
    var.insert(
        String::from("Unison3th"),
        SValue::from_str("Sub{ Key=4 #?1 Key=0 } #?1"),
    ); // @ 3度のユニゾンを演奏 (例 Unison3th{cde})
    var.insert(
        String::from("Unison"),
        SValue::from_str("Sub{ Key=#?2 #?1 Key=0 } #?1"),
    ); // @ N度のユニゾンを演奏 (例 Unison{cde},7)
       //
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
