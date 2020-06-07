use crate::cosmic::pos_hash::pos_hash::*;
use crate::cosmic::recording::CapturedMove;
use crate::cosmic::recording::{AddressPos, History, Movement};
use crate::cosmic::toy_box::GameTable;
use crate::law::generate_move::Piece;
use crate::spaceship::equipment::{Beam, DestinationDisplay};

/// 局面
pub enum PosNums {
    // 現局面
    Current,
    // 初期局面
    Start,
}

pub struct Game {
    /// 棋譜
    pub history: History,
    /// 初期の卓
    pub starting_table: GameTable,
    /// 現対局ハッシュ種☆（＾～＾）
    pub hash_seed: GameHashSeed,
    /// 現在の卓
    pub table: GameTable,
    /// 情報表示担当
    pub info: DestinationDisplay,
}
impl Default for Game {
    fn default() -> Game {
        Game {
            history: History::default(),
            starting_table: GameTable::default(),
            hash_seed: GameHashSeed::default(),
            table: GameTable::default(),
            info: DestinationDisplay::default(),
        }
    }
}
impl Game {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット
        self.hash_seed.big_bang();
    }

    /*
    pub fn get_meaning(&self, piece: Piece) -> PieceMeaning {
        self.table.get_meaning(piece)
    }
    */

    /// 棋譜の作成
    pub fn set_move(&mut self, move_: &Movement) {
        self.history.movements[self.history.ply as usize] = *move_; // クローンが入る☆（＾～＾）？
    }
    /// テスト用に棋譜表示☆（＾～＾）
    pub fn get_moves_history_text(&self) -> String {
        let mut s = String::new();
        for ply in 0..self.history.ply {
            let movement = &self.history.movements[ply as usize];
            s.push_str(&format!("[{}] {}", ply, movement));
        }
        s
    }

    pub fn get_table(&self, num: PosNums) -> &GameTable {
        match num {
            PosNums::Current => &self.table,
            PosNums::Start => &self.starting_table,
        }
    }
    pub fn mut_starting(&mut self) -> &mut GameTable {
        &mut self.starting_table
    }

    /// 初期局面、現局面ともにクリアーします。
    /// 手目も 0 に戻します。
    pub fn clear(&mut self) {
        self.starting_table.clear();
        self.table.clear();
        self.history.ply = 0;
    }

    /// テスト用に局面ハッシュ☆（＾～＾）
    pub fn get_positions_hash_text(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "[ini] {:20}\n",
            &self.history.starting_position_hash
        ));

        for ply in 0..self.history.ply {
            let hash = &self.history.position_hashs[ply as usize];
            // 64bitは10進数20桁。改行する
            s.push_str(&format!("[{:3}] {:20}\n", ply, hash));
        }
        s
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    /// TODO 初期局面を何に使ってるのか☆（＾～＾）？
    pub fn count_same_position(&self) -> isize {
        if self.history.ply < 1 {
            return 0;
        }

        let mut count = 0;
        let last_ply = self.history.ply - 1;
        let new_ply = self.history.ply;
        for i_ply in 0..new_ply {
            let t = last_ply - i_ply;
            if self.history.position_hashs[t as usize]
                == self.history.position_hashs[last_ply as usize]
            {
                count += 1;
            }
        }

        // 初期局面のハッシュ
        if self.history.starting_position_hash == self.history.position_hashs[last_ply as usize] {
            count += 1;
        }

        count
    }

    /// 入れた指し手の通り指すぜ☆（＾～＾）
    pub fn read_move(&mut self, move_: &Movement) {
        // 局面ハッシュを作り直す
        self.hash_seed
            .update_by_do_move(&mut self.history, &self.table, move_);

        // 動かす駒。Noneなことは無いが、将棋盤にセットするとき結局 Some を付けることになるので、わざわざ省かないぜ☆（＾～＾）
        let moveing_piece: Option<Piece> = match move_.source {
            AddressPos::Board(_src_sq) => {
                // 盤上の移動なら、元の升に駒はあるので、それを消す。
                let piece152: Option<Piece> = if move_.promote {
                    if let Some(piece) = self.table.pop_piece(&move_.source) {
                        // 成ったのなら、元のマスの駒を成らすぜ☆（＾～＾）
                        let meaning = self.table.get_meaning(piece).promoted();
                        Some(self.table.new_piece(meaning, piece.num))
                    } else {
                        panic!(Beam::trouble(
                            "(Err.248) 成ったのに、元の升に駒がなかった☆（＾～＾）"
                        ));
                    }
                } else {
                    // 移動元の駒。
                    self.table.pop_piece(&move_.source)
                };

                piece152
            }
            AddressPos::Hand(_drop) => {
                // 打なら
                // 自分の持ち駒を減らす
                Some(self.table.pop_piece(&move_.source).unwrap())
            }
        };
        // 移動先升に駒があるかどうか
        if let Some(collision_piece) = self.table.pop_piece(&move_.destination) {
            // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
            // 先後ひっくり返す。
            let meaning = self.table.get_meaning(collision_piece).captured();
            let captured_piece = self.table.new_piece(meaning, collision_piece.num);
            self.table.push_piece(
                &AddressPos::Hand(self.table.get_meaning(captured_piece).physical_piece()),
                Some(captured_piece),
            );
        }

        // 移動先升に駒を置く
        self.table.push_piece(&move_.destination, moveing_piece);

        // // 局面ハッシュを作り直す
        // let ky_hash = self.hash_seed.current_position(&self);
        // self.history.set_position_hash(ky_hash);

        self.history.ply += 1;
    }

    /// 逆順に指します。
    pub fn read_move_in_reverse(&mut self) -> bool {
        if 0 < self.history.ply {
            // 棋譜から読取、手目も減る
            self.history.ply -= 1;
            let move_ = &self.history.get_move();
            {
                // 動いた駒
                let moveing_piece: Option<Piece> = match move_.source {
                    AddressPos::Board(_source_val) => {
                        // 盤上の移動なら
                        if move_.promote {
                            // 成ったなら、成る前へ
                            if let Some(source_piece) = self.table.pop_piece(&move_.destination) {
                                let meaning = self.table.get_meaning(source_piece).demoted();
                                Some(self.table.new_piece(meaning, source_piece.num))
                            } else {
                                panic!(Beam::trouble(
                                    "(Err.305) 成ったのに移動先に駒が無いぜ☆（＾～＾）！"
                                ))
                            }
                        } else {
                            self.table.pop_piece(&move_.destination)
                        }
                    }
                    AddressPos::Hand(_drop) => {
                        // 打なら
                        // 打った場所に駒があるはずだぜ☆（＾～＾）
                        let piece = self.table.pop_piece(&move_.destination).unwrap();
                        // 自分の持ち駒を増やそうぜ☆（＾～＾）！
                        self.table.push_piece(
                            &AddressPos::Hand(self.table.get_meaning(piece).physical_piece()),
                            Some(piece),
                        );
                        Some(piece)
                    }
                };

                // 取った駒が有ったか。
                let captured_move: Option<CapturedMove> = move_.captured;
                if let Some(captured_move_val) = captured_move {
                    // 自分の持ち駒を減らす
                    self.table.pop_piece(&AddressPos::Hand(
                        self.table
                            .get_meaning(captured_move_val.piece)
                            .captured()
                            .physical_piece(),
                    ));
                    // 移動先の駒を、取った駒（あるいは空、ということがあるか？）に戻す
                    self.table
                        .push_piece(&move_.destination, Some(captured_move_val.piece));
                }

                if let AddressPos::Board(_src_sq) = move_.source {
                    // 打でなければ、移動元升に、動かした駒を置く☆（＾～＾）打なら何もしないぜ☆（＾～＾）
                    self.table.push_piece(&move_.source, moveing_piece);
                }
            }

            // TODO 局面ハッシュを作り直したいぜ☆（＾～＾）

            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }
}
