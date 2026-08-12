//! mml_def: 予約語の定義
use std::collections::HashMap;
use super::*;

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
