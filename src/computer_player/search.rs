//!
//! 駒たちが躍動するぜ☆（＾～＾）
//!

use crate::cosmic::recording::{Movement, Phase, SENNTITE_NUM};
use crate::cosmic::smart::evaluator::REPITITION_VALUE;
use crate::cosmic::smart::features::PieceType;
use crate::cosmic::universe::Universe;
use crate::law::generate_move::{MoveGen, Ways};
use crate::log::LogExt;
use crate::look_and_model::search::Search;
use crate::position::Position;
use crate::spaceship::equipment::PvString;
use casual_logger::Log;
use std::fmt;

impl Search {
    /// 反復深化探索だぜ☆（＾～＾）
    pub fn iteration_deeping(&mut self, universe: &mut Universe) -> TreeState {
        self.remake_info_display();

        let max_ply = std::cmp::max(
            universe.option_max_depth,
            universe.option_max_ply - universe.game.history.ply as usize,
        );
        // とりあえず 1手読み を叩き台にするぜ☆（＾～＾）
        // 初手の３０手が葉になるぜ☆（＾～＾）
        self.evaluation.before_search();
        self.max_depth0 = 0;
        let mut best_ts = self.node(&mut universe.game, Value::Win);
        self.evaluation.after_search();

        // 一番深く潜ったときの最善手を選ぼうぜ☆（＾～＾）
        for id in 1..max_ply {
            self.max_depth0 = id;
            // 現在のベストムーブ表示☆（＾～＾） PV にすると将棋所は符号を日本語に翻訳してくれるぜ☆（＾～＾）
            let movement = best_ts.bestmove.movement;
            Log::print_info(&Search::info_str(
                Some(self.max_depth0),
                Some((self.nodes, self.nps())),
                Some(best_ts.bestmove.value),
                movement,
                &Some(PvString::PV(
                    self.msec(),
                    format!(
                        "{}",
                        if let Some(movement_val) = movement {
                            format!("{}", movement_val)
                        } else {
                            "resign".to_string()
                        },
                    ),
                )), // この指し手を選んだ時の pv の読み筋が欲しいぜ☆（＾～＾）
            ));
            self.info.set_interval();

            if let None = movement {
                // すでに投了が見えているのなら探索終了だぜ☆（＾～＾）
                break;
            }

            // 横線で仕切るぜ☆（＾～＾）
            Log::print_info(&Search::info_str(
                None,
                None,
                None,
                None,
                &Some(PvString::String(format!(
                    "----------Iteration deeping----------"
                ))),
            ));
            self.info.set_interval();

            // 探索局面数は引き継ぐぜ☆（＾～＾）積み上げていった方が見てて面白いだろ☆（＾～＾）
            self.evaluation.before_search();
            let ts = self.node(&mut universe.game, Value::Win);
            self.evaluation.after_search();
            if ts.timeout {
                // 思考時間切れなら この探索結果は使わないぜ☆（＾～＾）
                break;
            }

            // 無条件に更新だぜ☆（＾～＾）初手の高得点を引きずられて王手回避漏れされたら嫌だしな☆（＾～＾）
            best_ts = ts.clone();
        }

        best_ts
    }

    /// 先手の気持ちで、勝てだぜ☆（*＾～＾*）
    ///
    /// # Arguments
    ///
    /// * `pos` - 局面。
    /// * `sibling_best` - アルファベータ探索のベータ値。兄弟で一番良い評価値。
    ///
    /// # Returns
    ///
    /// Best movement, Value, Sum nodes
    fn node(&mut self, pos: &mut Position, another_branch_best: Value) -> TreeState {
        let mut ts = TreeState::default();

        // この手を指すと負けてしまう、という手が見えていたら、このフラグを立てろだぜ☆（＾～＾）
        let mut exists_lose = false;

        // 指し手の一覧を作るぜ☆（＾～＾） 指し手はハッシュ値で入っている☆（＾～＾）
        let mut ways = {
            let mut ways = Ways::new();

            // 現局面で、各駒が、他に駒がないと考えた場合の最大数の指し手を生成しろだぜ☆（＾～＾）
            MoveGen::make_move(
                &pos,
                match pos.history.get_turn() {
                    Phase::First => &pos.movegen_phase.first_movegen,
                    Phase::Second => &pos.movegen_phase.second_movegen,
                },
                &mut |way| {
                    ways.push(&way);
                },
            );

            ways
        };

        // 指せる手が無ければ投了☆（＾～＾）
        if ways.is_empty() {
            return ts;
        }

        // 指し手のオーダリングをしたいぜ☆（＾～＾） 取った駒は指し手生成の段階で調べているし☆（＾～＾）
        let mut cap = 0;
        if 1 < ways.len() {
            for i in 0..ways.len() {
                if let Some(_captured) = ways.get(i).captured {
                    // 駒を取った手は、リストの先頭に集めるぜ☆（＾～＾）
                    // TODO .clone()いやなんで、インデックスだけソートした方がいいのか☆（＾～＾）？
                    ways.swap(cap, i);
                    cap += 1;
                }
            }
            // 次は駒を取ったグループの中で、玉を取った手をグループの先頭に集めるぜ☆（＾～＾）
            let mut king = 0;
            for i in 0..cap {
                let fire = if let Some(captured_move) = ways.get(i).captured {
                    captured_move.source
                } else {
                    panic!(Log::print_fatal("Invalid captured_move."));
                };
                let piece_type = pos.table.get_type(
                    if let Some(piece_type) = pos.table.piece_num_at(pos.history.get_turn(), &fire)
                    {
                        piece_type
                    } else {
                        panic!(Log::print_fatal("Invalid piece_type."));
                    },
                );
                match piece_type {
                    PieceType::King => {
                        // 玉を取った手は、リストの先頭に集めるぜ☆（＾～＾）
                        // TODO .clone()いやなんで、インデックスだけソートした方がいいのか☆（＾～＾）？
                        ways.swap(king, i);
                        king += 1;
                    }
                    _ => {}
                }
            }
        }

        // TODO ターン切替にしたいぜ☆（＾～＾）
        let coverage_sign = if pos.pv_len() % 2 == 0 {
            // 先手が指すところだぜ☆（＾～＾）
            1
        } else {
            // 後手が指すところだぜ☆（＾～＾）
            -1
        };
        self.evaluation.add_control(coverage_sign, &ways);
        for index in ways.indexes.iter() {
            let move_ = ways.get(*index);

            // Find out why you are not doing a forward search.
            // If not, I will search.
            // 前向き検索を行わない理由を調べてください。
            // 無ければ探索します。
            let mut forward_cut_off1 = None;
            if self.think_msec < self.msec() && self.depth_not_to_give_up <= self.max_depth0 {
                // とりあえず ランダム秒で探索を打ち切ろうぜ☆（＾～＾）？
                // タイムアウトしたんだったら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
                ts.timeout = true;
                forward_cut_off1 = Some(ForwardCutOff1::TimeOut);
            }

            if let Some(forward_cut_off1) = forward_cut_off1 {
                match forward_cut_off1 {
                    ForwardCutOff1::TimeOut => {
                        return ts;
                    }
                }
            }

            // Let's put a stone for now.
            // とりあえず石を置きましょう。
            self.nodes += 1;
            // * `promotion_value` - 評価値用。成ったら加点☆（＾～＾）
            let promotion_value = if move_.promote {
                pos.table.promotion_value_at(&pos.table, &move_.source)
            } else {
                0
            };

            // 1手進める前に、これから取ることになる駒を盤上から読み取っておきます。
            let captured_piece_type = if let Some(captured) = move_.captured {
                Some(
                    pos.table.get_type(
                        if let Some(piece_num) = pos
                            .table
                            .piece_num_at(pos.history.get_turn(), &captured.source)
                        {
                            piece_num
                        } else {
                            panic!(Log::print_fatal("Invalid piece_num."));
                        },
                    ),
                )
            // Some(captured.piece_type)
            } else {
                None
            };

            pos.do_move(pos.history.get_turn(), &move_);

            let (captured_piece_centi_pawn, delta_promotion_bonus) = self
                .evaluation
                .after_do_move(captured_piece_type, promotion_value);

            // TODO 廃止方針☆（＾～＾）
            let forward_cut_off2 = {
                let mut cut = None;
                if let Some(captured_piece_type_val) = captured_piece_type {
                    if captured_piece_type_val == PieceType::King {
                        // 玉を取る手より強い手はないぜ☆（＾～＾）！探索終了～☆（＾～＾）！この手を選べだぜ☆（＾～＾）！
                        cut = Some(ForwardCutOff2::KingCatch);
                    }
                }

                if let None = cut {
                    if self.max_depth0 < pos.pv_len() {
                        cut = Some(ForwardCutOff2::Leaf);
                    }
                }

                cut
            };

            if let Some(forward_cut_off2) = &forward_cut_off2 {
                match forward_cut_off2 {
                    ForwardCutOff2::KingCatch => {
                        ts.bestmove.catch_king(move_);
                    }
                    ForwardCutOff2::Leaf => {
                        // 葉だぜ☆（＾～＾）

                        // 評価を集計するぜ☆（＾～＾）
                        ts.choice_friend(&Value::CentiPawn(self.evaluation.centi_pawn()), move_);

                        if self.info.is_printable() {
                            // 何かあったタイミングで読み筋表示するのではなく、定期的に表示しようぜ☆（＾～＾）
                            // PV を表示するには、葉のタイミングで出すしかないぜ☆（＾～＾）
                            let movement = ts.bestmove.movement;
                            Log::print_info(&Search::info_str(
                                None,
                                None,
                                None,
                                None,
                                &Some(PvString::String(format!(
                                    "ways={} | komawari={} | promotion={}", //  | {} {} {} |
                                    self.evaluation.ways(),
                                    self.evaluation.komawari(),
                                    self.evaluation.promotion(),
                                ))),
                            ));
                            Log::print_info(&Search::info_str(
                                Some(pos.pv_len()),
                                Some((self.nodes, self.nps())),
                                Some(ts.bestmove.value),
                                movement,
                                &Some(PvString::PV(self.msec(), pos.pv_text().to_string())),
                            ));
                            self.info.set_interval();
                        }
                    }
                }
            }

            if let None = forward_cut_off2 {
                // 千日手かどうかを判定する☆（＾～＾）
                if SENNTITE_NUM <= pos.count_same_position() {
                    // 千日手か……☆（＾～＾） 一応覚えておくぜ☆（＾～＾）
                    ts.repetition_movement = Some(move_);
                } else if self.max_depth0 < pos.pv_len() {
                } else {
                    // 枝局面なら、更に深く進むぜ☆（＾～＾）
                    self.evaluation.before_search();
                    let opponent_ts = self.node(
                        pos,
                        match ts.bestmove.value {
                            Value::CentiPawn(centi_pawn) => Value::CentiPawn(-centi_pawn),
                            Value::Win => Value::Lose,
                            Value::Lose => Value::Win,
                        },
                    );

                    if ts.timeout {
                        // すでにタイムアウトしていたのなら、終了処理 すっとばして早よ終われだぜ☆（＾～＾）
                        return ts;
                    }
                    self.evaluation.after_search();

                    // 下の木の結果を、ひっくり返して、引き継ぎます。
                    exists_lose =
                        ts.turn_over_and_choice(&opponent_ts, move_, self.evaluation.centi_pawn());
                }
            }

            // (2) Remove the placed stone.
            // (二) 置いた石は取り除きます。
            self.evaluation
                .before_undo_move(captured_piece_centi_pawn, delta_promotion_bonus);
            pos.undo_move();

            if let Some(forward_cut_off2) = &forward_cut_off2 {
                match forward_cut_off2 {
                    ForwardCutOff2::KingCatch => {
                        break;
                    }
                    ForwardCutOff2::Leaf => {}
                }
            }

            let mut backword_cut_off = None;
            match ts.bestmove.value {
                Value::Win => {
                    // この手について、次の手ではなく、２手以上先で勝ったんだろ☆（＾～＾）もう探索しなくていいぜ☆（＾～＾）この手にしようぜ☆（＾～＾）！
                    backword_cut_off = Some(BackwardCutOff::YouWin);
                }
                Value::Lose => {
                    // この手について、次の相手の番で王さまを取られたか、３手先以上の奇数番で詰められたんだろ☆（＾～＾）詰められてない別の手を探すんだぜ、続行☆（＾～＾）！
                }
                Value::CentiPawn(current_centi_pawn) => {
                    // ベータカット判定☆（＾～＾）
                    match another_branch_best {
                        Value::CentiPawn(another_branch_best_centi_pawn) => {
                            // 兄弟局面より良い手を見つけたのなら、相手から見ればこの手は選ばないから、もう探索しなくていいぜ☆（＾～＾）
                            // これが　いわゆるベータカットだぜ☆（＾～＾）
                            if another_branch_best_centi_pawn <= current_centi_pawn {
                                backword_cut_off = Some(BackwardCutOff::BetaCut);
                            }
                        }
                        Value::Win => {
                            // 初手に、ゴミの最大値として入っているか、兄弟局面で勝ちの手があったようだぜ☆（＾～＾）
                            // ベータカットは起こらないぜ☆（＾～＾）
                        }
                        Value::Lose => {
                            // 兄弟局面に負けがあるんだったら、この
                            // 負けに比べればどんな手でも良いぜ☆（＾～＾）ベータカットな☆（＾～＾）！
                            backword_cut_off = Some(BackwardCutOff::BetaCut);
                        }
                    }
                }
            }

            // (4) Depending on the condition, the sibling node search is skipped.
            // (四) 条件によっては、兄弟ノードの検索がスキップされます。
            if let Some(backword_cut_off) = backword_cut_off {
                match backword_cut_off {
                    BackwardCutOff::YouWin | BackwardCutOff::BetaCut => {
                        break;
                    }
                }
            }
        }
        self.evaluation.add_control(-1 * coverage_sign, &ways);

        if !exists_lose {
            if let None = ts.bestmove.movement {
                if let Some(repetition_movement_val) = ts.repetition_movement {
                    // 負けを認めていないうえで、投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
                    ts.bestmove.update(
                        repetition_movement_val,
                        &Value::CentiPawn(REPITITION_VALUE),
                        Reason::RepetitionBetterThanResign,
                    );
                }
            }
        }

        ts
    }
}

/// The reason for ending the forward search.  
/// 前向き探索を終了した理由。  
pub enum ForwardCutOff1 {
    /// The urgency of the remaining time.  
    /// 残り時間の切迫。  
    TimeOut,
}
/// The reason for ending the forward search.  
/// 前向き探索を終了した理由。  
pub enum ForwardCutOff2 {
    /// Capture the king.
    /// 玉を取った。
    KingCatch,
    /// Leaf.
    /// 葉。
    Leaf,
}

/// The reason for ending the backward search.  
/// 後ろ向き探索を終了した理由。  
pub enum BackwardCutOff {
    /// End with a you win.
    /// あなたの勝ちにつき、終了。
    YouWin,
    /// Beta cut-off.
    /// ベータ・カット。
    BetaCut,
}

#[derive(Clone)]
pub struct Bestmove {
    pub value: Value,
    pub movement: Option<Movement>,
    /// この指し手を選んだ理由☆（＾～＾）
    pub reason: Reason,
}
impl Default for Bestmove {
    fn default() -> Self {
        Bestmove {
            value: Value::Lose,
            movement: None,
            // なんの手も無かったぜ☆（＾～＾）
            reason: Reason::NoUpdate,
        }
    }
}
impl Bestmove {
    /// TODO 廃止予定☆（＾～＾）
    pub fn catch_king(&mut self, movement: Movement) {
        // 玉を取る手より強い手はないぜ☆（＾～＾）！
        self.movement = Some(movement);
        self.value = Value::Win;
        self.reason = Reason::KingCatchIsStrongest;
    }
    pub fn update(&mut self, movement: Movement, value: &Value, reason: Reason) {
        self.movement = Some(movement);
        self.value = *value;
        self.reason = reason;
    }
}
#[derive(Clone)]
pub struct TreeState {
    pub bestmove: Bestmove,
    // あれば千日手の手☆（＾～＾）投了よりはマシ☆（＾～＾）
    pub repetition_movement: Option<Movement>,
    pub timeout: bool,
}
impl Default for TreeState {
    fn default() -> Self {
        TreeState {
            bestmove: Bestmove::default(),
            repetition_movement: None,
            timeout: false,
        }
    }
}
impl TreeState {
    pub fn turn_over_and_choice(
        &mut self,
        opponent_ts: &TreeState,
        friend_movement: Movement,
        friend_centi_pawn1: isize,
    ) -> bool {
        // TODO 玉を取られてたら、ここは投了すべき☆（＾～＾）？

        // TODO 相手が投了してたら、必ず選ぶべき☆（＾～＾）？

        match opponent_ts.bestmove.value {
            Value::Win => {
                // 相手が勝ったので、自分は負けてるぜ☆（＾～＾）この手は選んではいけないぜ☆（＾～＾）
                true
            }
            Value::Lose => {
                // 相手が負けてるので、自分が勝ってるぜ☆（＾～＾）
                self.bestmove
                    .update(friend_movement, &Value::Win, Reason::FriendWin);
                false
            }
            Value::CentiPawn(num) => {
                // 評価値は ひっくり返します。この指し手の駒の交換値も足します。
                let friend_centi_pawn2 = -num + friend_centi_pawn1;
                if let None = self.bestmove.movement {
                    // どんな悪手も、詰みでなければ 投了より良いだろ☆（＾～＾）
                    self.bestmove.update(
                        friend_movement,
                        &Value::CentiPawn(friend_centi_pawn2),
                        Reason::ThisBetterThanResign,
                    );
                } else {
                    match self.bestmove.value {
                        Value::Win => {
                            panic!(Log::print_fatal(
                                "(Err.405) 自分が勝つ手を既に読んでるのに、ここに来るのはおかしいぜ☆（＾～＾）"
                            ))
                        }
                        Value::Lose => {
                            // 自分が負けるところを、まだそうでない手があるのなら、更新するぜ☆（＾～＾）
                            self.bestmove
                            .update(friend_movement, &Value::CentiPawn(friend_centi_pawn2), Reason::AnyMoveMoreThanLose);
                        }
                        Value::CentiPawn(best_centi_pawn) => {
                            if best_centi_pawn < friend_centi_pawn2 {
                                // 上方修正
                                self.bestmove
                                .update(friend_movement, &Value::CentiPawn(friend_centi_pawn2), Reason::ValueUp);
                            }
                        }
                    }
                }
                false
            }
        }
    }

    /// 指し手のベストを選ぶぜ☆（＾～＾）
    pub fn choice_friend(&mut self, value: &Value, movement: Movement) {
        if let None = self.bestmove.movement {
            // どんな葉も 投了より良いだろ☆（＾～＾）
            // TODO でも、王さんが利きに飛び込んでいるかもしれないな……☆（＾～＾）
            self.bestmove
                .update(movement, value, Reason::AnyLeafBetterThanResign);
            return;
        } else {
            match self.bestmove.value {
                Value::Win => panic!(Log::print_fatal(
                    "(Err.397) 自分が勝つ手を読んでるなら、ここに来るのはおかしいぜ☆（＾～＾）"
                )),
                Value::Lose => {
                    // どんな評価値でも、負けるよりマシだろ☆（＾～＾）
                    self.bestmove
                        .update(movement, value, Reason::AnyLeafMoreThanLose);
                    return;
                }
                Value::CentiPawn(best_centi_pawn) => {
                    match value {
                        Value::Win => {
                            // 勝つんだから更新するぜ☆（＾～＾）
                            self.bestmove.update(movement, value, Reason::Win);
                            return;
                        }
                        Value::Lose => {
                            // TODO ここは通らないぜ☆（＾～＾）要対応☆（＾～＾）
                        }
                        Value::CentiPawn(leaf_centi_pawn) => {
                            if best_centi_pawn < *leaf_centi_pawn {
                                // 評価値が良かったから更新☆（＾～＾）
                                self.bestmove.update(movement, value, Reason::GoodPosition);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 指し手の評価値だぜ☆（＾～＾）
#[derive(Clone, Copy)]
pub enum Value {
    /// 歩１枚の交換値を 100 とするぜ☆（＾～＾）
    /// 将棋は、相手は駒を取られて損、自分は駒を取って得という風に痛手が２倍広がるので、
    /// 交換値が 100 ということは、200点差が開くということだぜ☆（＾～＾）
    CentiPawn(isize),

    /// 勝ち☆（＾～＾）
    Win,

    /// 負け☆（＾～＾）
    Lose,
}

#[derive(Clone)]
pub enum Reason {
    /// 負けを認めていないうえで、投了するぐらいなら千日手を選ぶぜ☆（＾～＾）
    RepetitionBetterThanResign,
    /// なんの手も無かったぜ☆（＾～＾）
    NoUpdate,
    /// 玉を取る手より強い手はないぜ☆（＾～＾）！
    KingCatchIsStrongest,
    /// 相手が負けてるので、自分が勝ってるぜ☆（＾～＾）
    FriendWin,
    /// どんな悪手も、詰みでなければ 投了より良いだろ☆（＾～＾）
    ThisBetterThanResign,
    /// どんな葉も 投了より良いだろ☆（＾～＾）でも、王さんが利きに飛び込んでいるかもしれないな……☆（＾～＾）
    AnyLeafBetterThanResign,
    /// どんな評価値でも、負けるよりマシだろ☆（＾～＾）
    AnyLeafMoreThanLose,
    /// 勝つんだから更新するぜ☆（＾～＾）
    Win,
    /// 評価値が良かったから更新☆（＾～＾）
    GoodPosition,
    /// 自分が負けるところを、まだそうでない手があるのなら、更新するぜ☆（＾～＾）
    AnyMoveMoreThanLose,
    /// 上方修正
    ValueUp,
}
impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Reason::RepetitionBetterThanResign => "RepetitionBetterThanResign",
                Reason::NoUpdate => "NoUpdate",
                Reason::KingCatchIsStrongest => "KingCatchIsStrongest",
                Reason::FriendWin => "FriendWin",
                Reason::ThisBetterThanResign => "ThisBetterThanResign",
                Reason::AnyLeafBetterThanResign => "AnyLeafBetterThanResign",
                Reason::AnyLeafMoreThanLose => "AnyLeafMoreThanLose",
                Reason::Win => "Win",
                Reason::GoodPosition => "GoodPosition",
                Reason::AnyMoveMoreThanLose => "AnyMoveMoreThanLose",
                Reason::ValueUp => "ValueUp",
            }
        )
    }
}
