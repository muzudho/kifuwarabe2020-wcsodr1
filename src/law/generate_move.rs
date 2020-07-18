//!
//! 現局面を使った指し手生成☆（＾～＾）
//!

use crate::cosmic::recording::{CapturedMove, FireAddress, HandAddress, Movement, Phase};
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
use crate::cosmic::smart::square::RANK9U8;
use crate::cosmic::smart::square::{AbsoluteAddress2D, Angle, RelAdr2D};
use crate::log::LogExt;
use crate::position::Position;
use casual_logger::Log;

/// 先手、後手で処理が変わるやつを吸収するぜ☆（＾～＾）
pub trait PhaseOperation {
    /// 先手から見て、４、５、６、７、８、９段目かどうか☆（＾～＾）
    fn is_rank456789(&self, destination: &FireAddress) -> bool;
    /// 先手から見て、１、２、３段目かどうか☆（＾～＾）いわゆる敵陣だぜ☆（＾～＾）
    fn is_rank123(&self, destination: &FireAddress) -> bool;
    /// 先手から見て、１、２段目かどうか☆（＾～＾）
    fn is_rank12(&self, destination: &FireAddress) -> bool;
    /// 先手から見て、２、３段目かどうか☆（＾～＾）
    fn is_rank23(&self, destination: &FireAddress) -> bool;
    /// 先手から見て、１段目かどうか☆（＾～＾）
    fn is_rank1(&self, destination: &FireAddress) -> bool;
    /// 先手から見て、３段目かどうか☆（＾～＾）
    fn is_rank3(&self, destination: &FireAddress) -> bool;
    /// 移動可能かどうか判定するぜ☆（＾～＾）
    fn check_permission(&self, dst_fire: &FireAddress, permission_type: PermissionType) -> bool;
}
pub struct FirstOperation {}
pub struct SecondOperation {}
impl Default for FirstOperation {
    fn default() -> Self {
        FirstOperation {}
    }
}
impl Default for SecondOperation {
    fn default() -> Self {
        SecondOperation {}
    }
}
impl PhaseOperation for FirstOperation {
    fn is_rank456789(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => RANK3U8 < dst_sq.rank(),
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn is_rank123(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => dst_sq.rank() < RANK4U8,
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn is_rank12(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => dst_sq.rank() < RANK3U8,
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn is_rank23(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => RANK1U8 < dst_sq.rank() && dst_sq.rank() < RANK4U8,
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn is_rank1(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => dst_sq.rank() == RANK1U8,
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn is_rank3(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => dst_sq.rank() == RANK3U8,
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn check_permission(&self, dst_fire: &FireAddress, permission_type: PermissionType) -> bool {
        match permission_type {
            PermissionType::PawnLance => match dst_fire {
                FireAddress::Board(sq) => {
                    // １段目には侵入できないぜ☆（＾～＾）
                    if sq.rank() < 2 {
                        return false;
                    }
                    true
                }
                FireAddress::Hand(_drop_type) => panic!(Log::panic(&format!(
                    "(Err.546) 盤上ではなかったぜ☆（＾～＾）！",
                ))),
            },
            PermissionType::Knight => match dst_fire {
                FireAddress::Board(sq) => {
                    // １、２段目には侵入できないぜ☆（＾～＾）
                    if sq.rank() < 3 {
                        return false;
                    }
                    true
                }
                FireAddress::Hand(_drop_type) => panic!(Log::panic(&format!(
                    "(Err.546) 盤上ではなかったぜ☆（＾～＾）！",
                ))),
            },
        }
    }
}
impl PhaseOperation for SecondOperation {
    fn is_rank456789(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => dst_sq.rank() < RANK7U8,
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn is_rank123(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => RANK6U8 < dst_sq.rank(),
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn is_rank12(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => RANK7U8 < dst_sq.rank(),
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn is_rank23(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => RANK6U8 < dst_sq.rank() && dst_sq.rank() < RANK9U8,
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn is_rank1(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => RANK9U8 == dst_sq.rank(),
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn is_rank3(&self, destination: &FireAddress) -> bool {
        match destination {
            FireAddress::Board(dst_sq) => RANK7U8 == dst_sq.rank(),
            _ => panic!(Log::panic(&format!(
                "(Err.905) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    fn check_permission(&self, dst_fire: &FireAddress, permission_type: PermissionType) -> bool {
        match permission_type {
            PermissionType::PawnLance => match dst_fire {
                FireAddress::Board(sq) => {
                    // ９段目には侵入できないぜ☆（＾～＾）
                    if 8 < sq.rank() {
                        return false;
                    }
                    true
                }
                FireAddress::Hand(_drop_type) => panic!(Log::panic(&format!(
                    "(Err.546) 盤上ではなかったぜ☆（＾～＾）！",
                ))),
            },
            PermissionType::Knight => match dst_fire {
                FireAddress::Board(sq) => {
                    // ８、９段目には侵入できないぜ☆（＾～＾）
                    if 7 < sq.rank() {
                        return false;
                    }
                    true
                }
                FireAddress::Hand(_drop_type) => panic!(Log::panic(&format!(
                    "(Err.546) 盤上ではなかったぜ☆（＾～＾）！",
                ))),
            },
        }
    }
}

/// Pseudo legal move(疑似合法手)☆（＾～＾）
/// このオブジェクトは、探索のノード１つごとに作られるぜ☆（＾～＾）
///
/// 先手の連続王手の千日手とか、空き王手とか、駒を見ただけでは調べられないだろ☆（＾～＾）
/// 棋譜や盤面を見ず、駒だけで調べる合法手が Pseudo legal move だぜ☆（＾～＾）
///
/// 二歩とか、打った後で調べた方が高速になるはずだが、探索部がまだできてないので、指し手生成の中でチェックしているぜ☆（＾～＾）
/// 香を２段目に打たないとか強い将棋を目指すことは　まだやってないぜ☆（＾～＾）
///
/// 指し手生成中に手番が変わることは無いんで turn は game.history.get_turn() で取れだぜ☆（＾～＾）
pub struct MoveGen {}
impl MoveGen {
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
    /// * `turn` - 後手視点にしたけりゃ turn.turn() しろだぜ☆（＾～＾）
    /// * `table` - 現局面の盤上だぜ☆（＾～＾）
    /// * `listen_move` - 指し手を受け取れだぜ☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// F1:
    /// * 指し手ハッシュ
    /// * 移動先にあった駒
    pub fn make_move<F1>(
        game: &Position,
        phase_operation: &Box<dyn PhaseOperation>,
        listen_move: &mut F1,
    ) where
        F1: FnMut(Movement),
    {
        game.table.for_some_pieces_on_list40(
            game.history.get_turn(),
            // 移動元と、その駒の種類。
            &mut |src_fire: &FireAddress| {
                MoveGen::start(game, phase_operation, src_fire, listen_move)
            },
        );
    }

    /// 盤上を見ようぜ☆（＾～＾） 盤上の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `turn` - 後手視点にしたけりゃ turn.turn() しろだぜ☆（＾～＾）
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
    fn start<F1>(
        game: &Position,
        phase_operation: &Box<dyn PhaseOperation>,
        source: &FireAddress,
        listen_move: &mut F1,
    ) where
        F1: FnMut(Movement),
    {
        match source {
            FireAddress::Board(_src_sq) => {
                let piece_type = game.table.get_type(
                    game.table
                        .piece_num_at(game.history.get_turn(), &source)
                        .unwrap(),
                );

                let moving =
                    &mut |destination: &FireAddress,
                          promotability,
                          _agility,
                          permission_type: Option<PermissionType>| {
                        let pseudo_captured_num = game
                            .table
                            .piece_num_at(game.history.get_turn(), &destination);

                        let space = if let Some(pseudo_captured_num_val) = pseudo_captured_num {
                            if game.table.get_phase(pseudo_captured_num_val)
                                == game.history.get_turn()
                            {
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
                        let forbidden = if let Some(permission_type_val) = permission_type {
                            // permission があれば forbidden じゃないぜ☆（＾～＾）
                            !phase_operation.check_permission(&destination, permission_type_val)
                        } else {
                            false
                        };

                        match &promotability {
                            Any => {
                                // 成ったり、成れなかったりできるとき。
                                if !forbidden {
                                    listen_move(Movement::new(
                                        game.table
                                            .piece_num_at(game.history.get_turn(), &source)
                                            .unwrap(),
                                        *source,
                                        *destination,
                                        false,
                                        if let Some(piece_num_val) = pseudo_captured_num {
                                            Some(CapturedMove::new(
                                                *destination,
                                                FireAddress::Hand(HandAddress::new(
                                                    game.table
                                                        .get_double_faced_piece_type(piece_num_val),
                                                    AbsoluteAddress2D::default(),
                                                )),
                                            ))
                                        } else {
                                            None
                                        },
                                    ));
                                }
                                listen_move(Movement::new(
                                    game.table
                                        .piece_num_at(game.history.get_turn(), &source)
                                        .unwrap(),
                                    *source,
                                    *destination,
                                    true,
                                    if let Some(piece_num_val) = pseudo_captured_num {
                                        Some(CapturedMove::new(
                                            *destination,
                                            FireAddress::Hand(HandAddress::new(
                                                game.table
                                                    .get_double_faced_piece_type(piece_num_val),
                                                AbsoluteAddress2D::default(),
                                            )),
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
                                        game.table
                                            .piece_num_at(game.history.get_turn(), &source)
                                            .unwrap(),
                                        *source,
                                        *destination,
                                        promotion,
                                        if let Some(piece_num_val) = pseudo_captured_num {
                                            Some(CapturedMove::new(
                                                *destination,
                                                FireAddress::Hand(HandAddress::new(
                                                    game.table
                                                        .get_double_faced_piece_type(piece_num_val),
                                                    AbsoluteAddress2D::default(),
                                                )),
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
                MoveGen::piece_of(game, phase_operation, piece_type, source, moving);
            }
            FireAddress::Hand(src_drop_type) => {
                if let Some((piece_type, fire_hand)) =
                    game.table.last_hand(game.history.get_turn(), &source)
                {
                    // 打つぜ☆（＾～＾）
                    let drop_fn = &mut |destination: &FireAddress| {
                        if let None = game
                            .table
                            .piece_num_at(game.history.get_turn(), &destination)
                        {
                            // 駒が無いところに打つ
                            use crate::cosmic::smart::features::PieceType::*;
                            match piece_type {
                                Pawn => {
                                    match destination {
                                        FireAddress::Board(sq) => {
                                            // ひよこ　は２歩できない☆（＾～＾
                                            if game.table.exists_pawn_on_file(
                                                game.history.get_turn(),
                                                sq.file(),
                                            ) {
                                                return;
                                            }
                                        }
                                        _ => panic!(Log::panic(&format!(
                                            "(Err.641) 盤上じゃなかったぜ☆（＾～＾）！",
                                        ))),
                                    }
                                }
                                _ => {}
                            }
                            listen_move(Movement::new(
                                game.table
                                    .piece_num_at(game.history.get_turn(), &fire_hand)
                                    .unwrap(),
                                fire_hand,    // 打った駒種類
                                *destination, // どの升へ行きたいか
                                false,        // 打に成りは無し
                                None,         // 打で取れる駒無し
                            ));
                        }
                    };
                    // 駒を持っていれば
                    use crate::cosmic::smart::features::DoubleFacedPieceType::*;
                    // 歩、香: 先手から見た歩、香車の打てる面積だぜ☆（＾～＾）
                    // 桂: 先手から見た桂馬の打てる面積だぜ☆（＾～＾）
                    // それ以外の駒が打てる範囲は盤面全体。駒を打つときに使うぜ☆（＾～＾）
                    for sq in match src_drop_type.old {
                        Pawn | Lance => game.table.area.drop_pawn_lance.iter(),
                        Knight => game.table.area.drop_knight.iter(),
                        _ => game.table.area.all_squares.iter(),
                    } {
                        drop_fn(sq);
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
    fn piece_of<F1>(
        game: &Position,
        phase_operation: &Box<dyn PhaseOperation>,
        piece_type: PieceType,
        source: &FireAddress,
        moving: &mut F1,
    ) where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        match piece_type {
            PieceType::Pawn => MoveGen::pawn(game, phase_operation, source, moving),
            PieceType::Lance => MoveGen::lance(game, phase_operation, source, moving),
            PieceType::Knight => MoveGen::knight(game, phase_operation, source, moving),
            PieceType::Silver => MoveGen::silver(game, phase_operation, source, moving),
            PieceType::Gold => MoveGen::gold(game, source, moving),
            PieceType::King => MoveGen::king(game, source, moving),
            PieceType::Bishop => MoveGen::bishop(game, phase_operation, source, moving),
            PieceType::Rook => MoveGen::rook(game, phase_operation, source, moving),
            PieceType::PromotedPawn => MoveGen::gold(game, source, moving),
            PieceType::PromotedLance => MoveGen::gold(game, source, moving),
            PieceType::PromotedKnight => MoveGen::gold(game, source, moving),
            PieceType::PromotedSilver => MoveGen::gold(game, source, moving),
            PieceType::Horse => MoveGen::horse(game, source, moving),
            PieceType::Dragon => MoveGen::dragon(game, source, moving),
        }
    }

    /// 先手から見た盤上の歩の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `turn` - 後手視点にしたけりゃ turn.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn pawn<F1>(
        game: &Position,
        phase_operation: &Box<dyn PhaseOperation>,
        source: &FireAddress,
        moving: &mut F1,
    ) where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            MoveGen::promote_pawn_lance(phase_operation, destination, moving)
        };

        for mobility in PieceType::Pawn.mobility().iter() {
            MoveGen::move_(game, source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の香の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `turn` - 後手視点にしたけりゃ turn.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn lance<F1>(
        game: &Position,
        phase_operation: &Box<dyn PhaseOperation>,
        source: &FireAddress,
        moving: &mut F1,
    ) where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            MoveGen::promote_pawn_lance(phase_operation, destination, moving)
        };

        for mobility in PieceType::Lance.mobility().iter() {
            MoveGen::move_(game, source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の桂の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `turn` - 後手視点にしたけりゃ turn.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn knight<F1>(
        game: &Position,
        phase_operation: &Box<dyn PhaseOperation>,
        source: &FireAddress,
        moving: &mut F1,
    ) where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            MoveGen::promote_knight(phase_operation, destination, moving)
        };

        for mobility in PieceType::Knight.mobility().iter() {
            MoveGen::move_(game, source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の銀の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn silver<F1>(
        game: &Position,
        phase_operation: &Box<dyn PhaseOperation>,
        source: &FireAddress,
        moving: &mut F1,
    ) where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            MoveGen::promote_silver(phase_operation, &source, destination, moving)
        };

        for mobility in PieceType::Silver.mobility().iter() {
            MoveGen::move_(game, source, *mobility, moving);
        }
    }

    /// 先手から見た盤上の金、と、杏、圭、全の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `turn` - 後手視点にしたけりゃ turn.turn() しろだぜ☆（＾～＾）
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn gold<F1>(game: &Position, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            moving(destination, Promotability::Deny, Agility::Hopping, None)
        };

        for mobility in PieceType::Gold.mobility().iter() {
            MoveGen::move_(game, source, *mobility, moving);
        }
    }

    /// 盤上の玉の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn king<F1>(game: &Position, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            moving(destination, Promotability::Deny, Agility::Hopping, None)
        };

        for mobility in PieceType::King.mobility().iter() {
            MoveGen::move_(game, source, *mobility, moving);
        }
    }

    /// 盤上の角の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn bishop<F1>(
        game: &Position,
        phase_operation: &Box<dyn PhaseOperation>,
        source: &FireAddress,
        moving: &mut F1,
    ) where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            MoveGen::promote_bishop_rook(phase_operation, source, destination, moving)
        };
        for mobility in PieceType::Bishop.mobility().iter() {
            MoveGen::move_(game, source, *mobility, moving);
        }
    }

    /// 盤上の飛の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn rook<F1>(
        game: &Position,
        phase_operation: &Box<dyn PhaseOperation>,
        source: &FireAddress,
        moving: &mut F1,
    ) where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, _agility| {
            MoveGen::promote_bishop_rook(phase_operation, source, destination, moving)
        };
        for mobility in PieceType::Rook.mobility().iter() {
            MoveGen::move_(game, source, *mobility, moving);
        }
    }

    /// 盤上の馬の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn horse<F1>(game: &Position, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, agility| {
            moving(destination, Promotability::Deny, agility, None)
        };

        for mobility in PieceType::Horse.mobility().iter() {
            MoveGen::move_(game, source, *mobility, moving);
        }
    }

    /// 盤上の竜の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `source` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn dragon<F1>(game: &Position, source: &FireAddress, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        let moving = &mut |destination: &FireAddress, agility| {
            moving(destination, Promotability::Deny, agility, None)
        };

        for mobility in PieceType::Dragon.mobility().iter() {
            MoveGen::move_(game, source, *mobility, moving);
        }
    }

    /// 盤上の駒を指すぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `game` - 先手か後手か、関係ないか☆（＾～＾）先後同型でも必要なのが変わってるだろ☆（＾～＾）
    /// * `start` - 移動元升☆（＾～＾）
    /// * `mobility` - 動き方☆（＾～＾）
    /// * `moving` - 絶対番地を受け取れだぜ☆（＾～＾）
    fn move_<F1>(game: &Position, start: &FireAddress, mobility: Mobility, moving: &mut F1)
    where
        F1: FnMut(&FireAddress, Agility) -> bool,
    {
        let angle =
            // 後手なら１８０°回転だぜ☆（＾～＾）
            if game.history.get_turn() == Phase::Second {
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
            _ => panic!(Log::panic(&format!(
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
        phase_operation: &Box<dyn PhaseOperation>,
        destination: &FireAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        callback(
            destination,
            if phase_operation.is_rank456789(destination) {
                Promotability::Deny
            } else if phase_operation.is_rank23(destination) {
                Promotability::Any
            } else {
                Promotability::Forced
            },
            Agility::Hopping,
            Some(PermissionType::PawnLance),
        )
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
        phase_operation: &Box<dyn PhaseOperation>,
        destination: &FireAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        callback(
            destination,
            if phase_operation.is_rank456789(destination) {
                Promotability::Deny
            } else if phase_operation.is_rank3(destination) {
                Promotability::Any
            } else {
                Promotability::Forced
            },
            Agility::Knight,
            Some(PermissionType::Knight),
        )
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
        phase_operation: &Box<dyn PhaseOperation>,
        source: &FireAddress,
        destination: &FireAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        callback(
            destination,
            // 戻って成るのがある☆（＾～＾）
            if phase_operation.is_rank123(destination) || phase_operation.is_rank123(source) {
                Promotability::Any
            } else {
                Promotability::Deny
            },
            Agility::Hopping,
            None,
        )
    }

    /// 角と飛のための、成れるか成れないか判定だぜ☆（＾～＾）！
    /// 非敵陣にいるとき、敵陣で成れます。敵陣にいるとき、どこでも成れます。
    ///
    /// Arguments
    /// ---------
    ///
    /// * `phase_operation` -
    /// * `source` -
    /// * `destination` -
    /// * `callback` -
    fn promote_bishop_rook<F1>(
        phase_operation: &Box<dyn PhaseOperation>,
        source: &FireAddress,
        destination: &FireAddress,
        callback: &mut F1,
    ) -> bool
    where
        F1: FnMut(&FireAddress, Promotability, Agility, Option<PermissionType>) -> bool,
    {
        callback(
            destination,
            // 戻って成るのがある☆（＾～＾）
            if phase_operation.is_rank123(destination) || phase_operation.is_rank123(source) {
                Promotability::Any
            } else {
                Promotability::Deny
            },
            Agility::Sliding,
            None,
        )
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

pub enum PermissionType {
    PawnLance,
    Knight,
}
