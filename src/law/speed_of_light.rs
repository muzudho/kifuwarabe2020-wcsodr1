//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
//!
//! 要は早引きのデータベースみたいなもんだな☆（＾～＾）
//!
//! 駒早見表 (PieceChart),
//! 駒種類早見表 (PieceTypeChart).
//!
use crate::cosmic::fire::{Fire, FireAddress};
use crate::cosmic::recording::AddressPos3;
use crate::cosmic::recording::Phase;
use crate::cosmic::recording::PHASE_LEN;
use crate::cosmic::smart::features::PHYSICAL_PIECES_LEN;
use crate::cosmic::smart::features::PHYSICAL_PIECE_TYPE_LEN;
use crate::cosmic::smart::features::PIECE_TYPE_LEN;
use crate::cosmic::smart::features::{DoubleFacedPiece, DoubleFacedPieceType, PieceType};
use crate::cosmic::smart::square::{Angle, RelAdr2D, ANGLE_LEN};
use crate::cosmic::toy_box::UnifiedAddress;
use crate::cosmic::toy_box::{Piece, PieceNum, SquareType, PIECE_LEN};
use crate::law::generate_move::{Agility, Mobility};

// グローバル定数
//
// 使い方（lazy_static!マクロ）
// ============================
// 定数の値を実行時に決めることができる。
//
// Cargo.toml に１行追記
// > [dependencies]
// > lazy_static = "1.0.0"
//
// main.rs の冒頭あたりに次の２行を記述
// > #[macro_use]
// > extern crate lazy_static;
//
// 「How can I use mutable lazy_static?」
// https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/3
lazy_static! {
    /// ９桁の有効数字☆（＾～＾）
    static ref NINE_299792458: SpeedOfLight = SpeedOfLight::default();
}

/// こいつが早引き表なわけだぜ☆（＾～＾）
struct SpeedOfLight {
    /// 駒構造体・マスター☆（＾～＾）イミュータブルなんでアクセッサなんか要らないぜ☆（＾～＾）
    piece_numbers: Vec<PieceNum>,

    /// 先後付きの駒☆（＾～＾）
    piece_to_phase_table: [Phase; PIECE_LEN],
    piece_type_table: [PieceType; PIECE_LEN],
    /// 駒→成駒　（成れない駒は、そのまま）
    piece_promoted_table: [Piece; PIECE_LEN],
    /// 成駒→駒　（成っていない駒は、そのまま）
    piece_demoted_table: [Piece; PIECE_LEN],
    /// この駒を取ったら、先後が反転して、相手の駒になる、というリンクだぜ☆（＾～＾）
    /// 探索部では、玉のような取れない駒も　らいおんきゃっち　しているので、玉も取れるように作っておけだぜ☆（＾～＾）
    piece_captured_table: [Piece; PIECE_LEN],
    piece_double_faced_table: [DoubleFacedPiece; PIECE_LEN],

    /// 駒種類☆（＾～＾）
    piece_type_to_promoted_table: [bool; PIECE_TYPE_LEN],
    piece_type_to_mobility_table: [Vec<Mobility>; PIECE_TYPE_LEN],
    piece_type_to_double_faced_piece_type: [DoubleFacedPieceType; PIECE_TYPE_LEN],
    /// 持ち駒☆（＾～＾）
    /// 玉２枚引く☆（＾～＾）
    double_faced_pieces_legal_all: [DoubleFacedPiece; PHYSICAL_PIECES_LEN - 2],
    double_faced_pieces: [[DoubleFacedPiece; PHYSICAL_PIECE_TYPE_LEN]; PHASE_LEN],
    double_faced_piece_to_phase_table: [Phase; PHYSICAL_PIECES_LEN],
    double_faced_piece_to_type_table: [DoubleFacedPieceType; PHYSICAL_PIECES_LEN],
    double_faced_piece_to_captured_value: [isize; PHYSICAL_PIECE_TYPE_LEN],
    double_faced_piece_to_nonpromoted_piece: [Piece; PHYSICAL_PIECES_LEN],

    // 相対番地と角度☆（＾～＾）
    west_ccw: [RelAdr2D; ANGLE_LEN],
    west_ccw_double_rank: [RelAdr2D; ANGLE_LEN],

    /// 点対称☆（＾～＾）
    rotate180: [Angle; ANGLE_LEN],

    /// 評価値☆（＾～＾）
    /// 成らないよりは、成った方がお得という、それだけの差を付けるだけの加点だぜ☆（＾～＾）
    /// 大きくすると、歩と交換に角が成り込むぜ☆（＾～＾）
    promotion_value: [isize; PHYSICAL_PIECE_TYPE_LEN],

    west: RelAdr2D,

    unified_address_to_address_pos3: [AddressPos3; 178],
    hand_legal_all: [Fire; PHYSICAL_PIECES_LEN - 2],
}
impl Default for SpeedOfLight {
    fn default() -> Self {
        use crate::cosmic::recording::Phase::*;
        use crate::cosmic::smart::features::PieceType::*;
        use crate::cosmic::toy_box::Piece::*;
        SpeedOfLight {
            /// ピースの早見表の生成は、アプリケーション開始時に全部済ませておけだぜ☆（＾～＾）
            piece_numbers: [
                PieceNum::King1,    // 1 先手玉
                PieceNum::King2,    // 2 後手玉
                PieceNum::Gold3,    // 3 金
                PieceNum::Gold4,    // 4 金
                PieceNum::Gold5,    // 5 金
                PieceNum::Gold6,    // 6 金
                PieceNum::Silver7,  // 7 銀
                PieceNum::Silver8,  // 8 銀
                PieceNum::Silver9,  // 9 銀
                PieceNum::Silver10, // 10 銀
                PieceNum::Knight11, // 11 桂
                PieceNum::Knight12, // 12 桂
                PieceNum::Knight13, // 13 桂
                PieceNum::Knight14, // 14 桂
                PieceNum::Lance15,  // 15 香
                PieceNum::Lance16,  // 16 香
                PieceNum::Lance17,  // 17 香
                PieceNum::Lance18,  // 18 香
                PieceNum::Bishop19, // 19 角
                PieceNum::Bishop20, // 20 角
                PieceNum::Rook21,   // 21 飛
                PieceNum::Rook22,   // 22 飛
                PieceNum::Pawn23,   // 23 歩
                PieceNum::Pawn24,   // 24 歩
                PieceNum::Pawn25,   // 25 歩
                PieceNum::Pawn26,   // 26 歩
                PieceNum::Pawn27,   // 27 歩
                PieceNum::Pawn28,   // 28 歩
                PieceNum::Pawn29,   // 29 歩
                PieceNum::Pawn30,   // 30 歩
                PieceNum::Pawn31,   // 31 歩
                PieceNum::Pawn32,   // 32 歩
                PieceNum::Pawn33,   // 33 歩
                PieceNum::Pawn34,   // 34 歩
                PieceNum::Pawn35,   // 35 歩
                PieceNum::Pawn36,   // 36 歩
                PieceNum::Pawn37,   // 37 歩
                PieceNum::Pawn38,   // 38 歩
                PieceNum::Pawn39,   // 39 歩
                PieceNum::Pawn40,   // 40 歩
            ]
            .to_vec(),

            /// 先後付きの駒☆（＾～＾）
            piece_to_phase_table: [
                First,  // King1
                First,  // Rook1
                First,  // Bishop1
                First,  // Gold1
                First,  // Silver1
                First,  // Knight1
                First,  // Lance1
                First,  // Pawn1
                First,  // Dragon1
                First,  // Horse1
                First,  // PromotedSilver1
                First,  // PromotedKnight1
                First,  // PromotedLance1
                First,  // PromotedPawn1
                Second, // King2
                Second, // Rook2
                Second, // Bishop2
                Second, // Gold2
                Second, // Silver2
                Second, // Knight2
                Second, // Lance2
                Second, // Pawn2
                Second, // Dragon2
                Second, // Horse2
                Second, // PromotedSilver2
                Second, // PromotedKnight2
                Second, // PromotedLance2
                Second, // PromotedPawn2
            ],
            piece_type_table: [
                King,           // King1
                Rook,           // Rook1
                Bishop,         // Bishop1
                Gold,           // Gold1
                Silver,         // Silver1
                Knight,         // Knight1
                Lance,          // Lance1
                Pawn,           // Pawn1
                Dragon,         // Dragon1
                Horse,          // Horse1
                PromotedSilver, // PromotedSilver1
                PromotedKnight, // PromotedKnight1
                PromotedLance,  // PromotedLance1
                PromotedPawn,   // PromotedPawn1
                King,           // King2
                Rook,           // Rook2
                Bishop,         // Bishop2
                Gold,           // Gold2
                Silver,         // Silver2
                Knight,         // Knight2
                Lance,          // Lance2
                Pawn,           // Pawn2
                Dragon,         // Dragon2
                Horse,          // Horse2
                PromotedSilver, // PromotedSilver2
                PromotedKnight, // PromotedKnight2
                PromotedLance,  // PromotedLance2
                PromotedPawn,   // PromotedPawn2
            ],
            piece_promoted_table: [
                King1,           // King1
                Dragon1,         // Rook1
                Horse1,          // Bishop1
                Gold1,           // Gold1
                PromotedSilver1, // Silver1
                PromotedKnight1, // Knight1
                PromotedLance1,  // Lance1
                PromotedPawn1,   // Pawn1
                Dragon1,         // Dragon1
                Horse1,          // Horse1
                PromotedSilver1, // PromotedSilver1
                PromotedKnight1, // PromotedKnight1
                PromotedLance1,  // PromotedLance1
                PromotedPawn1,   // PromotedPawn1
                King2,           // King2
                Dragon2,         // Rook2
                Horse2,          // Bishop2
                Gold2,           // Gold2
                PromotedSilver2, // Silver2
                PromotedKnight2, // Knight2
                PromotedLance2,  // Lance2
                PromotedPawn2,   // Pawn2
                Dragon2,         // Dragon2
                Horse2,          // Horse2
                PromotedSilver2, // PromotedSilver2
                PromotedKnight2, // PromotedKnight2
                PromotedLance2,  // PromotedLance2
                PromotedPawn2,   // PromotedPawn2
            ],
            piece_demoted_table: [
                King1,   // King1
                Rook1,   // Rook1
                Bishop1, // Bishop1
                Gold1,   // Gold1
                Silver1, // Silver1
                Knight1, // Knight1
                Lance1,  // Lance1
                Pawn1,   // Pawn1
                Rook1,   // Dragon1
                Bishop1, // Horse1
                Silver1, // PromotedSilver1
                Knight1, // PromotedKnight1
                Lance1,  // PromotedLance1
                Pawn1,   // PromotedPawn1
                King2,   // King2
                Rook2,   // Rook2
                Bishop2, // Bishop2
                Gold2,   // Gold2
                Silver2, // Silver2
                Knight2, // Knight2
                Lance2,  // Lance2
                Pawn2,   // Pawn2
                Rook2,   // Dragon2
                Bishop2, // Horse2
                Silver2, // PromotedSilver2
                Knight2, // PromotedKnight2
                Lance2,  // PromotedLance2
                Pawn2,   // PromotedPawn2
            ],
            piece_captured_table: [
                King2,   // King1
                Rook2,   // Rook1
                Bishop2, // Bishop1
                Gold2,   // Gold1
                Silver2, // Silver1
                Knight2, // Knight1
                Lance2,  // Lance1
                Pawn2,   // Pawn1
                Rook2,   // Dragon1
                Bishop2, // Horse1
                Silver2, // PromotedSilver1
                Knight2, // PromotedKnight1
                Lance2,  // PromotedLance1
                Pawn2,   // PromotedPawn1
                King1,   // King2
                Rook1,   // Rook2
                Bishop1, // Bishop2
                Gold1,   // Gold2
                Silver1, // Silver2
                Knight1, // Knight2
                Lance1,  // Lance2
                Pawn1,   // Pawn2
                Rook1,   // Dragon2
                Bishop1, // Horse2
                Silver1, // PromotedSilver2
                Knight1, // PromotedKnight2
                Lance1,  // PromotedLance2
                Pawn1,   // PromotedPawn2
            ],
            piece_double_faced_table: [
                DoubleFacedPiece::King1,   // King1
                DoubleFacedPiece::Rook1,   // Rook1
                DoubleFacedPiece::Bishop1, // Bishop1
                DoubleFacedPiece::Gold1,   // Gold1
                DoubleFacedPiece::Silver1, // Silver1
                DoubleFacedPiece::Knight1, // Knight1
                DoubleFacedPiece::Lance1,  // Lance1
                DoubleFacedPiece::Pawn1,   // Pawn1
                DoubleFacedPiece::Rook1,   // Dragon1
                DoubleFacedPiece::Bishop1, // Horse1
                DoubleFacedPiece::Silver1, // PromotedSilver1
                DoubleFacedPiece::Knight1, // PromotedKnight1
                DoubleFacedPiece::Lance1,  // PromotedLance1
                DoubleFacedPiece::Pawn1,   // PromotedPawn1
                DoubleFacedPiece::King2,   // King2
                DoubleFacedPiece::Rook2,   // Rook2
                DoubleFacedPiece::Bishop2, // Bishop2
                DoubleFacedPiece::Gold2,   // Gold2
                DoubleFacedPiece::Silver2, // Silver2
                DoubleFacedPiece::Knight2, // Knight2
                DoubleFacedPiece::Lance2,  // Lance2
                DoubleFacedPiece::Pawn2,   // Pawn2
                DoubleFacedPiece::Rook2,   // Dragon2
                DoubleFacedPiece::Bishop2, // Horse2
                DoubleFacedPiece::Silver2, // PromotedSilver2
                DoubleFacedPiece::Knight2, // PromotedKnight2
                DoubleFacedPiece::Lance2,  // PromotedLance2
                DoubleFacedPiece::Pawn2,   // PromotedPawn2
            ],

            // 成り駒か☆（＾～＾）？
            piece_type_to_promoted_table: [
                false, // King
                false, // Rook
                false, // Bishop
                false, // Gold
                false, // Silver
                false, // Knight
                false, // Lance
                false, // Pawn
                true,  // Dragon
                true,  // Horse
                true,  // PromotedSilver
                true,  // PromotedKnight
                true,  // PromotedLance
                true,  // PromotedPawn
            ],
            piece_type_to_mobility_table: [
                vec![
                    Mobility::new(Angle::Ccw0, Agility::Hopping),
                    Mobility::new(Angle::Ccw45, Agility::Hopping),
                    Mobility::new(Angle::Ccw90, Agility::Hopping),
                    Mobility::new(Angle::Ccw135, Agility::Hopping),
                    Mobility::new(Angle::Ccw180, Agility::Hopping),
                    Mobility::new(Angle::Ccw225, Agility::Hopping),
                    Mobility::new(Angle::Ccw270, Agility::Hopping),
                    Mobility::new(Angle::Ccw315, Agility::Hopping),
                ], // King
                vec![
                    Mobility::new(Angle::Ccw0, Agility::Sliding),
                    Mobility::new(Angle::Ccw90, Agility::Sliding),
                    Mobility::new(Angle::Ccw180, Agility::Sliding),
                    Mobility::new(Angle::Ccw270, Agility::Sliding),
                ], // Rook
                vec![
                    Mobility::new(Angle::Ccw45, Agility::Sliding),
                    Mobility::new(Angle::Ccw135, Agility::Sliding),
                    Mobility::new(Angle::Ccw225, Agility::Sliding),
                    Mobility::new(Angle::Ccw315, Agility::Sliding),
                ], // Bishop
                vec![
                    Mobility::new(Angle::Ccw270, Agility::Hopping),
                    Mobility::new(Angle::Ccw315, Agility::Hopping),
                    Mobility::new(Angle::Ccw0, Agility::Hopping),
                    Mobility::new(Angle::Ccw90, Agility::Hopping),
                    Mobility::new(Angle::Ccw180, Agility::Hopping),
                    Mobility::new(Angle::Ccw225, Agility::Hopping),
                ], // Gold
                vec![
                    Mobility::new(Angle::Ccw270, Agility::Hopping),
                    Mobility::new(Angle::Ccw315, Agility::Hopping),
                    Mobility::new(Angle::Ccw45, Agility::Hopping),
                    Mobility::new(Angle::Ccw135, Agility::Hopping),
                    Mobility::new(Angle::Ccw225, Agility::Hopping),
                ], // Silver
                vec![
                    Mobility::new(Angle::Ccw225, Agility::Knight),
                    Mobility::new(Angle::Ccw315, Agility::Knight),
                ], // Knight
                vec![Mobility::new(Angle::Ccw270, Agility::Sliding)], // Lance
                vec![Mobility::new(Angle::Ccw270, Agility::Hopping)], // Pawn
                vec![
                    Mobility::new(Angle::Ccw0, Agility::Sliding),
                    Mobility::new(Angle::Ccw90, Agility::Sliding),
                    Mobility::new(Angle::Ccw180, Agility::Sliding),
                    Mobility::new(Angle::Ccw270, Agility::Sliding),
                    Mobility::new(Angle::Ccw45, Agility::Hopping),
                    Mobility::new(Angle::Ccw135, Agility::Hopping),
                    Mobility::new(Angle::Ccw225, Agility::Hopping),
                    Mobility::new(Angle::Ccw315, Agility::Hopping),
                ], // Dragon
                vec![
                    Mobility::new(Angle::Ccw0, Agility::Hopping),
                    Mobility::new(Angle::Ccw90, Agility::Hopping),
                    Mobility::new(Angle::Ccw180, Agility::Hopping),
                    Mobility::new(Angle::Ccw270, Agility::Hopping),
                    Mobility::new(Angle::Ccw45, Agility::Sliding),
                    Mobility::new(Angle::Ccw135, Agility::Sliding),
                    Mobility::new(Angle::Ccw225, Agility::Sliding),
                    Mobility::new(Angle::Ccw315, Agility::Sliding),
                ], // Horse
                vec![
                    Mobility::new(Angle::Ccw270, Agility::Hopping),
                    Mobility::new(Angle::Ccw315, Agility::Hopping),
                    Mobility::new(Angle::Ccw0, Agility::Hopping),
                    Mobility::new(Angle::Ccw90, Agility::Hopping),
                    Mobility::new(Angle::Ccw180, Agility::Hopping),
                    Mobility::new(Angle::Ccw225, Agility::Hopping),
                ], // PromotedSilver (Same gold)
                vec![
                    Mobility::new(Angle::Ccw270, Agility::Hopping),
                    Mobility::new(Angle::Ccw315, Agility::Hopping),
                    Mobility::new(Angle::Ccw0, Agility::Hopping),
                    Mobility::new(Angle::Ccw90, Agility::Hopping),
                    Mobility::new(Angle::Ccw180, Agility::Hopping),
                    Mobility::new(Angle::Ccw225, Agility::Hopping),
                ], // PromotedKnight
                vec![
                    Mobility::new(Angle::Ccw270, Agility::Hopping),
                    Mobility::new(Angle::Ccw315, Agility::Hopping),
                    Mobility::new(Angle::Ccw0, Agility::Hopping),
                    Mobility::new(Angle::Ccw90, Agility::Hopping),
                    Mobility::new(Angle::Ccw180, Agility::Hopping),
                    Mobility::new(Angle::Ccw225, Agility::Hopping),
                ], // PromotedLance
                vec![
                    Mobility::new(Angle::Ccw270, Agility::Hopping),
                    Mobility::new(Angle::Ccw315, Agility::Hopping),
                    Mobility::new(Angle::Ccw0, Agility::Hopping),
                    Mobility::new(Angle::Ccw90, Agility::Hopping),
                    Mobility::new(Angle::Ccw180, Agility::Hopping),
                    Mobility::new(Angle::Ccw225, Agility::Hopping),
                ], // PromotedPawn
            ],
            piece_type_to_double_faced_piece_type: [
                DoubleFacedPieceType::King,   // King
                DoubleFacedPieceType::Rook,   // Rook
                DoubleFacedPieceType::Bishop, // Bishop
                DoubleFacedPieceType::Gold,   // Gold
                DoubleFacedPieceType::Silver, // Silver
                DoubleFacedPieceType::Knight, // Knight
                DoubleFacedPieceType::Lance,  // Lance
                DoubleFacedPieceType::Pawn,   // Pawn
                DoubleFacedPieceType::Rook,   // Dragon
                DoubleFacedPieceType::Bishop, // Horse
                DoubleFacedPieceType::Silver, // PromotedSilver
                DoubleFacedPieceType::Knight, // PromotedKnight
                DoubleFacedPieceType::Lance,  // PromotedLance
                DoubleFacedPieceType::Pawn,   // PromotedPawn
            ],
            // 持ち駒☆（＾～＾）
            double_faced_pieces_legal_all: [
                DoubleFacedPiece::Rook1,
                DoubleFacedPiece::Bishop1,
                DoubleFacedPiece::Gold1,
                DoubleFacedPiece::Silver1,
                DoubleFacedPiece::Knight1,
                DoubleFacedPiece::Lance1,
                DoubleFacedPiece::Pawn1,
                DoubleFacedPiece::Rook2,
                DoubleFacedPiece::Bishop2,
                DoubleFacedPiece::Gold2,
                DoubleFacedPiece::Silver2,
                DoubleFacedPiece::Knight2,
                DoubleFacedPiece::Lance2,
                DoubleFacedPiece::Pawn2,
            ],
            double_faced_pieces: [
                [
                    DoubleFacedPiece::King1,
                    DoubleFacedPiece::Rook1,
                    DoubleFacedPiece::Bishop1,
                    DoubleFacedPiece::Gold1,
                    DoubleFacedPiece::Silver1,
                    DoubleFacedPiece::Knight1,
                    DoubleFacedPiece::Lance1,
                    DoubleFacedPiece::Pawn1,
                ],
                [
                    DoubleFacedPiece::King2,
                    DoubleFacedPiece::Rook2,
                    DoubleFacedPiece::Bishop2,
                    DoubleFacedPiece::Gold2,
                    DoubleFacedPiece::Silver2,
                    DoubleFacedPiece::Knight2,
                    DoubleFacedPiece::Lance2,
                    DoubleFacedPiece::Pawn2,
                ],
            ],

            double_faced_piece_to_phase_table: [
                Phase::First,  // King1
                Phase::First,  // Rook1
                Phase::First,  // Bishop1
                Phase::First,  // Gold1
                Phase::First,  // Silver1
                Phase::First,  // Knight1
                Phase::First,  // Lance1
                Phase::First,  // Pawn1
                Phase::Second, // King2
                Phase::Second, // Rook2
                Phase::Second, // Bishop2
                Phase::Second, // Gold2
                Phase::Second, // Silver2
                Phase::Second, // Knight2
                Phase::Second, // Lance2
                Phase::Second, // Pawn2
            ],

            double_faced_piece_to_type_table: [
                DoubleFacedPieceType::King,
                DoubleFacedPieceType::Rook,
                DoubleFacedPieceType::Bishop,
                DoubleFacedPieceType::Gold,
                DoubleFacedPieceType::Silver,
                DoubleFacedPieceType::Knight,
                DoubleFacedPieceType::Lance,
                DoubleFacedPieceType::Pawn,
                DoubleFacedPieceType::King,
                DoubleFacedPieceType::Rook,
                DoubleFacedPieceType::Bishop,
                DoubleFacedPieceType::Gold,
                DoubleFacedPieceType::Silver,
                DoubleFacedPieceType::Knight,
                DoubleFacedPieceType::Lance,
                DoubleFacedPieceType::Pawn,
            ],

            double_faced_piece_to_nonpromoted_piece: [
                Piece::King1,
                Piece::Rook1,
                Piece::Bishop1,
                Piece::Gold1,
                Piece::Silver1,
                Piece::Knight1,
                Piece::Lance1,
                Piece::Pawn1,
                Piece::King2,
                Piece::Rook2,
                Piece::Bishop2,
                Piece::Gold2,
                Piece::Silver2,
                Piece::Knight2,
                Piece::Lance2,
                Piece::Pawn2,
            ],

            // よく使う、角度の付いた相対番地☆（＾～＾）
            west_ccw: [
                RelAdr2D::new(1, 0),
                RelAdr2D::new(1, 0).rotate(Angle::Ccw45).clone(),
                RelAdr2D::new(1, 0).rotate(Angle::Ccw90).clone(),
                RelAdr2D::new(1, 0).rotate(Angle::Ccw135).clone(),
                RelAdr2D::new(1, 0).rotate(Angle::Ccw180).clone(),
                RelAdr2D::new(1, 0).rotate(Angle::Ccw225).clone(),
                RelAdr2D::new(1, 0).rotate(Angle::Ccw270).clone(),
                RelAdr2D::new(1, 0).rotate(Angle::Ccw315).clone(),
            ],
            /// 回転してからダブル・ランクしろだぜ☆（＾～＾）逆だと結果が違う☆（＾～＾）非可換の群、知ってるだろ☆ｍ９（＾～＾）ルービック・キューブと同じだぜ☆（＾～＾）
            west_ccw_double_rank: [
                RelAdr2D::new(1, 0).double_rank().clone(),
                RelAdr2D::new(1, 0)
                    .rotate(Angle::Ccw45)
                    .double_rank()
                    .clone(),
                RelAdr2D::new(1, 0)
                    .rotate(Angle::Ccw90)
                    .double_rank()
                    .clone(),
                RelAdr2D::new(1, 0)
                    .rotate(Angle::Ccw135)
                    .double_rank()
                    .clone(),
                RelAdr2D::new(1, 0)
                    .rotate(Angle::Ccw180)
                    .double_rank()
                    .clone(),
                RelAdr2D::new(1, 0)
                    .rotate(Angle::Ccw225)
                    .double_rank()
                    .clone(),
                RelAdr2D::new(1, 0)
                    .rotate(Angle::Ccw270)
                    .double_rank()
                    .clone(),
                RelAdr2D::new(1, 0)
                    .rotate(Angle::Ccw315)
                    .double_rank()
                    .clone(),
            ],

            rotate180: [
                Angle::Ccw180,
                Angle::Ccw225,
                Angle::Ccw270,
                Angle::Ccw315,
                Angle::Ccw0,
                Angle::Ccw45,
                Angle::Ccw90,
                Angle::Ccw135,
            ],

            // 評価値☆（＾～＾）
            promotion_value: [0, 1, 1, 0, 0, 1, 1, 1],
            double_faced_piece_to_captured_value: [
                // 玉を取った時の評価は別にするから、ここではしないぜ☆（＾～＾）
                15000, // TODO 玉は 0 にしたい,
                // 駒割は取ったときにカウントしているので、成りを考慮しないぜ☆（＾～＾）
                1000, 900, 600, 500, 300, 200, 100,
            ],
            // 座標☆（＾～＾）
            west: RelAdr2D::new(1, 0),

            unified_address_to_address_pos3: [
                AddressPos3::FirstBoard(SquareType::Sq11),
                AddressPos3::FirstBoard(SquareType::Sq12),
                AddressPos3::FirstBoard(SquareType::Sq13),
                AddressPos3::FirstBoard(SquareType::Sq14),
                AddressPos3::FirstBoard(SquareType::Sq15),
                AddressPos3::FirstBoard(SquareType::Sq16),
                AddressPos3::FirstBoard(SquareType::Sq17),
                AddressPos3::FirstBoard(SquareType::Sq18),
                AddressPos3::FirstBoard(SquareType::Sq19),
                AddressPos3::FirstBoard(SquareType::Sq21),
                AddressPos3::FirstBoard(SquareType::Sq22),
                AddressPos3::FirstBoard(SquareType::Sq23),
                AddressPos3::FirstBoard(SquareType::Sq24),
                AddressPos3::FirstBoard(SquareType::Sq25),
                AddressPos3::FirstBoard(SquareType::Sq26),
                AddressPos3::FirstBoard(SquareType::Sq27),
                AddressPos3::FirstBoard(SquareType::Sq28),
                AddressPos3::FirstBoard(SquareType::Sq29),
                AddressPos3::FirstBoard(SquareType::Sq31),
                AddressPos3::FirstBoard(SquareType::Sq32),
                AddressPos3::FirstBoard(SquareType::Sq33),
                AddressPos3::FirstBoard(SquareType::Sq34),
                AddressPos3::FirstBoard(SquareType::Sq35),
                AddressPos3::FirstBoard(SquareType::Sq36),
                AddressPos3::FirstBoard(SquareType::Sq37),
                AddressPos3::FirstBoard(SquareType::Sq38),
                AddressPos3::FirstBoard(SquareType::Sq39),
                AddressPos3::FirstBoard(SquareType::Sq41),
                AddressPos3::FirstBoard(SquareType::Sq42),
                AddressPos3::FirstBoard(SquareType::Sq43),
                AddressPos3::FirstBoard(SquareType::Sq44),
                AddressPos3::FirstBoard(SquareType::Sq45),
                AddressPos3::FirstBoard(SquareType::Sq46),
                AddressPos3::FirstBoard(SquareType::Sq47),
                AddressPos3::FirstBoard(SquareType::Sq48),
                AddressPos3::FirstBoard(SquareType::Sq49),
                AddressPos3::FirstBoard(SquareType::Sq51),
                AddressPos3::FirstBoard(SquareType::Sq52),
                AddressPos3::FirstBoard(SquareType::Sq53),
                AddressPos3::FirstBoard(SquareType::Sq54),
                AddressPos3::FirstBoard(SquareType::Sq55),
                AddressPos3::FirstBoard(SquareType::Sq56),
                AddressPos3::FirstBoard(SquareType::Sq57),
                AddressPos3::FirstBoard(SquareType::Sq58),
                AddressPos3::FirstBoard(SquareType::Sq59),
                AddressPos3::FirstBoard(SquareType::Sq61),
                AddressPos3::FirstBoard(SquareType::Sq62),
                AddressPos3::FirstBoard(SquareType::Sq63),
                AddressPos3::FirstBoard(SquareType::Sq64),
                AddressPos3::FirstBoard(SquareType::Sq65),
                AddressPos3::FirstBoard(SquareType::Sq66),
                AddressPos3::FirstBoard(SquareType::Sq67),
                AddressPos3::FirstBoard(SquareType::Sq68),
                AddressPos3::FirstBoard(SquareType::Sq69),
                AddressPos3::FirstBoard(SquareType::Sq71),
                AddressPos3::FirstBoard(SquareType::Sq72),
                AddressPos3::FirstBoard(SquareType::Sq73),
                AddressPos3::FirstBoard(SquareType::Sq74),
                AddressPos3::FirstBoard(SquareType::Sq75),
                AddressPos3::FirstBoard(SquareType::Sq76),
                AddressPos3::FirstBoard(SquareType::Sq77),
                AddressPos3::FirstBoard(SquareType::Sq78),
                AddressPos3::FirstBoard(SquareType::Sq79),
                AddressPos3::FirstBoard(SquareType::Sq81),
                AddressPos3::FirstBoard(SquareType::Sq82),
                AddressPos3::FirstBoard(SquareType::Sq83),
                AddressPos3::FirstBoard(SquareType::Sq84),
                AddressPos3::FirstBoard(SquareType::Sq85),
                AddressPos3::FirstBoard(SquareType::Sq86),
                AddressPos3::FirstBoard(SquareType::Sq87),
                AddressPos3::FirstBoard(SquareType::Sq88),
                AddressPos3::FirstBoard(SquareType::Sq89),
                AddressPos3::FirstBoard(SquareType::Sq91),
                AddressPos3::FirstBoard(SquareType::Sq92),
                AddressPos3::FirstBoard(SquareType::Sq93),
                AddressPos3::FirstBoard(SquareType::Sq94),
                AddressPos3::FirstBoard(SquareType::Sq95),
                AddressPos3::FirstBoard(SquareType::Sq96),
                AddressPos3::FirstBoard(SquareType::Sq97),
                AddressPos3::FirstBoard(SquareType::Sq98),
                AddressPos3::FirstBoard(SquareType::Sq99),
                AddressPos3::SecondBoard(SquareType::Sq11),
                AddressPos3::SecondBoard(SquareType::Sq12),
                AddressPos3::SecondBoard(SquareType::Sq13),
                AddressPos3::SecondBoard(SquareType::Sq14),
                AddressPos3::SecondBoard(SquareType::Sq15),
                AddressPos3::SecondBoard(SquareType::Sq16),
                AddressPos3::SecondBoard(SquareType::Sq17),
                AddressPos3::SecondBoard(SquareType::Sq18),
                AddressPos3::SecondBoard(SquareType::Sq19),
                AddressPos3::SecondBoard(SquareType::Sq21),
                AddressPos3::SecondBoard(SquareType::Sq22),
                AddressPos3::SecondBoard(SquareType::Sq23),
                AddressPos3::SecondBoard(SquareType::Sq24),
                AddressPos3::SecondBoard(SquareType::Sq25),
                AddressPos3::SecondBoard(SquareType::Sq26),
                AddressPos3::SecondBoard(SquareType::Sq27),
                AddressPos3::SecondBoard(SquareType::Sq28),
                AddressPos3::SecondBoard(SquareType::Sq29),
                AddressPos3::SecondBoard(SquareType::Sq31),
                AddressPos3::SecondBoard(SquareType::Sq32),
                AddressPos3::SecondBoard(SquareType::Sq33),
                AddressPos3::SecondBoard(SquareType::Sq34),
                AddressPos3::SecondBoard(SquareType::Sq35),
                AddressPos3::SecondBoard(SquareType::Sq36),
                AddressPos3::SecondBoard(SquareType::Sq37),
                AddressPos3::SecondBoard(SquareType::Sq38),
                AddressPos3::SecondBoard(SquareType::Sq39),
                AddressPos3::SecondBoard(SquareType::Sq41),
                AddressPos3::SecondBoard(SquareType::Sq42),
                AddressPos3::SecondBoard(SquareType::Sq43),
                AddressPos3::SecondBoard(SquareType::Sq44),
                AddressPos3::SecondBoard(SquareType::Sq45),
                AddressPos3::SecondBoard(SquareType::Sq46),
                AddressPos3::SecondBoard(SquareType::Sq47),
                AddressPos3::SecondBoard(SquareType::Sq48),
                AddressPos3::SecondBoard(SquareType::Sq49),
                AddressPos3::SecondBoard(SquareType::Sq51),
                AddressPos3::SecondBoard(SquareType::Sq52),
                AddressPos3::SecondBoard(SquareType::Sq53),
                AddressPos3::SecondBoard(SquareType::Sq54),
                AddressPos3::SecondBoard(SquareType::Sq55),
                AddressPos3::SecondBoard(SquareType::Sq56),
                AddressPos3::SecondBoard(SquareType::Sq57),
                AddressPos3::SecondBoard(SquareType::Sq58),
                AddressPos3::SecondBoard(SquareType::Sq59),
                AddressPos3::SecondBoard(SquareType::Sq61),
                AddressPos3::SecondBoard(SquareType::Sq62),
                AddressPos3::SecondBoard(SquareType::Sq63),
                AddressPos3::SecondBoard(SquareType::Sq64),
                AddressPos3::SecondBoard(SquareType::Sq65),
                AddressPos3::SecondBoard(SquareType::Sq66),
                AddressPos3::SecondBoard(SquareType::Sq67),
                AddressPos3::SecondBoard(SquareType::Sq68),
                AddressPos3::SecondBoard(SquareType::Sq69),
                AddressPos3::SecondBoard(SquareType::Sq71),
                AddressPos3::SecondBoard(SquareType::Sq72),
                AddressPos3::SecondBoard(SquareType::Sq73),
                AddressPos3::SecondBoard(SquareType::Sq74),
                AddressPos3::SecondBoard(SquareType::Sq75),
                AddressPos3::SecondBoard(SquareType::Sq76),
                AddressPos3::SecondBoard(SquareType::Sq77),
                AddressPos3::SecondBoard(SquareType::Sq78),
                AddressPos3::SecondBoard(SquareType::Sq79),
                AddressPos3::SecondBoard(SquareType::Sq81),
                AddressPos3::SecondBoard(SquareType::Sq82),
                AddressPos3::SecondBoard(SquareType::Sq83),
                AddressPos3::SecondBoard(SquareType::Sq84),
                AddressPos3::SecondBoard(SquareType::Sq85),
                AddressPos3::SecondBoard(SquareType::Sq86),
                AddressPos3::SecondBoard(SquareType::Sq87),
                AddressPos3::SecondBoard(SquareType::Sq88),
                AddressPos3::SecondBoard(SquareType::Sq89),
                AddressPos3::SecondBoard(SquareType::Sq91),
                AddressPos3::SecondBoard(SquareType::Sq92),
                AddressPos3::SecondBoard(SquareType::Sq93),
                AddressPos3::SecondBoard(SquareType::Sq94),
                AddressPos3::SecondBoard(SquareType::Sq95),
                AddressPos3::SecondBoard(SquareType::Sq96),
                AddressPos3::SecondBoard(SquareType::Sq97),
                AddressPos3::SecondBoard(SquareType::Sq98),
                AddressPos3::SecondBoard(SquareType::Sq99),
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
            ],
            // 持ち駒☆（＾～＾）
            hand_legal_all: [
                Fire::new_hand(Phase::First, DoubleFacedPieceType::Rook),
                Fire::new_hand(Phase::First, DoubleFacedPieceType::Bishop),
                Fire::new_hand(Phase::First, DoubleFacedPieceType::Gold),
                Fire::new_hand(Phase::First, DoubleFacedPieceType::Silver),
                Fire::new_hand(Phase::First, DoubleFacedPieceType::Knight),
                Fire::new_hand(Phase::First, DoubleFacedPieceType::Lance),
                Fire::new_hand(Phase::First, DoubleFacedPieceType::Pawn),
                Fire::new_hand(Phase::Second, DoubleFacedPieceType::Rook),
                Fire::new_hand(Phase::Second, DoubleFacedPieceType::Bishop),
                Fire::new_hand(Phase::Second, DoubleFacedPieceType::Gold),
                Fire::new_hand(Phase::Second, DoubleFacedPieceType::Silver),
                Fire::new_hand(Phase::Second, DoubleFacedPieceType::Knight),
                Fire::new_hand(Phase::Second, DoubleFacedPieceType::Lance),
                Fire::new_hand(Phase::Second, DoubleFacedPieceType::Pawn),
            ],
        }
    }
}
/// コーディングを短くするためのものだぜ☆（＾～＾）
pub struct Nine299792458 {}
impl Nine299792458 {
    pub fn piece_numbers() -> &'static Vec<PieceNum> {
        &NINE_299792458.piece_numbers
    }
    pub fn west() -> RelAdr2D {
        NINE_299792458.west
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl Piece {
    pub fn phase(self) -> Phase {
        NINE_299792458.piece_to_phase_table[self as usize]
    }

    pub fn type_(self) -> PieceType {
        NINE_299792458.piece_type_table[self as usize]
    }

    pub fn promoted(self) -> Piece {
        NINE_299792458.piece_promoted_table[self as usize]
    }

    pub fn demoted(self) -> Piece {
        NINE_299792458.piece_demoted_table[self as usize]
    }

    pub fn captured(self) -> Piece {
        NINE_299792458.piece_captured_table[self as usize]
    }

    pub fn double_faced_piece(self) -> DoubleFacedPiece {
        NINE_299792458.piece_double_faced_table[self as usize]
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl PieceType {
    pub fn promoted(self) -> bool {
        NINE_299792458.piece_type_to_promoted_table[self as usize]
    }
    pub fn mobility(self) -> &'static Vec<Mobility> {
        &NINE_299792458.piece_type_to_mobility_table[self as usize]
    }
    pub fn double_faced_piece_type(self) -> DoubleFacedPieceType {
        NINE_299792458.piece_type_to_double_faced_piece_type[self as usize]
    }
}

/// 持駒種類
pub struct HandAddresses {}
impl HandAddresses {
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(&Fire),
    {
        for fire in &NINE_299792458.hand_legal_all {
            callback(fire);
        }
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl DoubleFacedPiece {
    pub fn from_phase_and_type(phase: Phase, adr: DoubleFacedPieceType) -> Self {
        NINE_299792458.double_faced_pieces[phase as usize][adr as usize]
    }
    pub fn phase(self) -> Phase {
        NINE_299792458.double_faced_piece_to_phase_table[self as usize]
    }
    pub fn type_(self) -> DoubleFacedPieceType {
        NINE_299792458.double_faced_piece_to_type_table[self as usize]
    }
    pub fn nonpromoted_piece(self) -> Piece {
        NINE_299792458.double_faced_piece_to_nonpromoted_piece[self as usize]
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl DoubleFacedPieceType {
    pub fn promotion_value(self) -> isize {
        NINE_299792458.promotion_value[self as usize]
    }
    pub fn captured_value(self) -> isize {
        NINE_299792458.double_faced_piece_to_captured_value[self as usize]
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl Angle {
    /// 点対称☆（＾～＾）
    pub fn rotate180(self) -> Angle {
        NINE_299792458.rotate180[self as usize]
    }
    pub fn west_ccw_double_rank(self) -> RelAdr2D {
        NINE_299792458.west_ccw_double_rank[self as usize]
    }
    pub fn west_ccw(self) -> RelAdr2D {
        NINE_299792458.west_ccw[self as usize]
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl UnifiedAddress {
    pub fn to_address_pos3(self) -> AddressPos3 {
        NINE_299792458.unified_address_to_address_pos3[self as usize]
    }
}
