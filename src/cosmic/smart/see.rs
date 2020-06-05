//! Static exchange evaluation

use crate::cosmic::daydream::Value;
use crate::cosmic::playing::Game;
use crate::cosmic::recording::AddressOnPosition;

pub struct SEE {}
impl SEE {
    /// 葉で駒を取ったら、取り返されるのも考慮しないとな☆（＾～＾）
    ///
    /// 価値の低い駒から順に使って、取りに行けだぜ☆（＾～＾）
    pub fn go(_game: &Game, _adr: &AddressOnPosition) -> Value {
        Value::CentiPawn(0)
    }
}
