//!
//! * History (棋譜)
//! * Movement (指し手)
//! * Phase (先後。手番,相手番)
//! * Person (先手,後手)
//!
use crate::cosmic::fire::{Fire, FireAddress};
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

/// 取ることになる駒の移動。
#[derive(Clone, Copy)]
pub struct CapturedMove {
    /// 元あった所。
    pub source: Fire,
    /// 移動先。
    pub destination: Fire,
}
impl CapturedMove {
    pub fn new(source: Fire, destination: Fire) -> Self {
        CapturedMove {
            source: source,
            destination: destination,
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
            FireAddress::Hand(_src_drop_type) => write!(
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
