use crate::cosmic::recording::Phase;
use crate::cosmic::smart::features::{DoubleFacedPiece, DoubleFacedPieceType};
use crate::cosmic::smart::square::AbsoluteAddress2D;
use crate::law::cryptographic::num_to_lower_case;
use std::fmt;

/// TODO 局面の差分
#[derive(Copy, Clone, Debug)]
pub struct Fire {
    pub friend: Phase,
    pub address: FireAddress,
}
/// USI向け。
impl fmt::Display for Fire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.address {
                FireAddress::Board(sq) => {
                    let (file, rank) = sq.to_file_rank();
                    format!("{}{}", file, num_to_lower_case(rank))
                }
                FireAddress::Hand(drop_type) => {
                    format!(
                        "{}",
                        DoubleFacedPiece::from_phase_and_type(self.friend, drop_type)
                    )
                }
            },
        )
    }
}
impl Default for Fire {
    /// ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        Fire {
            friend: Phase::First,
            address: FireAddress::Board(AbsoluteAddress2D::default()),
        }
    }
}
impl Fire {
    pub fn new_board(friend: Phase, addr: AbsoluteAddress2D) -> Self {
        Fire {
            friend: friend,
            address: FireAddress::Board(addr),
        }
    }
    pub fn new_hand(friend: Phase, drop_type: DoubleFacedPieceType) -> Self {
        Fire {
            friend: friend,
            address: FireAddress::Hand(drop_type),
        }
    }
}
/// 盤上と、駒台で　共通しないものを並列にします。
#[derive(Copy, Clone, Debug)]
pub enum FireAddress {
    Board(AbsoluteAddress2D),
    Hand(DoubleFacedPieceType),
}
