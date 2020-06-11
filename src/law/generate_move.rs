//!
//! 現局面を使った指し手生成☆（＾～＾）
//!

use crate::cosmic::recording::PHASE_LEN;
use crate::cosmic::recording::{AddressPos, CapturedMove, Movement, Phase};
use crate::cosmic::smart::features::{DoubleFacedPiece, PieceType};
use crate::cosmic::smart::square::{
    AbsoluteAddress2D, Angle, RelAdr2D, FILE_1, FILE_10, RANK_1, RANK_10, RANK_2, RANK_3, RANK_4,
    RANK_6, RANK_7, RANK_8, RANK_9,
};
use crate::cosmic::toy_box::{GameTable, UnifiedAddress};
use crate::spaceship::equipment::Beam;
use std::fmt;

/// ソートを高速にするためのものだぜ☆（＾～＾）
pub struct Ways {
    /// スワップしても割と速いだろ☆（＾～＾）
    pub indexes: Vec<usize>,
    /// こいつをスワップすると遅くなるぜ☆（＾～＾）
    body: Vec<Movement>,
}
impl Ways {
    /// この初期化が遅いかどうかだな☆（＾～＾）
    pub fn new() -> Self {
        Ways {
            indexes: Vec::<usize>::new(),
            body: Vec::<Movement>::new(),
        }
    }
    pub fn push(&mut self, move_: &Movement) {
        self.indexes.push(self.indexes.len());
        self.body.push(*move_);
    }
    /// usize型のコピーなら、オブジェクトのコピーより少しは速いだろ☆（＾～＾）
    pub fn swap(&mut self, a: usize, b: usize) {
        let temp = self.indexes[a];
        self.indexes[a] = self.indexes[b];
        self.indexes[b] = temp;
    }
    pub fn get(&self, index: usize) -> Movement {
        self.body[self.indexes[index]]
    }
    pub fn len(&self) -> usize {
        self.body.len()
    }
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}

#[derive(Clone, Copy)]
pub struct Mobility {
    pub angle: Angle,
    pub agility: Agility,
}
impl Mobility {
    pub fn new(angle: Angle, agility: Agility) -> Self {
        Mobility {
            angle: angle,
            agility: agility,
        }
    }
}

/// Pseudo legal move(疑似合法手)☆（＾～＾）
///
/// 先手の連続王手の千日手とか、空き王手とか、駒を見ただけでは調べられないだろ☆（＾～＾）
/// 棋譜や盤面を見ず、駒だけで調べる合法手が Pseudo legal move だぜ☆（＾～＾）
///
/// 二歩とか、打った後で調べた方が高速になるはずだが、探索部がまだできてないので、指し手生成の中でチェックしているぜ☆（＾～＾）
/// 香を２段目に打たないとか強い将棋を目指すことは　まだやってないぜ☆（＾～＾）
pub struct PseudoLegalMoves {}
impl PseudoLegalMoves {
    ///
    /// 現局面の、任意の移動先升の、
    /// - 盤上の駒の移動
    /// - 打
    /// の指し手を生成。
    ///
    /// 王手回避漏れや、千日手などのチェックは行っていない
    ///
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    ///
    /// Arguments
    /// ---------
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `table` - 現局面の盤上だぜ☆（＾～＾）
    /// * `listen_move` - 指し手を受け取れだぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// F1:
    /// * 指し手ハッシュ
    /// * 移動先にあった駒
    pub fn make_move<F1>(friend: Phase, table: &GameTable, listen_move: &mut F1)
    where
        F1: FnMut(Movement),
    {
        table.for_some_pieces_on_list40(friend, &mut |addr, piece_type| match addr {
            AddressPos::Board(_src_sq) => PseudoLegalMoves::start_on_board(
                friend,
                UnifiedAddress::from_address_pos(friend, &addr),
                piece_type,
                table,
                listen_move,
            ),
            AddressPos::Hand(drop) => {
                PseudoLegalMoves::make_drop(friend, drop, table, listen_move);
            }
        });
    }

    /// 盤上を見ようぜ☆（＾～＾） 盤上の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `piece` - 駒だぜ☆（＾～＾）
    /// * `table` - 現局面の盤上だぜ☆（＾～＾）
    /// * `listen_move` - 指し手を受け取れだぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// F1:
    /// * 指し手ハッシュ
    /// * 移動先にあった駒
    fn start_on_board<F1>(
        friend: Phase,
        source: UnifiedAddress,
        piece_type: PieceType,
        table: &GameTable,
        listen_move: &mut F1,
    ) where
        F1: FnMut(Movement),
    {
        let moving =
            &mut |destination, promotability, _agility, move_permission: Option<MovePermission>| {
                let pseudo_captured_num = table.piece_num_at(&destination);

                let (ok, space) = if let Some(pseudo_captured_num_val) = pseudo_captured_num {
                    if table.get_phase(pseudo_captured_num_val) == friend {
                        // 味方の駒を取った☆（＾～＾）なしだぜ☆（＾～＾）！
                        (false, false)
                    } else {
                        (true, false)
                    }
                } else {
                    (true, true)
                };

                if ok {
                    // 成れるかどうかの判定☆（＾ｑ＾）
                    use crate::law::generate_move::Promotability::*;
                    let promotion = match &promotability {
                        Forced => true,
                        _ => false,
                    };

                    // 成りじゃない場合は、行き先のない動きを制限されるぜ☆（＾～＾）
                    let forbidden = if let Some(move_permission_val) = move_permission {
                        if move_permission_val
                            .check(UnifiedAddress::from_address_pos(friend, &destination))
                        {
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    };

                    match &promotability {
                        Any => {
                            // 成ったり、成れなかったりできるとき。
                            if !forbidden {
                                listen_move(Movement::new(
                                    source.to_address_pos(),
                                    destination,
                                    false,
                                    if let Some(piece_num_val) = pseudo_captured_num {
                                        Some(CapturedMove::new(
                                            UnifiedAddress::from_address_pos(friend, &destination),
                                            table.get_type(piece_num_val),
                                        ))
                                    } else {
                                        None
                                    },
                                ));
                            }
                            listen_move(Movement::new(
                                source.to_address_pos(),
                                destination,
                                true,
                                if let Some(piece_num_val) = pseudo_captured_num {
                                    Some(CapturedMove::new(
                                        UnifiedAddress::from_address_pos(friend, &destination),
                                        table.get_type(piece_num_val),
                                    ))
                                } else {
                                    None
                                },
                            ));
                        }
                        _ => {
                            // 成れるか、成れないかのどちらかのとき。
                            if promotion || !forbidden {
                                listen_move(Movement::new(
                                    source.to_address_pos(),
                                    destination,
                                    promotion,
                                    if let Some(piece_num_val) = pseudo_captured_num {
                                        Some(CapturedMove::new(
                                            UnifiedAddress::from_address_pos(friend, &destination),
                                            table.get_type(piece_num_val),
                                        ))
                                    } else {
                                        None
                                    },
                                ));
                            }
                        }
                    };
                }

                !space
            };

        Area::piece_of(piece_type, friend, source, moving);
    }

    /// 駒台を見ようぜ☆（＾～＾） 駒台の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `table` - 現局面の盤上だぜ☆（＾～＾）
    /// * `listen_move` - 指し手を受け取れだぜ☆（＾～＾）
    /// * `listen_control` - 利きを受け取れだぜ☆（＾～＾）
    fn make_drop<F1>(friend: Phase, drop: DoubleFacedPiece, table: &GameTable, listen_move: &mut F1)
    where
        F1: FnMut(Movement),
    {
        if let Some((piece_type, hand_addr)) = table.last_hand(drop) {
            // 打つぜ☆（＾～＾）
            let drop_fn = &mut |destination: UnifiedAddress| {
                if let None = table.piece_num_at(&destination.to_address_pos()) {
                    // 駒が無いところに打つ
                    use crate::cosmic::smart::features::PieceType::*;
                    match piece_type {
                        Pawn => {
                            // ひよこ　は２歩できない☆（＾～＾）
                            match destination.to_address_pos() {
                                AddressPos::Board(dst_sq) => {
                                    if table.exists_pawn_on_file(friend, dst_sq.file()) {
                                        return;
                                    }
                                }
                                _ => panic!(Beam::trouble(&format!(
                                    "(Err.261) まだ実装してないぜ☆（＾～＾）！",
                                ))),
                            }
                        }
                        _ => {}
                    }
                    listen_move(Movement::new(
                        hand_addr,                    // 打った駒種類
                        destination.to_address_pos(), // どの升へ行きたいか
                        false,                        // 打に成りは無し
                        None,                         // 打で取れる駒無し
                    ));
                }
            };

            // 駒を持っていれば
            let ty = drop.type_();
            use crate::cosmic::smart::features::DoubleFacedPieceType::*;
            match ty {
                // 歩、香
                Pawn | Lance => {
                    // 先手から見た歩、香車の打てる面積だぜ☆（＾～＾）
                    for sq in table.area.drop_pawn_lance[friend as usize].iter() {
                        drop_fn(*sq);
                    }
                }
                // 桂
                Knight => {
                    // 先手から見た桂馬の打てる面積だぜ☆（＾～＾）
                    for sq in table.area.drop_knight[friend as usize].iter() {
                        drop_fn(*sq);
                    }
                }
                // それ以外の駒が打てる範囲は盤面全体。
                _ => {
                    // 全升の面積だぜ☆（＾～＾）駒を打つときに使うぜ☆（＾～＾）
                    for sq in table.area.all_squares[friend as usize].iter() {
                        drop_fn(*sq);
                    }
                }
            }
        }
    }
}

pub fn test_area() {
    let area = Area::default();
    /*
    for friend in 0..2 {
        for i in 0..81 {
            let uni_addr = area.all_squares[friend][i] as usize;
            println!("area: friend={} i={} uni_addr={}", friend, i, uni_addr)
        }
    }
    */
    for friend in 0..2 {
        for i in 0..81 {
            let uni_addr = area.all_squares[friend][i] as usize;
            let expected = friend * 81 + i;
            debug_assert!(
                expected == uni_addr,
                format!(
                    "area: friend={} i={} expected={} uni_addr={}",
                    friend, i, expected, uni_addr
                )
            );
        }
    }
}

/// 次の升☆（＾～＾）
pub struct Area {
    /// 変わっているが、すべてのマスは先後に分かれているぜ☆（＾～＾）
    all_squares: [[UnifiedAddress; 81]; PHASE_LEN],
    drop_pawn_lance: [[UnifiedAddress; 72]; PHASE_LEN],
    drop_knight: [[UnifiedAddress; 63]; PHASE_LEN],
}
impl Default for Area {
    fn default() -> Self {
        fn all_first_sq_fn() -> [UnifiedAddress; 81] {
            let mut v = [UnifiedAddress::default(); 81];
            let mut i = 0;
            for file in FILE_1..FILE_10 {
                for rank in RANK_1..RANK_10 {
                    v[i] = UnifiedAddress::from_address_pos(
                        Phase::First,
                        &AddressPos::Board(AbsoluteAddress2D::new(file, rank)),
                    );
                    i += 1;
                }
            }
            v
        }
        fn all_second_sq_fn() -> [UnifiedAddress; 81] {
            let mut v = [UnifiedAddress::default(); 81];
            let mut i = 0;
            for file in FILE_1..FILE_10 {
                for rank in RANK_1..RANK_10 {
                    v[i] = UnifiedAddress::from_address_pos(
                        Phase::Second,
                        &AddressPos::Board(AbsoluteAddress2D::new(file, rank)),
                    );
                    i += 1;
                }
            }
            v
        }
        fn first_drop_pawn_fn() -> [UnifiedAddress; 72] {
            let mut v = [UnifiedAddress::default(); 72];
            let mut i = 0;
            for rank in RANK_2..RANK_10 {
                for file in (FILE_1..FILE_10).rev() {
                    v[i] = UnifiedAddress::from_address_pos(
                        Phase::First,
                        &AddressPos::Board(AbsoluteAddress2D::new(file, rank)),
                    );
                    i += 1;
                }
            }
            v
        }
        fn second_drop_pawn_fn() -> [UnifiedAddress; 72] {
            let mut v = [UnifiedAddress::default(); 72];
            let mut i = 0;
            for rank in RANK_1..RANK_9 {
                for file in (FILE_1..FILE_10).rev() {
                    v[i] = UnifiedAddress::from_address_pos(
                        Phase::Second,
                        &AddressPos::Board(AbsoluteAddress2D::new(file, rank)),
                    );
                    i += 1;
                }
            }
            v
        }
        fn first_drop_knight_fn() -> [UnifiedAddress; 63] {
            let mut v = [UnifiedAddress::default(); 63];
            let mut i = 0;
            for rank in RANK_3..RANK_10 {
                for file in (FILE_1..FILE_10).rev() {
                    v[i] = UnifiedAddress::from_address_pos(
                        Phase::First,
                        &AddressPos::Board(AbsoluteAddress2D::new(file, rank)),
                    );
                    i += 1;
                }
            }
            v
        }
        fn second_drop_knight_fn() -> [UnifiedAddress; 63] {
            let mut v = [UnifiedAddress::default(); 63];
            let mut i = 0;
            for rank in RANK_3..RANK_10 {
                for file in (FILE_1..FILE_10).rev() {
                    v[i] = UnifiedAddress::from_address_pos(
                        Phase::Second,
                        &AddressPos::Board(AbsoluteAddress2D::new(file, rank).rotate_180()),
                    );
                    i += 1;
                }
            }
            v
        }

        Area {
            all_squares: [all_first_sq_fn(), all_second_sq_fn()],
            drop_pawn_lance: [first_drop_pawn_fn(), second_drop_pawn_fn()],
            drop_knight: [first_drop_knight_fn(), second_drop_knight_fn()],
        }
    }
}
impl Area {
    /// 先手から見た盤上の駒の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `piece_type` - 駒の種類だぜ☆（＾～＾）
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `hopping` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    /// * `sliding` -
    fn piece_of<F1>(piece_type: PieceType, friend: Phase, source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        match piece_type {
            PieceType::Pawn => Area::pawn(friend, source, moving),
            PieceType::Lance => Area::lance(friend, source, moving),
            PieceType::Knight => Area::knight(friend, source, moving),
            PieceType::Silver => Area::silver(friend, source, moving),
            PieceType::Gold => Area::gold(friend, source, moving),
            PieceType::King => Area::king(source, moving),
            PieceType::Bishop => Area::bishop(friend, source, moving),
            PieceType::Rook => Area::rook(friend, source, moving),
            PieceType::PromotedPawn => Area::gold(friend, source, moving),
            PieceType::PromotedLance => Area::gold(friend, source, moving),
            PieceType::PromotedKnight => Area::gold(friend, source, moving),
            PieceType::PromotedSilver => Area::gold(friend, source, moving),
            PieceType::Horse => Area::horse(source, moving),
            PieceType::Dragon => Area::dragon(source, moving),
        }
    }

    /// 先手から見た盤上の歩の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn pawn<F1>(friend: Phase, source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::pawn_lance(
                friend,
                UnifiedAddress::from_address_pos(friend, &destination),
                moving,
                Some(MovePermission::from_pawn_or_lance(friend)),
            )
        };

        for mobility in PieceType::Pawn.mobility().iter() {
            Area::move_(&Some(friend), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の香の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn lance<F1>(friend: Phase, source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::pawn_lance(
                friend,
                UnifiedAddress::from_address_pos(friend, &destination),
                moving,
                Some(MovePermission::from_pawn_or_lance(friend)),
            )
        };

        for mobility in PieceType::Lance.mobility().iter() {
            Area::move_(&Some(friend), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の桂の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn knight<F1>(friend: Phase, source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::knight(
                friend,
                UnifiedAddress::from_address_pos(friend, &destination),
                moving,
                Some(MovePermission::from_knight(friend)),
            )
        };

        for mobility in PieceType::Knight.mobility().iter() {
            Area::move_(&Some(friend), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の銀の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn silver<F1>(friend: Phase, source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::silver(
                friend,
                source,
                UnifiedAddress::from_address_pos(friend, &destination),
                moving,
            )
        };

        for mobility in PieceType::Silver.mobility().iter() {
            Area::move_(&Some(friend), source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の金、と、杏、圭、全の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` - 後手視点にしたけりゃ friend.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn gold<F1>(friend: Phase, source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            moving(destination, Promotability::Deny, Agility::Hopping, None)
        };

        for mobility in PieceType::Gold.mobility().iter() {
            Area::move_(&Some(friend), source, *mobility, moving);
        }
    }

    /// 盤上の玉の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn king<F1>(source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            moving(destination, Promotability::Deny, Agility::Hopping, None)
        };

        for mobility in PieceType::King.mobility().iter() {
            Area::move_(&None, source, *mobility, moving);
        }
    }

    /// 盤上の角の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn bishop<F1>(friend: Phase, source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::bishop_rook(
                friend,
                source,
                UnifiedAddress::from_address_pos(friend, &destination),
                moving,
            )
        };
        for mobility in PieceType::Bishop.mobility().iter() {
            Area::move_(&Some(friend), source, *mobility, moving);
        }
    }

    /// 盤上の飛の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn rook<F1>(friend: Phase, source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination, _agility| {
            Promoting::bishop_rook(
                friend,
                source,
                UnifiedAddress::from_address_pos(friend, &destination),
                moving,
            )
        };
        for mobility in PieceType::Rook.mobility().iter() {
            Area::move_(&Some(friend), source, *mobility, moving);
        }
    }

    /// 盤上の馬の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn horse<F1>(source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving =
            &mut |destination, agility| moving(destination, Promotability::Deny, agility, None);

        for mobility in PieceType::Horse.mobility().iter() {
            Area::move_(&None, source, *mobility, moving);
        }
    }

    /// 盤上の竜の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn dragon<F1>(source: UnifiedAddress, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        {
            let moving =
                &mut |destination, agility| moving(destination, Promotability::Deny, agility, None);

            for mobility in PieceType::Dragon.mobility().iter() {
                Area::move_(&None, source, *mobility, moving);
            }
        }
    }

    /// 盤上の駒を指すぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `friend` - 先手か後手か、関係ないか☆（＾～＾）先後同型なら関係ないしな☆（＾～＾）
    /// * `start` - 移動元升☆（＾～＾）
    /// * `angle` - 角度☆（＾～＾）
    /// * `agility` - 動き方☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    fn move_<F1>(friend: &Option<Phase>, start: UnifiedAddress, mobility: Mobility, moving: &mut F1)
    where
        F1: FnMut(AddressPos, Agility) -> bool,
    {
        let angle = if let Some(friend_val) = friend {
            // 先後同型でない駒は、後手なら１８０°回転だぜ☆（＾～＾）
            if *friend_val == Phase::Second {
                mobility.angle.rotate180()
            } else {
                mobility.angle
            }
        } else {
            // 先後同型だからそのままだぜ☆（＾～＾）
            mobility.angle
        };

        match start.to_address_pos() {
            AddressPos::Board(start_sq) => {
                match mobility.agility {
                    Agility::Sliding => {
                        let mut cur = start_sq.clone();
                        let r = RelAdr2D::new(1, 0).rotate(angle).clone();
                        loop {
                            // 西隣から反時計回りだぜ☆（＾～＾）
                            if cur.offset(&r).wall() {
                                break;
                            }
                            if moving(AddressPos::Board(cur), mobility.agility) {
                                break;
                            }
                        }
                    }
                    // 桂馬専用☆（＾～＾）行き先の無いところに置いてないはずだぜ☆（＾～＾）
                    Agility::Knight => {
                        let mut cur = start_sq.clone();
                        // 西隣から反時計回りだぜ☆（＾～＾）
                        if !cur.offset(&angle.west_ccw_double_rank()).wall() {
                            moving(AddressPos::Board(cur), mobility.agility);
                        }
                    }
                    Agility::Hopping => {
                        let mut cur = start_sq.clone();
                        // 西隣から反時計回りだぜ☆（＾～＾）
                        if !cur.offset(&angle.west_ccw()).wall() {
                            moving(AddressPos::Board(cur), mobility.agility);
                        }
                    }
                }
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.641) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
}

/// 機敏性。
#[derive(Clone, Copy, Debug)]
pub enum Agility {
    /// 隣へ１つ進む駒。
    Hopping,
    /// 長い利き。
    Sliding,
    /// 桂馬。
    Knight,
}

enum Promotability {
    /// 成ることはできないぜ☆（＾～＾）
    Deny,
    /// 成る、成らない両方あるぜ☆（＾～＾）
    Any,
    /// 必ず成れだぜ☆（＾～＾）
    Forced,
}

/// 行き先があるかないかのチェックに使うぜ☆（＾～＾）
/// 成れるときは使わないぜ☆（＾～＾）
struct MovePermission {
    min_rank: usize,
    max_rank: usize,
}
impl MovePermission {
    fn from_pawn_or_lance(friend: Phase) -> Self {
        // ▲P,▲L　は１段目(▽P,▽L　は９段目)には進めない
        match friend {
            Phase::First => MovePermission {
                min_rank: 2,
                max_rank: 9,
            },
            Phase::Second => MovePermission {
                min_rank: 1,
                max_rank: 8,
            },
        }
    }
    fn from_knight(friend: Phase) -> Self {
        // ▲N　は１、２段目(▽N　は８、９段目)には進めない
        match friend {
            Phase::First => MovePermission {
                min_rank: 3,
                max_rank: 9,
            },
            Phase::Second => MovePermission {
                min_rank: 1,
                max_rank: 7,
            },
        }
    }
    fn check(&self, dst_addr: UnifiedAddress) -> bool {
        match dst_addr.to_address_pos() {
            AddressPos::Board(dst_sq) => {
                if dst_sq.rank() < self.min_rank || self.max_rank < dst_sq.rank() {
                    return false;
                }
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.727) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
        true
    }
}
impl fmt::Debug for MovePermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(rank{}~{})", self.min_rank, self.max_rank)
    }
}

/// 成れるか、成れないか☆（＾～＾）
struct Promoting {}
impl Promoting {
    /// 歩と香のための、成れるか成れないか判定だぜ☆（＾～＾）！
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destinaion` -
    /// * `callback` -
    /// * `move_permission` - 成らずに一番奥の段に移動することはできません。
    fn pawn_lance<F1>(
        friend: Phase,
        destinaion: UnifiedAddress,
        callback: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_farthest_rank_from_friend(friend, destinaion) {
            // 自陣から見て一番奥の段
            callback(
                destinaion.to_address_pos(),
                Promotability::Forced,
                Agility::Hopping,
                move_permission,
            )
        } else if Promoting::is_second_third_farthest_rank_from_friend(friend, destinaion) {
            // 自陣から見て二番、三番目の奥の段
            callback(
                destinaion.to_address_pos(),
                Promotability::Any,
                Agility::Hopping,
                move_permission,
            )
        } else {
            callback(
                destinaion.to_address_pos(),
                Promotability::Deny,
                Agility::Hopping,
                move_permission,
            )
        }
    }

    /// 桂のための、成れるか成れないか判定だぜ☆（＾～＾）！
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destinaion` -
    /// * `callback` -
    /// * `move_permission` - 成らずに奥から２番目の段に移動することはできません。
    fn knight<F1>(
        friend: Phase,
        destination: UnifiedAddress,
        callback: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_first_second_farthest_rank_from_friend(friend, destination) {
            callback(
                destination.to_address_pos(),
                Promotability::Forced,
                Agility::Knight,
                move_permission,
            )
        } else if Promoting::is_third_farthest_rank_from_friend(friend, destination) {
            callback(
                destination.to_address_pos(),
                Promotability::Any,
                Agility::Knight,
                move_permission,
            )
        } else {
            callback(
                destination.to_address_pos(),
                Promotability::Deny,
                Agility::Knight,
                move_permission,
            )
        }
    }

    /// 銀のための、成れるか成れないか判定だぜ☆（＾～＾）！
    /// 自陣から見て奥から１～３段目に入るときに成れます。元位置が３段目のときは、動けば成るか選べます。
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `source` -
    /// * `destination` -
    /// * `callback` -
    fn silver<F1>(
        friend: Phase,
        source: UnifiedAddress,
        destination: UnifiedAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_third_farthest_rank_from_friend(friend, source) {
            callback(
                destination.to_address_pos(),
                Promotability::Any,
                Agility::Hopping,
                None,
            )
        } else if Promoting::is_opponent_region(friend, destination) {
            callback(
                destination.to_address_pos(),
                Promotability::Any,
                Agility::Hopping,
                None,
            )
        } else {
            callback(
                destination.to_address_pos(),
                Promotability::Deny,
                Agility::Hopping,
                None,
            )
        }
    }

    /// 角と飛のための、成れるか成れないか判定だぜ☆（＾～＾）！
    /// 非敵陣にいるとき、敵陣で成れます。敵陣にいるとき、どこでも成れます。
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `source` -
    /// * `destination` -
    /// * `callback` -
    fn bishop_rook<F1>(
        friend: Phase,
        source: UnifiedAddress,
        destination: UnifiedAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(AddressPos, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_opponent_region(friend, source)
            || Promoting::is_opponent_region(friend, destination)
        {
            callback(
                destination.to_address_pos(),
                Promotability::Any,
                Agility::Sliding,
                None,
            )
        } else {
            callback(
                destination.to_address_pos(),
                Promotability::Deny,
                Agility::Sliding,
                None,
            )
        }
    }

    /// 自陣から見て、一番遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_farthest_rank_from_friend(friend: Phase, destination: UnifiedAddress) -> bool {
        match destination.to_address_pos() {
            AddressPos::Board(dst_sq) => {
                (friend == Phase::First && dst_sq.rank() < RANK_2)
                    || (friend == Phase::Second && RANK_8 < dst_sq.rank())
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 自陣から見て、一番目、２番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_first_second_farthest_rank_from_friend(
        friend: Phase,
        destination: UnifiedAddress,
    ) -> bool {
        match destination.to_address_pos() {
            AddressPos::Board(dst_sq) => {
                (friend == Phase::First && dst_sq.rank() < RANK_3)
                    || (friend == Phase::Second && RANK_7 < dst_sq.rank())
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.919) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 自陣から見て、二番目、三番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_second_third_farthest_rank_from_friend(
        friend: Phase,
        destination: UnifiedAddress,
    ) -> bool {
        match destination.to_address_pos() {
            AddressPos::Board(dst_sq) => {
                (friend == Phase::First && RANK_1 < dst_sq.rank() && dst_sq.rank() < RANK_4)
                    || (friend == Phase::Second && RANK_6 < dst_sq.rank() && dst_sq.rank() < RANK_9)
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.937) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 自陣から見て、三番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_third_farthest_rank_from_friend(friend: Phase, destination: UnifiedAddress) -> bool {
        match destination.to_address_pos() {
            AddressPos::Board(dst_sq) => {
                (friend == Phase::First && dst_sq.rank() == RANK_3)
                    || (friend == Phase::Second && RANK_7 == dst_sq.rank())
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.946) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 敵陣
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_opponent_region(friend: Phase, destination: UnifiedAddress) -> bool {
        match destination.to_address_pos() {
            AddressPos::Board(dst_sq) => {
                (friend == Phase::First && dst_sq.rank() < RANK_4)
                    || (friend == Phase::Second && RANK_6 < dst_sq.rank())
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.957) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
}
