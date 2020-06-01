//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
//!
//! 要は早引きのデータベースみたいなもんだな☆（＾～＾）
//!
//! 駒早見表 (PieceChart),
//! 駒種類早見表 (PieceTypeChart).
//!
use crate::cosmic::recording::Phase;
use crate::cosmic::recording::PHASE_LEN;
use crate::cosmic::smart::features::PHYSICAL_PIECES_LEN;
use crate::cosmic::smart::features::PHYSICAL_PIECE_TYPE_LEN;
use crate::cosmic::smart::features::PIECE_MEANING_LEN;
use crate::cosmic::smart::features::PIECE_TYPE_LEN;
use crate::cosmic::smart::features::{PhysicalPiece, PhysicalPieceType, PieceMeaning, PieceType};
use crate::cosmic::smart::square::{Angle, RelAdr, ANGLE_LEN};
use crate::cosmic::toy_box::PieceNum;
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
    piece_meaning_to_phase_table: [Phase; PIECE_MEANING_LEN],
    piece_meaning_type_table: [PieceType; PIECE_MEANING_LEN],
    /// 駒→成駒　（成れない駒は、そのまま）
    piece_meaning_promoted_table: [PieceMeaning; PIECE_MEANING_LEN],
    /// 成駒→駒　（成っていない駒は、そのまま）
    piece_meaning_demoted_table: [PieceMeaning; PIECE_MEANING_LEN],
    /// この駒を取ったら、先後が反転して、相手の駒になる、というリンクだぜ☆（＾～＾）
    /// 探索部では、玉のような取れない駒も　らいおんきゃっち　しているので、玉も取れるように作っておけだぜ☆（＾～＾）
    piece_meaning_captured_table: [PieceMeaning; PIECE_MEANING_LEN],
    piece_meaning_physical_table: [PhysicalPiece; PIECE_MEANING_LEN],

    /// 駒種類☆（＾～＾）
    piece_type_to_promoted_table: [bool; PIECE_TYPE_LEN],
    piece_type_to_mobility_table: [Vec<Mobility>; PIECE_TYPE_LEN],
    /// 持ち駒☆（＾～＾）
    /// 玉２枚引く☆（＾～＾）
    physical_pieces_legal_all: [PhysicalPiece; PHYSICAL_PIECES_LEN - 2],
    physical_pieces: [[PhysicalPiece; PHYSICAL_PIECE_TYPE_LEN]; PHASE_LEN],
    physical_piece_to_type_table: [PhysicalPieceType; PHYSICAL_PIECES_LEN],
    physical_piece_to_captured_value: [isize; PHYSICAL_PIECE_TYPE_LEN],

    // 相対番地と角度☆（＾～＾）
    west_ccw: [RelAdr; ANGLE_LEN],
    west_ccw_double_rank: [RelAdr; ANGLE_LEN],

    /// 点対称☆（＾～＾）
    rotate180: [Angle; ANGLE_LEN],

    /// 評価値☆（＾～＾）
    /// 成らないよりは、成った方がお得という、それだけの差を付けるだけの加点だぜ☆（＾～＾）
    /// 大きくすると、歩と交換に角が成り込むぜ☆（＾～＾）
    promotion_value: [isize; PHYSICAL_PIECE_TYPE_LEN],

    west: RelAdr,
}
impl Default for SpeedOfLight {
    fn default() -> Self {
        use crate::cosmic::recording::Phase::*;
        use crate::cosmic::smart::features::PieceMeaning::*;
        use crate::cosmic::smart::features::PieceType::*;
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
            piece_meaning_to_phase_table: [
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
            piece_meaning_type_table: [
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
            piece_meaning_promoted_table: [
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
            piece_meaning_demoted_table: [
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
            piece_meaning_captured_table: [
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
            piece_meaning_physical_table: [
                PhysicalPiece::King1,   // King1
                PhysicalPiece::Rook1,   // Rook1
                PhysicalPiece::Bishop1, // Bishop1
                PhysicalPiece::Gold1,   // Gold1
                PhysicalPiece::Silver1, // Silver1
                PhysicalPiece::Knight1, // Knight1
                PhysicalPiece::Lance1,  // Lance1
                PhysicalPiece::Pawn1,   // Pawn1
                PhysicalPiece::Rook1,   // Dragon1
                PhysicalPiece::Bishop1, // Horse1
                PhysicalPiece::Silver1, // PromotedSilver1
                PhysicalPiece::Knight1, // PromotedKnight1
                PhysicalPiece::Lance1,  // PromotedLance1
                PhysicalPiece::Pawn1,   // PromotedPawn1
                PhysicalPiece::King2,   // King2
                PhysicalPiece::Rook2,   // Rook2
                PhysicalPiece::Bishop2, // Bishop2
                PhysicalPiece::Gold2,   // Gold2
                PhysicalPiece::Silver2, // Silver2
                PhysicalPiece::Knight2, // Knight2
                PhysicalPiece::Lance2,  // Lance2
                PhysicalPiece::Pawn2,   // Pawn2
                PhysicalPiece::Rook2,   // Dragon2
                PhysicalPiece::Bishop2, // Horse2
                PhysicalPiece::Silver2, // PromotedSilver2
                PhysicalPiece::Knight2, // PromotedKnight2
                PhysicalPiece::Lance2,  // PromotedLance2
                PhysicalPiece::Pawn2,   // PromotedPawn2
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
            // 持ち駒☆（＾～＾）
            physical_pieces_legal_all: [
                PhysicalPiece::Rook1,
                PhysicalPiece::Bishop1,
                PhysicalPiece::Gold1,
                PhysicalPiece::Silver1,
                PhysicalPiece::Knight1,
                PhysicalPiece::Lance1,
                PhysicalPiece::Pawn1,
                PhysicalPiece::Rook2,
                PhysicalPiece::Bishop2,
                PhysicalPiece::Gold2,
                PhysicalPiece::Silver2,
                PhysicalPiece::Knight2,
                PhysicalPiece::Lance2,
                PhysicalPiece::Pawn2,
            ],
            physical_pieces: [
                [
                    PhysicalPiece::King1,
                    PhysicalPiece::Rook1,
                    PhysicalPiece::Bishop1,
                    PhysicalPiece::Gold1,
                    PhysicalPiece::Silver1,
                    PhysicalPiece::Knight1,
                    PhysicalPiece::Lance1,
                    PhysicalPiece::Pawn1,
                ],
                [
                    PhysicalPiece::King2,
                    PhysicalPiece::Rook2,
                    PhysicalPiece::Bishop2,
                    PhysicalPiece::Gold2,
                    PhysicalPiece::Silver2,
                    PhysicalPiece::Knight2,
                    PhysicalPiece::Lance2,
                    PhysicalPiece::Pawn2,
                ],
            ],

            physical_piece_to_type_table: [
                PhysicalPieceType::King,
                PhysicalPieceType::Rook,
                PhysicalPieceType::Bishop,
                PhysicalPieceType::Gold,
                PhysicalPieceType::Silver,
                PhysicalPieceType::Knight,
                PhysicalPieceType::Lance,
                PhysicalPieceType::Pawn,
                PhysicalPieceType::King,
                PhysicalPieceType::Rook,
                PhysicalPieceType::Bishop,
                PhysicalPieceType::Gold,
                PhysicalPieceType::Silver,
                PhysicalPieceType::Knight,
                PhysicalPieceType::Lance,
                PhysicalPieceType::Pawn,
            ],

            // よく使う、角度の付いた相対番地☆（＾～＾）
            west_ccw: [
                RelAdr::new(1, 0),
                RelAdr::new(1, 0).rotate(Angle::Ccw45).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw90).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw135).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw180).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw225).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw270).clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw315).clone(),
            ],
            /// 回転してからダブル・ランクしろだぜ☆（＾～＾）逆だと結果が違う☆（＾～＾）非可換の群、知ってるだろ☆ｍ９（＾～＾）ルービック・キューブと同じだぜ☆（＾～＾）
            west_ccw_double_rank: [
                RelAdr::new(1, 0).double_rank().clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw45).double_rank().clone(),
                RelAdr::new(1, 0).rotate(Angle::Ccw90).double_rank().clone(),
                RelAdr::new(1, 0)
                    .rotate(Angle::Ccw135)
                    .double_rank()
                    .clone(),
                RelAdr::new(1, 0)
                    .rotate(Angle::Ccw180)
                    .double_rank()
                    .clone(),
                RelAdr::new(1, 0)
                    .rotate(Angle::Ccw225)
                    .double_rank()
                    .clone(),
                RelAdr::new(1, 0)
                    .rotate(Angle::Ccw270)
                    .double_rank()
                    .clone(),
                RelAdr::new(1, 0)
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
            physical_piece_to_captured_value: [
                // 玉を取った時の評価は別にするから、ここではしないぜ☆（＾～＾）
                15000, // TODO 玉は 0 にしたい,
                // 駒割は取ったときにカウントしているので、成りを考慮しないぜ☆（＾～＾）
                1000, 900, 600, 500, 300, 200, 100,
            ],
            // 座標☆（＾～＾）
            west: RelAdr::new(1, 0),
        }
    }
}
/// コーディングを短くするためのものだぜ☆（＾～＾）
pub struct Nine299792458 {}
impl Nine299792458 {
    pub fn piece_numbers() -> &'static Vec<PieceNum> {
        &NINE_299792458.piece_numbers
    }
    pub fn west() -> RelAdr {
        NINE_299792458.west
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl PieceMeaning {
    pub fn phase(self) -> Phase {
        NINE_299792458.piece_meaning_to_phase_table[self as usize]
    }

    pub fn type_(self) -> PieceType {
        NINE_299792458.piece_meaning_type_table[self as usize]
    }

    pub fn promoted(self) -> PieceMeaning {
        NINE_299792458.piece_meaning_promoted_table[self as usize]
    }

    pub fn demoted(self) -> PieceMeaning {
        NINE_299792458.piece_meaning_demoted_table[self as usize]
    }

    pub fn captured(self) -> PieceMeaning {
        NINE_299792458.piece_meaning_captured_table[self as usize]
    }

    pub fn physical_piece(self) -> PhysicalPiece {
        NINE_299792458.piece_meaning_physical_table[self as usize]
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
}

/// 持駒種類
pub struct HandAddresses {}
impl HandAddresses {
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(PhysicalPiece),
    {
        for adr in &NINE_299792458.physical_pieces_legal_all {
            callback(*adr);
        }
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl PhysicalPiece {
    pub fn from_phase_and_type(phase: Phase, adr: PhysicalPieceType) -> Self {
        NINE_299792458.physical_pieces[phase as usize][adr as usize]
    }
    pub fn type_(self) -> PhysicalPieceType {
        NINE_299792458.physical_piece_to_type_table[self as usize]
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl PhysicalPieceType {
    pub fn promotion_value(self) -> isize {
        NINE_299792458.promotion_value[self as usize]
    }
    pub fn captured_value(self) -> isize {
        NINE_299792458.physical_piece_to_captured_value[self as usize]
    }
}

/// コーディングを短くするためのものだぜ☆（＾～＾）
impl Angle {
    /// 点対称☆（＾～＾）
    pub fn rotate180(self) -> Angle {
        NINE_299792458.rotate180[self as usize]
    }
    pub fn west_ccw_double_rank(self) -> RelAdr {
        NINE_299792458.west_ccw_double_rank[self as usize]
    }
    pub fn west_ccw(self) -> RelAdr {
        NINE_299792458.west_ccw[self as usize]
    }
}
