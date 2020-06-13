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
const UNIFIED_ADDRESS_162_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::First, DoubleFacedPieceType::King));
const UNIFIED_ADDRESS_163_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::First, DoubleFacedPieceType::Rook));
const UNIFIED_ADDRESS_164_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::First, DoubleFacedPieceType::Bishop));
const UNIFIED_ADDRESS_165_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::First, DoubleFacedPieceType::Gold));
const UNIFIED_ADDRESS_166_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::First, DoubleFacedPieceType::Silver));
const UNIFIED_ADDRESS_167_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::First, DoubleFacedPieceType::Knight));
const UNIFIED_ADDRESS_168_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::First, DoubleFacedPieceType::Lance));
const UNIFIED_ADDRESS_169_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::First, DoubleFacedPieceType::Pawn));
const UNIFIED_ADDRESS_170_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::Second, DoubleFacedPieceType::King));
const UNIFIED_ADDRESS_171_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::Second, DoubleFacedPieceType::Rook));
const UNIFIED_ADDRESS_172_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::Second, DoubleFacedPieceType::Bishop));
const UNIFIED_ADDRESS_173_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::Second, DoubleFacedPieceType::Gold));
const UNIFIED_ADDRESS_174_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::Second, DoubleFacedPieceType::Silver));
const UNIFIED_ADDRESS_175_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::Second, DoubleFacedPieceType::Knight));
const UNIFIED_ADDRESS_176_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::Second, DoubleFacedPieceType::Lance));
const UNIFIED_ADDRESS_177_TO_ADDRESS_POS1: AddressPos1 =
    AddressPos1::Hand((Phase::Second, DoubleFacedPieceType::Pawn));

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
    pub fn get_double_faced_piece_type(&self, num: PieceNum) -> DoubleFacedPieceType {
        self.piece_list[num as usize].double_faced_piece().type_()
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
                &Fire::new_hand(
                    self.get_phase(collision_piece_num_val),
                    self.get_double_faced_piece_type(collision_piece_num_val),
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
            self.push_piece(&move_.destination, Some(piece_num));
        }
    }
    /// 駒を置く。
    pub fn push_piece(&mut self, fire: &Fire, piece_num: Option<PieceNum>) {
        match fire.address {
            FireAddress::Board(sq) => {
                if let Some(piece_num_val) = piece_num {
                    // マスに駒を置きます。
                    self.board[sq.serial_number() as usize] = piece_num;
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] =
                        Fire::new_board(self.get_phase(piece_num_val), sq);
                } else {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                }
            }
            FireAddress::Hand(drop_type) => {
                if let Some(piece_num_val) = piece_num {
                    // 持ち駒を１つ増やします。
                    self.phase_classification
                        .push(&Fire::new_hand(fire.friend, drop_type), piece_num_val);
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] = *fire;
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
                    self.address_list[piece_num_val as usize] = Fire::default();
                }
                piece_num
            }
            FireAddress::Hand(drop_type) => {
                // 場所で指定します。
                // 台から取りのぞきます。
                let piece_num = self.phase_classification.pop(&fire);
                // TODO 背番号の番地に、ゴミ値を入れて消去するが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                self.address_list[piece_num as usize] = Fire::default();
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
    pub fn last_hand(&self, fire: &Fire) -> Option<(PieceType, Fire)> {
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
                        Fire::new_hand(fire.friend, piece.double_faced_piece().type_()),
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
                piece_get(&Fire::new_hand(drop.phase(), drop.type_()), piece_type);
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
