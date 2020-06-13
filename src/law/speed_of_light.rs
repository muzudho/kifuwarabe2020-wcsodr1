//! 光速は定義☆（＾～＾）
//! 299,792,458 m/s (metre per second)
//! ニクク,ナクフタリ,ヨレバイツモハッピー
//!
//! 要は早引きのデータベースみたいなもんだな☆（＾～＾）
//!
//! 駒早見表 (PieceChart),
//! 駒種類早見表 (PieceTypeChart).
//!
use crate::cosmic::fire::Fire;
use crate::cosmic::recording::Phase;
use crate::cosmic::recording::PHASE_LEN;
use crate::cosmic::smart::features::PHYSICAL_PIECES_LEN;
use crate::cosmic::smart::features::PHYSICAL_PIECE_TYPE_LEN;
use crate::cosmic::smart::features::PIECE_TYPE_LEN;
use crate::cosmic::smart::features::{DoubleFacedPiece, DoubleFacedPieceType, PieceType};
use crate::cosmic::smart::square::{Angle, RelAdr2D, ANGLE_LEN};
use crate::cosmic::toy_box::{Piece, PieceNum};
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

    /// 駒種類☆（＾～＾）
    piece_type_to_promoted_table: [bool; PIECE_TYPE_LEN],
    piece_type_to_mobility_table: [Vec<Mobility>; PIECE_TYPE_LEN],
    piece_type_to_double_faced_piece_type: [DoubleFacedPieceType; PIECE_TYPE_LEN],
    /// 持ち駒☆（＾～＾）
    /// 玉２枚引く☆（＾～＾）
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

    hand_legal_all: [Fire; PHYSICAL_PIECES_LEN - 2],
}
impl Default for SpeedOfLight {
    fn default() -> Self {
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
