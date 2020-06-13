//!
//! 駒 と 盤
//!
use crate::cosmic::fire::{Fire, FireAddress};
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

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl DoubleFacedPiece {
    pub fn nonpromoted_piece_hash_index(self) -> usize {
        (match self {
            DoubleFacedPiece::King1 => Piece::King1,
            DoubleFacedPiece::Rook1 => Piece::Rook1,
            DoubleFacedPiece::Bishop1 => Piece::Bishop1,
            DoubleFacedPiece::Gold1 => Piece::Gold1,
            DoubleFacedPiece::Silver1 => Piece::Silver1,
            DoubleFacedPiece::Knight1 => Piece::Knight1,
            DoubleFacedPiece::Lance1 => Piece::Lance1,
            DoubleFacedPiece::Pawn1 => Piece::Pawn1,
            DoubleFacedPiece::King2 => Piece::King2,
            DoubleFacedPiece::Rook2 => Piece::Rook2,
            DoubleFacedPiece::Bishop2 => Piece::Bishop2,
            DoubleFacedPiece::Gold2 => Piece::Gold2,
            DoubleFacedPiece::Silver2 => Piece::Silver2,
            DoubleFacedPiece::Knight2 => Piece::Knight2,
            DoubleFacedPiece::Lance2 => Piece::Lance2,
            DoubleFacedPiece::Pawn2 => Piece::Pawn2,
        }) as usize
    }
}

pub const PIECE_LEN: usize = 28;

/// toy_boxの中にカプセル化するぜ☆（＾～＾）
/// 先後付きの駒と空白。
/// 接尾辞の 1 は先手、 2 は後手。
///
/// Copy: 配列の要素の初期化のために利用。
#[derive(Copy, Clone, PartialEq, FromPrimitive)]
enum Piece {
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
    pub fn from_phase_and_piece_type(friend: Phase, piece_type: PieceType) -> Self {
        match friend {
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

/// ちゆり「駒そのものではなく、駒の情報が欲しいだけなら、これだぜ☆」
pub struct PieceInfo {
    pub piece: String,
    pub num: String,
}
impl PieceInfo {
    pub fn new(piece_display: String, num: PieceNum) -> Self {
        PieceInfo {
            piece: piece_display,
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
            // アンドゥだから盤にまだない。
            let piece_type = self.last_hand_type(&move2_val.destination).unwrap();

            // 取った方の駒台の先後に合わせるぜ☆（＾～＾）
            // 取った方の持ち駒を減らす
            let piece_num = {
                // TODO テスト中☆（＾～＾）
                let double_faced_piece = DoubleFacedPiece::from_phase_and_type(
                    friend,
                    piece_type.double_faced_piece_type(),
                );
                let fire1 = Fire::new_hand(friend, double_faced_piece.type_());
                self.pop_piece(&fire1).unwrap()
            };
            // 先後をひっくり返す。
            self.turn_phase(piece_num);
            if piece_type.promoted() {
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
            FireAddress::Hand(_drop_type) => {
                // 場所で指定します。
                // 台から取りのぞきます。
                let piece_num = self.phase_classification.pop(&fire);
                // TODO 背番号の番地に、ゴミ値を入れて消去するが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                self.address_list[piece_num as usize] = Fire::default();
                Some(piece_num)
            }
        }
    }

    /// 散らばっている駒に、背番号を付けて、駒台に置くぜ☆（＾～＾）
    pub fn init_hand(&mut self, friend: Phase, piece_type: PieceType) {
        // 駒に背番号を付けるぜ☆（＾～＾）
        let piece_num = self.numbering_piece(friend, piece_type);
        // 駒台に置くぜ☆（＾～＾）
        let drop = Fire::new_hand(
            self.get_phase(piece_num),
            self.get_double_faced_piece_type(piece_num),
        );
        self.push_piece(&drop, Some(piece_num));
    }

    /// 駒の新しい背番号を生成します。
    pub fn numbering_piece(&mut self, friend: Phase, piece_type: PieceType) -> PieceNum {
        let piece = Piece::from_phase_and_piece_type(friend, piece_type);
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
            if let Some(piece_num) =
                self.piece_num_at(&FireAddress::Board(AbsoluteAddress2D::new(file, rank)))
            {
                if self.get_phase(piece_num) == friend
                    && self.get_type(piece_num) == PieceType::Pawn
                {
                    return true;
                }
            }
        }
        false
    }
    /// ハッシュを作るときに利用。盤上専用。
    pub fn get_piece_board_hash_index(&self, addr: &FireAddress) -> Option<usize> {
        match addr {
            FireAddress::Board(sq) => {
                if let Some(piece_num) = self.board[sq.serial_number() as usize] {
                    Some(self.piece_list[piece_num as usize] as usize)
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
                    Some(PieceInfo::new(
                        format!("{}", self.piece_list[piece_num_val as usize]),
                        piece_num_val,
                    ))
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
                        .get_double_faced_piece_type(piece_num_val)
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
                    let piece = self.piece_list[piece_num as usize];
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
        F: FnMut(&Fire),
    {
        for piece_num in Nine299792458::piece_numbers().iter() {
            // 盤上の駒だけを調べようぜ☆（＾～＾）
            let fire = self.address_list[*piece_num as usize];
            match fire.address {
                FireAddress::Board(_sq) => {
                    if self.get_phase(*piece_num) == friend {
                        piece_get(&fire);
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
            if let Some(_piece_type) = self.last_hand_type(&Fire::new_hand(friend, drop.type_())) {
                // 有無を確認しているぜ☆（＾～＾）
                piece_get(&Fire::new_hand(drop.phase(), drop.type_()));
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
