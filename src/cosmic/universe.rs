//!
//! Data transfer object.
//!
extern crate rand;

use crate::position::Position;

/// アプリケーション開始時に決め終えておくものだぜ☆（＾～＾）
pub struct Universe {
    /// 対局
    pub game: Position,
    /// 対話モード
    pub dialogue_mode: bool,
    /// 大会ルールの最大手数☆（＾～＾）
    pub option_max_ply: usize,
    /// 読みの最大深さ。
    pub option_max_depth: usize,
    /// 思考時間の最小秒☆（＾～＾）
    pub option_min_think_msec: u64,
    /// 思考時間の最大秒☆（＾～＾）
    pub option_max_think_msec: u64,
    /// 指し手がいっぱいあることを評価する重み☆（＾～＾）1000分率☆（＾～＾）
    pub option_many_ways_weight: isize,
    /// 諦めない深さ☆（＾～＾）読み終わるまで、思考時間を無視するぜ☆（＾～＾）max_depth - 1 より小さくしろだぜ☆（＾～＾）
    pub option_depth_not_to_give_up: usize,
    /// 駒割の重み☆（＾～＾）1000分率☆（＾～＾）
    pub option_komawari_weight: isize,
    /// 成りの重み☆（＾～＾）1000分率☆（＾～＾）
    pub option_promotion_weight: isize,
}
impl Default for Universe {
    fn default() -> Self {
        Universe {
            game: Position::default(),
            dialogue_mode: false,
            option_max_ply: 320,
            option_max_depth: 1,
            option_depth_not_to_give_up: 2,
            /// min < max にしろだぜ☆（＾～＾）
            option_min_think_msec: 7000,
            option_max_think_msec: 17000,
            option_many_ways_weight: 1000,
            option_komawari_weight: 1000,
            option_promotion_weight: 1000,
        }
    }
}
impl Universe {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        self.game.big_bang();
    }
}
