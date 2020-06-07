//!
//! 駒 と 盤
//!
use crate::cosmic::recording::Movement;
use crate::cosmic::recording::{AddressPos, Phase};
use crate::cosmic::smart::features::{DoubleFacedPiece, Piece, PieceType, PHYSICAL_PIECE_TYPE_LEN};
use crate::cosmic::smart::square::{AbsoluteAddress2D, BOARD_MEMORY_AREA, RANK_1, RANK_10};
use crate::law::speed_of_light::Nine299792458;
use crate::spaceship::equipment::Beam;
use crate::spaceship::equipment::PieceInfo;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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
    address_list: [AddressPos; NAMED_PIECES_LEN],
    /// 駒の背番号に、駒が紐づくぜ☆（＾～＾）
    piece_list: [Piece; NAMED_PIECES_LEN],
    /// 駒の背番号を付けるのに使うぜ☆（＾～＾）
    double_faced_piece_type_index: [usize; PHYSICAL_PIECE_TYPE_LEN],
    /// 持ち駒☆（＾～＾）TODO 固定長サイズのスタックを用意したいぜ☆（＾～＾）
    pub hands: HandStack,
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
            address_list: [AddressPos::Board(AbsoluteAddress2D::default()); NAMED_PIECES_LEN],
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
            hands: HandStack::default(),
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
        self.address_list = [AddressPos::Board(AbsoluteAddress2D::default()); NAMED_PIECES_LEN];
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
        self.hands = HandStack::default();
    }

    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, table: &GameTable) {
        self.board = table.board.clone();
        self.address_list = table.address_list.clone();
        self.piece_list = table.piece_list.clone();
        self.double_faced_piece_type_index = table.double_faced_piece_type_index.clone();
        self.hands = table.hands.clone();
    }

    pub fn get_piece(&self, num: PieceNum) -> Piece {
        self.piece_list[num as usize]
    }
    pub fn new_piece_num(&mut self, piece: Piece, num: PieceNum) -> PieceNum {
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
            // TODO 元データを反転させたいぜ☆（＾～＾）
            let captured_piece_num = self.new_piece_num(
                self.get_piece(collision_piece_num_val).captured(),
                collision_piece_num_val,
            );
            self.push_piece(
                &AddressPos::Hand(self.get_piece(captured_piece_num).double_faced_piece()),
                Some(captured_piece_num),
            );
        }
    }

    /// アンドゥ時の動き。
    /// あれば、指し手で取った駒の先後をひっくり返せば、自分の駒台にある駒を取り出せるので取り出して、盤の上に指し手の取った駒のまま駒を置きます。
    pub fn rotate_piece_hand_to_board(&mut self, friend: Phase, move_: &Movement) {
        if let Some(move2_val) = move_.captured {
            // 取った方の駒台の先後に合わせるぜ☆（＾～＾）
            // 取った方の持ち駒を減らす
            let piece_num = self
                .pop_piece(&AddressPos::Hand(DoubleFacedPiece::from_phase_and_type(
                    friend, //.turn(),
                    move2_val.piece_type.double_faced_piece_type(),
                )))
                .unwrap();
            // 先後をひっくり返す。
            self.turn_phase(piece_num);
            if move2_val.piece_type.promoted() {
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
    pub fn push_piece(&mut self, addr: &AddressPos, piece_num: Option<PieceNum>) {
        match addr {
            AddressPos::Board(sq) => {
                if let Some(piece_num_val) = piece_num {
                    // マスに駒を置きます。
                    self.board[sq.serial_number() as usize] = piece_num;
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] = AddressPos::Board(*sq);
                } else {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                }
            }
            AddressPos::Hand(_old_drop) => {
                if let Some(piece_num_val) = piece_num {
                    // 持ち駒を１つ増やします。
                    let new_drop = self.get_piece(piece_num_val).double_faced_piece();
                    self.hands.push(new_drop /* *drop*/, piece_num_val);
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] = *addr;
                }
            }
        }
    }
    /// 駒を取りのぞく。
    pub fn pop_piece(&mut self, addr: &AddressPos) -> Option<PieceNum> {
        match addr {
            AddressPos::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                    // TODO 背番号の番地を、ゴミ値で塗りつぶすが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                    self.address_list[piece_num_val as usize] =
                        AddressPos::Board(AbsoluteAddress2D::default());
                }
                piece_num
            }
            AddressPos::Hand(drop) => {
                // 場所で指定します。
                // 台から取りのぞきます。
                let piece_num = self.hands.pop(*drop);
                // TODO 背番号の番地に、ゴミ値を入れて消去するが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                self.address_list[piece_num as usize] =
                    AddressPos::Board(AbsoluteAddress2D::default());
                Some(piece_num)
            }
        }
    }

    /// 駒の新しい背番号を生成します。
    pub fn naming_piece(&mut self, piece: Piece) -> PieceNum {
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
    pub fn exists_pawn_on_file(&self, phase: Phase, file: usize) -> bool {
        for rank in RANK_1..RANK_10 {
            let addr = AddressPos::Board(AbsoluteAddress2D::new(file, rank));
            if let Some(piece_val) = self.piece_at(&addr) {
                if piece_val.phase() == phase && piece_val.type_() == PieceType::Pawn {
                    return true;
                }
            }
        }
        false
    }
    /// TODO Piece をカプセル化したい。外に出したくないぜ☆（＾～＾）
    /// 升で指定して駒を取得。
    /// 駒台には対応してない。 -> 何に使っている？
    pub fn piece_num_at(&self, addr: &AddressPos) -> Option<PieceNum> {
        match addr {
            AddressPos::Board(sq) => self.board[sq.serial_number() as usize],
            _ => panic!(Beam::trouble(&format!(
                "(Err.254) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 駒台には対応してない。 -> 何に使っている？
    pub fn piece_info_at(&self, addr: &AddressPos) -> Option<PieceInfo> {
        match addr {
            AddressPos::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    Some(PieceInfo::new(self.get_piece(piece_num_val), piece_num_val))
                } else {
                    None
                }
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.321) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    pub fn promotion_value_at(&self, table: &GameTable, addr: &AddressPos) -> isize {
        match addr {
            AddressPos::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    table
                        .get_piece(piece_num_val)
                        .double_faced_piece()
                        .type_()
                        .promotion_value()
                } else {
                    // 打なら成りは無いぜ☆（＾～＾）
                    0
                }
            }
            AddressPos::Hand(_old_drop) => panic!(Beam::trouble(&format!(
                "(Err.254) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    pub fn piece_at(&self, addr: &AddressPos) -> Option<Piece> {
        match addr {
            AddressPos::Board(sq) => {
                if let Some(piece_num) = self.board[sq.serial_number() as usize] {
                    Some(self.get_piece(piece_num))
                } else {
                    None
                }
            }
            AddressPos::Hand(_old_drop) => panic!(Beam::trouble(&format!(
                "(Err.345) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand_num(&self, drop: DoubleFacedPiece) -> Option<PieceNum> {
        self.hands.last(drop)
    }
    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand(&self, table: &GameTable, phy: DoubleFacedPiece) -> Option<Piece> {
        if let Some(old_piece) = self.hands.last(phy) {
            Some(table.get_piece(old_piece))
        } else {
            None
        }
    }
    pub fn count_hand(&self, phy: DoubleFacedPiece) -> usize {
        self.hands.len(phy)
    }

    /// 表示に使うだけ☆（＾～＾）
    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_table<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, Option<&AbsoluteAddress2D>, Option<PieceInfo>),
    {
        for (i, addr) in self.address_list.iter().enumerate() {
            match addr {
                AddressPos::Board(sq) => {
                    // 盤上の駒☆（＾～＾）
                    let piece_info = self.piece_info_at(addr).unwrap();
                    piece_get(i, Some(sq), Some(piece_info));
                }
                AddressPos::Hand(_old_drop) => {
                    // TODO 持ち駒☆（＾～＾）
                    piece_get(i, None, None);
                }
            }
        }
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_some_pieces_on_list40<F>(&self, friend: Phase, piece_get: &mut F)
    where
        F: FnMut(AddressPos, PieceNum),
    {
        for piece_num in Nine299792458::piece_numbers().iter() {
            let addr = self.address_list[*piece_num as usize];
            match addr {
                AddressPos::Board(_sq) => {
                    // 盤上の駒☆（＾～＾）
                    let piece = self.piece_num_at(&addr).unwrap();
                    if self.get_piece(*piece_num).phase() == friend {
                        piece_get(addr, piece);
                    }
                }
                AddressPos::Hand(_old_drop) => {
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
        for old_drop in &FIRST_SECOND[friend as usize] {
            if let Some(piece_num) = self.last_hand_num(*old_drop) {
                piece_get(AddressPos::Hand(*old_drop), piece_num);
            }
        }
    }
}

/// 駒台だぜ☆（＾～＾）これ１つで２人分あるんで☆（＾～＾）
#[derive(Clone)]
pub struct HandStack {
    king: Hand2Piece,
    gold: Hand4Piece,
    silver: Hand4Piece,
    knight: Hand4Piece,
    lance: Hand4Piece,
    rook: Hand2Piece,
    bishop: Hand2Piece,
    pawn: Hand18Piece,
}
impl Default for HandStack {
    // ゴミ値で埋めるぜ☆（＾～＾）
    fn default() -> Self {
        HandStack {
            king: Hand2Piece::default(),
            gold: Hand4Piece::default(),
            silver: Hand4Piece::default(),
            knight: Hand4Piece::default(),
            lance: Hand4Piece::default(),
            rook: Hand2Piece::default(),
            bishop: Hand2Piece::default(),
            pawn: Hand18Piece::default(),
        }
    }
}
impl HandStack {
    /// ひっくり返してから入れてください。
    fn push(&mut self, drop: DoubleFacedPiece, num: PieceNum) {
        match drop {
            DoubleFacedPiece::King1 => {
                self.king.push_head(num);
            }
            DoubleFacedPiece::King2 => {
                self.king.push_tail(num);
            }
            DoubleFacedPiece::Gold1 => {
                self.gold.push_head(num);
            }
            DoubleFacedPiece::Gold2 => {
                self.gold.push_tail(num);
            }
            DoubleFacedPiece::Silver1 => {
                self.silver.push_head(num);
            }
            DoubleFacedPiece::Silver2 => {
                self.silver.push_tail(num);
            }
            DoubleFacedPiece::Knight1 => {
                self.knight.push_head(num);
            }
            DoubleFacedPiece::Knight2 => {
                self.knight.push_tail(num);
            }
            DoubleFacedPiece::Lance1 => {
                self.lance.push_head(num);
            }
            DoubleFacedPiece::Lance2 => {
                self.lance.push_tail(num);
            }
            DoubleFacedPiece::Rook1 => {
                self.rook.push_head(num);
            }
            DoubleFacedPiece::Rook2 => {
                self.rook.push_tail(num);
            }
            DoubleFacedPiece::Bishop1 => {
                self.bishop.push_head(num);
            }
            DoubleFacedPiece::Bishop2 => {
                self.bishop.push_tail(num);
            }
            DoubleFacedPiece::Pawn1 => {
                self.pawn.push_head(num);
            }
            DoubleFacedPiece::Pawn2 => {
                self.pawn.push_tail(num);
            }
        }
    }

    /// ゴミ値は消さないぜ☆（＾～＾）
    fn pop(&mut self, drop: DoubleFacedPiece) -> PieceNum {
        match drop {
            DoubleFacedPiece::King1 => self.king.pop_head(),
            DoubleFacedPiece::King2 => self.king.pop_tail(),
            DoubleFacedPiece::Gold1 => self.gold.pop_head(),
            DoubleFacedPiece::Gold2 => self.gold.pop_tail(),
            DoubleFacedPiece::Silver1 => self.silver.pop_head(),
            DoubleFacedPiece::Silver2 => self.silver.pop_tail(),
            DoubleFacedPiece::Knight1 => self.knight.pop_head(),
            DoubleFacedPiece::Knight2 => self.knight.pop_tail(),
            DoubleFacedPiece::Lance1 => self.lance.pop_head(),
            DoubleFacedPiece::Lance2 => self.lance.pop_tail(),
            DoubleFacedPiece::Rook1 => self.rook.pop_head(),
            DoubleFacedPiece::Rook2 => self.rook.pop_tail(),
            DoubleFacedPiece::Bishop1 => self.bishop.pop_head(),
            DoubleFacedPiece::Bishop2 => self.bishop.pop_tail(),
            DoubleFacedPiece::Pawn1 => self.pawn.pop_head(),
            DoubleFacedPiece::Pawn2 => self.pawn.pop_tail(),
        }
    }

    fn last(&self, drop: DoubleFacedPiece) -> Option<PieceNum> {
        match drop {
            DoubleFacedPiece::King1 => self.king.last_head(),
            DoubleFacedPiece::King2 => self.king.last_tail(),
            DoubleFacedPiece::Gold1 => self.gold.last_head(),
            DoubleFacedPiece::Gold2 => self.gold.last_tail(),
            DoubleFacedPiece::Silver1 => self.silver.last_head(),
            DoubleFacedPiece::Silver2 => self.silver.last_tail(),
            DoubleFacedPiece::Knight1 => self.knight.last_head(),
            DoubleFacedPiece::Knight2 => self.knight.last_tail(),
            DoubleFacedPiece::Lance1 => self.lance.last_head(),
            DoubleFacedPiece::Lance2 => self.lance.last_tail(),
            DoubleFacedPiece::Rook1 => self.rook.last_head(),
            DoubleFacedPiece::Rook2 => self.rook.last_tail(),
            DoubleFacedPiece::Bishop1 => self.bishop.last_head(),
            DoubleFacedPiece::Bishop2 => self.bishop.last_tail(),
            DoubleFacedPiece::Pawn1 => self.pawn.last_head(),
            DoubleFacedPiece::Pawn2 => self.pawn.last_tail(),
        }
    }

    fn len(&self, drop: DoubleFacedPiece) -> usize {
        match drop {
            DoubleFacedPiece::King1 => self.king.len_head(),
            DoubleFacedPiece::King2 => self.king.len_tail(),
            DoubleFacedPiece::Gold1 => self.gold.len_head(),
            DoubleFacedPiece::Gold2 => self.gold.len_tail(),
            DoubleFacedPiece::Silver1 => self.silver.len_head(),
            DoubleFacedPiece::Silver2 => self.silver.len_tail(),
            DoubleFacedPiece::Knight1 => self.knight.len_head(),
            DoubleFacedPiece::Knight2 => self.knight.len_tail(),
            DoubleFacedPiece::Lance1 => self.lance.len_head(),
            DoubleFacedPiece::Lance2 => self.lance.len_tail(),
            DoubleFacedPiece::Rook1 => self.rook.len_head(),
            DoubleFacedPiece::Rook2 => self.rook.len_tail(),
            DoubleFacedPiece::Bishop1 => self.bishop.len_head(),
            DoubleFacedPiece::Bishop2 => self.bishop.len_tail(),
            DoubleFacedPiece::Pawn1 => self.pawn.len_head(),
            DoubleFacedPiece::Pawn2 => self.pawn.len_tail(),
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
pub struct Hand2Piece {
    items: [PieceNum; 2],
    head: usize,
    tail: usize,
}
impl Default for Hand2Piece {
    /// ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        Hand2Piece {
            items: [PieceNum::King1; 2],
            head: 0,
            tail: 1,
        }
    }
}
impl Hand2Piece {
    pub fn push_head(&mut self, num: PieceNum) {
        self.items[self.head] = num;
        self.head += 1;
    }
    pub fn push_tail(&mut self, num: PieceNum) {
        self.items[self.tail] = num;
        self.tail -= 1;
    }
    pub fn pop_head(&mut self) -> PieceNum {
        self.head -= 1;
        let num = self.items[self.head];
        num
    }
    pub fn pop_tail(&mut self) -> PieceNum {
        self.tail += 1;
        let num = self.items[self.tail];
        num
    }
    pub fn last_head(&self) -> Option<PieceNum> {
        if 0 < self.head {
            Some(self.items[self.head - 1])
        } else {
            None
        }
    }
    pub fn last_tail(&self) -> Option<PieceNum> {
        if self.tail < 1 {
            Some(self.items[self.tail + 1])
        } else {
            None
        }
    }
    pub fn len_head(&self) -> usize {
        self.head
    }
    pub fn len_tail(&self) -> usize {
        1 - self.tail
    }
}
#[derive(Clone)]
pub struct Hand4Piece {
    items: [PieceNum; 4],
    head: usize,
    tail: usize,
}
impl Default for Hand4Piece {
    /// ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        Hand4Piece {
            items: [PieceNum::King1; 4],
            head: 0,
            tail: 3,
        }
    }
}
impl Hand4Piece {
    pub fn push_head(&mut self, num: PieceNum) {
        self.items[self.head] = num;
        self.head += 1;
    }
    pub fn push_tail(&mut self, num: PieceNum) {
        self.items[self.tail] = num;
        self.tail -= 1;
    }
    pub fn pop_head(&mut self) -> PieceNum {
        self.head -= 1;
        let num = self.items[self.head];
        num
    }
    pub fn pop_tail(&mut self) -> PieceNum {
        self.tail += 1;
        let num = self.items[self.tail];
        num
    }
    pub fn last_head(&self) -> Option<PieceNum> {
        if 0 < self.head {
            Some(self.items[self.head - 1])
        } else {
            None
        }
    }
    pub fn last_tail(&self) -> Option<PieceNum> {
        if self.tail < 3 {
            Some(self.items[self.tail + 1])
        } else {
            None
        }
    }
    pub fn len_head(&self) -> usize {
        self.head
    }
    pub fn len_tail(&self) -> usize {
        3 - self.tail
    }
}
#[derive(Clone)]
pub struct Hand18Piece {
    items: [PieceNum; 18],
    head: usize,
    tail: usize,
}
impl Default for Hand18Piece {
    /// ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        Hand18Piece {
            items: [PieceNum::King1; 18],
            head: 0,
            tail: 17,
        }
    }
}
impl Hand18Piece {
    pub fn push_head(&mut self, num: PieceNum) {
        self.items[self.head] = num;
        self.head += 1;
    }
    pub fn push_tail(&mut self, num: PieceNum) {
        self.items[self.tail] = num;
        self.tail -= 1;
    }
    pub fn pop_head(&mut self) -> PieceNum {
        self.head -= 1;
        let num = self.items[self.head];
        num
    }
    pub fn pop_tail(&mut self) -> PieceNum {
        self.tail += 1;
        let num = self.items[self.tail];
        num
    }
    pub fn last_head(&self) -> Option<PieceNum> {
        if 0 < self.head {
            Some(self.items[self.head - 1])
        } else {
            None
        }
    }
    pub fn last_tail(&self) -> Option<PieceNum> {
        if self.tail < 17 {
            Some(self.items[self.tail + 1])
        } else {
            None
        }
    }
    pub fn len_head(&self) -> usize {
        self.head
    }
    pub fn len_tail(&self) -> usize {
        17 - self.tail
    }
}
