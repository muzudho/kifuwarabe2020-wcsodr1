//! Static exchange evaluation

use crate::cosmic::daydream::Value;
use crate::cosmic::fire::Fire;
use crate::cosmic::playing::Game;

pub struct SEE {}
impl SEE {
    /// 葉で駒を取ったら、取り返されるのも考慮しないとな☆（＾～＾）
    ///
    /// 価値の低い駒から順に使って、取りに行けだぜ☆（＾～＾）
    pub fn go(_game: &Game, _fire: &Fire) -> Value {
        Value::CentiPawn(0)
    }
}
