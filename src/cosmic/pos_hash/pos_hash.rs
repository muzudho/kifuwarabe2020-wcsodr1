//! 局面ハッシュ。
//!

use crate::cosmic::playing::Game;
use crate::cosmic::recording::{AddressPos, History, Movement, PHASE_LEN, PHASE_SECOND};
use crate::cosmic::smart::features::{HAND_MAX, PHYSICAL_PIECES_LEN};
use crate::cosmic::smart::square::{
    AbsoluteAddress2D, BOARD_MEMORY_AREA, FILE_1, FILE_10, RANK_1, RANK_10, SQUARE_NONE,
};
use crate::cosmic::toy_box::{GameTable, PIECE_LEN};
use crate::law::speed_of_light::HandAddresses;
use crate::spaceship::equipment::Beam;
use rand::Rng;

/// 現対局ハッシュ種
/// ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
pub struct GameHashSeed {
    // 盤上の駒
    pub piece: [[u64; PIECE_LEN]; BOARD_MEMORY_AREA as usize],
    // 持ち駒
    pub hands: [[u64; HAND_MAX]; PHYSICAL_PIECES_LEN],
    // 先後
    pub phase: [u64; PHASE_LEN],
}
impl Default for GameHashSeed {
    fn default() -> Self {
        GameHashSeed {
            // 盤上の駒
            piece: [[0; PIECE_LEN]; BOARD_MEMORY_AREA as usize],
            // 持ち駒
            hands: [[0; HAND_MAX]; PHYSICAL_PIECES_LEN],
            // 先後
            phase: [0; PHASE_LEN],
        }
    }
}
impl GameHashSeed {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_square in SQUARE_NONE..BOARD_MEMORY_AREA {
            for i_piece in 0..PIECE_LEN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.piece[i_square as usize][i_piece] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 持ち駒
        for i_piece in 0..PHYSICAL_PIECES_LEN {
            for i_count in 0..HAND_MAX {
                self.hands[i_piece][i_count] =
                    rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
            }
        }
        // 先後
        for i_phase in 0..PHASE_LEN {
            self.phase[i_phase] = rand::thread_rng().gen_range(0, 18_446_744_073_709_551_615);
        }
    }

    /// TODO 指し手を使って差分更新
    /// 駒を動かしたあとに使う。
    /// TODO 持ち駒の枚数がトグルになってないぜ☆（＾～＾）？
    pub fn update_by_do_move(&self, history: &mut History, table: &GameTable, move_: &Movement) {
        // TODO １つ前の局面のハッシュ。
        let mut prev_hash = if history.ply == 0 {
            history.starting_position_hash
        } else {
            history.position_hashs[history.ply as usize - 1]
        };
        // TODO 指し手 で差分を適用
        // 移動する駒。
        match move_.source {
            AddressPos::Board(sq) => {
                let source_piece = table.piece_at(&move_.source).unwrap() as usize;
                // 移動前マスに、動かしたい駒があるときのハッシュ。
                prev_hash ^= self.piece[sq.serial_number()][source_piece];
                // 移動後マスに、動かしたい駒があるときのハッシュ。
                match move_.destination {
                    AddressPos::Board(sq) => {
                        prev_hash ^= self.piece[sq.serial_number()][source_piece];
                    }
                    _ => panic!(Beam::trouble(&format!(
                        "(Err.87) まだ実装してないぜ☆（＾～＾）！",
                    ))),
                }
            }
            AddressPos::Hand(old_drop) => {
                let count = table.count_hand(old_drop);
                // 打つ前の駒の枚数のハッシュ。
                prev_hash ^= self.hands[old_drop as usize][count as usize];
                // 移動後マスに、打った駒があるときのハッシュ。
                match move_.destination {
                    AddressPos::Board(sq) => {
                        prev_hash ^=
                            self.piece[sq.serial_number()][old_drop.nonpromoted_piece() as usize];
                    }
                    _ => panic!(Beam::trouble(&format!(
                        "(Err.101) まだ実装してないぜ☆（＾～＾）！",
                    ))),
                }
            }
        }
        // TODO 移動先にある駒
        match move_.destination {
            AddressPos::Board(dst_sq) => {
                // 移動先にある駒があれば
                if let Some(dst_piece_val) = table.piece_at(&move_.destination) {
                    prev_hash ^= self.piece[dst_sq.serial_number()][dst_piece_val as usize];
                    // 持ち駒になるとき。
                    let double_faced_piece = dst_piece_val.double_faced_piece();
                    let count = table.count_hand(double_faced_piece);
                    // 打つ前の駒の枚数のハッシュ。
                    prev_hash ^= self.hands[double_faced_piece as usize][count as usize + 1];
                }
            }
            _ => panic!(Beam::trouble(&format!(
                "(Err.123) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }

        // TODO ハッシュ更新
        history.position_hashs[history.ply as usize] = prev_hash;
    }

    /*
    /// 局面ハッシュを作り直す
    pub fn current_position(&self, game: &Game) -> u64 {
        let mut hash = self.table(&game.table);

        // 手番ハッシュ
        use crate::cosmic::recording::Phase::*;
        match game.history.get_friend() {
            First => hash ^= self.phase[PHASE_FIRST],
            Second => hash ^= self.phase[PHASE_SECOND],
        }

        hash
    }
    */

    /// 初期局面ハッシュを作り直す
    pub fn starting_position(&self, game: &Game) -> u64 {
        let mut hash = self.from_table(&game.starting_table);

        // 手番ハッシュ（後手固定）
        hash ^= self.phase[PHASE_SECOND];

        hash
    }

    /// 盤面からハッシュ作成
    fn from_table(&self, table: &GameTable) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for rank in RANK_1..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let sq = AbsoluteAddress2D::new(file, rank);
                if let Some(piece_val) = table.piece_at(&AddressPos::Board(sq)) {
                    hash ^= self.piece[sq.serial_number()][piece_val as usize];
                }
            }
        }

        // 持ち駒ハッシュ
        HandAddresses::for_all(&mut |adr| {
            let count = table.count_hand(adr);
            debug_assert!(
                count <= HAND_MAX,
                "持ち駒 {:?} の枚数 {} <= {}",
                adr,
                count,
                HAND_MAX
            );
            hash ^= self.hands[adr as usize][count as usize];
        });

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }
}
