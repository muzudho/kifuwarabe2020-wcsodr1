//!
//! 駒 と 盤
//!
use crate::cosmic::recording::{AddressPos, Phase};
use crate::cosmic::smart::features::{
    PhysicalPiece, PieceMeaning, PieceType, HAND_MAX, PHYSICAL_PIECES_LEN, PHYSICAL_PIECE_TYPE_LEN,
};
use crate::cosmic::smart::square::{AbsoluteAddress2D, BOARD_MEMORY_AREA, RANK_1, RANK_10};
use crate::law::generate_move::Piece;
use crate::law::speed_of_light::Nine299792458;
use crate::spaceship::equipment::Beam;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt;

/// 背番号付きの駒の数。
pub const PIECE_NUM_LEN: usize = 40;

/// 駒に背番号を付けたものだぜ☆（＾～＾）
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

/// 現局面、または初期局面☆（＾～＾）
/// でかいのでコピーもクローンも不可☆（＾～＾）！
/// 10の位を筋、1の位を段とする。
/// 0筋、0段は未使用
pub struct Board {
    // いわゆる盤☆（＾～＾）
    pieces: [Option<Piece>; BOARD_MEMORY_AREA as usize],
    /// 駒の居場所☆（＾～＾）
    address: [AddressPos; PIECE_NUM_LEN],
    /// 駒の背番号を付けるのに使うぜ☆（＾～＾）
    physical_piece_type_index: [usize; PHYSICAL_PIECE_TYPE_LEN],
    /// 持ち駒☆（＾～＾）TODO 固定長サイズのスタックを用意したいぜ☆（＾～＾）
    pub hands: [HandAddressTypeStack; PHYSICAL_PIECES_LEN],
}
impl Default for Board {
    fn default() -> Self {
        Board {
            // 盤上
            pieces: [
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
            address: [AddressPos::Board(AbsoluteAddress2D::default()); PIECE_NUM_LEN],
            physical_piece_type_index: [
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
            hands: [
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
                HandAddressTypeStack::default(),
            ],
        }
    }
}
impl Board {
    pub fn clear(&mut self) {
        self.pieces = [
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
        self.address = [AddressPos::Board(AbsoluteAddress2D::default()); PIECE_NUM_LEN];
        self.physical_piece_type_index = [
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
        self.hands = [
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
            HandAddressTypeStack::default(),
        ];
    }

    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, board: &Board) {
        self.pieces = board.pieces.clone();
        self.address = board.address.clone();
        self.physical_piece_type_index = board.physical_piece_type_index.clone();
        self.hands = board.hands.clone();
    }

    pub fn push_to_board(&mut self, addr: &AddressPos, piece: Option<Piece>) {
        match addr {
            AddressPos::Board(sq) => {
                if let Some(piece_val) = piece {
                    // マスに駒を置きます。
                    self.pieces[sq.serial_number() as usize] = piece;
                    // 背番号に番地を紐づけます。
                    self.address[piece_val.num as usize] = AddressPos::Board(*sq);
                } else {
                    // マスを空にします。
                    self.pieces[sq.serial_number() as usize] = None;
                }
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.233) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 台に駒を置く
    pub fn push_to_hand(&mut self, piece: &Piece) {
        let pp = piece.meaning.physical_piece();
        // 持ち駒を１つ増やします。
        self.hands[pp as usize].push(piece);
        // 背番号に番地を紐づけます。
        self.address[piece.num as usize] = AddressPos::Hand(pp);
    }
    /// 盤から駒を取りのぞく
    pub fn pop_from_board(&mut self, addr: &AddressPos) -> Option<Piece> {
        // まず、駒があるか確認するぜ☆（＾～＾）
        match addr {
            AddressPos::Board(sq) => {
                let piece = self.pieces[sq.serial_number() as usize];
                if let Some(piece_val) = piece {
                    // マスを空にします。
                    self.pieces[sq.serial_number() as usize] = None;
                    // TODO 背番号の番地を、ゴミ値で塗りつぶすが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                    self.address[piece_val.num as usize] =
                        AddressPos::Board(AbsoluteAddress2D::default());
                }
                piece
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.254) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 台から駒を取りのぞく
    pub fn pop_from_hand(&mut self, addr: AddressPos) -> Option<Piece> {
        match addr {
            AddressPos::Board(_sq) => panic!(Beam::trouble(&format!(
                "(Err.254) まだ実装してないぜ☆（＾～＾）！",
            ))),
            AddressPos::Hand(drop) => {
                // 台から取りのぞきます。
                let piece = self.hands[drop as usize].pop();
                // TODO 背番号の番地に、ゴミ値を入れて消去するが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                self.address[piece.num as usize] = AddressPos::Board(AbsoluteAddress2D::default());
                Some(piece)
            }
        }
    }

    /// 駒の新しい背番号を生成します。
    pub fn make_piece_number(&mut self, piece_meaning: PieceMeaning) -> Piece {
        match piece_meaning {
            // 玉だけ、先後は決まってるから従えだぜ☆（＾～＾）
            PieceMeaning::King1 => Piece::new(piece_meaning, PieceNum::King1),
            PieceMeaning::King2 => Piece::new(piece_meaning, PieceNum::King2),
            _ => {
                let phy_pct = piece_meaning.physical_piece().type_() as usize;
                // 玉以外の背番号は、先後に関わりなく SFENに書いてあった順で☆（＾～＾）
                let pn = PieceNum::from_usize(self.physical_piece_type_index[phy_pct]).unwrap();
                // カウントアップ☆（＾～＾）
                self.physical_piece_type_index[phy_pct] += 1;
                Piece::new(piece_meaning, pn)
            }
        }
    }

    /// 先手玉、後手玉なら、その位置を確定させます。背番号も付けます。
    pub fn push_to_board_from_sfen(&mut self, addr: &AddressPos, piece_meaning: PieceMeaning) {
        /*
        if !(FILE_0 < addr.file()
            && addr.file() < FILE_10
            && RANK_0 < addr.rank()
            && addr.rank() < RANK_10)
        {
            panic!(Beam::trouble(&format!(
                "(Err.323) 盤上の初期化で盤の外を指定するのは止めろだぜ☆（＾～＾）！ ({}, {})",
                addr.file(),
                addr.rank()
            )))
        }
        */

        // 駒に背番号を付けるぜ☆（＾～＾）
        let piece = self.make_piece_number(piece_meaning);

        // 盤に置くぜ☆（＾～＾）
        self.push_to_board(&addr, Some(piece));
    }

    /// 持ち駒に背番号を付ける
    pub fn push_to_hand_from_sfen(&mut self, piece_meaning: PieceMeaning, number: isize) {
        for _i in 0..number {
            // 駒に背番号を付けるぜ☆（＾～＾）
            let piece = self.make_piece_number(piece_meaning);

            // 駒台に置くぜ☆（＾～＾）
            self.push_to_hand(&piece);
        }
    }

    /// 歩が置いてあるか確認
    pub fn exists_pawn_on_file(&self, phase: Phase, file: usize) -> bool {
        for rank in RANK_1..RANK_10 {
            let addr = AddressPos::Board(AbsoluteAddress2D::new(file, rank));
            if let Some(piece) = self.piece_at(&addr) {
                if piece.meaning.phase() == phase && piece.meaning.type_() == PieceType::Pawn {
                    return true;
                }
            }
        }
        false
    }
    /// 升で指定して駒を取得
    pub fn piece_at(&self, addr: &AddressPos) -> Option<Piece> {
        match addr {
            AddressPos::Board(sq) => self.pieces[sq.serial_number() as usize],
            _ => panic!(Beam::trouble(&format!(
                "(Err.254) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand(&self, adr: PhysicalPiece) -> Option<&Piece> {
        self.hands[adr as usize].last()
    }
    pub fn count_hand(&self, adr: PhysicalPiece) -> usize {
        self.hands[adr as usize].len()
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_board<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, Option<&AbsoluteAddress2D>, Option<Piece>),
    {
        for (i, addr) in self.address.iter().enumerate() {
            match addr {
                AddressPos::Board(sq) => {
                    // 盤上の駒☆（＾～＾）
                    let piece = self.piece_at(addr).unwrap();
                    piece_get(i, Some(sq), Some(piece));
                }
                AddressPos::Hand(_drop) => {
                    // TODO 持ち駒☆（＾～＾）
                    piece_get(i, None, None);
                }
            }
        }
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_some_pieces_on_list40<F>(&self, friend: Phase, piece_get: &mut F)
    where
        F: FnMut(AddressPos, Piece),
    {
        for piece_num in Nine299792458::piece_numbers().iter() {
            let addr = self.address[*piece_num as usize];
            match addr {
                AddressPos::Board(_sq) => {
                    // 盤上の駒☆（＾～＾）
                    let piece = self.piece_at(&addr).unwrap();
                    if piece.meaning.phase() == friend {
                        piece_get(addr, piece);
                    }
                }
                AddressPos::Hand(_drop) => {
                    // 持ち駒はここで調べるのは無駄な気がするよな☆（＾～＾）持ち駒に歩が１８個とか☆（＾～＾）
                }
            }
        }

        const FIRST_SECOND: [[PhysicalPiece; PHYSICAL_PIECE_TYPE_LEN - 1]; 2] = [
            [
                // King なし
                PhysicalPiece::Rook1,
                PhysicalPiece::Bishop1,
                PhysicalPiece::Gold1,
                PhysicalPiece::Silver1,
                PhysicalPiece::Knight1,
                PhysicalPiece::Lance1,
                PhysicalPiece::Pawn1,
            ],
            [
                // King なし
                PhysicalPiece::Rook2,
                PhysicalPiece::Bishop2,
                PhysicalPiece::Gold2,
                PhysicalPiece::Silver2,
                PhysicalPiece::Knight2,
                PhysicalPiece::Lance2,
                PhysicalPiece::Pawn2,
            ],
        ];
        for adr in &FIRST_SECOND[friend as usize] {
            if let Some(piece) = self.last_hand(*adr) {
                piece_get(AddressPos::Hand(*adr), *piece);
            }
        }
    }
}

#[derive(Clone)]
pub struct HandAddressTypeStack {
    items: [Piece; HAND_MAX],
    count: usize,
}
impl Default for HandAddressTypeStack {
    fn default() -> Self {
        HandAddressTypeStack {
            // ゴミ値で埋めるぜ☆（＾～＾）
            items: [Piece::new(PieceMeaning::King1, PieceNum::King1); HAND_MAX],
            count: 0,
        }
    }
}
impl HandAddressTypeStack {
    fn push(&mut self, piece: &Piece) {
        self.items[self.count] = *piece;
        self.count += 1;
    }

    fn pop(&mut self) -> Piece {
        self.count -= 1;
        let piece = self.items[self.count];
        // ゴミ値は消さないぜ☆（＾～＾）
        piece
    }

    fn last(&self) -> Option<&Piece> {
        if 0 < self.count {
            Some(&self.items[self.count - 1])
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.count
    }
}
impl fmt::Display for HandAddressTypeStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for i in 0..=self.count {
            buffer.push_str(&format!(
                "({}, {:?}) ",
                self.items[i].meaning, self.items[i].num
            ));
        }
        write!(f, "{}", buffer.trim_end())
    }
}
