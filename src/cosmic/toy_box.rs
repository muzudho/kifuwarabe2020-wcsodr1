//!
//! 駒 と 盤
//!
use crate::cosmic::fire::{Fire, FireAddress};
use crate::cosmic::recording::AddressPos1;
use crate::cosmic::recording::Movement;
use crate::cosmic::recording::Phase;
use crate::cosmic::smart::features::{
    DoubleFacedPiece, DoubleFacedPieceType, PieceType, PHYSICAL_PIECE_TYPE_LEN,
};
use crate::cosmic::smart::square::{AbsoluteAddress2D, BOARD_MEMORY_AREA, RANK_1, RANK_10};
use crate::law::generate_move::Area;
use crate::law::speed_of_light::Nine299792458;
use crate::spaceship::equipment::Beam;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::*;

pub const PIECE_LEN: usize = 28;

// 配列へのアクセスは遅い気がするので、定数で作っておいて……☆（＾～＾）
const UNIFIED_SQ_0_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 1, rank: 1 };
const UNIFIED_SQ_1_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 1, rank: 2 };
const UNIFIED_SQ_2_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 1, rank: 3 };
const UNIFIED_SQ_3_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 1, rank: 4 };
const UNIFIED_SQ_4_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 1, rank: 5 };
const UNIFIED_SQ_5_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 1, rank: 6 };
const UNIFIED_SQ_6_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 1, rank: 7 };
const UNIFIED_SQ_7_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 1, rank: 8 };
const UNIFIED_SQ_8_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 1, rank: 9 };
const UNIFIED_SQ_9_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 2, rank: 1 };
const UNIFIED_SQ_10_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 2, rank: 2 };
const UNIFIED_SQ_11_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 2, rank: 3 };
const UNIFIED_SQ_12_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 2, rank: 4 };
const UNIFIED_SQ_13_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 2, rank: 5 };
const UNIFIED_SQ_14_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 2, rank: 6 };
const UNIFIED_SQ_15_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 2, rank: 7 };
const UNIFIED_SQ_16_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 2, rank: 8 };
const UNIFIED_SQ_17_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 2, rank: 9 };
const UNIFIED_SQ_18_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 3, rank: 1 };
const UNIFIED_SQ_19_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 3, rank: 2 };
const UNIFIED_SQ_20_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 3, rank: 3 };
const UNIFIED_SQ_21_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 3, rank: 4 };
const UNIFIED_SQ_22_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 3, rank: 5 };
const UNIFIED_SQ_23_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 3, rank: 6 };
const UNIFIED_SQ_24_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 3, rank: 7 };
const UNIFIED_SQ_25_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 3, rank: 8 };
const UNIFIED_SQ_26_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 3, rank: 9 };
const UNIFIED_SQ_27_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 4, rank: 1 };
const UNIFIED_SQ_28_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 4, rank: 2 };
const UNIFIED_SQ_29_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 4, rank: 3 };
const UNIFIED_SQ_30_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 4, rank: 4 };
const UNIFIED_SQ_31_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 4, rank: 5 };
const UNIFIED_SQ_32_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 4, rank: 6 };
const UNIFIED_SQ_33_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 4, rank: 7 };
const UNIFIED_SQ_34_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 4, rank: 8 };
const UNIFIED_SQ_35_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 4, rank: 9 };
const UNIFIED_SQ_36_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 5, rank: 1 };
const UNIFIED_SQ_37_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 5, rank: 2 };
const UNIFIED_SQ_38_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 5, rank: 3 };
const UNIFIED_SQ_39_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 5, rank: 4 };
const UNIFIED_SQ_40_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 5, rank: 5 };
const UNIFIED_SQ_41_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 5, rank: 6 };
const UNIFIED_SQ_42_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 5, rank: 7 };
const UNIFIED_SQ_43_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 5, rank: 8 };
const UNIFIED_SQ_44_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 5, rank: 9 };
const UNIFIED_SQ_45_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 6, rank: 1 };
const UNIFIED_SQ_46_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 6, rank: 2 };
const UNIFIED_SQ_47_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 6, rank: 3 };
const UNIFIED_SQ_48_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 6, rank: 4 };
const UNIFIED_SQ_49_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 6, rank: 5 };
const UNIFIED_SQ_50_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 6, rank: 6 };
const UNIFIED_SQ_51_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 6, rank: 7 };
const UNIFIED_SQ_52_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 6, rank: 8 };
const UNIFIED_SQ_53_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 6, rank: 9 };
const UNIFIED_SQ_54_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 7, rank: 1 };
const UNIFIED_SQ_55_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 7, rank: 2 };
const UNIFIED_SQ_56_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 7, rank: 3 };
const UNIFIED_SQ_57_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 7, rank: 4 };
const UNIFIED_SQ_58_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 7, rank: 5 };
const UNIFIED_SQ_59_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 7, rank: 6 };
const UNIFIED_SQ_60_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 7, rank: 7 };
const UNIFIED_SQ_61_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 7, rank: 8 };
const UNIFIED_SQ_62_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 7, rank: 9 };
const UNIFIED_SQ_63_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 8, rank: 1 };
const UNIFIED_SQ_64_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 8, rank: 2 };
const UNIFIED_SQ_65_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 8, rank: 3 };
const UNIFIED_SQ_66_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 8, rank: 4 };
const UNIFIED_SQ_67_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 8, rank: 5 };
const UNIFIED_SQ_68_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 8, rank: 6 };
const UNIFIED_SQ_69_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 8, rank: 7 };
const UNIFIED_SQ_70_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 8, rank: 8 };
const UNIFIED_SQ_71_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 8, rank: 9 };
const UNIFIED_SQ_72_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 9, rank: 1 };
const UNIFIED_SQ_73_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 9, rank: 2 };
const UNIFIED_SQ_74_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 9, rank: 3 };
const UNIFIED_SQ_75_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 9, rank: 4 };
const UNIFIED_SQ_76_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 9, rank: 5 };
const UNIFIED_SQ_77_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 9, rank: 6 };
const UNIFIED_SQ_78_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 9, rank: 7 };
const UNIFIED_SQ_79_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 9, rank: 8 };
const UNIFIED_SQ_80_TO_ABSOLUTE_ADDRESS_2D: AbsoluteAddress2D =
    AbsoluteAddress2D { file: 9, rank: 9 };

const UNIFIED_ADDRESS_0_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 1 });
const UNIFIED_ADDRESS_1_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 2 });
const UNIFIED_ADDRESS_2_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 3 });
const UNIFIED_ADDRESS_3_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 4 });
const UNIFIED_ADDRESS_4_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 5 });
const UNIFIED_ADDRESS_5_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 6 });
const UNIFIED_ADDRESS_6_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 7 });
const UNIFIED_ADDRESS_7_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 8 });
const UNIFIED_ADDRESS_8_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 9 });
const UNIFIED_ADDRESS_9_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 1 });
const UNIFIED_ADDRESS_10_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 2 });
const UNIFIED_ADDRESS_11_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 3 });
const UNIFIED_ADDRESS_12_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 4 });
const UNIFIED_ADDRESS_13_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 5 });
const UNIFIED_ADDRESS_14_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 6 });
const UNIFIED_ADDRESS_15_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 7 });
const UNIFIED_ADDRESS_16_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 8 });
const UNIFIED_ADDRESS_17_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 9 });
const UNIFIED_ADDRESS_18_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 1 });
const UNIFIED_ADDRESS_19_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 2 });
const UNIFIED_ADDRESS_20_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 3 });
const UNIFIED_ADDRESS_21_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 4 });
const UNIFIED_ADDRESS_22_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 5 });
const UNIFIED_ADDRESS_23_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 6 });
const UNIFIED_ADDRESS_24_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 7 });
const UNIFIED_ADDRESS_25_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 8 });
const UNIFIED_ADDRESS_26_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 9 });
const UNIFIED_ADDRESS_27_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 1 });
const UNIFIED_ADDRESS_28_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 2 });
const UNIFIED_ADDRESS_29_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 3 });
const UNIFIED_ADDRESS_30_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 4 });
const UNIFIED_ADDRESS_31_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 5 });
const UNIFIED_ADDRESS_32_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 6 });
const UNIFIED_ADDRESS_33_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 7 });
const UNIFIED_ADDRESS_34_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 8 });
const UNIFIED_ADDRESS_35_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 9 });
const UNIFIED_ADDRESS_36_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 1 });
const UNIFIED_ADDRESS_37_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 2 });
const UNIFIED_ADDRESS_38_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 3 });
const UNIFIED_ADDRESS_39_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 4 });
const UNIFIED_ADDRESS_40_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 5 });
const UNIFIED_ADDRESS_41_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 6 });
const UNIFIED_ADDRESS_42_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 7 });
const UNIFIED_ADDRESS_43_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 8 });
const UNIFIED_ADDRESS_44_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 9 });
const UNIFIED_ADDRESS_45_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 1 });
const UNIFIED_ADDRESS_46_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 2 });
const UNIFIED_ADDRESS_47_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 3 });
const UNIFIED_ADDRESS_48_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 4 });
const UNIFIED_ADDRESS_49_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 5 });
const UNIFIED_ADDRESS_50_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 6 });
const UNIFIED_ADDRESS_51_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 7 });
const UNIFIED_ADDRESS_52_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 8 });
const UNIFIED_ADDRESS_53_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 9 });
const UNIFIED_ADDRESS_54_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 1 });
const UNIFIED_ADDRESS_55_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 2 });
const UNIFIED_ADDRESS_56_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 3 });
const UNIFIED_ADDRESS_57_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 4 });
const UNIFIED_ADDRESS_58_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 5 });
const UNIFIED_ADDRESS_59_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 6 });
const UNIFIED_ADDRESS_60_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 7 });
const UNIFIED_ADDRESS_61_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 8 });
const UNIFIED_ADDRESS_62_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 9 });
const UNIFIED_ADDRESS_63_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 1 });
const UNIFIED_ADDRESS_64_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 2 });
const UNIFIED_ADDRESS_65_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 3 });
const UNIFIED_ADDRESS_66_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 4 });
const UNIFIED_ADDRESS_67_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 5 });
const UNIFIED_ADDRESS_68_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 6 });
const UNIFIED_ADDRESS_69_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 7 });
const UNIFIED_ADDRESS_70_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 8 });
const UNIFIED_ADDRESS_71_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 9 });
const UNIFIED_ADDRESS_72_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 1 });
const UNIFIED_ADDRESS_73_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 2 });
const UNIFIED_ADDRESS_74_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 3 });
const UNIFIED_ADDRESS_75_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 4 });
const UNIFIED_ADDRESS_76_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 5 });
const UNIFIED_ADDRESS_77_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 6 });
const UNIFIED_ADDRESS_78_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 7 });
const UNIFIED_ADDRESS_79_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 8 });
const UNIFIED_ADDRESS_80_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 9 });
const UNIFIED_ADDRESS_81_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 1 });
const UNIFIED_ADDRESS_82_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 2 });
const UNIFIED_ADDRESS_83_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 3 });
const UNIFIED_ADDRESS_84_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 4 });
const UNIFIED_ADDRESS_85_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 5 });
const UNIFIED_ADDRESS_86_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 6 });
const UNIFIED_ADDRESS_87_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 7 });
const UNIFIED_ADDRESS_88_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 8 });
const UNIFIED_ADDRESS_89_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 1, rank: 9 });
const UNIFIED_ADDRESS_90_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 1 });
const UNIFIED_ADDRESS_91_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 2 });
const UNIFIED_ADDRESS_92_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 3 });
const UNIFIED_ADDRESS_93_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 4 });
const UNIFIED_ADDRESS_94_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 5 });
const UNIFIED_ADDRESS_95_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 6 });
const UNIFIED_ADDRESS_96_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 7 });
const UNIFIED_ADDRESS_97_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 8 });
const UNIFIED_ADDRESS_98_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 2, rank: 9 });
const UNIFIED_ADDRESS_99_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 1 });
const UNIFIED_ADDRESS_100_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 2 });
const UNIFIED_ADDRESS_101_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 3 });
const UNIFIED_ADDRESS_102_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 4 });
const UNIFIED_ADDRESS_103_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 5 });
const UNIFIED_ADDRESS_104_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 6 });
const UNIFIED_ADDRESS_105_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 7 });
const UNIFIED_ADDRESS_106_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 8 });
const UNIFIED_ADDRESS_107_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 3, rank: 9 });
const UNIFIED_ADDRESS_108_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 1 });
const UNIFIED_ADDRESS_109_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 2 });
const UNIFIED_ADDRESS_110_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 3 });
const UNIFIED_ADDRESS_111_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 4 });
const UNIFIED_ADDRESS_112_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 5 });
const UNIFIED_ADDRESS_113_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 6 });
const UNIFIED_ADDRESS_114_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 7 });
const UNIFIED_ADDRESS_115_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 8 });
const UNIFIED_ADDRESS_116_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 4, rank: 9 });
const UNIFIED_ADDRESS_117_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 1 });
const UNIFIED_ADDRESS_118_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 2 });
const UNIFIED_ADDRESS_119_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 3 });
const UNIFIED_ADDRESS_120_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 4 });
const UNIFIED_ADDRESS_121_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 5 });
const UNIFIED_ADDRESS_122_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 6 });
const UNIFIED_ADDRESS_123_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 7 });
const UNIFIED_ADDRESS_124_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 8 });
const UNIFIED_ADDRESS_125_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 5, rank: 9 });
const UNIFIED_ADDRESS_126_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 1 });
const UNIFIED_ADDRESS_127_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 2 });
const UNIFIED_ADDRESS_128_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 3 });
const UNIFIED_ADDRESS_129_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 4 });
const UNIFIED_ADDRESS_130_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 5 });
const UNIFIED_ADDRESS_131_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 6 });
const UNIFIED_ADDRESS_132_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 7 });
const UNIFIED_ADDRESS_133_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 8 });
const UNIFIED_ADDRESS_134_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 6, rank: 9 });
const UNIFIED_ADDRESS_135_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 1 });
const UNIFIED_ADDRESS_136_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 2 });
const UNIFIED_ADDRESS_137_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 3 });
const UNIFIED_ADDRESS_138_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 4 });
const UNIFIED_ADDRESS_139_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 5 });
const UNIFIED_ADDRESS_140_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 6 });
const UNIFIED_ADDRESS_141_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 7 });
const UNIFIED_ADDRESS_142_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 8 });
const UNIFIED_ADDRESS_143_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 7, rank: 9 });
const UNIFIED_ADDRESS_144_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 1 });
const UNIFIED_ADDRESS_145_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 2 });
const UNIFIED_ADDRESS_146_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 3 });
const UNIFIED_ADDRESS_147_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 4 });
const UNIFIED_ADDRESS_148_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 5 });
const UNIFIED_ADDRESS_149_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 6 });
const UNIFIED_ADDRESS_150_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 7 });
const UNIFIED_ADDRESS_151_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 8 });
const UNIFIED_ADDRESS_152_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 8, rank: 9 });
const UNIFIED_ADDRESS_153_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 1 });
const UNIFIED_ADDRESS_154_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 2 });
const UNIFIED_ADDRESS_155_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 3 });
const UNIFIED_ADDRESS_156_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 4 });
const UNIFIED_ADDRESS_157_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 5 });
const UNIFIED_ADDRESS_158_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 6 });
const UNIFIED_ADDRESS_159_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 7 });
const UNIFIED_ADDRESS_160_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 8 });
const UNIFIED_ADDRESS_161_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Board(AbsoluteAddress2D { file: 9, rank: 9 });
const UNIFIED_ADDRESS_162_TO_ADDRESS_POS1: AddressPos1 = AddressPos1::Hand(DoubleFacedPiece::King1);
const UNIFIED_ADDRESS_163_TO_ADDRESS_POS1: AddressPos1 = AddressPos1::Hand(DoubleFacedPiece::Rook1);
const UNIFIED_ADDRESS_164_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand(DoubleFacedPiece::Bishop1);
const UNIFIED_ADDRESS_165_TO_ADDRESS_POS1: AddressPos1 = AddressPos1::Hand(DoubleFacedPiece::Gold1);
const UNIFIED_ADDRESS_166_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand(DoubleFacedPiece::Silver1);
const UNIFIED_ADDRESS_167_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand(DoubleFacedPiece::Knight1);
const UNIFIED_ADDRESS_168_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand(DoubleFacedPiece::Lance1);
const UNIFIED_ADDRESS_169_TO_ADDRESS_POS1: AddressPos1 = AddressPos1::Hand(DoubleFacedPiece::Pawn1);
const UNIFIED_ADDRESS_170_TO_ADDRESS_POS1: AddressPos1 = AddressPos1::Hand(DoubleFacedPiece::King2);
const UNIFIED_ADDRESS_171_TO_ADDRESS_POS1: AddressPos1 = AddressPos1::Hand(DoubleFacedPiece::Rook2);
const UNIFIED_ADDRESS_172_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand(DoubleFacedPiece::Bishop2);
const UNIFIED_ADDRESS_173_TO_ADDRESS_POS1: AddressPos1 = AddressPos1::Hand(DoubleFacedPiece::Gold2);
const UNIFIED_ADDRESS_174_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand(DoubleFacedPiece::Silver2);
const UNIFIED_ADDRESS_175_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand(DoubleFacedPiece::Knight2);
const UNIFIED_ADDRESS_176_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand(DoubleFacedPiece::Lance2);
const UNIFIED_ADDRESS_177_TO_ADDRESS_POS1: AddressPos1 = AddressPos1::Hand(DoubleFacedPiece::Pawn2);

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

    pub fn from_fire(fire: &Fire) -> Self {
        match fire.address {
            FireAddress::Board(sq) => UnifiedAddress::from_absolute_address(fire.friend, &sq),
            FireAddress::Hand(drop_type) => UnifiedAddress::from_double_faced_piece(
                DoubleFacedPiece::from_phase_and_type(fire.friend, drop_type),
            ),
        }
    }

    pub fn to_fire(self) -> Fire {
        use crate::cosmic::toy_box::UnifiedAddress::*;
        match self {
            Sq11_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 1, rank: 1 }),
            Sq12_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 1, rank: 2 }),
            Sq13_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 1, rank: 3 }),
            Sq14_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 1, rank: 4 }),
            Sq15_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 1, rank: 5 }),
            Sq16_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 1, rank: 6 }),
            Sq17_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 1, rank: 7 }),
            Sq18_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 1, rank: 8 }),
            Sq19_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 1, rank: 9 }),
            Sq21_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 2, rank: 1 }),
            Sq22_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 2, rank: 2 }),
            Sq23_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 2, rank: 3 }),
            Sq24_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 2, rank: 4 }),
            Sq25_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 2, rank: 5 }),
            Sq26_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 2, rank: 6 }),
            Sq27_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 2, rank: 7 }),
            Sq28_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 2, rank: 8 }),
            Sq29_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 2, rank: 9 }),
            Sq31_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 3, rank: 1 }),
            Sq32_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 3, rank: 2 }),
            Sq33_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 3, rank: 3 }),
            Sq34_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 3, rank: 4 }),
            Sq35_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 3, rank: 5 }),
            Sq36_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 3, rank: 6 }),
            Sq37_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 3, rank: 7 }),
            Sq38_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 3, rank: 8 }),
            Sq39_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 3, rank: 9 }),
            Sq41_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 4, rank: 1 }),
            Sq42_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 4, rank: 2 }),
            Sq43_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 4, rank: 3 }),
            Sq44_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 4, rank: 4 }),
            Sq45_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 4, rank: 5 }),
            Sq46_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 4, rank: 6 }),
            Sq47_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 4, rank: 7 }),
            Sq48_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 4, rank: 8 }),
            Sq49_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 4, rank: 9 }),
            Sq51_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 5, rank: 1 }),
            Sq52_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 5, rank: 2 }),
            Sq53_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 5, rank: 3 }),
            Sq54_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 5, rank: 4 }),
            Sq55_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 5, rank: 5 }),
            Sq56_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 5, rank: 6 }),
            Sq57_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 5, rank: 7 }),
            Sq58_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 5, rank: 8 }),
            Sq59_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 5, rank: 9 }),
            Sq61_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 6, rank: 1 }),
            Sq62_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 6, rank: 2 }),
            Sq63_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 6, rank: 3 }),
            Sq64_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 6, rank: 4 }),
            Sq65_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 6, rank: 5 }),
            Sq66_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 6, rank: 6 }),
            Sq67_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 6, rank: 7 }),
            Sq68_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 6, rank: 8 }),
            Sq69_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 6, rank: 9 }),
            Sq71_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 7, rank: 1 }),
            Sq72_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 7, rank: 2 }),
            Sq73_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 7, rank: 3 }),
            Sq74_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 7, rank: 4 }),
            Sq75_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 7, rank: 5 }),
            Sq76_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 7, rank: 6 }),
            Sq77_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 7, rank: 7 }),
            Sq78_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 7, rank: 8 }),
            Sq79_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 7, rank: 9 }),
            Sq81_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 8, rank: 1 }),
            Sq82_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 8, rank: 2 }),
            Sq83_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 8, rank: 3 }),
            Sq84_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 8, rank: 4 }),
            Sq85_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 8, rank: 5 }),
            Sq86_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 8, rank: 6 }),
            Sq87_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 8, rank: 7 }),
            Sq88_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 8, rank: 8 }),
            Sq89_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 8, rank: 9 }),
            Sq91_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 9, rank: 1 }),
            Sq92_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 9, rank: 2 }),
            Sq93_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 9, rank: 3 }),
            Sq94_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 9, rank: 4 }),
            Sq95_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 9, rank: 5 }),
            Sq96_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 9, rank: 6 }),
            Sq97_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 9, rank: 7 }),
            Sq98_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 9, rank: 8 }),
            Sq99_1 => Fire::new_board(Phase::First, AbsoluteAddress2D { file: 9, rank: 9 }),
            Sq11_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 1, rank: 1 }),
            Sq12_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 1, rank: 2 }),
            Sq13_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 1, rank: 3 }),
            Sq14_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 1, rank: 4 }),
            Sq15_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 1, rank: 5 }),
            Sq16_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 1, rank: 6 }),
            Sq17_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 1, rank: 7 }),
            Sq18_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 1, rank: 8 }),
            Sq19_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 1, rank: 9 }),
            Sq21_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 2, rank: 1 }),
            Sq22_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 2, rank: 2 }),
            Sq23_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 2, rank: 3 }),
            Sq24_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 2, rank: 4 }),
            Sq25_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 2, rank: 5 }),
            Sq26_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 2, rank: 6 }),
            Sq27_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 2, rank: 7 }),
            Sq28_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 2, rank: 8 }),
            Sq29_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 2, rank: 9 }),
            Sq31_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 3, rank: 1 }),
            Sq32_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 3, rank: 2 }),
            Sq33_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 3, rank: 3 }),
            Sq34_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 3, rank: 4 }),
            Sq35_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 3, rank: 5 }),
            Sq36_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 3, rank: 6 }),
            Sq37_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 3, rank: 7 }),
            Sq38_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 3, rank: 8 }),
            Sq39_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 3, rank: 9 }),
            Sq41_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 4, rank: 1 }),
            Sq42_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 4, rank: 2 }),
            Sq43_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 4, rank: 3 }),
            Sq44_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 4, rank: 4 }),
            Sq45_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 4, rank: 5 }),
            Sq46_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 4, rank: 6 }),
            Sq47_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 4, rank: 7 }),
            Sq48_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 4, rank: 8 }),
            Sq49_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 4, rank: 9 }),
            Sq51_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 5, rank: 1 }),
            Sq52_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 5, rank: 2 }),
            Sq53_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 5, rank: 3 }),
            Sq54_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 5, rank: 4 }),
            Sq55_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 5, rank: 5 }),
            Sq56_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 5, rank: 6 }),
            Sq57_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 5, rank: 7 }),
            Sq58_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 5, rank: 8 }),
            Sq59_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 5, rank: 9 }),
            Sq61_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 6, rank: 1 }),
            Sq62_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 6, rank: 2 }),
            Sq63_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 6, rank: 3 }),
            Sq64_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 6, rank: 4 }),
            Sq65_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 6, rank: 5 }),
            Sq66_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 6, rank: 6 }),
            Sq67_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 6, rank: 7 }),
            Sq68_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 6, rank: 8 }),
            Sq69_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 6, rank: 9 }),
            Sq71_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 7, rank: 1 }),
            Sq72_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 7, rank: 2 }),
            Sq73_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 7, rank: 3 }),
            Sq74_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 7, rank: 4 }),
            Sq75_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 7, rank: 5 }),
            Sq76_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 7, rank: 6 }),
            Sq77_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 7, rank: 7 }),
            Sq78_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 7, rank: 8 }),
            Sq79_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 7, rank: 9 }),
            Sq81_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 8, rank: 1 }),
            Sq82_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 8, rank: 2 }),
            Sq83_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 8, rank: 3 }),
            Sq84_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 8, rank: 4 }),
            Sq85_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 8, rank: 5 }),
            Sq86_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 8, rank: 6 }),
            Sq87_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 8, rank: 7 }),
            Sq88_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 8, rank: 8 }),
            Sq89_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 8, rank: 9 }),
            Sq91_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 9, rank: 1 }),
            Sq92_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 9, rank: 2 }),
            Sq93_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 9, rank: 3 }),
            Sq94_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 9, rank: 4 }),
            Sq95_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 9, rank: 5 }),
            Sq96_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 9, rank: 6 }),
            Sq97_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 9, rank: 7 }),
            Sq98_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 9, rank: 8 }),
            Sq99_2 => Fire::new_board(Phase::Second, AbsoluteAddress2D { file: 9, rank: 9 }),
            King1 => Fire::new_hand(Phase::First, DoubleFacedPieceType::King),
            Rook1 => Fire::new_hand(Phase::First, DoubleFacedPieceType::Rook),
            Bishop1 => Fire::new_hand(Phase::First, DoubleFacedPieceType::Bishop),
            Gold1 => Fire::new_hand(Phase::First, DoubleFacedPieceType::Gold),
            Silver1 => Fire::new_hand(Phase::First, DoubleFacedPieceType::Silver),
            Knight1 => Fire::new_hand(Phase::First, DoubleFacedPieceType::Knight),
            Lance1 => Fire::new_hand(Phase::First, DoubleFacedPieceType::Lance),
            Pawn1 => Fire::new_hand(Phase::First, DoubleFacedPieceType::Pawn),
            King2 => Fire::new_hand(Phase::Second, DoubleFacedPieceType::King),
            Rook2 => Fire::new_hand(Phase::Second, DoubleFacedPieceType::Rook),
            Bishop2 => Fire::new_hand(Phase::Second, DoubleFacedPieceType::Bishop),
            Gold2 => Fire::new_hand(Phase::Second, DoubleFacedPieceType::Gold),
            Silver2 => Fire::new_hand(Phase::Second, DoubleFacedPieceType::Silver),
            Knight2 => Fire::new_hand(Phase::Second, DoubleFacedPieceType::Knight),
            Lance2 => Fire::new_hand(Phase::Second, DoubleFacedPieceType::Lance),
            Pawn2 => Fire::new_hand(Phase::Second, DoubleFacedPieceType::Pawn),
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
        match drop {
            DoubleFacedPiece::King1 => UnifiedAddress::King1,
            DoubleFacedPiece::Rook1 => UnifiedAddress::Rook1,
            DoubleFacedPiece::Bishop1 => UnifiedAddress::Bishop1,
            DoubleFacedPiece::Gold1 => UnifiedAddress::Gold1,
            DoubleFacedPiece::Silver1 => UnifiedAddress::Silver1,
            DoubleFacedPiece::Knight1 => UnifiedAddress::Knight1,
            DoubleFacedPiece::Lance1 => UnifiedAddress::Lance1,
            DoubleFacedPiece::Pawn1 => UnifiedAddress::Pawn1,
            DoubleFacedPiece::King2 => UnifiedAddress::King2,
            DoubleFacedPiece::Rook2 => UnifiedAddress::Rook2,
            DoubleFacedPiece::Bishop2 => UnifiedAddress::Bishop2,
            DoubleFacedPiece::Gold2 => UnifiedAddress::Gold2,
            DoubleFacedPiece::Silver2 => UnifiedAddress::Silver2,
            DoubleFacedPiece::Knight2 => UnifiedAddress::Knight2,
            DoubleFacedPiece::Lance2 => UnifiedAddress::Lance2,
            DoubleFacedPiece::Pawn2 => UnifiedAddress::Pawn2,
        }
    }

    pub fn to_double_faced_piece(self) -> DoubleFacedPiece {
        use crate::cosmic::toy_box::UnifiedAddress::*;
        match self {
            Sq11_1 | Sq12_1 | Sq13_1 | Sq14_1 | Sq15_1 | Sq16_1 | Sq17_1 | Sq18_1 | Sq19_1
            | Sq21_1 | Sq22_1 | Sq23_1 | Sq24_1 | Sq25_1 | Sq26_1 | Sq27_1 | Sq28_1 | Sq29_1
            | Sq31_1 | Sq32_1 | Sq33_1 | Sq34_1 | Sq35_1 | Sq36_1 | Sq37_1 | Sq38_1 | Sq39_1
            | Sq41_1 | Sq42_1 | Sq43_1 | Sq44_1 | Sq45_1 | Sq46_1 | Sq47_1 | Sq48_1 | Sq49_1
            | Sq51_1 | Sq52_1 | Sq53_1 | Sq54_1 | Sq55_1 | Sq56_1 | Sq57_1 | Sq58_1 | Sq59_1
            | Sq61_1 | Sq62_1 | Sq63_1 | Sq64_1 | Sq65_1 | Sq66_1 | Sq67_1 | Sq68_1 | Sq69_1
            | Sq71_1 | Sq72_1 | Sq73_1 | Sq74_1 | Sq75_1 | Sq76_1 | Sq77_1 | Sq78_1 | Sq79_1
            | Sq81_1 | Sq82_1 | Sq83_1 | Sq84_1 | Sq85_1 | Sq86_1 | Sq87_1 | Sq88_1 | Sq89_1
            | Sq91_1 | Sq92_1 | Sq93_1 | Sq94_1 | Sq95_1 | Sq96_1 | Sq97_1 | Sq98_1 | Sq99_1
            | Sq11_2 | Sq12_2 | Sq13_2 | Sq14_2 | Sq15_2 | Sq16_2 | Sq17_2 | Sq18_2 | Sq19_2
            | Sq21_2 | Sq22_2 | Sq23_2 | Sq24_2 | Sq25_2 | Sq26_2 | Sq27_2 | Sq28_2 | Sq29_2
            | Sq31_2 | Sq32_2 | Sq33_2 | Sq34_2 | Sq35_2 | Sq36_2 | Sq37_2 | Sq38_2 | Sq39_2
            | Sq41_2 | Sq42_2 | Sq43_2 | Sq44_2 | Sq45_2 | Sq46_2 | Sq47_2 | Sq48_2 | Sq49_2
            | Sq51_2 | Sq52_2 | Sq53_2 | Sq54_2 | Sq55_2 | Sq56_2 | Sq57_2 | Sq58_2 | Sq59_2
            | Sq61_2 | Sq62_2 | Sq63_2 | Sq64_2 | Sq65_2 | Sq66_2 | Sq67_2 | Sq68_2 | Sq69_2
            | Sq71_2 | Sq72_2 | Sq73_2 | Sq74_2 | Sq75_2 | Sq76_2 | Sq77_2 | Sq78_2 | Sq79_2
            | Sq81_2 | Sq82_2 | Sq83_2 | Sq84_2 | Sq85_2 | Sq86_2 | Sq87_2 | Sq88_2 | Sq89_2
            | Sq91_2 | Sq92_2 | Sq93_2 | Sq94_2 | Sq95_2 | Sq96_2 | Sq97_2 | Sq98_2 | Sq99_2 => {
                panic!(Beam::trouble("(Err.176) 番地を変換できね☆（＾～＾）"))
            }
            King1 => DoubleFacedPiece::King1,
            Rook1 => DoubleFacedPiece::Rook1,
            Bishop1 => DoubleFacedPiece::Bishop1,
            Gold1 => DoubleFacedPiece::Gold1,
            Silver1 => DoubleFacedPiece::Silver1,
            Knight1 => DoubleFacedPiece::Knight1,
            Lance1 => DoubleFacedPiece::Lance1,
            Pawn1 => DoubleFacedPiece::Pawn1,
            King2 => DoubleFacedPiece::King2,
            Rook2 => DoubleFacedPiece::Rook2,
            Bishop2 => DoubleFacedPiece::Bishop2,
            Gold2 => DoubleFacedPiece::Gold2,
            Silver2 => DoubleFacedPiece::Silver2,
            Knight2 => DoubleFacedPiece::Knight2,
            Lance2 => DoubleFacedPiece::Lance2,
            Pawn2 => DoubleFacedPiece::Pawn2,
        }
    }

    pub fn from_address_pos1(friend: Phase, addr: AddressPos1) -> Self {
        match addr {
            AddressPos1::Board(sq) => UnifiedAddress::from_absolute_address(friend, &sq),
            AddressPos1::Hand(drop) => UnifiedAddress::from_double_faced_piece(drop),
        }
    }

    /// 二歩チェックで使うぜ☆（＾～＾）
    pub fn to_file(self) -> usize {
        use crate::cosmic::toy_box::UnifiedAddress::*;
        match self {
            Sq11_1 => 1,
            Sq12_1 => 1,
            Sq13_1 => 1,
            Sq14_1 => 1,
            Sq15_1 => 1,
            Sq16_1 => 1,
            Sq17_1 => 1,
            Sq18_1 => 1,
            Sq19_1 => 1,
            Sq21_1 => 2,
            Sq22_1 => 2,
            Sq23_1 => 2,
            Sq24_1 => 2,
            Sq25_1 => 2,
            Sq26_1 => 2,
            Sq27_1 => 2,
            Sq28_1 => 2,
            Sq29_1 => 2,
            Sq31_1 => 3,
            Sq32_1 => 3,
            Sq33_1 => 3,
            Sq34_1 => 3,
            Sq35_1 => 3,
            Sq36_1 => 3,
            Sq37_1 => 3,
            Sq38_1 => 3,
            Sq39_1 => 3,
            Sq41_1 => 4,
            Sq42_1 => 4,
            Sq43_1 => 4,
            Sq44_1 => 4,
            Sq45_1 => 4,
            Sq46_1 => 4,
            Sq47_1 => 4,
            Sq48_1 => 4,
            Sq49_1 => 4,
            Sq51_1 => 5,
            Sq52_1 => 5,
            Sq53_1 => 5,
            Sq54_1 => 5,
            Sq55_1 => 5,
            Sq56_1 => 5,
            Sq57_1 => 5,
            Sq58_1 => 5,
            Sq59_1 => 5,
            Sq61_1 => 6,
            Sq62_1 => 6,
            Sq63_1 => 6,
            Sq64_1 => 6,
            Sq65_1 => 6,
            Sq66_1 => 6,
            Sq67_1 => 6,
            Sq68_1 => 6,
            Sq69_1 => 6,
            Sq71_1 => 7,
            Sq72_1 => 7,
            Sq73_1 => 7,
            Sq74_1 => 7,
            Sq75_1 => 7,
            Sq76_1 => 7,
            Sq77_1 => 7,
            Sq78_1 => 7,
            Sq79_1 => 7,
            Sq81_1 => 8,
            Sq82_1 => 8,
            Sq83_1 => 8,
            Sq84_1 => 8,
            Sq85_1 => 8,
            Sq86_1 => 8,
            Sq87_1 => 8,
            Sq88_1 => 8,
            Sq89_1 => 8,
            Sq91_1 => 9,
            Sq92_1 => 9,
            Sq93_1 => 9,
            Sq94_1 => 9,
            Sq95_1 => 9,
            Sq96_1 => 9,
            Sq97_1 => 9,
            Sq98_1 => 9,
            Sq99_1 => 9,
            Sq11_2 => 1,
            Sq12_2 => 1,
            Sq13_2 => 1,
            Sq14_2 => 1,
            Sq15_2 => 1,
            Sq16_2 => 1,
            Sq17_2 => 1,
            Sq18_2 => 1,
            Sq19_2 => 1,
            Sq21_2 => 2,
            Sq22_2 => 2,
            Sq23_2 => 2,
            Sq24_2 => 2,
            Sq25_2 => 2,
            Sq26_2 => 2,
            Sq27_2 => 2,
            Sq28_2 => 2,
            Sq29_2 => 2,
            Sq31_2 => 3,
            Sq32_2 => 3,
            Sq33_2 => 3,
            Sq34_2 => 3,
            Sq35_2 => 3,
            Sq36_2 => 3,
            Sq37_2 => 3,
            Sq38_2 => 3,
            Sq39_2 => 3,
            Sq41_2 => 4,
            Sq42_2 => 4,
            Sq43_2 => 4,
            Sq44_2 => 4,
            Sq45_2 => 4,
            Sq46_2 => 4,
            Sq47_2 => 4,
            Sq48_2 => 4,
            Sq49_2 => 4,
            Sq51_2 => 5,
            Sq52_2 => 5,
            Sq53_2 => 5,
            Sq54_2 => 5,
            Sq55_2 => 5,
            Sq56_2 => 5,
            Sq57_2 => 5,
            Sq58_2 => 5,
            Sq59_2 => 5,
            Sq61_2 => 6,
            Sq62_2 => 6,
            Sq63_2 => 6,
            Sq64_2 => 6,
            Sq65_2 => 6,
            Sq66_2 => 6,
            Sq67_2 => 6,
            Sq68_2 => 6,
            Sq69_2 => 6,
            Sq71_2 => 7,
            Sq72_2 => 7,
            Sq73_2 => 7,
            Sq74_2 => 7,
            Sq75_2 => 7,
            Sq76_2 => 7,
            Sq77_2 => 7,
            Sq78_2 => 7,
            Sq79_2 => 7,
            Sq81_2 => 8,
            Sq82_2 => 8,
            Sq83_2 => 8,
            Sq84_2 => 8,
            Sq85_2 => 8,
            Sq86_2 => 8,
            Sq87_2 => 8,
            Sq88_2 => 8,
            Sq89_2 => 8,
            Sq91_2 => 9,
            Sq92_2 => 9,
            Sq93_2 => 9,
            Sq94_2 => 9,
            Sq95_2 => 9,
            Sq96_2 => 9,
            Sq97_2 => 9,
            Sq98_2 => 9,
            Sq99_2 => 9,
            King1 | Rook1 | Bishop1 | Gold1 | Silver1 | Knight1 | Lance1 | Pawn1 | King2
            | Rook2 | Bishop2 | Gold2 | Silver2 | Knight2 | Lance2 | Pawn2 => panic!(
                Beam::trouble(&format!("(Err.526) 盤上ではなかったぜ☆（＾～＾）！",))
            ),
        }
    }

    /// 打の範囲チェックに使うぜ☆（＾～＾）
    pub fn to_rank(self) -> usize {
        use crate::cosmic::toy_box::UnifiedAddress::*;
        match self {
            Sq11_1 => 1,
            Sq12_1 => 2,
            Sq13_1 => 3,
            Sq14_1 => 4,
            Sq15_1 => 5,
            Sq16_1 => 6,
            Sq17_1 => 7,
            Sq18_1 => 8,
            Sq19_1 => 9,
            Sq21_1 => 1,
            Sq22_1 => 2,
            Sq23_1 => 3,
            Sq24_1 => 4,
            Sq25_1 => 5,
            Sq26_1 => 6,
            Sq27_1 => 7,
            Sq28_1 => 8,
            Sq29_1 => 9,
            Sq31_1 => 1,
            Sq32_1 => 2,
            Sq33_1 => 3,
            Sq34_1 => 4,
            Sq35_1 => 5,
            Sq36_1 => 6,
            Sq37_1 => 7,
            Sq38_1 => 8,
            Sq39_1 => 9,
            Sq41_1 => 1,
            Sq42_1 => 2,
            Sq43_1 => 3,
            Sq44_1 => 4,
            Sq45_1 => 5,
            Sq46_1 => 6,
            Sq47_1 => 7,
            Sq48_1 => 8,
            Sq49_1 => 9,
            Sq51_1 => 1,
            Sq52_1 => 2,
            Sq53_1 => 3,
            Sq54_1 => 4,
            Sq55_1 => 5,
            Sq56_1 => 6,
            Sq57_1 => 7,
            Sq58_1 => 8,
            Sq59_1 => 9,
            Sq61_1 => 1,
            Sq62_1 => 2,
            Sq63_1 => 3,
            Sq64_1 => 4,
            Sq65_1 => 5,
            Sq66_1 => 6,
            Sq67_1 => 7,
            Sq68_1 => 8,
            Sq69_1 => 9,
            Sq71_1 => 1,
            Sq72_1 => 2,
            Sq73_1 => 3,
            Sq74_1 => 4,
            Sq75_1 => 5,
            Sq76_1 => 6,
            Sq77_1 => 7,
            Sq78_1 => 8,
            Sq79_1 => 9,
            Sq81_1 => 1,
            Sq82_1 => 2,
            Sq83_1 => 3,
            Sq84_1 => 4,
            Sq85_1 => 5,
            Sq86_1 => 6,
            Sq87_1 => 7,
            Sq88_1 => 8,
            Sq89_1 => 9,
            Sq91_1 => 1,
            Sq92_1 => 2,
            Sq93_1 => 3,
            Sq94_1 => 4,
            Sq95_1 => 5,
            Sq96_1 => 6,
            Sq97_1 => 7,
            Sq98_1 => 8,
            Sq99_1 => 9,
            Sq11_2 => 1,
            Sq12_2 => 2,
            Sq13_2 => 3,
            Sq14_2 => 4,
            Sq15_2 => 5,
            Sq16_2 => 6,
            Sq17_2 => 7,
            Sq18_2 => 8,
            Sq19_2 => 9,
            Sq21_2 => 1,
            Sq22_2 => 2,
            Sq23_2 => 3,
            Sq24_2 => 4,
            Sq25_2 => 5,
            Sq26_2 => 6,
            Sq27_2 => 7,
            Sq28_2 => 8,
            Sq29_2 => 9,
            Sq31_2 => 1,
            Sq32_2 => 2,
            Sq33_2 => 3,
            Sq34_2 => 4,
            Sq35_2 => 5,
            Sq36_2 => 6,
            Sq37_2 => 7,
            Sq38_2 => 8,
            Sq39_2 => 9,
            Sq41_2 => 1,
            Sq42_2 => 2,
            Sq43_2 => 3,
            Sq44_2 => 4,
            Sq45_2 => 5,
            Sq46_2 => 6,
            Sq47_2 => 7,
            Sq48_2 => 8,
            Sq49_2 => 9,
            Sq51_2 => 1,
            Sq52_2 => 2,
            Sq53_2 => 3,
            Sq54_2 => 4,
            Sq55_2 => 5,
            Sq56_2 => 6,
            Sq57_2 => 7,
            Sq58_2 => 8,
            Sq59_2 => 9,
            Sq61_2 => 1,
            Sq62_2 => 2,
            Sq63_2 => 3,
            Sq64_2 => 4,
            Sq65_2 => 5,
            Sq66_2 => 6,
            Sq67_2 => 7,
            Sq68_2 => 8,
            Sq69_2 => 9,
            Sq71_2 => 1,
            Sq72_2 => 2,
            Sq73_2 => 3,
            Sq74_2 => 4,
            Sq75_2 => 5,
            Sq76_2 => 6,
            Sq77_2 => 7,
            Sq78_2 => 8,
            Sq79_2 => 9,
            Sq81_2 => 1,
            Sq82_2 => 2,
            Sq83_2 => 3,
            Sq84_2 => 4,
            Sq85_2 => 5,
            Sq86_2 => 6,
            Sq87_2 => 7,
            Sq88_2 => 8,
            Sq89_2 => 9,
            Sq91_2 => 1,
            Sq92_2 => 2,
            Sq93_2 => 3,
            Sq94_2 => 4,
            Sq95_2 => 5,
            Sq96_2 => 6,
            Sq97_2 => 7,
            Sq98_2 => 8,
            Sq99_2 => 9,
            King1 | Rook1 | Bishop1 | Gold1 | Silver1 | Knight1 | Lance1 | Pawn1 | King2
            | Rook2 | Bishop2 | Gold2 | Silver2 | Knight2 | Lance2 | Pawn2 => panic!(
                Beam::trouble(&format!("(Err.546) 盤上ではなかったぜ☆（＾～＾）！",))
            ),
        }
    }

    pub fn to_address_pos1(self) -> AddressPos1 {
        // 配列アクセスは遅い気がするので、match構文で書こうぜ☆（＾～＾）
        use crate::cosmic::toy_box::UnifiedAddress::*;
        match self {
            Sq11_1 => UNIFIED_ADDRESS_0_TO_ADDRESS_POS1,
            Sq12_1 => UNIFIED_ADDRESS_1_TO_ADDRESS_POS1,
            Sq13_1 => UNIFIED_ADDRESS_2_TO_ADDRESS_POS1,
            Sq14_1 => UNIFIED_ADDRESS_3_TO_ADDRESS_POS1,
            Sq15_1 => UNIFIED_ADDRESS_4_TO_ADDRESS_POS1,
            Sq16_1 => UNIFIED_ADDRESS_5_TO_ADDRESS_POS1,
            Sq17_1 => UNIFIED_ADDRESS_6_TO_ADDRESS_POS1,
            Sq18_1 => UNIFIED_ADDRESS_7_TO_ADDRESS_POS1,
            Sq19_1 => UNIFIED_ADDRESS_8_TO_ADDRESS_POS1,
            Sq21_1 => UNIFIED_ADDRESS_9_TO_ADDRESS_POS1,
            Sq22_1 => UNIFIED_ADDRESS_10_TO_ADDRESS_POS1,
            Sq23_1 => UNIFIED_ADDRESS_11_TO_ADDRESS_POS1,
            Sq24_1 => UNIFIED_ADDRESS_12_TO_ADDRESS_POS1,
            Sq25_1 => UNIFIED_ADDRESS_13_TO_ADDRESS_POS1,
            Sq26_1 => UNIFIED_ADDRESS_14_TO_ADDRESS_POS1,
            Sq27_1 => UNIFIED_ADDRESS_15_TO_ADDRESS_POS1,
            Sq28_1 => UNIFIED_ADDRESS_16_TO_ADDRESS_POS1,
            Sq29_1 => UNIFIED_ADDRESS_17_TO_ADDRESS_POS1,
            Sq31_1 => UNIFIED_ADDRESS_18_TO_ADDRESS_POS1,
            Sq32_1 => UNIFIED_ADDRESS_19_TO_ADDRESS_POS1,
            Sq33_1 => UNIFIED_ADDRESS_20_TO_ADDRESS_POS1,
            Sq34_1 => UNIFIED_ADDRESS_21_TO_ADDRESS_POS1,
            Sq35_1 => UNIFIED_ADDRESS_22_TO_ADDRESS_POS1,
            Sq36_1 => UNIFIED_ADDRESS_23_TO_ADDRESS_POS1,
            Sq37_1 => UNIFIED_ADDRESS_24_TO_ADDRESS_POS1,
            Sq38_1 => UNIFIED_ADDRESS_25_TO_ADDRESS_POS1,
            Sq39_1 => UNIFIED_ADDRESS_26_TO_ADDRESS_POS1,
            Sq41_1 => UNIFIED_ADDRESS_27_TO_ADDRESS_POS1,
            Sq42_1 => UNIFIED_ADDRESS_28_TO_ADDRESS_POS1,
            Sq43_1 => UNIFIED_ADDRESS_29_TO_ADDRESS_POS1,
            Sq44_1 => UNIFIED_ADDRESS_30_TO_ADDRESS_POS1,
            Sq45_1 => UNIFIED_ADDRESS_31_TO_ADDRESS_POS1,
            Sq46_1 => UNIFIED_ADDRESS_32_TO_ADDRESS_POS1,
            Sq47_1 => UNIFIED_ADDRESS_33_TO_ADDRESS_POS1,
            Sq48_1 => UNIFIED_ADDRESS_34_TO_ADDRESS_POS1,
            Sq49_1 => UNIFIED_ADDRESS_35_TO_ADDRESS_POS1,
            Sq51_1 => UNIFIED_ADDRESS_36_TO_ADDRESS_POS1,
            Sq52_1 => UNIFIED_ADDRESS_37_TO_ADDRESS_POS1,
            Sq53_1 => UNIFIED_ADDRESS_38_TO_ADDRESS_POS1,
            Sq54_1 => UNIFIED_ADDRESS_39_TO_ADDRESS_POS1,
            Sq55_1 => UNIFIED_ADDRESS_40_TO_ADDRESS_POS1,
            Sq56_1 => UNIFIED_ADDRESS_41_TO_ADDRESS_POS1,
            Sq57_1 => UNIFIED_ADDRESS_42_TO_ADDRESS_POS1,
            Sq58_1 => UNIFIED_ADDRESS_43_TO_ADDRESS_POS1,
            Sq59_1 => UNIFIED_ADDRESS_44_TO_ADDRESS_POS1,
            Sq61_1 => UNIFIED_ADDRESS_45_TO_ADDRESS_POS1,
            Sq62_1 => UNIFIED_ADDRESS_46_TO_ADDRESS_POS1,
            Sq63_1 => UNIFIED_ADDRESS_47_TO_ADDRESS_POS1,
            Sq64_1 => UNIFIED_ADDRESS_48_TO_ADDRESS_POS1,
            Sq65_1 => UNIFIED_ADDRESS_49_TO_ADDRESS_POS1,
            Sq66_1 => UNIFIED_ADDRESS_50_TO_ADDRESS_POS1,
            Sq67_1 => UNIFIED_ADDRESS_51_TO_ADDRESS_POS1,
            Sq68_1 => UNIFIED_ADDRESS_52_TO_ADDRESS_POS1,
            Sq69_1 => UNIFIED_ADDRESS_53_TO_ADDRESS_POS1,
            Sq71_1 => UNIFIED_ADDRESS_54_TO_ADDRESS_POS1,
            Sq72_1 => UNIFIED_ADDRESS_55_TO_ADDRESS_POS1,
            Sq73_1 => UNIFIED_ADDRESS_56_TO_ADDRESS_POS1,
            Sq74_1 => UNIFIED_ADDRESS_57_TO_ADDRESS_POS1,
            Sq75_1 => UNIFIED_ADDRESS_58_TO_ADDRESS_POS1,
            Sq76_1 => UNIFIED_ADDRESS_59_TO_ADDRESS_POS1,
            Sq77_1 => UNIFIED_ADDRESS_60_TO_ADDRESS_POS1,
            Sq78_1 => UNIFIED_ADDRESS_61_TO_ADDRESS_POS1,
            Sq79_1 => UNIFIED_ADDRESS_62_TO_ADDRESS_POS1,
            Sq81_1 => UNIFIED_ADDRESS_63_TO_ADDRESS_POS1,
            Sq82_1 => UNIFIED_ADDRESS_64_TO_ADDRESS_POS1,
            Sq83_1 => UNIFIED_ADDRESS_65_TO_ADDRESS_POS1,
            Sq84_1 => UNIFIED_ADDRESS_66_TO_ADDRESS_POS1,
            Sq85_1 => UNIFIED_ADDRESS_67_TO_ADDRESS_POS1,
            Sq86_1 => UNIFIED_ADDRESS_68_TO_ADDRESS_POS1,
            Sq87_1 => UNIFIED_ADDRESS_69_TO_ADDRESS_POS1,
            Sq88_1 => UNIFIED_ADDRESS_70_TO_ADDRESS_POS1,
            Sq89_1 => UNIFIED_ADDRESS_71_TO_ADDRESS_POS1,
            Sq91_1 => UNIFIED_ADDRESS_72_TO_ADDRESS_POS1,
            Sq92_1 => UNIFIED_ADDRESS_73_TO_ADDRESS_POS1,
            Sq93_1 => UNIFIED_ADDRESS_74_TO_ADDRESS_POS1,
            Sq94_1 => UNIFIED_ADDRESS_75_TO_ADDRESS_POS1,
            Sq95_1 => UNIFIED_ADDRESS_76_TO_ADDRESS_POS1,
            Sq96_1 => UNIFIED_ADDRESS_77_TO_ADDRESS_POS1,
            Sq97_1 => UNIFIED_ADDRESS_78_TO_ADDRESS_POS1,
            Sq98_1 => UNIFIED_ADDRESS_79_TO_ADDRESS_POS1,
            Sq99_1 => UNIFIED_ADDRESS_80_TO_ADDRESS_POS1,
            Sq11_2 => UNIFIED_ADDRESS_81_TO_ADDRESS_POS1,
            Sq12_2 => UNIFIED_ADDRESS_82_TO_ADDRESS_POS1,
            Sq13_2 => UNIFIED_ADDRESS_83_TO_ADDRESS_POS1,
            Sq14_2 => UNIFIED_ADDRESS_84_TO_ADDRESS_POS1,
            Sq15_2 => UNIFIED_ADDRESS_85_TO_ADDRESS_POS1,
            Sq16_2 => UNIFIED_ADDRESS_86_TO_ADDRESS_POS1,
            Sq17_2 => UNIFIED_ADDRESS_87_TO_ADDRESS_POS1,
            Sq18_2 => UNIFIED_ADDRESS_88_TO_ADDRESS_POS1,
            Sq19_2 => UNIFIED_ADDRESS_89_TO_ADDRESS_POS1,
            Sq21_2 => UNIFIED_ADDRESS_90_TO_ADDRESS_POS1,
            Sq22_2 => UNIFIED_ADDRESS_91_TO_ADDRESS_POS1,
            Sq23_2 => UNIFIED_ADDRESS_92_TO_ADDRESS_POS1,
            Sq24_2 => UNIFIED_ADDRESS_93_TO_ADDRESS_POS1,
            Sq25_2 => UNIFIED_ADDRESS_94_TO_ADDRESS_POS1,
            Sq26_2 => UNIFIED_ADDRESS_95_TO_ADDRESS_POS1,
            Sq27_2 => UNIFIED_ADDRESS_96_TO_ADDRESS_POS1,
            Sq28_2 => UNIFIED_ADDRESS_97_TO_ADDRESS_POS1,
            Sq29_2 => UNIFIED_ADDRESS_98_TO_ADDRESS_POS1,
            Sq31_2 => UNIFIED_ADDRESS_99_TO_ADDRESS_POS1,
            Sq32_2 => UNIFIED_ADDRESS_100_TO_ADDRESS_POS1,
            Sq33_2 => UNIFIED_ADDRESS_101_TO_ADDRESS_POS1,
            Sq34_2 => UNIFIED_ADDRESS_102_TO_ADDRESS_POS1,
            Sq35_2 => UNIFIED_ADDRESS_103_TO_ADDRESS_POS1,
            Sq36_2 => UNIFIED_ADDRESS_104_TO_ADDRESS_POS1,
            Sq37_2 => UNIFIED_ADDRESS_105_TO_ADDRESS_POS1,
            Sq38_2 => UNIFIED_ADDRESS_106_TO_ADDRESS_POS1,
            Sq39_2 => UNIFIED_ADDRESS_107_TO_ADDRESS_POS1,
            Sq41_2 => UNIFIED_ADDRESS_108_TO_ADDRESS_POS1,
            Sq42_2 => UNIFIED_ADDRESS_109_TO_ADDRESS_POS1,
            Sq43_2 => UNIFIED_ADDRESS_110_TO_ADDRESS_POS1,
            Sq44_2 => UNIFIED_ADDRESS_111_TO_ADDRESS_POS1,
            Sq45_2 => UNIFIED_ADDRESS_112_TO_ADDRESS_POS1,
            Sq46_2 => UNIFIED_ADDRESS_113_TO_ADDRESS_POS1,
            Sq47_2 => UNIFIED_ADDRESS_114_TO_ADDRESS_POS1,
            Sq48_2 => UNIFIED_ADDRESS_115_TO_ADDRESS_POS1,
            Sq49_2 => UNIFIED_ADDRESS_116_TO_ADDRESS_POS1,
            Sq51_2 => UNIFIED_ADDRESS_117_TO_ADDRESS_POS1,
            Sq52_2 => UNIFIED_ADDRESS_118_TO_ADDRESS_POS1,
            Sq53_2 => UNIFIED_ADDRESS_119_TO_ADDRESS_POS1,
            Sq54_2 => UNIFIED_ADDRESS_120_TO_ADDRESS_POS1,
            Sq55_2 => UNIFIED_ADDRESS_121_TO_ADDRESS_POS1,
            Sq56_2 => UNIFIED_ADDRESS_122_TO_ADDRESS_POS1,
            Sq57_2 => UNIFIED_ADDRESS_123_TO_ADDRESS_POS1,
            Sq58_2 => UNIFIED_ADDRESS_124_TO_ADDRESS_POS1,
            Sq59_2 => UNIFIED_ADDRESS_125_TO_ADDRESS_POS1,
            Sq61_2 => UNIFIED_ADDRESS_126_TO_ADDRESS_POS1,
            Sq62_2 => UNIFIED_ADDRESS_127_TO_ADDRESS_POS1,
            Sq63_2 => UNIFIED_ADDRESS_128_TO_ADDRESS_POS1,
            Sq64_2 => UNIFIED_ADDRESS_129_TO_ADDRESS_POS1,
            Sq65_2 => UNIFIED_ADDRESS_130_TO_ADDRESS_POS1,
            Sq66_2 => UNIFIED_ADDRESS_131_TO_ADDRESS_POS1,
            Sq67_2 => UNIFIED_ADDRESS_132_TO_ADDRESS_POS1,
            Sq68_2 => UNIFIED_ADDRESS_133_TO_ADDRESS_POS1,
            Sq69_2 => UNIFIED_ADDRESS_134_TO_ADDRESS_POS1,
            Sq71_2 => UNIFIED_ADDRESS_135_TO_ADDRESS_POS1,
            Sq72_2 => UNIFIED_ADDRESS_136_TO_ADDRESS_POS1,
            Sq73_2 => UNIFIED_ADDRESS_137_TO_ADDRESS_POS1,
            Sq74_2 => UNIFIED_ADDRESS_138_TO_ADDRESS_POS1,
            Sq75_2 => UNIFIED_ADDRESS_139_TO_ADDRESS_POS1,
            Sq76_2 => UNIFIED_ADDRESS_140_TO_ADDRESS_POS1,
            Sq77_2 => UNIFIED_ADDRESS_141_TO_ADDRESS_POS1,
            Sq78_2 => UNIFIED_ADDRESS_142_TO_ADDRESS_POS1,
            Sq79_2 => UNIFIED_ADDRESS_143_TO_ADDRESS_POS1,
            Sq81_2 => UNIFIED_ADDRESS_144_TO_ADDRESS_POS1,
            Sq82_2 => UNIFIED_ADDRESS_145_TO_ADDRESS_POS1,
            Sq83_2 => UNIFIED_ADDRESS_146_TO_ADDRESS_POS1,
            Sq84_2 => UNIFIED_ADDRESS_147_TO_ADDRESS_POS1,
            Sq85_2 => UNIFIED_ADDRESS_148_TO_ADDRESS_POS1,
            Sq86_2 => UNIFIED_ADDRESS_149_TO_ADDRESS_POS1,
            Sq87_2 => UNIFIED_ADDRESS_150_TO_ADDRESS_POS1,
            Sq88_2 => UNIFIED_ADDRESS_151_TO_ADDRESS_POS1,
            Sq89_2 => UNIFIED_ADDRESS_152_TO_ADDRESS_POS1,
            Sq91_2 => UNIFIED_ADDRESS_153_TO_ADDRESS_POS1,
            Sq92_2 => UNIFIED_ADDRESS_154_TO_ADDRESS_POS1,
            Sq93_2 => UNIFIED_ADDRESS_155_TO_ADDRESS_POS1,
            Sq94_2 => UNIFIED_ADDRESS_156_TO_ADDRESS_POS1,
            Sq95_2 => UNIFIED_ADDRESS_157_TO_ADDRESS_POS1,
            Sq96_2 => UNIFIED_ADDRESS_158_TO_ADDRESS_POS1,
            Sq97_2 => UNIFIED_ADDRESS_159_TO_ADDRESS_POS1,
            Sq98_2 => UNIFIED_ADDRESS_160_TO_ADDRESS_POS1,
            Sq99_2 => UNIFIED_ADDRESS_161_TO_ADDRESS_POS1,
            King1 => UNIFIED_ADDRESS_162_TO_ADDRESS_POS1,
            Rook1 => UNIFIED_ADDRESS_163_TO_ADDRESS_POS1,
            Bishop1 => UNIFIED_ADDRESS_164_TO_ADDRESS_POS1,
            Gold1 => UNIFIED_ADDRESS_165_TO_ADDRESS_POS1,
            Silver1 => UNIFIED_ADDRESS_166_TO_ADDRESS_POS1,
            Knight1 => UNIFIED_ADDRESS_167_TO_ADDRESS_POS1,
            Lance1 => UNIFIED_ADDRESS_168_TO_ADDRESS_POS1,
            Pawn1 => UNIFIED_ADDRESS_169_TO_ADDRESS_POS1,
            King2 => UNIFIED_ADDRESS_170_TO_ADDRESS_POS1,
            Rook2 => UNIFIED_ADDRESS_171_TO_ADDRESS_POS1,
            Bishop2 => UNIFIED_ADDRESS_172_TO_ADDRESS_POS1,
            Gold2 => UNIFIED_ADDRESS_173_TO_ADDRESS_POS1,
            Silver2 => UNIFIED_ADDRESS_174_TO_ADDRESS_POS1,
            Knight2 => UNIFIED_ADDRESS_175_TO_ADDRESS_POS1,
            Lance2 => UNIFIED_ADDRESS_176_TO_ADDRESS_POS1,
            Pawn2 => UNIFIED_ADDRESS_177_TO_ADDRESS_POS1,
        }
    }

    pub fn to_square_serial_number(self) -> usize {
        use crate::cosmic::toy_box::UnifiedAddress::*;
        match self {
            Sq11_1 => 11,
            Sq12_1 => 12,
            Sq13_1 => 13,
            Sq14_1 => 14,
            Sq15_1 => 15,
            Sq16_1 => 16,
            Sq17_1 => 17,
            Sq18_1 => 18,
            Sq19_1 => 19,
            Sq21_1 => 21,
            Sq22_1 => 22,
            Sq23_1 => 23,
            Sq24_1 => 24,
            Sq25_1 => 25,
            Sq26_1 => 26,
            Sq27_1 => 27,
            Sq28_1 => 28,
            Sq29_1 => 29,
            Sq31_1 => 31,
            Sq32_1 => 32,
            Sq33_1 => 33,
            Sq34_1 => 34,
            Sq35_1 => 35,
            Sq36_1 => 36,
            Sq37_1 => 37,
            Sq38_1 => 38,
            Sq39_1 => 39,
            Sq41_1 => 41,
            Sq42_1 => 42,
            Sq43_1 => 43,
            Sq44_1 => 44,
            Sq45_1 => 45,
            Sq46_1 => 46,
            Sq47_1 => 47,
            Sq48_1 => 48,
            Sq49_1 => 49,
            Sq51_1 => 51,
            Sq52_1 => 52,
            Sq53_1 => 53,
            Sq54_1 => 54,
            Sq55_1 => 55,
            Sq56_1 => 56,
            Sq57_1 => 57,
            Sq58_1 => 58,
            Sq59_1 => 59,
            Sq61_1 => 61,
            Sq62_1 => 62,
            Sq63_1 => 63,
            Sq64_1 => 64,
            Sq65_1 => 65,
            Sq66_1 => 66,
            Sq67_1 => 67,
            Sq68_1 => 68,
            Sq69_1 => 69,
            Sq71_1 => 71,
            Sq72_1 => 72,
            Sq73_1 => 73,
            Sq74_1 => 74,
            Sq75_1 => 75,
            Sq76_1 => 76,
            Sq77_1 => 77,
            Sq78_1 => 78,
            Sq79_1 => 79,
            Sq81_1 => 81,
            Sq82_1 => 82,
            Sq83_1 => 83,
            Sq84_1 => 84,
            Sq85_1 => 85,
            Sq86_1 => 86,
            Sq87_1 => 87,
            Sq88_1 => 88,
            Sq89_1 => 89,
            Sq91_1 => 91,
            Sq92_1 => 92,
            Sq93_1 => 93,
            Sq94_1 => 94,
            Sq95_1 => 95,
            Sq96_1 => 96,
            Sq97_1 => 97,
            Sq98_1 => 98,
            Sq99_1 => 99,
            Sq11_2 => 11,
            Sq12_2 => 12,
            Sq13_2 => 13,
            Sq14_2 => 14,
            Sq15_2 => 15,
            Sq16_2 => 16,
            Sq17_2 => 17,
            Sq18_2 => 18,
            Sq19_2 => 19,
            Sq21_2 => 21,
            Sq22_2 => 22,
            Sq23_2 => 23,
            Sq24_2 => 24,
            Sq25_2 => 25,
            Sq26_2 => 26,
            Sq27_2 => 27,
            Sq28_2 => 28,
            Sq29_2 => 29,
            Sq31_2 => 31,
            Sq32_2 => 32,
            Sq33_2 => 33,
            Sq34_2 => 34,
            Sq35_2 => 35,
            Sq36_2 => 36,
            Sq37_2 => 37,
            Sq38_2 => 38,
            Sq39_2 => 39,
            Sq41_2 => 41,
            Sq42_2 => 42,
            Sq43_2 => 43,
            Sq44_2 => 44,
            Sq45_2 => 45,
            Sq46_2 => 46,
            Sq47_2 => 47,
            Sq48_2 => 48,
            Sq49_2 => 49,
            Sq51_2 => 51,
            Sq52_2 => 52,
            Sq53_2 => 53,
            Sq54_2 => 54,
            Sq55_2 => 55,
            Sq56_2 => 56,
            Sq57_2 => 57,
            Sq58_2 => 58,
            Sq59_2 => 59,
            Sq61_2 => 61,
            Sq62_2 => 62,
            Sq63_2 => 63,
            Sq64_2 => 64,
            Sq65_2 => 65,
            Sq66_2 => 66,
            Sq67_2 => 67,
            Sq68_2 => 68,
            Sq69_2 => 69,
            Sq71_2 => 71,
            Sq72_2 => 72,
            Sq73_2 => 73,
            Sq74_2 => 74,
            Sq75_2 => 75,
            Sq76_2 => 76,
            Sq77_2 => 77,
            Sq78_2 => 78,
            Sq79_2 => 79,
            Sq81_2 => 81,
            Sq82_2 => 82,
            Sq83_2 => 83,
            Sq84_2 => 84,
            Sq85_2 => 85,
            Sq86_2 => 86,
            Sq87_2 => 87,
            Sq88_2 => 88,
            Sq89_2 => 89,
            Sq91_2 => 91,
            Sq92_2 => 92,
            Sq93_2 => 93,
            Sq94_2 => 94,
            Sq95_2 => 95,
            Sq96_2 => 96,
            Sq97_2 => 97,
            Sq98_2 => 98,
            Sq99_2 => 99,
            King1 | Rook1 | Bishop1 | Gold1 | Silver1 | Knight1 | Lance1 | Pawn1 | King2
            | Rook2 | Bishop2 | Gold2 | Silver2 | Knight2 | Lance2 | Pawn2 => panic!(
                Beam::trouble(&format!("(Err.710) 盤上ではなかったぜ☆（＾～＾）！",))
            ),
        }
    }

    pub fn to_phase(self) -> Phase {
        use crate::cosmic::toy_box::UnifiedAddress::*;
        match self {
            Sq11_1 => Phase::First,
            Sq12_1 => Phase::First,
            Sq13_1 => Phase::First,
            Sq14_1 => Phase::First,
            Sq15_1 => Phase::First,
            Sq16_1 => Phase::First,
            Sq17_1 => Phase::First,
            Sq18_1 => Phase::First,
            Sq19_1 => Phase::First,
            Sq21_1 => Phase::First,
            Sq22_1 => Phase::First,
            Sq23_1 => Phase::First,
            Sq24_1 => Phase::First,
            Sq25_1 => Phase::First,
            Sq26_1 => Phase::First,
            Sq27_1 => Phase::First,
            Sq28_1 => Phase::First,
            Sq29_1 => Phase::First,
            Sq31_1 => Phase::First,
            Sq32_1 => Phase::First,
            Sq33_1 => Phase::First,
            Sq34_1 => Phase::First,
            Sq35_1 => Phase::First,
            Sq36_1 => Phase::First,
            Sq37_1 => Phase::First,
            Sq38_1 => Phase::First,
            Sq39_1 => Phase::First,
            Sq41_1 => Phase::First,
            Sq42_1 => Phase::First,
            Sq43_1 => Phase::First,
            Sq44_1 => Phase::First,
            Sq45_1 => Phase::First,
            Sq46_1 => Phase::First,
            Sq47_1 => Phase::First,
            Sq48_1 => Phase::First,
            Sq49_1 => Phase::First,
            Sq51_1 => Phase::First,
            Sq52_1 => Phase::First,
            Sq53_1 => Phase::First,
            Sq54_1 => Phase::First,
            Sq55_1 => Phase::First,
            Sq56_1 => Phase::First,
            Sq57_1 => Phase::First,
            Sq58_1 => Phase::First,
            Sq59_1 => Phase::First,
            Sq61_1 => Phase::First,
            Sq62_1 => Phase::First,
            Sq63_1 => Phase::First,
            Sq64_1 => Phase::First,
            Sq65_1 => Phase::First,
            Sq66_1 => Phase::First,
            Sq67_1 => Phase::First,
            Sq68_1 => Phase::First,
            Sq69_1 => Phase::First,
            Sq71_1 => Phase::First,
            Sq72_1 => Phase::First,
            Sq73_1 => Phase::First,
            Sq74_1 => Phase::First,
            Sq75_1 => Phase::First,
            Sq76_1 => Phase::First,
            Sq77_1 => Phase::First,
            Sq78_1 => Phase::First,
            Sq79_1 => Phase::First,
            Sq81_1 => Phase::First,
            Sq82_1 => Phase::First,
            Sq83_1 => Phase::First,
            Sq84_1 => Phase::First,
            Sq85_1 => Phase::First,
            Sq86_1 => Phase::First,
            Sq87_1 => Phase::First,
            Sq88_1 => Phase::First,
            Sq89_1 => Phase::First,
            Sq91_1 => Phase::First,
            Sq92_1 => Phase::First,
            Sq93_1 => Phase::First,
            Sq94_1 => Phase::First,
            Sq95_1 => Phase::First,
            Sq96_1 => Phase::First,
            Sq97_1 => Phase::First,
            Sq98_1 => Phase::First,
            Sq99_1 => Phase::First,
            Sq11_2 => Phase::Second,
            Sq12_2 => Phase::Second,
            Sq13_2 => Phase::Second,
            Sq14_2 => Phase::Second,
            Sq15_2 => Phase::Second,
            Sq16_2 => Phase::Second,
            Sq17_2 => Phase::Second,
            Sq18_2 => Phase::Second,
            Sq19_2 => Phase::Second,
            Sq21_2 => Phase::Second,
            Sq22_2 => Phase::Second,
            Sq23_2 => Phase::Second,
            Sq24_2 => Phase::Second,
            Sq25_2 => Phase::Second,
            Sq26_2 => Phase::Second,
            Sq27_2 => Phase::Second,
            Sq28_2 => Phase::Second,
            Sq29_2 => Phase::Second,
            Sq31_2 => Phase::Second,
            Sq32_2 => Phase::Second,
            Sq33_2 => Phase::Second,
            Sq34_2 => Phase::Second,
            Sq35_2 => Phase::Second,
            Sq36_2 => Phase::Second,
            Sq37_2 => Phase::Second,
            Sq38_2 => Phase::Second,
            Sq39_2 => Phase::Second,
            Sq41_2 => Phase::Second,
            Sq42_2 => Phase::Second,
            Sq43_2 => Phase::Second,
            Sq44_2 => Phase::Second,
            Sq45_2 => Phase::Second,
            Sq46_2 => Phase::Second,
            Sq47_2 => Phase::Second,
            Sq48_2 => Phase::Second,
            Sq49_2 => Phase::Second,
            Sq51_2 => Phase::Second,
            Sq52_2 => Phase::Second,
            Sq53_2 => Phase::Second,
            Sq54_2 => Phase::Second,
            Sq55_2 => Phase::Second,
            Sq56_2 => Phase::Second,
            Sq57_2 => Phase::Second,
            Sq58_2 => Phase::Second,
            Sq59_2 => Phase::Second,
            Sq61_2 => Phase::Second,
            Sq62_2 => Phase::Second,
            Sq63_2 => Phase::Second,
            Sq64_2 => Phase::Second,
            Sq65_2 => Phase::Second,
            Sq66_2 => Phase::Second,
            Sq67_2 => Phase::Second,
            Sq68_2 => Phase::Second,
            Sq69_2 => Phase::Second,
            Sq71_2 => Phase::Second,
            Sq72_2 => Phase::Second,
            Sq73_2 => Phase::Second,
            Sq74_2 => Phase::Second,
            Sq75_2 => Phase::Second,
            Sq76_2 => Phase::Second,
            Sq77_2 => Phase::Second,
            Sq78_2 => Phase::Second,
            Sq79_2 => Phase::Second,
            Sq81_2 => Phase::Second,
            Sq82_2 => Phase::Second,
            Sq83_2 => Phase::Second,
            Sq84_2 => Phase::Second,
            Sq85_2 => Phase::Second,
            Sq86_2 => Phase::Second,
            Sq87_2 => Phase::Second,
            Sq88_2 => Phase::Second,
            Sq89_2 => Phase::Second,
            Sq91_2 => Phase::Second,
            Sq92_2 => Phase::Second,
            Sq93_2 => Phase::Second,
            Sq94_2 => Phase::Second,
            Sq95_2 => Phase::Second,
            Sq96_2 => Phase::Second,
            Sq97_2 => Phase::Second,
            Sq98_2 => Phase::Second,
            Sq99_2 => Phase::Second,
            King1 => Phase::First,
            Rook1 => Phase::First,
            Bishop1 => Phase::First,
            Gold1 => Phase::First,
            Silver1 => Phase::First,
            Knight1 => Phase::First,
            Lance1 => Phase::First,
            Pawn1 => Phase::First,
            King2 => Phase::Second,
            Rook2 => Phase::Second,
            Bishop2 => Phase::Second,
            Gold2 => Phase::Second,
            Silver2 => Phase::Second,
            Knight2 => Phase::Second,
            Lance2 => Phase::Second,
            Pawn2 => Phase::Second,
        }
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
    address_list: [Fire; NAMED_PIECES_LEN],
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
            address_list: [Fire::default(); NAMED_PIECES_LEN],
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
        self.address_list = [Fire::default(); NAMED_PIECES_LEN];
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
        if let Some(collision_piece_num_val) = self.pop_piece(&move_.destination) {
            // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
            // 先後ひっくり返す。
            self.turn_phase(collision_piece_num_val);
            self.push_piece(
                UnifiedAddress::from_address_pos1(
                    self.get_phase(collision_piece_num_val),
                    AddressPos1::Hand(self.get_double_faced_piece(collision_piece_num_val)),
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
                let fire1 = Fire::new_hand(friend, double_faced_piece.type_());
                // let uni_addr = UnifiedAddress::from_address_pos1(friend, addr_pos1);
                // let addr_pos2 = uni_addr.to_address_pos();
                /*
                Beam::shoot(&format!(
                    "addr_pos {} -> {:?} -> {}",
                    addr_pos1, uni_addr, addr_pos2
                ));
                */
                self.pop_piece(&fire1).unwrap()
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
            self.push_piece(
                UnifiedAddress::from_fire(&move_.destination),
                Some(piece_num),
            );
        }
    }
    /// 駒を置く。
    pub fn push_piece(
        &mut self,
        addr: UnifiedAddress, /*TODO これの置き換えはバグりやすい。*/
        piece_num: Option<PieceNum>,
    ) {
        match addr.to_address_pos1() {
            AddressPos1::Board(sq) => {
                if let Some(piece_num_val) = piece_num {
                    // マスに駒を置きます。
                    self.board[sq.serial_number() as usize] = piece_num;
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] =
                        UnifiedAddress::from_absolute_address(self.get_phase(piece_num_val), &sq)
                            .to_fire();
                } else {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                }
            }
            AddressPos1::Hand(drop) => {
                if let Some(piece_num_val) = piece_num {
                    // 持ち駒を１つ増やします。
                    self.phase_classification
                        .push(&Fire::new_hand(drop.phase(), drop.type_()), piece_num_val);
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] = addr.to_fire();
                }
            }
        }
    }
    /// 駒を取りのぞく。
    pub fn pop_piece(&mut self, fire: &Fire) -> Option<PieceNum> {
        match fire.address {
            FireAddress::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                    // TODO 背番号の番地を、ゴミ値で塗りつぶすが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                    self.address_list[piece_num_val as usize] = UnifiedAddress::from_address_pos1(
                        self.get_phase(piece_num_val),
                        AddressPos1::default(),
                    )
                    .to_fire();
                }
                piece_num
            }
            FireAddress::Hand(drop_type) => {
                // 場所で指定します。
                // 台から取りのぞきます。
                let piece_num = self.phase_classification.pop(&fire);
                // TODO 背番号の番地に、ゴミ値を入れて消去するが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                self.address_list[piece_num as usize] = UnifiedAddress::from_address_pos1(
                    self.get_phase(piece_num),
                    AddressPos1::default(),
                )
                .to_fire();
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
            if let Some(piece_val) =
                self.piece_at1(&FireAddress::Board(AbsoluteAddress2D::new(file, rank)))
            {
                if piece_val.phase() == friend && piece_val.type_() == PieceType::Pawn {
                    return true;
                }
            }
        }
        false
    }
    /// ハッシュを作るときにも利用。盤上専用。
    pub fn piece_at1(&self, addr: &FireAddress) -> Option<Piece> {
        match addr {
            FireAddress::Board(sq) => {
                if let Some(piece_num) = self.board[sq.serial_number() as usize] {
                    Some(self.get_piece(piece_num))
                } else {
                    None
                }
            }
            FireAddress::Hand(_drop_type) => panic!(Beam::trouble(&format!(
                "(Err.345) 駒台は非対応☆（＾～＾）！",
            ))),
        }
    }
    /// TODO Piece をカプセル化したい。外に出したくないぜ☆（＾～＾）
    /// 升で指定して駒を取得。
    /// 駒台には対応してない。 -> 何に使っている？
    pub fn piece_num_at(&self, addr: &FireAddress) -> Option<PieceNum> {
        match addr {
            FireAddress::Board(sq) => self.board[sq.serial_number() as usize],
            _ => panic!(Beam::trouble(&format!(
                "(Err.254) まだ駒台は実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 駒台には対応してない。 -> 何に使っている？
    pub fn piece_info_at1(&self, addr: &FireAddress) -> Option<PieceInfo> {
        match addr {
            FireAddress::Board(sq) => {
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
    pub fn promotion_value_at(&self, table: &GameTable, fire: &Fire) -> isize {
        match fire.address {
            FireAddress::Board(sq) => {
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
            FireAddress::Hand(_drop_type) => panic!(Beam::trouble(&format!(
                "(Err.254) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 指し手生成で使うぜ☆（＾～＾）有無を調べるぜ☆（＾～＾）
    pub fn last_hand_type(&self, fire: &Fire) -> Option<PieceType> {
        if let Some(piece_num) = self.phase_classification.last(&fire) {
            Some(self.get_type(piece_num))
        } else {
            None
        }
    }
    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand(&self, fire: &Fire) -> Option<(PieceType, UnifiedAddress)> {
        match fire.address {
            FireAddress::Board(_sq) => {
                panic!(Beam::trouble(&format!("(Err.3251) 未対応☆（＾～＾）！",)))
            }
            FireAddress::Hand(drop_type) => {
                if let Some(piece_num) = self
                    .phase_classification
                    .last(&Fire::new_hand(fire.friend, drop_type))
                {
                    let piece = self.get_piece(piece_num);
                    Some((
                        piece.type_(),
                        UnifiedAddress::from_double_faced_piece(piece.double_faced_piece()),
                    ))
                } else {
                    None
                }
            }
        }
    }
    pub fn count_hand(&self, fire: &Fire) -> usize {
        match fire.address {
            FireAddress::Board(_sq) => {
                panic!(Beam::trouble(&format!("(Err.3266) 未対応☆（＾～＾）！",)))
            }
            FireAddress::Hand(drop_type) => self
                .phase_classification
                .len(&Fire::new_hand(fire.friend, drop_type)),
        }
    }

    /// 表示に使うだけ☆（＾～＾）
    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_table<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, Option<&AbsoluteAddress2D>, Option<PieceInfo>),
    {
        for (i, fire) in self.address_list.iter().enumerate() {
            match fire.address {
                FireAddress::Board(sq) => {
                    // 盤上の駒☆（＾～＾）
                    let piece_info = self.piece_info_at1(&fire.address).unwrap();
                    piece_get(i, Some(&sq), Some(piece_info));
                }
                FireAddress::Hand(_drop) => {
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
        F: FnMut(&Fire, PieceType),
    {
        for piece_num in Nine299792458::piece_numbers().iter() {
            // 盤上の駒だけを調べようぜ☆（＾～＾）
            let fire = self.address_list[*piece_num as usize];
            match fire.address {
                FireAddress::Board(_sq) => {
                    if self.get_phase(*piece_num) == friend {
                        piece_get(&fire, self.get_type(*piece_num));
                    }
                }
                FireAddress::Hand(_drop) => {
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
            if let Some(piece_type) = self.last_hand_type(&Fire::new_hand(friend, drop.type_())) {
                // 有無を確認しているぜ☆（＾～＾）
                piece_get(
                    &UnifiedAddress::from_double_faced_piece(*drop).to_fire(),
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
    pub fn push(&mut self, fire: &Fire, num: PieceNum) {
        match fire.address {
            FireAddress::Board(_sq) => panic!(Beam::trouble("(Err.3407) 未対応☆（＾～＾）")),
            FireAddress::Hand(drop_type) => {
                let drop = DoubleFacedPiece::from_phase_and_type(fire.friend, drop_type);
                let area = &self.areas[drop as usize];
                // 駒台に駒を置くぜ☆（＾～＾）
                self.items[self.currents[drop as usize] as usize] = num;
                // 位置を増減するぜ☆（＾～＾）
                self.currents[drop as usize] += area.direction;
            }
        }
    }
    /// ゴミ値は消さないぜ☆（＾～＾）
    pub fn pop(&mut self, fire: &Fire) -> PieceNum {
        match fire.address {
            FireAddress::Board(_sq) => panic!(Beam::trouble("(Err.3419) 未対応☆（＾～＾）")),
            FireAddress::Hand(drop_type) => {
                let drop = DoubleFacedPiece::from_phase_and_type(fire.friend, drop_type);
                let area = &self.areas[drop as usize];
                // 位置を増減するぜ☆（＾～＾）
                self.currents[drop as usize] -= area.direction;
                // 駒台の駒をはがすぜ☆（＾～＾）
                self.items[self.currents[drop as usize] as usize]
            }
        }
    }

    fn last(&self, fire: &Fire) -> Option<PieceNum> {
        match fire.address {
            FireAddress::Board(_sq) => panic!(Beam::trouble("(Err.3431) 未対応☆（＾～＾）")),
            FireAddress::Hand(drop_type) => {
                let drop = DoubleFacedPiece::from_phase_and_type(fire.friend, drop_type);
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
        }
    }

    fn len(&self, fire: &Fire) -> usize {
        match fire.address {
            FireAddress::Board(_sq) => panic!(Beam::trouble("(Err.3431) 未対応☆（＾～＾）")),
            FireAddress::Hand(drop_type) => {
                let drop = DoubleFacedPiece::from_phase_and_type(fire.friend, drop_type);
                let area = &self.areas[drop as usize];
                if area.direction == 1 {
                    (self.currents[drop as usize] - area.start) as usize
                } else {
                    (area.start - self.currents[drop as usize]) as usize
                }
            }
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
