//! mml_def: リズムマクロの初期定義
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
    rhthm_macro['m' as usize - 0x40] = String::from("n46,");
    rhthm_macro['c' as usize - 0x40] = String::from("n49,");
    rhthm_macro['H' as usize - 0x40] = String::from("n50,");
    rhthm_macro['M' as usize - 0x40] = String::from("n47,");
    rhthm_macro['L' as usize - 0x40] = String::from("n43,");
    rhthm_macro['o' as usize - 0x40] = String::from("n46,");
    rhthm_macro['_' as usize - 0x40] = String::from("r");
    // </RHYTHM_MACRO>
    //
    rhthm_macro
}
