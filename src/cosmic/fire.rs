use crate::cosmic::recording::Phase;
use crate::cosmic::smart::features::DoubleFacedPieceType;
use crate::cosmic::smart::square::AbsoluteAddress2D;

/// TODO 局面の差分
#[derive(Debug)]
pub struct Fire {
    pub phase: Phase,
    pub address: FireAddress,
}
impl Fire {
    pub fn new_board(phase: Phase, addr: AbsoluteAddress2D) -> Self {
        Fire {
            phase: phase,
            address: FireAddress::Board(addr),
        }
    }
    pub fn new_hand(phase: Phase, drop_type: DoubleFacedPieceType) -> Self {
        Fire {
            phase: phase,
            address: FireAddress::Hand(drop_type),
        }
    }
}
/// 盤上と、駒台で　共通しないものを並列にします。
#[derive(Debug)]
pub enum FireAddress {
    Board(AbsoluteAddress2D),
    Hand(DoubleFacedPieceType),
}
