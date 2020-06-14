//!
//! 現局面を使った指し手生成☆（＾～＾）
//!

use crate::cosmic::recording::{CapturedMove, FireAddress, Movement, Phase};
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::smart::square::FILE10U8;
use crate::cosmic::smart::square::FILE1U8;
use crate::cosmic::smart::square::RANK10U8;
use crate::cosmic::smart::square::RANK1U8;
use crate::cosmic::smart::square::RANK2U8;
use crate::cosmic::smart::square::RANK3U8;
use crate::cosmic::smart::square::RANK4U8;
use crate::cosmic::smart::square::RANK6U8;
use crate::cosmic::smart::square::RANK7U8;
use crate::cosmic::smart::square::RANK8U8;
use crate::cosmic::smart::square::RANK9U8;
use crate::cosmic::smart::square::{AbsoluteAddress2D, Angle, RelAdr2D};
use crate::cosmic::toy_box::GameTable;
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
pub struct PseudoLegalMoves {
    /// 指し手生成中に手番が変わることは無いんで☆（＾～＾）
    friend: Phase,
}
impl PseudoLegalMoves {
    pub fn new(friend: Phase) -> Self {
        PseudoLegalMoves { friend: friend }
    }
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
    pub fn make_move<F1>(&self, table: &GameTable, listen_move: &mut F1)
    where
        F1: FnMut(Movement),
    {
        table.for_some_pieces_on_list40(
            self.friend,
            // 移動元と、その駒の種類。
            &mut |src_fire: &FireAddress| self.start(src_fire, table, listen_move),
        );
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
    fn start<F1>(&self, source: &FireAddress, table: &GameTable, listen_move: &mut F1)
    where
        F1: FnMut(Movement),
    {
        match source {
            FireAddress::Board(_src_sq) => {
                let piece_type = table.get_type(table.piece_num_at(self.friend, &source).unwrap());

                let moving =
                    &mut |destination: &FireAddress,
                          promotability,
                          _agility,
                          move_permission: Option<MovePermission>| {
                        let pseudo_captured_num = table.piece_num_at(self.friend, &destination);

                        let space = if let Some(pseudo_captured_num_val) = pseudo_captured_num {
                            if table.get_phase(pseudo_captured_num_val) == self.friend {
                                // 味方の駒を取った☆（＾～＾）なしだぜ☆（＾～＾）！
                                // 真を返して処理を中断だぜ☆（＾～＾）！
                                return true;
                            } else {
                                false
                            }
                        } else {
                            true
                        };

                        // 成れるかどうかの判定☆（＾ｑ＾）
                        use crate::law::generate_move::Promotability::*;
                        let promotion = match &promotability {
                            Forced => true,
                            _ => false,
                        };

                        // 成りじゃない場合は、行き先のない動きを制限されるぜ☆（＾～＾）
                        let forbidden = if let Some(move_permission_val) = move_permission {
                            if move_permission_val.check(&destination) {
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
                                        table.piece_num_at(self.friend, &source).unwrap(),
                                        *source,
                                        *destination,
                                        false,
                                        if let Some(piece_num_val) = pseudo_captured_num {
                                            Some(CapturedMove::new(
                                                *destination,
                                                FireAddress::Hand(
                                                    table
                                                        .get_double_faced_piece_type(piece_num_val),
                                                ),
                                            ))
                                        } else {
                                            None
                                        },
                                    ));
                                }
                                listen_move(Movement::new(
                                    table.piece_num_at(self.friend, &source).unwrap(),
                                    *source,
                                    *destination,
                                    true,
                                    if let Some(piece_num_val) = pseudo_captured_num {
                                        Some(CapturedMove::new(
                                            *destination,
                                            FireAddress::Hand(
                                                table.get_double_faced_piece_type(piece_num_val),
                                            ),
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
                                        table.piece_num_at(self.friend, &source).unwrap(),
                                        *source,
                                        *destination,
                                        promotion,
                                        if let Some(piece_num_val) = pseudo_captured_num {
                                            Some(CapturedMove::new(
                                                *destination,
                                                FireAddress::Hand(
                                                    table
                                                        .get_double_faced_piece_type(piece_num_val),
                                                ),
                                            ))
                                        } else {
                                            None
                                        },
                                    ));
                                }
                            }
                        };

                        !space
                    };
                self.piece_of(piece_type, source, moving);
            }
            FireAddress::Hand(src_drop_type) => {
                if let Some((piece_type, fire_hand)) = table.last_hand(self.friend, &source) {
                    // 打つぜ☆（＾～＾）
                    let drop_fn = &mut |destination: &FireAddress| {
                        if let None = table.piece_num_at(self.friend, &destination) {
                            // 駒が無いところに打つ
                            use crate::cosmic::smart::features::PieceType::*;
                            match piece_type {
                                Pawn => {
                                    match destination {
                                        FireAddress::Board(sq) => {
                                            // ひよこ　は２歩できない☆（＾～＾
                                            if table.exists_pawn_on_file(self.friend, sq.file()) {
                                                return;
                                            }
                                        }
                                        _ => panic!(Beam::trouble(&format!(
                                            "(Err.641) 盤上じゃなかったぜ☆（＾～＾）！",
                                        ))),
                                    }
                                }
                                _ => {}
                            }
                            listen_move(Movement::new(
                                table.piece_num_at(self.friend, &fire_hand).unwrap(),
                                fire_hand,    // 打った駒種類
                                *destination, // どの升へ行きたいか
                                false,        // 打に成りは無し
                                None,         // 打で取れる駒無し
                            ));
                        }
                    };
                    // 駒を持っていれば
                    use crate::cosmic::smart::features::DoubleFacedPieceType::*;
                    match src_drop_type {
                        // 歩、香
                        Pawn | Lance => {
                            // 先手から見た歩、香車の打てる面積だぜ☆（＾～＾）
                            for sq in table.area.drop_pawn_lance.iter() {
                                drop_fn(sq);
                            }
                        }
                        // 桂
                        Knight => {
                            // 先手から見た桂馬の打てる面積だぜ☆（＾～＾）
                            for sq in table.area.drop_knight.iter() {
                                drop_fn(sq);
                            }
                        }
                        // それ以外の駒が打てる範囲は盤面全体。
                        _ => {
                            // 全升の面積だぜ☆（＾～＾）駒を打つときに使うぜ☆（＾～＾）
                            for sq in table.area.all_squares.iter() {
                                drop_fn(sq);
                            }
                        }
                    }
                }
            }
        }
    }

    /// 先手から見た盤上の駒の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `piece_type` - 駒の種類だぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `hopping` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    /// * `sliding` -
    fn piece_of<F1>(&self, piece_type: PieceType, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        match piece_type {
            PieceType::Pawn => self.pawn(source, moving),
            PieceType::Lance => self.lance(source, moving),
            PieceType::Knight => self.knight(source, moving),
            PieceType::Silver => self.silver(source, moving),
            PieceType::Gold => self.gold(source, moving),
            PieceType::King => self.king(source, moving),
            PieceType::Bishop => self.bishop(source, moving),
            PieceType::Rook => self.rook(source, moving),
            PieceType::PromotedPawn => self.gold(source, moving),
            PieceType::PromotedLance => self.gold(source, moving),
            PieceType::PromotedKnight => self.gold(source, moving),
            PieceType::PromotedSilver => self.gold(source, moving),
            PieceType::Horse => self.horse(source, moving),
            PieceType::Dragon => self.dragon(source, moving),
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
    fn pawn<F1>(&self, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            self.promote_pawn_lance(
                destination,
                moving,
                Some(MovePermission::from_pawn_or_lance(self.friend)),
            )
        };

        for mobility in PieceType::Pawn.mobility().iter() {
            self.move_(source, *mobility, moving);
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
    fn lance<F1>(&self, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            self.promote_pawn_lance(
                destination,
                moving,
                Some(MovePermission::from_pawn_or_lance(self.friend)),
            )
        };

        for mobility in PieceType::Lance.mobility().iter() {
            self.move_(source, *mobility, moving);
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
    fn knight<F1>(&self, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            self.promote_knight(
                destination,
                moving,
                Some(MovePermission::from_knight(self.friend)),
            )
        };

        for mobility in PieceType::Knight.mobility().iter() {
            self.move_(source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の銀の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn silver<F1>(&self, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            self.promote_silver(&source, destination, moving)
        };

        for mobility in PieceType::Silver.mobility().iter() {
            self.move_(source, *mobility, moving);
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
    fn gold<F1>(&self, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            moving(destination, Promotability::Deny, Agility::Hopping, None)
        };

        for mobility in PieceType::Gold.mobility().iter() {
            self.move_(source, *mobility, moving);
        }
    }

    /// 盤上の玉の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn king<F1>(&self, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            moving(destination, Promotability::Deny, Agility::Hopping, None)
        };

        for mobility in PieceType::King.mobility().iter() {
            self.move_(source, *mobility, moving);
        }
    }

    /// 盤上の角の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn bishop<F1>(&self, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            self.promote_bishop_rook(source, destination, moving)
        };
        for mobility in PieceType::Bishop.mobility().iter() {
            self.move_(source, *mobility, moving);
        }
    }

    /// 盤上の飛の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn rook<F1>(&self, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            self.promote_bishop_rook(source, destination, moving)
        };
        for mobility in PieceType::Rook.mobility().iter() {
            self.move_(source, *mobility, moving);
        }
    }

    /// 盤上の馬の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn horse<F1>(&self, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, agility| {
            moving(destination, Promotability::Deny, agility, None)
        };

        for mobility in PieceType::Horse.mobility().iter() {
            self.move_(source, *mobility, moving);
        }
    }

    /// 盤上の竜の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn dragon<F1>(&self, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, agility| {
            moving(destination, Promotability::Deny, agility, None)
        };

        for mobility in PieceType::Dragon.mobility().iter() {
            self.move_(source, *mobility, moving);
        }
    }

    /// 盤上の駒を指すぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `friend` - 先手か後手か、関係ないか☆（＾～＾）先後同型でも必要なのが変わってるだろ☆（＾～＾）
    /// * `start` - 移動元升☆（＾～＾）
    /// * `angle` - 角度☆（＾～＾）
    /// * `agility` - 動き方☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    fn move_<F1>(&self, start: &FireAddress, mobility: Mobility, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Agility) -> bool,
    {
        let angle =
            // 後手なら１８０°回転だぜ☆（＾～＾）
            if self.friend == Phase::Second {
                mobility.angle.rotate180()
            } else {
                mobility.angle
            };

        match start {
            FireAddress::Board(start_sq) => {
                match mobility.agility {
                    Agility::Sliding => {
                        let mut cur = start_sq.clone();
                        let r = RelAdr2D::new(1, 0).rotate(angle).clone();
                        loop {
                            // 西隣から反時計回りだぜ☆（＾～＾）
                            if cur.offset(&r).wall() {
                                break;
                            }
                            if moving(&FireAddress::Board(cur), mobility.agility) {
                                break;
                            }
                        }
                    }
                    // 桂馬専用☆（＾～＾）行き先の無いところに置いてないはずだぜ☆（＾～＾）
                    Agility::Knight => {
                        let mut cur = start_sq.clone();
                        // 西隣から反時計回りだぜ☆（＾～＾）
                        if !cur.offset(&angle.west_ccw_double_rank()).wall() {
                            moving(&FireAddress::Board(cur), mobility.agility);
                        }
                    }
                    Agility::Hopping => {
                        let mut cur = start_sq.clone();
                        // 西隣から反時計回りだぜ☆（＾～＾）
                        if !cur.offset(&angle.west_ccw()).wall() {
                            moving(&FireAddress::Board(cur), mobility.agility);
                        }
                    }
                }
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.641) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }

    /// 歩と香のための、成れるか成れないか判定だぜ☆（＾～＾）！
    ///
    /// Arguments
    /// ---------
    ///
    /// * `destinaion` -
    /// * `callback` -
    /// * `move_permission` - 成らずに一番奥の段に移動することはできません。
    fn promote_pawn_lance<F1>(
        &self,
        destination: &FireAddress,
        callback: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if self.is_promote_farthest_rank_from_friend(destination) {
            // 自陣から見て一番奥の段
            callback(
                destination,
                Promotability::Forced,
                Agility::Hopping,
                move_permission,
            )
        } else if self.is_promote_second_third_farthest_rank_from_friend(destination) {
            // 自陣から見て二番、三番目の奥の段
            callback(
                destination,
                Promotability::Any,
                Agility::Hopping,
                move_permission,
            )
        } else {
            callback(
                destination,
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
    /// * `destinaion` -
    /// * `callback` -
    /// * `move_permission` - 成らずに奥から２番目の段に移動することはできません。
    fn promote_knight<F1>(
        &self,
        destination: &FireAddress,
        callback: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if self.is_promote_first_second_farthest_rank_from_friend(destination) {
            callback(
                destination,
                Promotability::Forced,
                Agility::Knight,
                move_permission,
            )
        } else if self.is_promote_third_farthest_rank_from_friend(destination) {
            callback(
                destination,
                Promotability::Any,
                Agility::Knight,
                move_permission,
            )
        } else {
            callback(
                destination,
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
    /// * `source` -
    /// * `destination` -
    /// * `callback` -
    fn promote_silver<F1>(
        &self,
        source: &FireAddress,
        destination: &FireAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if self.is_promote_third_farthest_rank_from_friend(source)
            || self.is_promote_opponent_region(destination)
        {
            callback(destination, Promotability::Any, Agility::Hopping, None)
        } else {
            callback(destination, Promotability::Deny, Agility::Hopping, None)
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
    fn promote_bishop_rook<F1>(
        &self,
        source: &FireAddress,
        destination: &FireAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if self.is_promote_opponent_region(source) || self.is_promote_opponent_region(destination) {
            callback(destination, Promotability::Any, Agility::Sliding, None)
        } else {
            callback(destination, Promotability::Deny, Agility::Sliding, None)
        }
    }

    /// 自陣から見て、一番遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `friend` -
    /// * `destination` -
    fn is_promote_farthest_rank_from_friend(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => match self.friend {
                Phase::First => dst_sq.rank() < RANK2U8,
                Phase::Second => RANK8U8 < dst_sq.rank(),
            },
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
    fn is_promote_first_second_farthest_rank_from_friend(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => match self.friend {
                Phase::First => dst_sq.rank() < RANK3U8,
                Phase::Second => RANK7U8 < dst_sq.rank(),
            },
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
    fn is_promote_second_third_farthest_rank_from_friend(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => match self.friend {
                Phase::First => RANK1U8 < dst_sq.rank() && dst_sq.rank() < RANK4U8,
                Phase::Second => RANK6U8 < dst_sq.rank() && dst_sq.rank() < RANK9U8,
            },
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
    fn is_promote_third_farthest_rank_from_friend(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => match self.friend {
                Phase::First => dst_sq.rank() == RANK3U8,
                Phase::Second => RANK7U8 == dst_sq.rank(),
            },
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
    fn is_promote_opponent_region(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => match self.friend {
                Phase::First => dst_sq.rank() < RANK4U8,
                Phase::Second => RANK6U8 < dst_sq.rank(),
            },
            _ => panic!(Beam::trouble(&format!(
                "(Err.957) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
}

/// 次の升☆（＾～＾）
pub struct Area {
    /// 変わっているが、すべてのマスは先後に分かれているぜ☆（＾～＾）
    all_squares: [FireAddress; 81],
    drop_pawn_lance: [FireAddress; 72],
    drop_knight: [FireAddress; 63],
}
impl Default for Area {
    fn default() -> Self {
        fn all_sq_fn() -> [FireAddress; 81] {
            let mut v = [FireAddress::default(); 81];
            let mut i = 0;
            for file in FILE1U8..FILE10U8 {
                for rank in RANK1U8..RANK10U8 {
                    v[i] = FireAddress::Board(AbsoluteAddress2D::new(file, rank));
                    i += 1;
                }
            }
            v
        }
        fn drop_pawn_fn() -> [FireAddress; 72] {
            let mut v = [FireAddress::default(); 72];
            let mut i = 0;
            for rank in RANK2U8..RANK10U8 {
                for file in (FILE1U8..FILE10U8).rev() {
                    v[i] = FireAddress::Board(AbsoluteAddress2D::new(file, rank));
                    i += 1;
                }
            }
            v
        }
        fn drop_knight_fn() -> [FireAddress; 63] {
            let mut v = [FireAddress::default(); 63];
            let mut i = 0;
            for rank in RANK3U8..RANK10U8 {
                for file in (FILE1U8..FILE10U8).rev() {
                    v[i] = FireAddress::Board(AbsoluteAddress2D::new(file, rank));
                    i += 1;
                }
            }
            v
        }

        Area {
            all_squares: all_sq_fn(),
            drop_pawn_lance: drop_pawn_fn(),
            drop_knight: drop_knight_fn(),
        }
    }
}
impl Area {}

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
    min_rank: u8,
    max_rank: u8,
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
    fn check(&self, dst_fire: &FireAddress) -> bool {
        match dst_fire {
            FireAddress::Board(sq) => {
                if sq.rank() < self.min_rank || self.max_rank < sq.rank() {
                    return false;
                }
                true
            }
            FireAddress::Hand(_drop_type) => panic!(Beam::trouble(&format!(
                "(Err.546) 盤上ではなかったぜ☆（＾～＾）！",
            ))),
        }
    }
}
impl fmt::Debug for MovePermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(rank{}~{})", self.min_rank, self.max_rank)
    }
}
