//!
//! * History (棋譜)
//! * Movement (指し手)
//! * Phase (先後。手番,相手番)
//! * Person (先手,後手)
//!
use crate::cosmic::fire::{Fire, FireAddress};
use crate::cosmic::smart::features::DoubleFacedPiece;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::smart::square::AbsoluteAddress2D;
use crate::cosmic::toy_box::SquareType;
use crate::cosmic::toy_box::UnifiedAddress;
use crate::law::cryptographic::num_to_lower_case;
use std::fmt;

/// 手数☆（＾～＾） 大会ルールとは別で、このプログラムが対応できる上限値☆（＾～＾）
/// 主要大会では、一番大きくても　電竜戦の 512 だろ☆（＾～＾）
pub const PLY_SIZE: usize = 1024;

/// 同一局面何回で千日手
pub const SENNTITE_NUM: isize = 4;

pub struct History {
    /// 手目。増減するので符号付きにしておくぜ☆（＾～＾）i8 は -128～127 なんで手数が収まらん☆（＾～＾）
    pub ply: isize,
    /// 棋譜
    /// TODO 0手目を初期局面にしたいので、最初にパスを入れてほしい☆（＾～＾）
    pub movements: [Movement; PLY_SIZE],
    /// 棋譜に対応した各局面の局面ハッシュ
    pub position_hashs: [u64; PLY_SIZE],
    /// 初期局面ハッシュ
    pub starting_position_hash: u64,
}
impl Default for History {
    fn default() -> History {
        History {
            ply: 0,
            movements: [Movement::default(); PLY_SIZE],
            position_hashs: [0; PLY_SIZE],
            starting_position_hash: 0,
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
    /// 現在の指し手
    pub fn get_move(&self) -> &Movement {
        &self.movements[self.ply as usize]
    }
    /*
    /// 局面ハッシュを更新
    pub fn set_position_hash(&mut self, hash: u64) {
        self.position_hashs[self.ply as usize] = hash;
    }
    */
}

/// 局面(Position)全体を範囲にして振られた番地(Address)。
#[derive(Clone, Copy, Debug)]
pub enum AddressPos1 {
    // 盤上の番地 TODO これを先手盤上、後手盤上の２つに分けれる☆（＾～＾）
    Board(SquareType),
    // 持ち駒の種類
    Hand(DoubleFacedPiece),
}
impl Default for AddressPos1 {
    // ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        AddressPos1::Board(SquareType::Sq11)
    }
}
/// USI向け。
impl fmt::Display for AddressPos1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AddressPos1::Board(sq) => {
                    let (file, rank) = sq.to_file_rank();
                    format!("{}{}", file, num_to_lower_case(rank))
                }
                AddressPos1::Hand(drop) => {
                    format!("{}", drop)
                }
            },
        )
    }
}

/// 局面(Position)全体を範囲にして振られた番地(Address)。
#[derive(Clone, Copy)]
pub enum AddressPos3 {
    // 先手の盤上の番地
    FirstBoard(SquareType),
    // 後手の盤上の番地
    SecondBoard(SquareType),
    // 持ち駒の種類
    Hand(DoubleFacedPiece),
}
impl Default for AddressPos3 {
    // ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        AddressPos3::FirstBoard(SquareType::Sq11)
    }
}
impl fmt::Display for AddressPos3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AddressPos3::FirstBoard(sq) | AddressPos3::SecondBoard(sq) => {
                    let (file, rank) = sq.to_file_rank();
                    format!("{}{}", file, num_to_lower_case(rank))
                }
                AddressPos3::Hand(drop) => {
                    format!("{}", drop)
                }
            },
        )
    }
}
impl fmt::Debug for AddressPos3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AddressPos3::FirstBoard(sq) | AddressPos3::SecondBoard(sq) => {
                    sq.to_serial_number().to_string()
                }
                AddressPos3::Hand(drop) => {
                    format!("{:?}", drop)
                }
            },
        )
    }
}

/// 取ることになる駒の移動。
#[derive(Clone, Copy)]
pub struct CapturedMove {
    /// 取ることになる駒
    pub piece_type: PieceType,
    /// 元あった所。
    pub source: Fire,
    /*
    /// TODO 移動先。
    pub destination: UnifiedAddress,
    */
}
impl CapturedMove {
    // TODO Piece を持ちまわすのは止めたいが……☆（＾～＾）
    pub fn new(source: Fire, piece_type: PieceType) -> Self {
        CapturedMove {
            source: source,
            piece_type: piece_type,
        }
    }
}

/// 棋譜にも使うので、取った駒の情報を記憶しておくんだぜ☆（＾～＾）
/// 投了なら これを使わず、None にしろだぜ☆（＾～＾）
///
/// 移動していないことを表すには、移動元と移動先を同じにすればいいんだぜ☆（＾～＾）
///
/// Copy: 配列の要素の初期化時に使う☆（＾～＾）
#[derive(Clone, Copy)]
pub struct Movement {
    /// 移動元マス。
    pub source: Fire,
    /// 移動先マス。リバーシブルに作りたいので、駒台にも指せる。
    pub destination: Fire,
    /// 移動後に成るなら真
    pub promote: bool,
    /// 取ることになる駒
    pub captured: Option<CapturedMove>,
}
impl Default for Movement {
    /// ゴミの値を作るぜ☆（＾～＾）
    fn default() -> Self {
        Movement {
            source: Fire::default(),
            destination: Fire::default(),
            promote: false,
            captured: None,
        }
    }
}
impl Movement {
    pub fn new(
        source: Fire,
        destination: Fire,
        promote: bool,
        captured: Option<CapturedMove>,
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
        match self.source.address {
            FireAddress::Board(src_sq) => {
                let (sx, sy) = src_sq.to_file_rank();
                write!(
                    f,
                    "{}{}{}{}",
                    sx,
                    num_to_lower_case(sy),
                    self.destination,
                    if self.promote { "+" } else { "" }
                )
            }
            FireAddress::Hand(src_drop_type) => write!(
                f,
                "{}{}{}",
                self.source, //src_drop_type,
                self.destination,
                if self.promote { "+" } else { "" }
            ),
        }
    }
}
impl fmt::Debug for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Movement({:?}{:?}{})",
            self.source, self.destination, self.promote,
        )
    }
}

/// 局面ハッシュを作るときに、フェーズ用に配列があって、それのサイズに使ってるぜ☆（＾～＾）
// pub const PHASE_FIRST: usize = 0;
pub const PHASE_SECOND: usize = 1;
pub const PHASE_LEN: usize = 2;

/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    First,
    Second,
}
/*
impl Phase {
    pub fn turn(self) -> Phase {
        use self::Phase::*;
        match self {
            First => Second,
            Second => First,
        }
    }
}
*/
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
