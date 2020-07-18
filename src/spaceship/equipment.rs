//! 宇宙船の備品だぜ☆（＾～＾）
use crate::cosmic::smart::square::test_rotation;

/// ちゆり「望遠鏡だぜ☆」
/// 夢見　「何も見えないんだけど？」
/// ちゆり「そうか、残念だな……☆」
pub struct Telescope {}
impl Telescope {
    pub fn look() {
        test_rotation();
    }
}

/// PV表示、または 文字列表示だぜ☆（＾～＾）
pub enum PvString {
    /// 思考を開始してからのミリ秒と、読み筋。
    PV(u128, String),
    String(String),
}
