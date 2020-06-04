//! 局面ハッシュ。
//!

use crate::cosmic::playing::Game;
use crate::cosmic::recording::{
    AddressOnPosition, History, Movement, PHASE_FIRST, PHASE_LEN, PHASE_SECOND,
};
use crate::cosmic::smart::features::{HAND_MAX, PHYSICAL_PIECES_LEN, PIECE_MEANING_LEN};
use crate::cosmic::smart::square::{
    AbsoluteAddress, BOARD_MEMORY_AREA, FILE_1, FILE_10, RANK_1, RANK_10, SQUARE_NONE,
};
use crate::cosmic::toy_box::Board;
use crate::law::speed_of_light::HandAddresses;
use crate::spaceship::equipment::Beam;
use rand::Rng;

/// 現対局ハッシュ種
/// ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
pub struct GameHashSeed {
    // 盤上の駒
    pub piece: [[u64; PIECE_MEANING_LEN]; BOARD_MEMORY_AREA as usize],
    // 持ち駒
    pub hands: [[u64; HAND_MAX]; PHYSICAL_PIECES_LEN],
    // 先後
    pub phase: [u64; PHASE_LEN],
}
impl Default for GameHashSeed {
    fn default() -> Self {
        GameHashSeed {
            // 盤上の駒
            piece: [[0; PIECE_MEANING_LEN]; BOARD_MEMORY_AREA as usize],
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
            for i_piece in 0..PIECE_MEANING_LEN {
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
    pub fn update_by_diff(&self, history: &mut History, board: &Board, move_: &Movement) {
        // TODO １つ前の局面のハッシュ。
        let mut prev_hash = if history.ply == 0 {
            history.starting_position_hash
        } else {
            history.position_hashs[history.ply as usize - 1]
        };
        // TODO 指し手 で差分を適用
        // TODO 移動前の駒
        prev_hash ^= match move_.source {
            AddressOnPosition::Board(source) => {
                let source_piece = board.piece_at(&source);
                self.piece[source.serial_number()][source_piece.unwrap().meaning as usize]
            }
            AddressOnPosition::Hand(physical_piece) => {
                let count = board.count_hand(physical_piece);
                self.hands[physical_piece as usize][count as usize]
            }
            AddressOnPosition::Busy => panic!(Beam::trouble(
                "(Err.85) 移動前の駒が設定されていないだって☆（＾～＾）！？"
            )),
        };
        // TODO 移動後の駒
        // TODO 取られた駒
        // TODO 駒台に乗った駒
        // TODO ハッシュ更新
    }

    /// 局面ハッシュを作り直す
    pub fn current_position(&self, game: &Game) -> u64 {
        let mut hash = self.board(&game.board);

        // 手番ハッシュ
        use crate::cosmic::recording::Phase::*;
        match game.history.get_friend() {
            First => hash ^= self.phase[PHASE_FIRST],
            Second => hash ^= self.phase[PHASE_SECOND],
        }

        hash
    }

    /// 初期局面ハッシュを作り直す
    pub fn starting_position(&self, game: &Game) -> u64 {
        let mut hash = self.board(&game.starting_board);

        // 手番ハッシュ（後手固定）
        hash ^= self.phase[PHASE_SECOND];

        hash
    }

    /// 盤面からハッシュ作成
    fn board(&self, board: &Board) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for rank in RANK_1..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let ab_adr = &AbsoluteAddress::new(file, rank);
                if let Some(piece) = board.piece_at(ab_adr) {
                    hash ^= self.piece[ab_adr.serial_number()][piece.meaning as usize];
                }
            }
        }

        // 持ち駒ハッシュ
        HandAddresses::for_all(&mut |adr| {
            let count = board.count_hand(adr);
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
