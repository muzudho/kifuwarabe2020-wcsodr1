use crate::cosmic::recording::Phase;
use crate::cosmic::smart::features::{DoubleFacedPiece, PieceType};
use num_derive::FromPrimitive;
use std::fmt;

pub const PIECE_LEN: usize = 28;

/// toy_boxの中にカプセル化するぜ☆（＾～＾）
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
/// きふわらべ「USIでは使わないから、独自の表記をして構わないぜ☆」
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // ▲ が半角サイズ、▽ が見た目が全角の半角サイズなのは、Windows Terminal の担当者 いい加減だぜ☆（＾～＾）
        use crate::look_and_model::piece::Piece::*;
        write!(
            f,
            "{}",
            match *self {
                King1 => " ▲K ",
                Rook1 => " ▲R ",
                Bishop1 => " ▲B ",
                Gold1 => " ▲G ",
                Silver1 => " ▲S ",
                Knight1 => " ▲N ",
                Lance1 => " ▲L ",
                Pawn1 => " ▲P ",
                Dragon1 => " ▲PR",
                Horse1 => " ▲PB",
                PromotedSilver1 => " ▲PS",
                PromotedKnight1 => " ▲PN",
                PromotedLance1 => " ▲PL",
                PromotedPawn1 => " ▲PP",
                King2 => " ▽k ",
                Rook2 => " ▽r ",
                Bishop2 => " ▽b ",
                Gold2 => " ▽g ",
                Silver2 => " ▽s ",
                Knight2 => " ▽n ",
                Lance2 => " ▽l ",
                Pawn2 => " ▽p ",
                Dragon2 => " ▽pr",
                Horse2 => " ▽pb",
                PromotedSilver2 => " ▽ps",
                PromotedKnight2 => " ▽pn",
                PromotedLance2 => " ▽pl",
                PromotedPawn2 => " ▽pp",
            }
        )
    }
}
impl Piece {
    pub fn from_phase_and_piece_type(turn: Phase, piece_type: PieceType) -> Self {
        match turn {
            Phase::First => match piece_type {
                PieceType::King => Piece::King1,
                PieceType::Rook => Piece::Rook1,
                PieceType::Bishop => Piece::Bishop1,
                PieceType::Gold => Piece::Gold1,
                PieceType::Silver => Piece::Silver1,
                PieceType::Knight => Piece::Knight1,
                PieceType::Lance => Piece::Lance1,
                PieceType::Pawn => Piece::Pawn1,
                PieceType::Dragon => Piece::Dragon1,
                PieceType::Horse => Piece::Horse1,
                PieceType::PromotedSilver => Piece::PromotedSilver1,
                PieceType::PromotedKnight => Piece::PromotedKnight1,
                PieceType::PromotedLance => Piece::PromotedLance1,
                PieceType::PromotedPawn => Piece::PromotedPawn1,
            },
            Phase::Second => match piece_type {
                PieceType::King => Piece::King2,
                PieceType::Rook => Piece::Rook2,
                PieceType::Bishop => Piece::Bishop2,
                PieceType::Gold => Piece::Gold2,
                PieceType::Silver => Piece::Silver2,
                PieceType::Knight => Piece::Knight2,
                PieceType::Lance => Piece::Lance2,
                PieceType::Pawn => Piece::Pawn2,
                PieceType::Dragon => Piece::Dragon2,
                PieceType::Horse => Piece::Horse2,
                PieceType::PromotedSilver => Piece::PromotedSilver2,
                PieceType::PromotedKnight => Piece::PromotedKnight2,
                PieceType::PromotedLance => Piece::PromotedLance2,
                PieceType::PromotedPawn => Piece::PromotedPawn2,
            },
        }
    }
}
/// コーディングを短くするためのものだぜ☆（＾～＾）
impl Piece {
    pub fn phase(self) -> Phase {
        match self {
            Piece::King1 => Phase::First,
            Piece::Rook1 => Phase::First,
            Piece::Bishop1 => Phase::First,
            Piece::Gold1 => Phase::First,
            Piece::Silver1 => Phase::First,
            Piece::Knight1 => Phase::First,
            Piece::Lance1 => Phase::First,
            Piece::Pawn1 => Phase::First,
            Piece::Dragon1 => Phase::First,
            Piece::Horse1 => Phase::First,
            Piece::PromotedSilver1 => Phase::First,
            Piece::PromotedKnight1 => Phase::First,
            Piece::PromotedLance1 => Phase::First,
            Piece::PromotedPawn1 => Phase::First,
            Piece::King2 => Phase::Second,
            Piece::Rook2 => Phase::Second,
            Piece::Bishop2 => Phase::Second,
            Piece::Gold2 => Phase::Second,
            Piece::Silver2 => Phase::Second,
            Piece::Knight2 => Phase::Second,
            Piece::Lance2 => Phase::Second,
            Piece::Pawn2 => Phase::Second,
            Piece::Dragon2 => Phase::Second,
            Piece::Horse2 => Phase::Second,
            Piece::PromotedSilver2 => Phase::Second,
            Piece::PromotedKnight2 => Phase::Second,
            Piece::PromotedLance2 => Phase::Second,
            Piece::PromotedPawn2 => Phase::Second,
        }
    }

    pub fn type_(self) -> PieceType {
        match self {
            Piece::King1 => PieceType::King,
            Piece::Rook1 => PieceType::Rook,
            Piece::Bishop1 => PieceType::Bishop,
            Piece::Gold1 => PieceType::Gold,
            Piece::Silver1 => PieceType::Silver,
            Piece::Knight1 => PieceType::Knight,
            Piece::Lance1 => PieceType::Lance,
            Piece::Pawn1 => PieceType::Pawn,
            Piece::Dragon1 => PieceType::Dragon,
            Piece::Horse1 => PieceType::Horse,
            Piece::PromotedSilver1 => PieceType::PromotedSilver,
            Piece::PromotedKnight1 => PieceType::PromotedKnight,
            Piece::PromotedLance1 => PieceType::PromotedLance,
            Piece::PromotedPawn1 => PieceType::PromotedPawn,
            Piece::King2 => PieceType::King,
            Piece::Rook2 => PieceType::Rook,
            Piece::Bishop2 => PieceType::Bishop,
            Piece::Gold2 => PieceType::Gold,
            Piece::Silver2 => PieceType::Silver,
            Piece::Knight2 => PieceType::Knight,
            Piece::Lance2 => PieceType::Lance,
            Piece::Pawn2 => PieceType::Pawn,
            Piece::Dragon2 => PieceType::Dragon,
            Piece::Horse2 => PieceType::Horse,
            Piece::PromotedSilver2 => PieceType::PromotedSilver,
            Piece::PromotedKnight2 => PieceType::PromotedKnight,
            Piece::PromotedLance2 => PieceType::PromotedLance,
            Piece::PromotedPawn2 => PieceType::PromotedPawn,
        }
    }

    /// 駒→成駒　（成れない駒は、そのまま）
    pub fn promoted(self) -> Self {
        match self {
            Piece::King1 => Piece::King1,
            Piece::Rook1 => Piece::Dragon1,
            Piece::Bishop1 => Piece::Horse1,
            Piece::Gold1 => Piece::Gold1,
            Piece::Silver1 => Piece::PromotedSilver1,
            Piece::Knight1 => Piece::PromotedKnight1,
            Piece::Lance1 => Piece::PromotedLance1,
            Piece::Pawn1 => Piece::PromotedPawn1,
            Piece::Dragon1 => Piece::Dragon1,
            Piece::Horse1 => Piece::Horse1,
            Piece::PromotedSilver1 => Piece::PromotedSilver1,
            Piece::PromotedKnight1 => Piece::PromotedKnight1,
            Piece::PromotedLance1 => Piece::PromotedLance1,
            Piece::PromotedPawn1 => Piece::PromotedPawn1,
            Piece::King2 => Piece::King2,
            Piece::Rook2 => Piece::Dragon2,
            Piece::Bishop2 => Piece::Horse2,
            Piece::Gold2 => Piece::Gold2,
            Piece::Silver2 => Piece::PromotedSilver2,
            Piece::Knight2 => Piece::PromotedKnight2,
            Piece::Lance2 => Piece::PromotedLance2,
            Piece::Pawn2 => Piece::PromotedPawn2,
            Piece::Dragon2 => Piece::Dragon2,
            Piece::Horse2 => Piece::Horse2,
            Piece::PromotedSilver2 => Piece::PromotedSilver2,
            Piece::PromotedKnight2 => Piece::PromotedKnight2,
            Piece::PromotedLance2 => Piece::PromotedLance2,
            Piece::PromotedPawn2 => Piece::PromotedPawn2,
        }
    }

    /// 成駒→駒　（成っていない駒は、そのまま）
    pub fn demoted(self) -> Self {
        match self {
            Piece::King1 => Piece::King1,
            Piece::Rook1 => Piece::Rook1,
            Piece::Bishop1 => Piece::Bishop1,
            Piece::Gold1 => Piece::Gold1,
            Piece::Silver1 => Piece::Silver1,
            Piece::Knight1 => Piece::Knight1,
            Piece::Lance1 => Piece::Lance1,
            Piece::Pawn1 => Piece::Pawn1,
            Piece::Dragon1 => Piece::Rook1,
            Piece::Horse1 => Piece::Bishop1,
            Piece::PromotedSilver1 => Piece::Silver1,
            Piece::PromotedKnight1 => Piece::Knight1,
            Piece::PromotedLance1 => Piece::Lance1,
            Piece::PromotedPawn1 => Piece::Pawn1,
            Piece::King2 => Piece::King2,
            Piece::Rook2 => Piece::Rook2,
            Piece::Bishop2 => Piece::Bishop2,
            Piece::Gold2 => Piece::Gold2,
            Piece::Silver2 => Piece::Silver2,
            Piece::Knight2 => Piece::Knight2,
            Piece::Lance2 => Piece::Lance2,
            Piece::Pawn2 => Piece::Pawn2,
            Piece::Dragon2 => Piece::Rook2,
            Piece::Horse2 => Piece::Bishop2,
            Piece::PromotedSilver2 => Piece::Silver2,
            Piece::PromotedKnight2 => Piece::Knight2,
            Piece::PromotedLance2 => Piece::Lance2,
            Piece::PromotedPawn2 => Piece::Pawn2,
        }
    }

    /// この駒を取ったら、先後が反転して、相手の駒になる、というリンクだぜ☆（＾～＾）
    /// 探索部では、玉のような取れない駒も　らいおんきゃっち　しているので、玉も取れるように作っておけだぜ☆（＾～＾）
    pub fn captured(self) -> Self {
        match self {
            Piece::King1 => Piece::King2,
            Piece::Rook1 => Piece::Rook2,
            Piece::Bishop1 => Piece::Bishop2,
            Piece::Gold1 => Piece::Gold2,
            Piece::Silver1 => Piece::Silver2,
            Piece::Knight1 => Piece::Knight2,
            Piece::Lance1 => Piece::Lance2,
            Piece::Pawn1 => Piece::Pawn2,
            Piece::Dragon1 => Piece::Rook2,
            Piece::Horse1 => Piece::Bishop2,
            Piece::PromotedSilver1 => Piece::Silver2,
            Piece::PromotedKnight1 => Piece::Knight2,
            Piece::PromotedLance1 => Piece::Lance2,
            Piece::PromotedPawn1 => Piece::Pawn2,
            Piece::King2 => Piece::King1,
            Piece::Rook2 => Piece::Rook1,
            Piece::Bishop2 => Piece::Bishop1,
            Piece::Gold2 => Piece::Gold1,
            Piece::Silver2 => Piece::Silver1,
            Piece::Knight2 => Piece::Knight1,
            Piece::Lance2 => Piece::Lance1,
            Piece::Pawn2 => Piece::Pawn1,
            Piece::Dragon2 => Piece::Rook1,
            Piece::Horse2 => Piece::Bishop1,
            Piece::PromotedSilver2 => Piece::Silver1,
            Piece::PromotedKnight2 => Piece::Knight1,
            Piece::PromotedLance2 => Piece::Lance1,
            Piece::PromotedPawn2 => Piece::Pawn1,
        }
    }

    pub fn double_faced_piece(self) -> DoubleFacedPiece {
        match self {
            Piece::King1 => DoubleFacedPiece::King1,
            Piece::Rook1 => DoubleFacedPiece::Rook1,
            Piece::Bishop1 => DoubleFacedPiece::Bishop1,
            Piece::Gold1 => DoubleFacedPiece::Gold1,
            Piece::Silver1 => DoubleFacedPiece::Silver1,
            Piece::Knight1 => DoubleFacedPiece::Knight1,
            Piece::Lance1 => DoubleFacedPiece::Lance1,
            Piece::Pawn1 => DoubleFacedPiece::Pawn1,
            Piece::Dragon1 => DoubleFacedPiece::Rook1,
            Piece::Horse1 => DoubleFacedPiece::Bishop1,
            Piece::PromotedSilver1 => DoubleFacedPiece::Silver1,
            Piece::PromotedKnight1 => DoubleFacedPiece::Knight1,
            Piece::PromotedLance1 => DoubleFacedPiece::Lance1,
            Piece::PromotedPawn1 => DoubleFacedPiece::Pawn1,
            Piece::King2 => DoubleFacedPiece::King2,
            Piece::Rook2 => DoubleFacedPiece::Rook2,
            Piece::Bishop2 => DoubleFacedPiece::Bishop2,
            Piece::Gold2 => DoubleFacedPiece::Gold2,
            Piece::Silver2 => DoubleFacedPiece::Silver2,
            Piece::Knight2 => DoubleFacedPiece::Knight2,
            Piece::Lance2 => DoubleFacedPiece::Lance2,
            Piece::Pawn2 => DoubleFacedPiece::Pawn2,
            Piece::Dragon2 => DoubleFacedPiece::Rook2,
            Piece::Horse2 => DoubleFacedPiece::Bishop2,
            Piece::PromotedSilver2 => DoubleFacedPiece::Silver2,
            Piece::PromotedKnight2 => DoubleFacedPiece::Knight2,
            Piece::PromotedLance2 => DoubleFacedPiece::Lance2,
            Piece::PromotedPawn2 => DoubleFacedPiece::Pawn2,
        }
    }
}
