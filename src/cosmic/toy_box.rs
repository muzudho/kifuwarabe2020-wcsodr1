//!
//! 駒 と 盤
//!
use crate::cosmic::smart::features::DoubleFacedPiece;
use crate::cosmic::smart::square::BOARD_MEMORY_AREA;
use crate::look_and_model::piece::Piece;
use num_derive::FromPrimitive;
use std::*;

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

/// ちゆり「駒そのものではなく、駒の情報が欲しいだけなら、これだぜ☆」
/// きふわらべ「USIでは使わないから、独自の表記でOKだぜ☆」
/// 夢美「new()で引数２つ設定する必要があるけど、必ずしもそれを利用する必要はないのね」
pub struct PieceInfo {
    pub text1: String,
    pub num: String,
}
impl PieceInfo {
    pub fn new(piece_display: &str, num: PieceNum) -> Self {
        PieceInfo {
            text1: piece_display.to_string(),
            num: format!("{:?}", num),
        }
    }
}

/// 背番号(名前)付きの駒の数。
pub const NAMED_PIECES_LEN: usize = 40;

/// 駒の背番号（名前）だぜ☆（＾～＾）大橋流で触る駒の順だぜ☆（＾～＾）
#[derive(Clone, Copy, FromPrimitive, Debug, PartialEq)]
pub enum PieceNum {
    /// 1 先手玉
    King1,
    /// 2 後手玉
    King2,
    /// 3 金
    Gold3,
    /// 4 金
    Gold4,
    /// 5 金
    Gold5,
    /// 6 金
    Gold6,
    /// 7 銀
    Silver7,
    /// 8 銀
    Silver8,
    /// 9 銀
    Silver9,
    /// 10 銀
    Silver10,
    /// 11 桂
    Knight11,
    /// 12 桂
    Knight12,
    /// 13 桂
    Knight13,
    /// 14 桂
    Knight14,
    /// 15 香
    Lance15,
    /// 16 香
    Lance16,
    /// 17 香
    Lance17,
    /// 18 香
    Lance18,
    /// 19 角
    Bishop19,
    /// 20 角
    Bishop20,
    /// 21 飛
    Rook21,
    /// 22 飛
    Rook22,
    /// 23 歩
    Pawn23,
    /// 24 歩
    Pawn24,
    /// 25 歩
    Pawn25,
    /// 26 歩
    Pawn26,
    /// 27 歩
    Pawn27,
    /// 28 歩
    Pawn28,
    /// 29 歩
    Pawn29,
    /// 30 歩
    Pawn30,
    /// 31 歩
    Pawn31,
    /// 32 歩
    Pawn32,
    /// 33 歩
    Pawn33,
    /// 34 歩
    Pawn34,
    /// 35 歩
    Pawn35,
    /// 36 歩
    Pawn36,
    /// 37 歩
    Pawn37,
    /// 38 歩
    Pawn38,
    /// 39 歩
    Pawn39,
    /// 40 歩
    Pawn40,
}
/// きふわらべ「USIでは使わないから、独自の表記をして構わないぜ☆」
impl fmt::Display for PieceNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        // ▲ が半角サイズ、▽ が見た目が全角の半角サイズなのは、Windows Terminal の担当者 いい加減だぜ☆（＾～＾）
        use crate::cosmic::toy_box::PieceNum::*;
        write!(
            f,
            "{}",
            match *self {
                King1 => "01K ",
                King2 => "02K ",
                Gold3 => "03G ",
                Gold4 => "04G ",
                Gold5 => "05G ",
                Gold6 => "06G ",
                Silver7 => "07S ",
                Silver8 => "08S ",
                Silver9 => "09S ",
                Silver10 => "10S ",
                Knight11 => "11N ",
                Knight12 => "12N ",
                Knight13 => "13N ",
                Knight14 => "14N ",
                Lance15 => "15L ",
                Lance16 => "16L ",
                Lance17 => "17L ",
                Lance18 => "18L ",
                Bishop19 => "19B ",
                Bishop20 => "20B ",
                Rook21 => "21R ",
                Rook22 => "22R ",
                Pawn23 => "23P ",
                Pawn24 => "24P ",
                Pawn25 => "25P ",
                Pawn26 => "26P ",
                Pawn27 => "27P ",
                Pawn28 => "28P ",
                Pawn29 => "29P ",
                Pawn30 => "30P ",
                Pawn31 => "31P ",
                Pawn32 => "32P ",
                Pawn33 => "33P ",
                Pawn34 => "34P ",
                Pawn35 => "35P ",
                Pawn36 => "36P ",
                Pawn37 => "37P ",
                Pawn38 => "38P ",
                Pawn39 => "39P ",
                Pawn40 => "40P ",
            }
        )
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
    items: [Option<PieceNum>; BOARD_MEMORY_AREA],
    // board1_cur: isize,
    // board2_cur: isize,
}
impl Default for PhaseClassification {
    // ゴミ値で埋めるぜ☆（＾～＾）
    fn default() -> Self {
        PhaseClassification {
            items: [None; BOARD_MEMORY_AREA],
            // board1_cur: 0,
            // board2_cur: 39,
        }
    }
}
impl PhaseClassification {
    /*
    /// TODO
    fn board_cur(&self, turn: Phase) -> isize {
        0
        /*
        match turn {
            Phase::First => self.board1_cur,
            Phase::Second => self.board2_cur,
        }
        */
    }
    */
    /*
    /// TODO
    fn add_board_cur(&mut self, turn: Phase, direction: isize) {
        /*
        match turn {
            Phase::First => self.board1_cur += direction,
            Phase::Second => self.board2_cur += direction,
        }
        */
    }
    */
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
