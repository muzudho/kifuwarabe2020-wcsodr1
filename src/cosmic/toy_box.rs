//!
//! 駒 と 盤
//!
use crate::cosmic::recording::AddressPos3;
use crate::cosmic::recording::Movement;
use crate::cosmic::recording::{AddressPos, Phase};
use crate::cosmic::smart::features::{DoubleFacedPiece, PieceType, PHYSICAL_PIECE_TYPE_LEN};
use crate::cosmic::smart::square::{AbsoluteAddress2D, BOARD_MEMORY_AREA, RANK_1, RANK_10};
use crate::law::generate_move::Area;
use crate::law::speed_of_light::Nine299792458;
use crate::spaceship::equipment::Beam;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::*;

pub const PIECE_LEN: usize = 28;

/// 統一アドレス。
#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum UnifiedAddress {
    Sq11_1,
    Sq12_1,
    Sq13_1,
    Sq14_1,
    Sq15_1,
    Sq16_1,
    Sq17_1,
    Sq18_1,
    Sq19_1,
    Sq21_1,
    Sq22_1,
    Sq23_1,
    Sq24_1,
    Sq25_1,
    Sq26_1,
    Sq27_1,
    Sq28_1,
    Sq29_1,
    Sq31_1,
    Sq32_1,
    Sq33_1,
    Sq34_1,
    Sq35_1,
    Sq36_1,
    Sq37_1,
    Sq38_1,
    Sq39_1,
    Sq41_1,
    Sq42_1,
    Sq43_1,
    Sq44_1,
    Sq45_1,
    Sq46_1,
    Sq47_1,
    Sq48_1,
    Sq49_1,
    Sq51_1,
    Sq52_1,
    Sq53_1,
    Sq54_1,
    Sq55_1,
    Sq56_1,
    Sq57_1,
    Sq58_1,
    Sq59_1,
    Sq61_1,
    Sq62_1,
    Sq63_1,
    Sq64_1,
    Sq65_1,
    Sq66_1,
    Sq67_1,
    Sq68_1,
    Sq69_1,
    Sq71_1,
    Sq72_1,
    Sq73_1,
    Sq74_1,
    Sq75_1,
    Sq76_1,
    Sq77_1,
    Sq78_1,
    Sq79_1,
    Sq81_1,
    Sq82_1,
    Sq83_1,
    Sq84_1,
    Sq85_1,
    Sq86_1,
    Sq87_1,
    Sq88_1,
    Sq89_1,
    Sq91_1,
    Sq92_1,
    Sq93_1,
    Sq94_1,
    Sq95_1,
    Sq96_1,
    Sq97_1,
    Sq98_1,
    Sq99_1,
    Sq11_2,
    Sq12_2,
    Sq13_2,
    Sq14_2,
    Sq15_2,
    Sq16_2,
    Sq17_2,
    Sq18_2,
    Sq19_2,
    Sq21_2,
    Sq22_2,
    Sq23_2,
    Sq24_2,
    Sq25_2,
    Sq26_2,
    Sq27_2,
    Sq28_2,
    Sq29_2,
    Sq31_2,
    Sq32_2,
    Sq33_2,
    Sq34_2,
    Sq35_2,
    Sq36_2,
    Sq37_2,
    Sq38_2,
    Sq39_2,
    Sq41_2,
    Sq42_2,
    Sq43_2,
    Sq44_2,
    Sq45_2,
    Sq46_2,
    Sq47_2,
    Sq48_2,
    Sq49_2,
    Sq51_2,
    Sq52_2,
    Sq53_2,
    Sq54_2,
    Sq55_2,
    Sq56_2,
    Sq57_2,
    Sq58_2,
    Sq59_2,
    Sq61_2,
    Sq62_2,
    Sq63_2,
    Sq64_2,
    Sq65_2,
    Sq66_2,
    Sq67_2,
    Sq68_2,
    Sq69_2,
    Sq71_2,
    Sq72_2,
    Sq73_2,
    Sq74_2,
    Sq75_2,
    Sq76_2,
    Sq77_2,
    Sq78_2,
    Sq79_2,
    Sq81_2,
    Sq82_2,
    Sq83_2,
    Sq84_2,
    Sq85_2,
    Sq86_2,
    Sq87_2,
    Sq88_2,
    Sq89_2,
    Sq91_2,
    Sq92_2,
    Sq93_2,
    Sq94_2,
    Sq95_2,
    Sq96_2,
    Sq97_2,
    Sq98_2,
    Sq99_2,
    King1,
    Rook1,
    Bishop1,
    Gold1,
    Silver1,
    Knight1,
    Lance1,
    Pawn1,
    King2,
    Rook2,
    Bishop2,
    Gold2,
    Silver2,
    Knight2,
    Lance2,
    Pawn2,
}
impl Default for UnifiedAddress {
    /// ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        UnifiedAddress::Sq11_1
    }
}
impl UnifiedAddress {
    pub fn from_absolute_address(friend: Phase, addr: &AbsoluteAddress2D) -> Self {
        let second = if friend == Phase::Second { 81 } else { 0 };
        let num = addr.serial_number();
        if 10 < num && num < 20 {
            if let Some(val) = UnifiedAddress::from_usize(second + num - 11) {
                val
            } else {
                panic!(Beam::trouble("(Err.124) 番地を変換できね☆（＾～＾）"))
            }
        } else if 20 < num && num < 30 {
            if let Some(val) = UnifiedAddress::from_usize(second + num - 21 + 9) {
                val
            } else {
                panic!(Beam::trouble("(Err.131) 番地を変換できね☆（＾～＾）"))
            }
        } else if 30 < num && num < 40 {
            if let Some(val) = UnifiedAddress::from_usize(second + num - 31 + 2 * 9) {
                val
            } else {
                panic!(Beam::trouble("(Err.137) 番地を変換できね☆（＾～＾）"))
            }
        } else if 40 < num && num < 50 {
            if let Some(val) = UnifiedAddress::from_usize(second + num - 41 + 3 * 9) {
                val
            } else {
                panic!(Beam::trouble("(Err.143) 番地を変換できね☆（＾～＾）"))
            }
        } else if 50 < num && num < 60 {
            if let Some(val) = UnifiedAddress::from_usize(second + num - 51 + 4 * 9) {
                val
            } else {
                panic!(Beam::trouble("(Err.149) 番地を変換できね☆（＾～＾）"))
            }
        } else if 60 < num && num < 70 {
            if let Some(val) = UnifiedAddress::from_usize(second + num - 61 + 5 * 9) {
                val
            } else {
                panic!(Beam::trouble("(Err.155) 番地を変換できね☆（＾～＾）"))
            }
        } else if 70 < num && num < 80 {
            if let Some(val) = UnifiedAddress::from_usize(second + num - 71 + 6 * 9) {
                val
            } else {
                panic!(Beam::trouble("(Err.161) 番地を変換できね☆（＾～＾）"))
            }
        } else if 80 < num && num < 90 {
            if let Some(val) = UnifiedAddress::from_usize(second + num - 81 + 7 * 9) {
                val
            } else {
                panic!(Beam::trouble("(Err.167) 番地を変換できね☆（＾～＾）"))
            }
        } else if 90 < num && num < 100 {
            if let Some(val) = UnifiedAddress::from_usize(second + num - 91 + 8 * 9) {
                val
            } else {
                panic!(Beam::trouble("(Err.173) 番地を変換できね☆（＾～＾）"))
            }
        } else {
            panic!(Beam::trouble("(Err.176) 番地を変換できね☆（＾～＾）"))
        }
    }

    pub fn to_absolute_address(self) -> AbsoluteAddress2D {
        if let Some(val) = AbsoluteAddress2D::from_absolute_address(self as usize) {
            val
        } else {
            panic!(Beam::trouble("(Err.135) 番地を変換できね☆（＾～＾）"))
        }
    }

    pub fn from_double_faced_piece(drop: DoubleFacedPiece) -> UnifiedAddress {
        const MAP: [UnifiedAddress; 16] = [
            UnifiedAddress::King1,
            UnifiedAddress::Rook1,
            UnifiedAddress::Bishop1,
            UnifiedAddress::Gold1,
            UnifiedAddress::Silver1,
            UnifiedAddress::Knight1,
            UnifiedAddress::Lance1,
            UnifiedAddress::Pawn1,
            UnifiedAddress::King2,
            UnifiedAddress::Rook2,
            UnifiedAddress::Bishop2,
            UnifiedAddress::Gold2,
            UnifiedAddress::Silver2,
            UnifiedAddress::Knight2,
            UnifiedAddress::Lance2,
            UnifiedAddress::Pawn2,
        ];
        MAP[drop as usize]
    }

    pub fn to_double_faced_piece(self) -> DoubleFacedPiece {
        const MAP: [DoubleFacedPiece; 16] = [
            DoubleFacedPiece::King1,
            DoubleFacedPiece::Rook1,
            DoubleFacedPiece::Bishop1,
            DoubleFacedPiece::Gold1,
            DoubleFacedPiece::Silver1,
            DoubleFacedPiece::Knight1,
            DoubleFacedPiece::Lance1,
            DoubleFacedPiece::Pawn1,
            DoubleFacedPiece::King2,
            DoubleFacedPiece::Rook2,
            DoubleFacedPiece::Bishop2,
            DoubleFacedPiece::Gold2,
            DoubleFacedPiece::Silver2,
            DoubleFacedPiece::Knight2,
            DoubleFacedPiece::Lance2,
            DoubleFacedPiece::Pawn2,
        ];
        MAP[self as usize]
    }

    pub fn from_address_pos(friend: Phase, addr: &AddressPos) -> Self {
        match addr {
            AddressPos::Board(sq) => UnifiedAddress::from_absolute_address(friend, sq),
            AddressPos::Hand(drop) => UnifiedAddress::from_double_faced_piece(*drop),
        }
    }

    pub fn to_address_pos(self) -> AddressPos {
        const MAP: [AddressPos; 178] = [
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 1, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 2, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 3, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 4, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 5, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 6, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 7, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 8, rank: 9 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 1 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 2 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 3 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 4 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 5 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 6 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 7 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 8 }),
            AddressPos::Board(AbsoluteAddress2D { file: 9, rank: 9 }),
            AddressPos::Hand(DoubleFacedPiece::King1),
            AddressPos::Hand(DoubleFacedPiece::Rook1),
            AddressPos::Hand(DoubleFacedPiece::Bishop1),
            AddressPos::Hand(DoubleFacedPiece::Gold1),
            AddressPos::Hand(DoubleFacedPiece::Silver1),
            AddressPos::Hand(DoubleFacedPiece::Knight1),
            AddressPos::Hand(DoubleFacedPiece::Lance1),
            AddressPos::Hand(DoubleFacedPiece::Pawn1),
            AddressPos::Hand(DoubleFacedPiece::King2),
            AddressPos::Hand(DoubleFacedPiece::Rook2),
            AddressPos::Hand(DoubleFacedPiece::Bishop2),
            AddressPos::Hand(DoubleFacedPiece::Gold2),
            AddressPos::Hand(DoubleFacedPiece::Silver2),
            AddressPos::Hand(DoubleFacedPiece::Knight2),
            AddressPos::Hand(DoubleFacedPiece::Lance2),
            AddressPos::Hand(DoubleFacedPiece::Pawn2),
        ];
        MAP[self as usize]
    }

    pub fn to_address_pos3(self) -> AddressPos3 {
        const MAP: [AddressPos3; 178] = [
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 1, rank: 1 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 1, rank: 2 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 1, rank: 3 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 1, rank: 4 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 1, rank: 5 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 1, rank: 6 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 1, rank: 7 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 1, rank: 8 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 1, rank: 9 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 2, rank: 1 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 2, rank: 2 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 2, rank: 3 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 2, rank: 4 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 2, rank: 5 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 2, rank: 6 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 2, rank: 7 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 2, rank: 8 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 2, rank: 9 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 3, rank: 1 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 3, rank: 2 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 3, rank: 3 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 3, rank: 4 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 3, rank: 5 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 3, rank: 6 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 3, rank: 7 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 3, rank: 8 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 3, rank: 9 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 4, rank: 1 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 4, rank: 2 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 4, rank: 3 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 4, rank: 4 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 4, rank: 5 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 4, rank: 6 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 4, rank: 7 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 4, rank: 8 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 4, rank: 9 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 5, rank: 1 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 5, rank: 2 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 5, rank: 3 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 5, rank: 4 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 5, rank: 5 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 5, rank: 6 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 5, rank: 7 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 5, rank: 8 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 5, rank: 9 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 6, rank: 1 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 6, rank: 2 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 6, rank: 3 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 6, rank: 4 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 6, rank: 5 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 6, rank: 6 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 6, rank: 7 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 6, rank: 8 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 6, rank: 9 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 7, rank: 1 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 7, rank: 2 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 7, rank: 3 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 7, rank: 4 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 7, rank: 5 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 7, rank: 6 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 7, rank: 7 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 7, rank: 8 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 7, rank: 9 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 8, rank: 1 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 8, rank: 2 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 8, rank: 3 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 8, rank: 4 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 8, rank: 5 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 8, rank: 6 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 8, rank: 7 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 8, rank: 8 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 8, rank: 9 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 9, rank: 1 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 9, rank: 2 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 9, rank: 3 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 9, rank: 4 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 9, rank: 5 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 9, rank: 6 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 9, rank: 7 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 9, rank: 8 }),
            AddressPos3::FirstBoard(AbsoluteAddress2D { file: 9, rank: 9 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 1, rank: 1 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 1, rank: 2 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 1, rank: 3 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 1, rank: 4 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 1, rank: 5 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 1, rank: 6 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 1, rank: 7 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 1, rank: 8 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 1, rank: 9 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 2, rank: 1 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 2, rank: 2 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 2, rank: 3 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 2, rank: 4 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 2, rank: 5 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 2, rank: 6 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 2, rank: 7 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 2, rank: 8 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 2, rank: 9 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 3, rank: 1 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 3, rank: 2 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 3, rank: 3 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 3, rank: 4 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 3, rank: 5 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 3, rank: 6 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 3, rank: 7 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 3, rank: 8 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 3, rank: 9 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 4, rank: 1 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 4, rank: 2 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 4, rank: 3 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 4, rank: 4 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 4, rank: 5 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 4, rank: 6 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 4, rank: 7 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 4, rank: 8 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 4, rank: 9 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 5, rank: 1 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 5, rank: 2 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 5, rank: 3 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 5, rank: 4 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 5, rank: 5 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 5, rank: 6 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 5, rank: 7 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 5, rank: 8 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 5, rank: 9 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 6, rank: 1 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 6, rank: 2 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 6, rank: 3 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 6, rank: 4 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 6, rank: 5 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 6, rank: 6 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 6, rank: 7 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 6, rank: 8 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 6, rank: 9 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 7, rank: 1 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 7, rank: 2 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 7, rank: 3 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 7, rank: 4 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 7, rank: 5 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 7, rank: 6 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 7, rank: 7 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 7, rank: 8 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 7, rank: 9 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 8, rank: 1 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 8, rank: 2 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 8, rank: 3 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 8, rank: 4 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 8, rank: 5 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 8, rank: 6 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 8, rank: 7 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 8, rank: 8 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 8, rank: 9 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 9, rank: 1 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 9, rank: 2 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 9, rank: 3 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 9, rank: 4 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 9, rank: 5 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 9, rank: 6 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 9, rank: 7 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 9, rank: 8 }),
            AddressPos3::SecondBoard(AbsoluteAddress2D { file: 9, rank: 9 }),
            AddressPos3::Hand(DoubleFacedPiece::King1),
            AddressPos3::Hand(DoubleFacedPiece::Rook1),
            AddressPos3::Hand(DoubleFacedPiece::Bishop1),
            AddressPos3::Hand(DoubleFacedPiece::Gold1),
            AddressPos3::Hand(DoubleFacedPiece::Silver1),
            AddressPos3::Hand(DoubleFacedPiece::Knight1),
            AddressPos3::Hand(DoubleFacedPiece::Lance1),
            AddressPos3::Hand(DoubleFacedPiece::Pawn1),
            AddressPos3::Hand(DoubleFacedPiece::King2),
            AddressPos3::Hand(DoubleFacedPiece::Rook2),
            AddressPos3::Hand(DoubleFacedPiece::Bishop2),
            AddressPos3::Hand(DoubleFacedPiece::Gold2),
            AddressPos3::Hand(DoubleFacedPiece::Silver2),
            AddressPos3::Hand(DoubleFacedPiece::Knight2),
            AddressPos3::Hand(DoubleFacedPiece::Lance2),
            AddressPos3::Hand(DoubleFacedPiece::Pawn2),
        ];
        MAP[self as usize]
    }

    pub fn to_square_serial_number(self) -> usize {
        const MAP: [usize; 178] = [
            11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 32, 33, 34,
            35, 36, 37, 38, 39, 41, 42, 43, 44, 45, 46, 47, 48, 49, 51, 52, 53, 54, 55, 56, 57, 58,
            59, 61, 62, 63, 64, 65, 66, 67, 68, 69, 71, 72, 73, 74, 75, 76, 77, 78, 79, 81, 82, 83,
            84, 85, 86, 87, 88, 89, 91, 92, 93, 94, 95, 96, 97, 98, 99, 11, 12, 13, 14, 15, 16, 17,
            18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 32, 33, 34, 35, 36, 37, 38, 39, 41, 42,
            43, 44, 45, 46, 47, 48, 49, 51, 52, 53, 54, 55, 56, 57, 58, 59, 61, 62, 63, 64, 65, 66,
            67, 68, 69, 71, 72, 73, 74, 75, 76, 77, 78, 79, 81, 82, 83, 84, 85, 86, 87, 88, 89, 91,
            92, 93, 94, 95, 96, 97, 98, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let sq = MAP[self as usize];
        if sq == 0 {
            panic!(Beam::trouble(&format!(
                "(Err.710) 盤上ではなかったぜ☆（＾～＾）！",
            )))
        }
        sq
    }

    pub fn to_phase(self) -> Phase {
        const MAP: [Phase; 178] = [
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::First,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
            Phase::Second,
        ];
        MAP[self as usize]
    }
}

/// 先後付きの駒と空白。
/// 接尾辞の 1 は先手、 2 は後手。
///
/// Copy: 配列の要素の初期化のために利用。
#[derive(Copy, Clone, PartialEq, FromPrimitive)]
pub enum Piece {
    // ▲玉
    King1,
    // ▲きりん
    Rook1,
    // ▲ぞう
    Bishop1,
    // ▲いぬ
    Gold1,
    // ▲ねこ
    Silver1,
    // ▲うさぎ
    Knight1,
    // ▲いのしし
    Lance1,
    // ▲ひよこ
    Pawn1,
    // ▲ぱわーあっぷきりん
    Dragon1,
    // ▲ぱわーあっぷぞう
    Horse1,
    // ▲ぱわーあっぷねこ
    PromotedSilver1,
    // ▲ぱわーあっぷうさぎ
    PromotedKnight1,
    // ▲ぱわーあっぷいのしし
    PromotedLance1,
    // ▲ぱわーあっぷひよこ
    PromotedPawn1,
    // ▽ライオン
    King2,
    // ▽キリン
    Rook2,
    // ▽ゾウ
    Bishop2,
    // ▽イヌ
    Gold2,
    // ▽ネコ
    Silver2,
    // ▽ウサギ
    Knight2,
    // ▽イノシシ
    Lance2,
    // ▽ヒヨコ
    Pawn2,
    // ▽パワーアップキリン
    Dragon2,
    // ▽パワーアップゾウ
    Horse2,
    // ▽パワーアップネコ
    PromotedSilver2,
    // ▽パワーアップウサギ
    PromotedKnight2,
    // ▽パワーアップイノシシ
    PromotedLance2,
    // ▽パワーアップヒヨコ
    PromotedPawn2,
}
pub static PIECE_WHITE_SPACE: &str = "    ";
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // ▲、▽ が半角サイズなのは、Windows Terminal の担当者 いい加減だぜ☆（＾～＾）
        use crate::cosmic::toy_box::Piece::*;
        match *self {
            King1 => write!(f, " ▲K "),
            Rook1 => write!(f, " ▲R "),
            Bishop1 => write!(f, " ▲B "),
            Gold1 => write!(f, " ▲G "),
            Silver1 => write!(f, " ▲S "),
            Knight1 => write!(f, " ▲N "),
            Lance1 => write!(f, " ▲L "),
            Pawn1 => write!(f, " ▲P "),
            Dragon1 => write!(f, " ▲PR"),
            Horse1 => write!(f, " ▲PB"),
            PromotedSilver1 => write!(f, " ▲PS"),
            PromotedKnight1 => write!(f, " ▲PN"),
            PromotedLance1 => write!(f, " ▲PL"),
            PromotedPawn1 => write!(f, " ▲PP"),
            King2 => write!(f, " ▽k "),
            Rook2 => write!(f, " ▽r "),
            Bishop2 => write!(f, " ▽b "),
            Gold2 => write!(f, " ▽g "),
            Silver2 => write!(f, " ▽s "),
            Knight2 => write!(f, " ▽n "),
            Lance2 => write!(f, " ▽l "),
            Pawn2 => write!(f, " ▽p "),
            Dragon2 => write!(f, " ▽pr"),
            Horse2 => write!(f, " ▽pb"),
            PromotedSilver2 => write!(f, " ▽ps"),
            PromotedKnight2 => write!(f, " ▽pn"),
            PromotedLance2 => write!(f, " ▽pl"),
            PromotedPawn2 => write!(f, " ▽pp"),
        }
    }
}

/// ちゆり「駒そのものではなく、駒の情報が欲しいだけなら、これだぜ☆」
pub struct PieceInfo {
    pub piece: String,
    pub num: String,
}
impl PieceInfo {
    pub fn new(piece: Piece, num: PieceNum) -> Self {
        PieceInfo {
            piece: format!("{}", piece),
            num: format!("{:?}", num),
        }
    }
}

/// 背番号(名前)付きの駒の数。
pub const NAMED_PIECES_LEN: usize = 40;

/// 駒の背番号（名前）だぜ☆（＾～＾）大橋流で触る駒の順だぜ☆（＾～＾）
#[derive(Clone, Copy, FromPrimitive, Debug, PartialEq)]
pub enum PieceNum {
    // 1 先手玉
    King1,
    // 2 後手玉
    King2,
    // 3 金
    Gold3,
    // 4 金
    Gold4,
    // 5 金
    Gold5,
    // 6 金
    Gold6,
    // 7 銀
    Silver7,
    // 8 銀
    Silver8,
    // 9 銀
    Silver9,
    // 10 銀
    Silver10,
    // 11 桂
    Knight11,
    // 12 桂
    Knight12,
    // 13 桂
    Knight13,
    // 14 桂
    Knight14,
    // 15 香
    Lance15,
    // 16 香
    Lance16,
    // 17 香
    Lance17,
    // 18 香
    Lance18,
    // 19 角
    Bishop19,
    // 20 角
    Bishop20,
    // 21 飛
    Rook21,
    // 22 飛
    Rook22,
    // 23 歩
    Pawn23,
    // 24 歩
    Pawn24,
    // 25 歩
    Pawn25,
    // 26 歩
    Pawn26,
    // 27 歩
    Pawn27,
    // 28 歩
    Pawn28,
    // 29 歩
    Pawn29,
    // 30 歩
    Pawn30,
    // 31 歩
    Pawn31,
    // 32 歩
    Pawn32,
    // 33 歩
    Pawn33,
    // 34 歩
    Pawn34,
    // 35 歩
    Pawn35,
    // 36 歩
    Pawn36,
    // 37 歩
    Pawn37,
    // 38 歩
    Pawn38,
    // 39 歩
    Pawn39,
    // 40 歩
    Pawn40,
}

/// 卓☆（＾～＾）
/// でかいのでコピーもクローンも不可☆（＾～＾）！
/// 10の位を筋、1の位を段とする。
/// 0筋、0段は未使用
pub struct GameTable {
    /// 盤に、駒が紐づくぜ☆（＾～＾）
    board: [Option<PieceNum>; BOARD_MEMORY_AREA as usize],
    /// 背番号付きの駒に、番地が紐づいているぜ☆（＾～＾）
    address_list: [UnifiedAddress; NAMED_PIECES_LEN],
    /// 駒の背番号に、駒が紐づくぜ☆（＾～＾）
    piece_list: [Piece; NAMED_PIECES_LEN],
    /// 駒の背番号を付けるのに使うぜ☆（＾～＾）
    double_faced_piece_type_index: [usize; PHYSICAL_PIECE_TYPE_LEN],
    /// 持ち駒☆（＾～＾）TODO 固定長サイズのスタックを用意したいぜ☆（＾～＾）
    phase_classification: PhaseClassification,
    /// 指し手生成に利用☆（＾～＾）
    pub area: Area,
}
impl Default for GameTable {
    fn default() -> Self {
        GameTable {
            // 盤上
            board: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            /// 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
            address_list: [UnifiedAddress::default(); NAMED_PIECES_LEN],
            /// 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
            piece_list: [Piece::King1; NAMED_PIECES_LEN],
            double_faced_piece_type_index: [
                PieceNum::King1 as usize,
                PieceNum::Rook21 as usize,
                PieceNum::Bishop19 as usize,
                PieceNum::Gold3 as usize,
                PieceNum::Silver7 as usize,
                PieceNum::Knight11 as usize,
                PieceNum::Lance15 as usize,
                PieceNum::Pawn23 as usize,
            ],
            // 持ち駒
            phase_classification: PhaseClassification::default(),
            area: Area::default(),
        }
    }
}
impl GameTable {
    pub fn clear(&mut self) {
        self.board = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None,
        ];
        // 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
        self.address_list = [UnifiedAddress::default(); NAMED_PIECES_LEN];
        // 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
        self.piece_list = [Piece::King1; NAMED_PIECES_LEN];
        self.double_faced_piece_type_index = [
            PieceNum::King1 as usize,
            PieceNum::Rook21 as usize,
            PieceNum::Bishop19 as usize,
            PieceNum::Gold3 as usize,
            PieceNum::Silver7 as usize,
            PieceNum::Knight11 as usize,
            PieceNum::Lance15 as usize,
            PieceNum::Pawn23 as usize,
        ];
        // 持ち駒☆（＾～＾）
        self.phase_classification = PhaseClassification::default();
    }

    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, table: &GameTable) {
        self.board = table.board.clone();
        self.address_list = table.address_list.clone();
        self.piece_list = table.piece_list.clone();
        self.double_faced_piece_type_index = table.double_faced_piece_type_index.clone();
        self.phase_classification = table.phase_classification.clone();
    }

    /// TODO 駒はカプセル化したいんで、なるべく他のメソッド使えだぜ☆（＾～＾）
    pub fn get_piece(&self, num: PieceNum) -> Piece {
        self.piece_list[num as usize]
    }
    pub fn get_phase(&self, num: PieceNum) -> Phase {
        self.piece_list[num as usize].phase()
    }
    pub fn get_type(&self, num: PieceNum) -> PieceType {
        self.piece_list[num as usize].type_()
    }
    pub fn get_double_faced_piece(&self, num: PieceNum) -> DoubleFacedPiece {
        self.piece_list[num as usize].double_faced_piece()
    }
    fn new_piece_num(&mut self, piece: Piece, num: PieceNum) -> PieceNum {
        self.piece_list[num as usize] = piece;
        num
    }
    pub fn turn_phase(&mut self, num: PieceNum) {
        self.piece_list[num as usize] = self.piece_list[num as usize].captured();
    }
    // 成り駒にします。
    pub fn promote(&mut self, num: PieceNum) {
        self.piece_list[num as usize] = self.piece_list[num as usize].promoted();
    }
    // 成っていない駒にします。
    pub fn demote(&mut self, num: PieceNum) {
        self.piece_list[num as usize] = self.piece_list[num as usize].demoted();
    }

    /// ドゥ時の動き。
    /// 駒の先後を反転させるぜ☆（＾～＾）
    // あれば　盤の相手の駒を先後反転して、自分の駒台に置きます。
    pub fn rotate_piece_board_to_hand(&mut self, move_: &Movement) {
        if let Some(collision_piece_num_val) = self.pop_piece(move_.destination) {
            // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
            // 先後ひっくり返す。
            self.turn_phase(collision_piece_num_val);
            self.push_piece(
                UnifiedAddress::from_address_pos(
                    self.get_phase(collision_piece_num_val),
                    &AddressPos::Hand(self.get_double_faced_piece(collision_piece_num_val)),
                ),
                Some(collision_piece_num_val),
            );
        }
    }

    /// アンドゥ時の動き。
    /// あれば、指し手で取った駒の先後をひっくり返せば、自分の駒台にある駒を取り出せるので取り出して、盤の上に指し手の取った駒のまま駒を置きます。
    pub fn rotate_piece_hand_to_board(&mut self, friend: Phase, move_: &Movement) {
        if let Some(move2_val) = move_.captured {
            // 取った方の駒台の先後に合わせるぜ☆（＾～＾）
            // 取った方の持ち駒を減らす
            let piece_num = {
                // TODO テスト中☆（＾～＾）
                let double_faced_piece = DoubleFacedPiece::from_phase_and_type(
                    friend,
                    // friend.turn(),
                    move2_val.piece_type.double_faced_piece_type(),
                );
                let addr_pos1 = AddressPos::Hand(double_faced_piece);
                let uni_addr = UnifiedAddress::from_address_pos(friend, &addr_pos1);
                // let addr_pos2 = uni_addr.to_address_pos();
                /*
                Beam::shoot(&format!(
                    "addr_pos {} -> {:?} -> {}",
                    addr_pos1, uni_addr, addr_pos2
                ));
                */
                self.pop_piece(uni_addr).unwrap()
            };
            // 先後をひっくり返す。
            self.turn_phase(piece_num);
            if move2_val.piece_type.promoted() {
                // 成り駒にします。
                self.promote(piece_num);
            } else {
                // 成っていない駒にします。
                self.demote(piece_num);
            }
            // 取られた方に、駒を返すぜ☆（＾～＾）置くのは指し手の移動先☆（＾～＾）
            self.push_piece(move_.destination, Some(piece_num));
        }
    }
    /// 駒を置く。
    pub fn push_piece(&mut self, addr: UnifiedAddress, piece_num: Option<PieceNum>) {
        match addr.to_address_pos() {
            AddressPos::Board(sq) => {
                if let Some(piece_num_val) = piece_num {
                    // マスに駒を置きます。
                    self.board[sq.serial_number() as usize] = piece_num;
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] = UnifiedAddress::from_address_pos(
                        self.get_phase(piece_num_val),
                        &AddressPos::Board(sq),
                    );
                } else {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                }
            }
            AddressPos::Hand(drop) => {
                if let Some(piece_num_val) = piece_num {
                    // 持ち駒を１つ増やします。
                    self.phase_classification.push(drop, piece_num_val);
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] = addr;
                }
            }
        }
    }
    /// 駒を取りのぞく。
    pub fn pop_piece(&mut self, addr: UnifiedAddress) -> Option<PieceNum> {
        match addr.to_address_pos() {
            AddressPos::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                    // TODO 背番号の番地を、ゴミ値で塗りつぶすが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                    self.address_list[piece_num_val as usize] = UnifiedAddress::from_address_pos(
                        self.get_phase(piece_num_val),
                        &AddressPos::Board(AbsoluteAddress2D::default()),
                    );
                }
                piece_num
            }
            AddressPos::Hand(drop) => {
                // 場所で指定します。
                // 台から取りのぞきます。
                let piece_num = self.phase_classification.pop(drop);
                // TODO 背番号の番地に、ゴミ値を入れて消去するが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                self.address_list[piece_num as usize] = UnifiedAddress::from_address_pos(
                    self.get_phase(piece_num),
                    &AddressPos::Board(AbsoluteAddress2D::default()),
                );
                Some(piece_num)
            }
        }
    }

    /// 駒の新しい背番号を生成します。
    pub fn numbering_piece(&mut self, piece: Piece) -> PieceNum {
        match piece {
            // 玉だけ、先後は決まってるから従えだぜ☆（＾～＾）
            Piece::King1 => self.new_piece_num(piece, PieceNum::King1),
            Piece::King2 => self.new_piece_num(piece, PieceNum::King2),
            _ => {
                let drop_type = piece.double_faced_piece().type_() as usize;
                // 玉以外の背番号は、先後に関わりなく SFENに書いてあった順で☆（＾～＾）
                let piece_num =
                    PieceNum::from_usize(self.double_faced_piece_type_index[drop_type]).unwrap();
                // カウントアップ☆（＾～＾）
                self.double_faced_piece_type_index[drop_type] += 1;
                self.new_piece_num(piece, piece_num)
            }
        }
    }

    /// 歩が置いてあるか確認
    pub fn exists_pawn_on_file(&self, friend: Phase, file: usize) -> bool {
        for rank in RANK_1..RANK_10 {
            let addr = UnifiedAddress::from_address_pos(
                friend,
                &AddressPos::Board(AbsoluteAddress2D::new(file, rank)),
            );
            if let Some(piece_val) = self.piece_at(&addr.to_address_pos()) {
                if piece_val.phase() == friend && piece_val.type_() == PieceType::Pawn {
                    return true;
                }
            }
        }
        false
    }
    /// ハッシュを作るときにも利用。盤上専用。
    pub fn piece_at(&self, addr: &AddressPos) -> Option<Piece> {
        match addr {
            AddressPos::Board(sq) => {
                if let Some(piece_num) = self.board[sq.serial_number() as usize] {
                    Some(self.get_piece(piece_num))
                } else {
                    None
                }
            }
            AddressPos::Hand(_drop) => panic!(Beam::trouble(&format!(
                "(Err.345) 駒台は非対応☆（＾～＾）！",
            ))),
        }
    }
    /// TODO Piece をカプセル化したい。外に出したくないぜ☆（＾～＾）
    /// 升で指定して駒を取得。
    /// 駒台には対応してない。 -> 何に使っている？
    pub fn piece_num_at(&self, addr: UnifiedAddress) -> Option<PieceNum> {
        match addr.to_address_pos() {
            AddressPos::Board(sq) => self.board[sq.serial_number() as usize],
            _ => panic!(Beam::trouble(&format!(
                "(Err.254) まだ駒台は実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 駒台には対応してない。 -> 何に使っている？
    pub fn piece_info_at(&self, addr: &AddressPos) -> Option<PieceInfo> {
        match addr {
            AddressPos::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    Some(PieceInfo::new(self.get_piece(piece_num_val), piece_num_val))
                } else {
                    None
                }
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.321) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    pub fn promotion_value_at(&self, table: &GameTable, addr: UnifiedAddress) -> isize {
        match addr.to_address_pos() {
            AddressPos::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    table
                        .get_double_faced_piece(piece_num_val)
                        .type_()
                        .promotion_value()
                } else {
                    // 打なら成りは無いぜ☆（＾～＾）
                    0
                }
            }
            AddressPos::Hand(_drop) => panic!(Beam::trouble(&format!(
                "(Err.254) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 指し手生成で使うぜ☆（＾～＾）有無を調べるぜ☆（＾～＾）
    pub fn last_hand_type(&self, drop: DoubleFacedPiece) -> Option<PieceType> {
        if let Some(piece_num) = self.phase_classification.last(drop) {
            Some(self.get_type(piece_num))
        } else {
            None
        }
    }
    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand(&self, drop: DoubleFacedPiece) -> Option<(PieceType, UnifiedAddress)> {
        if let Some(piece_num) = self.phase_classification.last(drop) {
            let piece = self.get_piece(piece_num);
            Some((
                piece.type_(),
                UnifiedAddress::from_address_pos(
                    self.get_phase(piece_num),
                    &AddressPos::Hand(piece.double_faced_piece()),
                ),
            ))
        } else {
            None
        }
    }
    pub fn count_hand(&self, drop: DoubleFacedPiece) -> usize {
        self.phase_classification.len(drop)
    }

    /// 表示に使うだけ☆（＾～＾）
    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_table<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, Option<&AbsoluteAddress2D>, Option<PieceInfo>),
    {
        for (i, addr) in self.address_list.iter().enumerate() {
            match addr.to_address_pos() {
                AddressPos::Board(sq) => {
                    // 盤上の駒☆（＾～＾）
                    let piece_info = self.piece_info_at(&addr.to_address_pos()).unwrap();
                    piece_get(i, Some(&sq), Some(piece_info));
                }
                AddressPos::Hand(_drop) => {
                    // TODO 持ち駒☆（＾～＾）
                    piece_get(i, None, None);
                }
            }
        }
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    /// TODO 自分、相手で分けて持っておけば２倍ぐらい短縮できないか☆（＾～＾）？
    /// TODO できれば、「自分の盤上の駒」「自分の持ち駒」「相手の盤上の駒」「相手の持ち駒」の４チャンネルで分けておけないか☆（＾～＾）？
    pub fn for_some_pieces_on_list40<F>(&self, friend: Phase, piece_get: &mut F)
    where
        F: FnMut(UnifiedAddress, PieceType),
    {
        for piece_num in Nine299792458::piece_numbers().iter() {
            // 盤上の駒だけを調べようぜ☆（＾～＾）
            let addr = self.address_list[*piece_num as usize];
            match addr.to_address_pos() {
                AddressPos::Board(_sq) => {
                    if self.get_phase(*piece_num) == friend {
                        piece_get(addr, self.get_type(*piece_num));
                    }
                }
                AddressPos::Hand(_drop) => {
                    // 持ち駒はここで調べるのは無駄な気がするよな☆（＾～＾）持ち駒に歩が１８個とか☆（＾～＾）
                }
            }
        }

        const FIRST_SECOND: [[DoubleFacedPiece; PHYSICAL_PIECE_TYPE_LEN - 1]; 2] = [
            [
                // King なし
                DoubleFacedPiece::Rook1,
                DoubleFacedPiece::Bishop1,
                DoubleFacedPiece::Gold1,
                DoubleFacedPiece::Silver1,
                DoubleFacedPiece::Knight1,
                DoubleFacedPiece::Lance1,
                DoubleFacedPiece::Pawn1,
            ],
            [
                // King なし
                DoubleFacedPiece::Rook2,
                DoubleFacedPiece::Bishop2,
                DoubleFacedPiece::Gold2,
                DoubleFacedPiece::Silver2,
                DoubleFacedPiece::Knight2,
                DoubleFacedPiece::Lance2,
                DoubleFacedPiece::Pawn2,
            ],
        ];
        for drop in &FIRST_SECOND[friend as usize] {
            if let Some(piece_type) = self.last_hand_type(*drop) {
                piece_get(
                    UnifiedAddress::from_address_pos(friend, &AddressPos::Hand(*drop)),
                    piece_type,
                );
            }
        }
    }
}

/// 以下の４つを、漏れなく被りなく　分類するぜ☆（＾～＾）
/// * 盤上の先手の駒
/// * 盤上の後手の駒
/// * 駒台の先手の駒
/// * 駒台の後手の駒
/// 駒台だぜ☆（＾～＾）これ１つで２人分あるんで☆（＾～＾）
#[derive(Clone)]
pub struct PhaseClassification {
    items: [PieceNum; 80],
    areas: [HandStackArea; 18],
    currents: [isize; 18],
}
impl Default for PhaseClassification {
    // ゴミ値で埋めるぜ☆（＾～＾）
    fn default() -> Self {
        PhaseClassification {
            items: [PieceNum::King1; 80],
            areas: [
                HandStackArea::new(40, 1),  // King1
                HandStackArea::new(60, 1),  // Rook1
                HandStackArea::new(58, 1),  // Bishop1
                HandStackArea::new(42, 1),  // Gold1
                HandStackArea::new(46, 1),  // Silver1
                HandStackArea::new(50, 1),  // Knight1
                HandStackArea::new(54, 1),  // Lance1
                HandStackArea::new(62, 1),  // Pawn1
                HandStackArea::new(41, -1), // King2
                HandStackArea::new(61, -1), // Rook2
                HandStackArea::new(59, -1), // Bishop2
                HandStackArea::new(45, -1), // Gold2
                HandStackArea::new(49, -1), // Silver2
                HandStackArea::new(53, -1), // Knight2
                HandStackArea::new(57, -1), // Lance2
                HandStackArea::new(79, -1), // Pawn2
                HandStackArea::new(0, 1),   // Board1
                HandStackArea::new(39, -1), // Board2
            ],
            currents: [
                40, // King1
                60, // Rook1
                58, // Bishop1
                42, // Gold1
                46, // Silver1
                50, // Knight1
                54, // Lance1
                62, // Pawn1
                41, // King2
                61, // Rook2
                59, // Bishop2
                45, // Gold2
                49, // Silver2
                53, // Knight2
                57, // Lance2
                79, // Pawn2
                0,  // Board1
                39, // Board2
            ],
        }
    }
}
impl PhaseClassification {
    /// 駒の先後を ひっくり返してから入れてください。
    pub fn push(&mut self, drop: DoubleFacedPiece, num: PieceNum) {
        let area = &self.areas[drop as usize];
        // 駒台に駒を置くぜ☆（＾～＾）
        self.items[self.currents[drop as usize] as usize] = num;
        // 位置を増減するぜ☆（＾～＾）
        self.currents[drop as usize] += area.direction;
    }
    /// ゴミ値は消さないぜ☆（＾～＾）
    pub fn pop(&mut self, drop: DoubleFacedPiece) -> PieceNum {
        let area = &self.areas[drop as usize];
        // 位置を増減するぜ☆（＾～＾）
        self.currents[drop as usize] -= area.direction;
        // 駒台の駒をはがすぜ☆（＾～＾）
        self.items[self.currents[drop as usize] as usize]
    }

    fn last(&self, drop: DoubleFacedPiece) -> Option<PieceNum> {
        let area = &self.areas[drop as usize];
        if area.direction == 1 {
            if area.start < self.currents[drop as usize] {
                Some(self.items[(self.currents[drop as usize] - 1) as usize])
            } else {
                None
            }
        } else {
            if self.currents[drop as usize] < area.start {
                Some(self.items[(self.currents[drop as usize] + 1) as usize])
            } else {
                None
            }
        }
    }

    fn len(&self, drop: DoubleFacedPiece) -> usize {
        let area = &self.areas[drop as usize];
        if area.direction == 1 {
            (self.currents[drop as usize] - area.start) as usize
        } else {
            (area.start - self.currents[drop as usize]) as usize
        }
    }

    /*
    fn to_debug(&self, table: &GameTable) -> String {
        let mut buffer = String::new();
        for i in 0..=self.count {
            buffer.push_str(&format!(
                "({}, {:?}) ",
                self.items[i].piece, self.items[i].num
            ));
        }
        buffer.trim_end().to_string()
    }
    */
}
#[derive(Clone)]
pub struct HandStackArea {
    // 開始地点。
    start: isize,
    // 向き。+1, -1。
    direction: isize,
}
impl HandStackArea {
    pub fn new(start: isize, direction: isize) -> Self {
        HandStackArea {
            start: start,
            direction: direction,
        }
    }
}
