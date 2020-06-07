//!
//! 駒 と 盤
//!
use crate::cosmic::recording::Movement;
use crate::cosmic::recording::{AddressPos, Phase};
use crate::cosmic::smart::features::{
    PhysicalPiece, PieceMeaning, PieceType, HAND_MAX, PHYSICAL_PIECES_LEN, PHYSICAL_PIECE_TYPE_LEN,
};
use crate::cosmic::smart::square::{AbsoluteAddress2D, BOARD_MEMORY_AREA, RANK_1, RANK_10};
use crate::law::generate_move::OldPiece;
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
    board: [Option<OldPiece>; BOARD_MEMORY_AREA as usize],
    /// 背番号付きの駒に、番地が紐づいているぜ☆（＾～＾）
    old_address_list: [AddressPos; NAMED_PIECES_LEN],
    /// 駒の背番号に、駒が紐づくぜ☆（＾～＾）
    new_piece_list: [PieceMeaning; NAMED_PIECES_LEN],
    /// 駒の背番号を付けるのに使うぜ☆（＾～＾）
    physical_piece_type_index: [usize; PHYSICAL_PIECE_TYPE_LEN],
    /// 持ち駒☆（＾～＾）TODO 固定長サイズのスタックを用意したいぜ☆（＾～＾）
    pub hands: OldHandStack,
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
            old_address_list: [AddressPos::Board(AbsoluteAddress2D::default()); NAMED_PIECES_LEN],
            /// 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
            new_piece_list: [PieceMeaning::King1; NAMED_PIECES_LEN],
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
            hands: OldHandStack::default(),
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
        self.old_address_list = [AddressPos::Board(AbsoluteAddress2D::default()); NAMED_PIECES_LEN];
        // 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
        self.new_piece_list = [PieceMeaning::King1; NAMED_PIECES_LEN];
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
        self.hands = OldHandStack::default();
    }

    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, table: &GameTable) {
        self.board = table.board.clone();
        self.old_address_list = table.old_address_list.clone();
        self.new_piece_list = table.new_piece_list.clone();
        self.physical_piece_type_index = table.physical_piece_type_index.clone();
        self.hands = table.hands.clone();
    }

    pub fn get_meaning(&self, piece: &OldPiece) -> PieceMeaning {
        piece.old_meaning
        // self.new_piece_list[piece.num as usize]
    }
    pub fn new_piece(&mut self, meaning: PieceMeaning, num: PieceNum) -> OldPiece {
        self.new_piece_list[num as usize] = meaning;
        OldPiece {
            old_meaning: meaning,
            num: num,
        }
    }

    /// ドゥ時の動き。
    /// 駒の先後を反転させるぜ☆（＾～＾）
    // あれば　盤の相手の駒を先後反転して、自分の駒台に置きます。
    pub fn rotate_piece_board_to_hand(&mut self, move_: &Movement) {
        if let Some(collision_piece) = self.pop_piece(&move_.destination) {
            // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
            // 先後ひっくり返す。
            // TODO 元データを反転させたいぜ☆（＾～＾）
            let captured_piece = self.new_piece(
                self.get_meaning(&collision_piece).captured(),
                collision_piece.num,
            );
            self.push_piece(
                &AddressPos::Hand(self.get_meaning(&captured_piece).physical_piece()),
                Some(captured_piece),
            );
        }
    }

    /// アンドゥ時の動き。
    /// あれば、指し手で取った駒の先後をひっくり返せば、自分の駒台にある駒を取り出せるので取り出して、盤の上に指し手の取った駒のまま駒を置きます。
    pub fn rotate_piece_hand_to_board(&mut self, friend: Phase, move_: &Movement) {
        if let Some(move2_val) = move_.captured {
            // TODO 元データを反転させたいぜ☆（＾～＾）
            // 棋譜には、取られた方の先後が記録されているぜ☆（＾～＾）
            // 取った方の駒台の先後に合わせるぜ☆（＾～＾）
            // 取った方の持ち駒を減らす
            let mut old_piece = self
                .pop_piece(&AddressPos::Hand(PhysicalPiece::from_phase_and_type(
                    friend, //.turn(),
                    move2_val.piece_type.physical_piece_type(),
                )))
                .unwrap();
            // 先後をひっくり返す。
            old_piece.turn_phase();
            if move2_val.piece_type.promoted() {
                // 成り駒にします。
                old_piece.promote();
            } else {
                // 成っていない駒にします。
                old_piece.demote();
            }
            /*
            if old_piece.old_meaning.phase() != move2_val.piece.old_meaning.phase()
                || old_piece.old_meaning.type_() != move2_val.piece_type
            {
                panic!(Beam::trouble(&format!(
                    "(Err.276) 分けわからん☆（＾～＾） old_piece.old_meaning.phase()=|{}| move2_val.piece.old_meaning.phase()=|{}| old_piece.old_meaning.type_()=|{}| move2_val.piece.old_meaning.type_()=|{}|",
                    old_piece.old_meaning.phase(),
                    move2_val.piece.old_meaning.phase(),
                    old_piece.old_meaning.type_(),
                    move2_val.piece.old_meaning.type_(),
                )))
            }
            */
            // TODO 指し手に 駒オブジェクト が入っているのは設計上おかしいぜ☆（＾～＾）
            // × let opponent = move2_val.piece.old_meaning.phase();
            // × let piece_type = move2_val.piece.old_meaning.type_();
            // 取られた方に、駒を返すぜ☆（＾～＾）置くのは指し手の移動先☆（＾～＾）
            // 動いてたやつ: self.push_piece(&move_.destination, Some(move2_val.piece));
            self.push_piece(&move_.destination, Some(old_piece));
        }
    }
    /// 駒を置く。
    pub fn push_piece(&mut self, addr: &AddressPos, old_piece: Option<OldPiece>) {
        match addr {
            AddressPos::Board(sq) => {
                if let Some(piece_val) = old_piece {
                    // マスに駒を置きます。
                    self.board[sq.serial_number() as usize] = old_piece;
                    // 背番号に番地を紐づけます。
                    self.old_address_list[piece_val.num as usize] = AddressPos::Board(*sq);
                } else {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                }
            }
            AddressPos::Hand(drop) => {
                if let Some(old_piece_val) = old_piece {
                    // 持ち駒を１つ増やします。
                    self.hands.push(*drop, &old_piece_val);
                    // 背番号に番地を紐づけます。
                    self.old_address_list[old_piece_val.num as usize] = *addr;
                }
            }
        }
    }
    /// 駒を取りのぞく。
    pub fn pop_piece(&mut self, addr: &AddressPos) -> Option<OldPiece> {
        match addr {
            AddressPos::Board(sq) => {
                let piece = self.board[sq.serial_number() as usize];
                if let Some(piece_val) = piece {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                    // TODO 背番号の番地を、ゴミ値で塗りつぶすが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                    self.old_address_list[piece_val.num as usize] =
                        AddressPos::Board(AbsoluteAddress2D::default());
                }
                piece
            }
            AddressPos::Hand(drop) => {
                // 台から取りのぞきます。
                let old_piece = self.hands.pop(*drop);
                // TODO 背番号の番地に、ゴミ値を入れて消去するが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                self.old_address_list[old_piece.num as usize] =
                    AddressPos::Board(AbsoluteAddress2D::default());
                Some(old_piece)
            }
        }
    }

    /// 駒の新しい背番号を生成します。
    pub fn naming_piece(&mut self, piece_meaning: PieceMeaning) -> OldPiece {
        match piece_meaning {
            // 玉だけ、先後は決まってるから従えだぜ☆（＾～＾）
            PieceMeaning::King1 => self.new_piece(piece_meaning, PieceNum::King1),
            PieceMeaning::King2 => self.new_piece(piece_meaning, PieceNum::King2),
            _ => {
                let phy_pct = piece_meaning.physical_piece().type_() as usize;
                // 玉以外の背番号は、先後に関わりなく SFENに書いてあった順で☆（＾～＾）
                let pn = PieceNum::from_usize(self.physical_piece_type_index[phy_pct]).unwrap();
                // カウントアップ☆（＾～＾）
                self.physical_piece_type_index[phy_pct] += 1;
                self.new_piece(piece_meaning, pn)
            }
        }
    }

    /// 歩が置いてあるか確認
    pub fn exists_pawn_on_file(&self, phase: Phase, file: usize) -> bool {
        for rank in RANK_1..RANK_10 {
            let addr = AddressPos::Board(AbsoluteAddress2D::new(file, rank));
            if let Some(piece_meaning_val) = self.piece_meaning_at(&addr) {
                if piece_meaning_val.phase() == phase
                    && piece_meaning_val.type_() == PieceType::Pawn
                {
                    return true;
                }
            }
        }
        false
    }
    /// TODO Piece をカプセル化したい。外に出したくないぜ☆（＾～＾）
    /// 升で指定して駒を取得。
    /// 駒台には対応してない。 -> 何に使っている？
    pub fn piece_at(&self, addr: &AddressPos) -> Option<OldPiece> {
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
                let piece = self.board[sq.serial_number() as usize];
                if let Some(piece_val) = piece {
                    Some(PieceInfo::new(self.get_meaning(&piece_val), piece_val.num))
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
                let piece = self.board[sq.serial_number() as usize];
                if let Some(piece_val) = piece {
                    table
                        .get_meaning(&piece_val)
                        .physical_piece()
                        .type_()
                        .promotion_value()
                } else {
                    // 打なら成りは無いぜ☆（＾～＾）
                    0
                }
            }
            AddressPos::Hand(_drop) => panic!(Beam::trouble(&format!(
                "(Err.254) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    pub fn piece_meaning_at(&self, addr: &AddressPos) -> Option<PieceMeaning> {
        match addr {
            AddressPos::Board(sq) => {
                if let Some(piece) = self.board[sq.serial_number() as usize] {
                    Some(self.get_meaning(&piece))
                } else {
                    None
                }
            }
            AddressPos::Hand(_drop) => panic!(Beam::trouble(&format!(
                "(Err.345) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand(&self, phy: PhysicalPiece) -> Option<&OldPiece> {
        self.hands.last(phy)
    }
    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand_meaning(&self, table: &GameTable, phy: PhysicalPiece) -> Option<PieceMeaning> {
        if let Some(old_piece) = self.hands.last(phy) {
            Some(table.get_meaning(old_piece))
        } else {
            None
        }
    }
    pub fn count_hand(&self, phy: PhysicalPiece) -> usize {
        self.hands.len(phy)
    }

    /// 表示に使うだけ☆（＾～＾）
    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_table<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, Option<&AbsoluteAddress2D>, Option<PieceInfo>),
    {
        for (i, addr) in self.old_address_list.iter().enumerate() {
            match addr {
                AddressPos::Board(sq) => {
                    // 盤上の駒☆（＾～＾）
                    let piece_info = self.piece_info_at(addr).unwrap();
                    piece_get(i, Some(sq), Some(piece_info));
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
        F: FnMut(AddressPos, OldPiece),
    {
        for piece_num in Nine299792458::piece_numbers().iter() {
            let addr = self.old_address_list[*piece_num as usize];
            match addr {
                AddressPos::Board(_sq) => {
                    // 盤上の駒☆（＾～＾）
                    let piece = self.piece_at(&addr).unwrap();
                    if self.get_meaning(&piece).phase() == friend {
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
pub struct OldHandStack {
    pub hands: [OldHandStackElement; PHYSICAL_PIECES_LEN],
}
impl Default for OldHandStack {
    fn default() -> Self {
        OldHandStack {
            hands: [
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
                OldHandStackElement::default(),
            ],
        }
    }
}
impl OldHandStack {
    pub fn push(&mut self, drop: PhysicalPiece, old_piece_val: &OldPiece) {
        self.hands[drop as usize].push(&old_piece_val);
    }
    pub fn pop(&mut self, drop: PhysicalPiece) -> OldPiece {
        self.hands[drop as usize].pop()
    }
    pub fn last(&self, drop: PhysicalPiece) -> Option<&OldPiece> {
        self.hands[drop as usize].last()
    }
    pub fn len(&self, drop: PhysicalPiece) -> usize {
        self.hands[drop as usize].len()
    }
}
#[derive(Clone)]
pub struct OldHandStackElement {
    items: [OldPiece; HAND_MAX],
    count: usize,
}
impl Default for OldHandStackElement {
    fn default() -> Self {
        OldHandStackElement {
            // ゴミ値で埋めるぜ☆（＾～＾）
            items: [OldPiece::default(); HAND_MAX],
            count: 0,
        }
    }
}
impl OldHandStackElement {
    fn push(&mut self, old_piece: &OldPiece) {
        self.items[self.count] = *old_piece;
        self.count += 1;
    }

    fn pop(&mut self) -> OldPiece {
        self.count -= 1;
        let piece = self.items[self.count];
        // ゴミ値は消さないぜ☆（＾～＾）
        piece
    }

    fn last(&self) -> Option<&OldPiece> {
        if 0 < self.count {
            Some(&self.items[self.count - 1])
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.count
    }

    /*
    fn to_debug(&self, table: &GameTable) -> String {
        let mut buffer = String::new();
        for i in 0..=self.count {
            buffer.push_str(&format!(
                "({}, {:?}) ",
                self.items[i].meaning, self.items[i].num
            ));
        }
        buffer.trim_end().to_string()
    }
    */
}

/// 駒台だぜ☆（＾～＾）これ１つで２人分あるんで☆（＾～＾）
#[derive(Clone)]
pub struct NewHandStack {
    king: Hand2Piece,
    gold: Hand4Piece,
    silver: Hand4Piece,
    knight: Hand4Piece,
    lance: Hand4Piece,
    rook: Hand2Piece,
    bishop: Hand2Piece,
    pawn: Hand18Piece,
}
impl Default for NewHandStack {
    // ゴミ値で埋めるぜ☆（＾～＾）
    fn default() -> Self {
        NewHandStack {
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
impl NewHandStack {
    /// ひっくり返してから入れてください。
    fn push(&mut self, old_piece: &OldPiece) {
        match (*old_piece).old_meaning.physical_piece() {
            PhysicalPiece::King1 => {
                self.king.push_head(*old_piece);
            }
            PhysicalPiece::King2 => {
                self.king.push_tail(*old_piece);
            }
            PhysicalPiece::Gold1 => {
                self.gold.push_head(*old_piece);
            }
            PhysicalPiece::Gold2 => {
                self.gold.push_tail(*old_piece);
            }
            PhysicalPiece::Silver1 => {
                self.silver.push_head(*old_piece);
            }
            PhysicalPiece::Silver2 => {
                self.silver.push_tail(*old_piece);
            }
            PhysicalPiece::Knight1 => {
                self.knight.push_head(*old_piece);
            }
            PhysicalPiece::Knight2 => {
                self.knight.push_tail(*old_piece);
            }
            PhysicalPiece::Lance1 => {
                self.lance.push_head(*old_piece);
            }
            PhysicalPiece::Lance2 => {
                self.lance.push_tail(*old_piece);
            }
            PhysicalPiece::Rook1 => {
                self.rook.push_head(*old_piece);
            }
            PhysicalPiece::Rook2 => {
                self.rook.push_tail(*old_piece);
            }
            PhysicalPiece::Bishop1 => {
                self.bishop.push_head(*old_piece);
            }
            PhysicalPiece::Bishop2 => {
                self.bishop.push_tail(*old_piece);
            }
            PhysicalPiece::Pawn1 => {
                self.pawn.push_head(*old_piece);
            }
            PhysicalPiece::Pawn2 => {
                self.pawn.push_tail(*old_piece);
            }
        }
    }

    /// ゴミ値は消さないぜ☆（＾～＾）
    fn pop(&mut self, phy: PhysicalPiece) -> OldPiece {
        match phy {
            PhysicalPiece::King1 => self.king.pop_head(),
            PhysicalPiece::King2 => self.king.pop_tail(),
            PhysicalPiece::Gold1 => self.gold.pop_head(),
            PhysicalPiece::Gold2 => self.gold.pop_tail(),
            PhysicalPiece::Silver1 => self.silver.pop_head(),
            PhysicalPiece::Silver2 => self.silver.pop_tail(),
            PhysicalPiece::Knight1 => self.knight.pop_head(),
            PhysicalPiece::Knight2 => self.knight.pop_tail(),
            PhysicalPiece::Lance1 => self.lance.pop_head(),
            PhysicalPiece::Lance2 => self.lance.pop_tail(),
            PhysicalPiece::Rook1 => self.rook.pop_head(),
            PhysicalPiece::Rook2 => self.rook.pop_tail(),
            PhysicalPiece::Bishop1 => self.bishop.pop_head(),
            PhysicalPiece::Bishop2 => self.bishop.pop_tail(),
            PhysicalPiece::Pawn1 => self.pawn.pop_head(),
            PhysicalPiece::Pawn2 => self.pawn.pop_tail(),
        }
    }

    fn last(&self, phy: PhysicalPiece) -> Option<&OldPiece> {
        match phy {
            PhysicalPiece::King1 => self.king.last_head(),
            PhysicalPiece::King2 => self.king.last_tail(),
            PhysicalPiece::Gold1 => self.gold.last_head(),
            PhysicalPiece::Gold2 => self.gold.last_tail(),
            PhysicalPiece::Silver1 => self.silver.last_head(),
            PhysicalPiece::Silver2 => self.silver.last_tail(),
            PhysicalPiece::Knight1 => self.knight.last_head(),
            PhysicalPiece::Knight2 => self.knight.last_tail(),
            PhysicalPiece::Lance1 => self.lance.last_head(),
            PhysicalPiece::Lance2 => self.lance.last_tail(),
            PhysicalPiece::Rook1 => self.rook.last_head(),
            PhysicalPiece::Rook2 => self.rook.last_tail(),
            PhysicalPiece::Bishop1 => self.bishop.last_head(),
            PhysicalPiece::Bishop2 => self.bishop.last_tail(),
            PhysicalPiece::Pawn1 => self.pawn.last_head(),
            PhysicalPiece::Pawn2 => self.pawn.last_tail(),
        }
    }

    fn len(&self, phy: PhysicalPiece) -> usize {
        match phy {
            PhysicalPiece::King1 => self.king.len_head(),
            PhysicalPiece::King2 => self.king.len_tail(),
            PhysicalPiece::Gold1 => self.gold.len_head(),
            PhysicalPiece::Gold2 => self.gold.len_tail(),
            PhysicalPiece::Silver1 => self.silver.len_head(),
            PhysicalPiece::Silver2 => self.silver.len_tail(),
            PhysicalPiece::Knight1 => self.knight.len_head(),
            PhysicalPiece::Knight2 => self.knight.len_tail(),
            PhysicalPiece::Lance1 => self.lance.len_head(),
            PhysicalPiece::Lance2 => self.lance.len_tail(),
            PhysicalPiece::Rook1 => self.rook.len_head(),
            PhysicalPiece::Rook2 => self.rook.len_tail(),
            PhysicalPiece::Bishop1 => self.bishop.len_head(),
            PhysicalPiece::Bishop2 => self.bishop.len_tail(),
            PhysicalPiece::Pawn1 => self.pawn.len_head(),
            PhysicalPiece::Pawn2 => self.pawn.len_tail(),
        }
    }

    /*
    fn to_debug(&self, table: &GameTable) -> String {
        let mut buffer = String::new();
        for i in 0..=self.count {
            buffer.push_str(&format!(
                "({}, {:?}) ",
                self.items[i].meaning, self.items[i].num
            ));
        }
        buffer.trim_end().to_string()
    }
    */
}

#[derive(Clone)]
pub struct Hand2Piece {
    items: [OldPiece; 2],
    head: usize,
    tail: usize,
}
impl Default for Hand2Piece {
    /// ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        Hand2Piece {
            items: [OldPiece::default(); 2],
            head: 0,
            tail: 1,
        }
    }
}
impl Hand2Piece {
    pub fn push_head(&mut self, num: OldPiece) {
        self.items[self.head] = num;
        self.head += 1;
    }
    pub fn push_tail(&mut self, num: OldPiece) {
        self.items[self.tail] = num;
        self.tail -= 1;
    }
    pub fn pop_head(&mut self) -> OldPiece {
        let num = self.items[self.head];
        self.head -= 1;
        num
    }
    pub fn pop_tail(&mut self) -> OldPiece {
        let num = self.items[self.tail];
        self.tail += 1;
        num
    }
    pub fn last_head(&self) -> Option<&OldPiece> {
        if 0 < self.head {
            Some(&self.items[self.head - 1])
        } else {
            None
        }
    }
    pub fn last_tail(&self) -> Option<&OldPiece> {
        if self.tail < 1 {
            Some(&self.items[self.tail + 1])
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
    items: [OldPiece; 4],
    head: usize,
    tail: usize,
}
impl Default for Hand4Piece {
    /// ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        Hand4Piece {
            items: [OldPiece::default(); 4],
            head: 0,
            tail: 3,
        }
    }
}
impl Hand4Piece {
    pub fn push_head(&mut self, num: OldPiece) {
        self.items[self.head] = num;
        self.head += 1;
    }
    pub fn push_tail(&mut self, num: OldPiece) {
        self.items[self.tail] = num;
        self.tail -= 1;
    }
    pub fn pop_head(&mut self) -> OldPiece {
        let num = self.items[self.head];
        self.head -= 1;
        num
    }
    pub fn pop_tail(&mut self) -> OldPiece {
        let num = self.items[self.tail];
        self.tail += 1;
        num
    }
    pub fn last_head(&self) -> Option<&OldPiece> {
        if 0 < self.head {
            Some(&self.items[self.head - 1])
        } else {
            None
        }
    }
    pub fn last_tail(&self) -> Option<&OldPiece> {
        if self.tail < 3 {
            Some(&self.items[self.tail + 1])
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
    items: [OldPiece; 18],
    head: usize,
    tail: usize,
}
impl Default for Hand18Piece {
    /// ゴミ値だぜ☆（＾～＾）
    fn default() -> Self {
        Hand18Piece {
            items: [OldPiece::default(); 18],
            head: 0,
            tail: 17,
        }
    }
}
impl Hand18Piece {
    pub fn push_head(&mut self, num: OldPiece) {
        self.items[self.head] = num;
        self.head += 1;
    }
    pub fn push_tail(&mut self, num: OldPiece) {
        self.items[self.tail] = num;
        self.tail -= 1;
    }
    pub fn pop_head(&mut self) -> OldPiece {
        let num = self.items[self.head];
        self.head -= 1;
        num
    }
    pub fn pop_tail(&mut self) -> OldPiece {
        let num = self.items[self.tail];
        self.tail += 1;
        num
    }
    pub fn last_head(&self) -> Option<&OldPiece> {
        if 0 < self.head {
            Some(&self.items[self.head - 1])
        } else {
            None
        }
    }
    pub fn last_tail(&self) -> Option<&OldPiece> {
        if self.tail < 17 {
            Some(&self.items[self.tail + 1])
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
