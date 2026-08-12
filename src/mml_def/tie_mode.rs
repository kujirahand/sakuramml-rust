//! mml_def: タイ(&)のモード定義
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
