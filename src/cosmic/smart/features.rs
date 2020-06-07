//!
//! 駒種類
//!
//! 先後なしの駒と空白
//!

use num_derive::FromPrimitive;
use std::fmt;

// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const HAND_MAX: usize = 18;

pub const PIECE_TYPE_LEN: usize = 14;

/// USIでCopyするので、Copyが要る。
#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    // 玉
    King,
    // 飛
    Rook,
    // 角
    Bishop,
    // 金
    Gold,
    // 銀
    Silver,
    // 桂
    Knight,
    // 香
    Lance,
    // 歩
    Pawn,
    // 竜
    Dragon,
    // 馬
    Horse,
    // 全
    PromotedSilver,
    // 圭
    PromotedKnight,
    // 杏
    PromotedLance,
    // ぱわーあっぷひよこ
    PromotedPawn,
}
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::PieceType::*;
        match *self {
            King => write!(f, "ら"),
            Rook => write!(f, "き"),
            Bishop => write!(f, "ぞ"),
            Gold => write!(f, "い"),
            Silver => write!(f, "ね"),
            Knight => write!(f, "う"),
            Lance => write!(f, "い"),
            Pawn => write!(f, "ひ"),
            Dragon => write!(f, "PK"),
            Horse => write!(f, "PZ"),
            PromotedSilver => write!(f, "PN"),
            PromotedKnight => write!(f, "PU"),
            PromotedLance => write!(f, "PS"),
            PromotedPawn => write!(f, "PH"),
        }
    }
}

pub const PHYSICAL_PIECE_TYPE_LEN: usize = 8;
#[derive(Clone, Copy, Debug, FromPrimitive)]
/// 物理的な駒の種類。玉を除けば、持ち駒の種類。
pub enum DoubleFacedPieceType {
    King,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
}

// Note: 持ち駒には玉も含むぜ☆（＾～＾）
pub const PHYSICAL_PIECES_LEN: usize = 16;

#[derive(Clone, Copy, Debug)]
/// 表面と裏面の組み合わせで１つとしたときの種類。先後区別。玉を除けば、持ち駒の種類。
pub enum DoubleFacedPiece {
    // ▲ 玉と印字無し
    King1,
    // ▲ 飛と竜
    Rook1,
    // ▲ 角と馬
    Bishop1,
    // ▲ 金と印字無し
    Gold1,
    // ▲ 銀と全
    Silver1,
    // ▲ 桂と圭
    Knight1,
    // ▲ 香と杏
    Lance1,
    // ▲ 歩とと
    Pawn1,
    // △ 玉と印字無し
    King2,
    // △ 飛と竜
    Rook2,
    // △ 角と馬
    Bishop2,
    // △ 金と印字無し
    Gold2,
    // △ 銀と全
    Silver2,
    // △ 桂と圭
    Knight2,
    // △ 香と杏
    Lance2,
    // △ 歩とと
    Pawn2,
}
/// USIの Drop に合わせるぜ☆（＾～＾） 先後を区別しないぜ☆（＾～＾）
impl fmt::Display for DoubleFacedPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use self::DoubleFacedPiece::*;
        match *self {
            King1 => write!(f, "?*"),
            Rook1 => write!(f, "R*"),
            Bishop1 => write!(f, "B*"),
            Gold1 => write!(f, "G*"),
            Silver1 => write!(f, "S*"),
            Knight1 => write!(f, "N*"),
            Lance1 => write!(f, "L*"),
            Pawn1 => write!(f, "P*"),
            King2 => write!(f, "?*"),
            Rook2 => write!(f, "R*"),
            Bishop2 => write!(f, "B*"),
            Gold2 => write!(f, "G*"),
            Silver2 => write!(f, "S*"),
            Knight2 => write!(f, "N*"),
            Lance2 => write!(f, "L*"),
            Pawn2 => write!(f, "P*"),
        }
    }
}

// 利きボード☆（＾～＾）
#[derive(Clone, Copy)]
pub struct ControlBoard {}
impl Default for ControlBoard {
    fn default() -> Self {
        ControlBoard {}
    }
}
