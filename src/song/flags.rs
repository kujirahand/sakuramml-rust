//! song: 実行時フラグ
use super::*;

#[derive(Debug)]
pub struct Flags {
    pub harmony_flag: bool,
    pub harmony_time: isize,
    pub harmony_events: Vec<Event>,
    /// 和音の音符が実際に使ったゲート指定 (#127)
    /// (値, ステップ指定か, ゲートの先行指定が有効だったか)
    pub harmony_qlen: Option<(isize, bool, bool)>,
    pub octave_once: isize,
    pub measure_shift: isize,
    pub break_flag: isize, // 0: none 1: break 2: continue 3: return
    pub max_loop: isize,
    pub function_needs_return_value: bool,
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            harmony_flag: false,
            harmony_time: 0,
            harmony_events: vec![],
            harmony_qlen: None,
            octave_once: 0,
            measure_shift: 0,
            break_flag: 0,
            max_loop: 10000,
            function_needs_return_value: false,
        }
    }
}
