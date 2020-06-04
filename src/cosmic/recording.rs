//!
//! * History (棋譜)
//! * Movement (指し手)
//! * Phase (先後。手番,相手番)
//! * Person (先手,後手)
//!
use crate::cosmic::smart::features::PhysicalPieceType;
use crate::cosmic::smart::square::AbsoluteAddress;
use crate::law::cryptographic::num_to_lower_case;
use crate::law::generate_move::Piece;
use std::fmt;

/// 手目数。何手目まで指せるか。
/// 棋譜を残す配列のサイズでもある。
/// 大会ルールで 320手が上限なので、終端子として投了を１個入れておけるように +1 する。
pub const PLY_LEN: usize = 321;

/// 同一局面何回で千日手
pub const SENNTITE_NUM: isize = 4;

pub struct History {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    pub ply: isize,
    /// 棋譜
    /// TODO 0手目を初期局面にしたいので、最初にパスを入れてほしい☆（＾～＾）
    pub movements: [Movement; PLY_LEN],
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hashs: [u64; PLY_LEN],
}
impl Default for History {
    fn default() -> History {
        History {
            ply: 0,
            movements: [Movement::default(); PLY_LEN],
            position_hashs: [0; PLY_LEN],
        }
    }
}
impl History {
    /// 手番
    pub fn get_friend(&self) -> Phase {
        // 手番
        if self.ply % 2 == 0 {
            Phase::First
        } else {
            Phase::Second
        }
    }
}

#[derive(Clone, Copy)]
pub enum AddressTypeOnPosition {
    // 盤上の移動なら、移動元升。
    Move(AbsoluteAddress),
    // 打の場合、打った駒種類
    Drop(PhysicalPieceType),
    // 未設定ならこれ。
    Busy,
}

/// 棋譜にも使うので、取った駒の情報を記憶しておくんだぜ☆（＾～＾）
/// 投了なら これを使わず、None にしろだぜ☆（＾～＾）
///
/// Copy: 配列の要素の初期化時に使う☆（＾～＾）
#[derive(Clone, Copy)]
pub struct Movement {
    pub source: AddressTypeOnPosition,
    // 移動先升。
    pub destination: AbsoluteAddress,
    // 移動後に成るなら真
    pub promote: bool,
    // 取ることになる駒
    pub captured: Option<Piece>,
}
impl Default for Movement {
    /// ゴミの値を作るぜ☆（＾～＾）
    fn default() -> Self {
        Movement {
            source: AddressTypeOnPosition::Busy,
            destination: AbsoluteAddress::default(),
            promote: false,
            captured: None,
        }
    }
}
impl Movement {
    pub fn new(
        source: AddressTypeOnPosition,
        destination: AbsoluteAddress,
        promote: bool,
        captured: Option<Piece>,
    ) -> Self {
        Movement {
            source: source,
            destination: destination,
            promote: promote,
            captured: captured,
        }
    }

    pub fn set(&mut self, b: &Movement) {
        self.source = b.source;
        self.destination = b.destination;
        self.promote = b.promote;
    }
}
impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (dx, dy) = self.destination.to_file_rank();

        match self.source {
            AddressTypeOnPosition::Move(source_val) => {
                let (sx, sy) = source_val.to_file_rank();
                write!(
                    f,
                    "{}{}{}{}{}",
                    sx,
                    num_to_lower_case(sy),
                    dx,
                    num_to_lower_case(dy),
                    if self.promote { "+" } else { "" }
                )
            }
            AddressTypeOnPosition::Drop(drop) => {
                const DROPS: [&str; 8] = ["?", "R", "B", "G", "S", "N", "L", "P"];
                write!(
                    f,
                    "{}*{}{}{}",
                    DROPS[drop as usize],
                    dx,
                    num_to_lower_case(dy),
                    if self.promote { "+" } else { "" }
                )
            }
            AddressTypeOnPosition::Busy => write!(f, "Busy",),
        }
    }
}
impl fmt::Debug for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Movement({}{}{})",
            match self.source {
                AddressTypeOnPosition::Move(source_val) => {
                    source_val.serial_number().to_string()
                }
                AddressTypeOnPosition::Drop(drop) => {
                    format!("{:?}", drop)
                }
                AddressTypeOnPosition::Busy => {
                    "-".to_string()
                }
            },
            self.destination.serial_number(),
            self.promote,
        )
    }
}

/// 局面ハッシュを作るときに、フェーズ用に配列があって、それのサイズに使ってるぜ☆（＾～＾）
pub const PHASE_FIRST: usize = 0;
pub const PHASE_SECOND: usize = 1;
pub const PHASE_LEN: usize = 2;

/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    First,
    Second,
}
impl Phase {
    /*
    pub fn turn(self) -> Phase {
        use self::Phase::*;
        match self {
            First => Second,
            Second => First,
        }
    }
    */
}
/// 後手（上手）を盤の下側に持ってきて表示するのを基本とするぜ☆（＾～＾）
impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // Windows Terminal では ▲、▽が半角サイズで表示されるので、それに合わせている☆（＾～＾） Microsoft 製品に最適化していいのか知らないが……☆（＾～＾）
        use self::Phase::*;
        match *self {
            First => write!(f, " ▲"),
            Second => write!(f, " ▽"),
        }
    }
}
