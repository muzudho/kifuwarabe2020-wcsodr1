//! GameTable. A record of the game used to suspend or resume it.  
//! 局面。 ゲームを中断したり、再開したりするときに使うゲームの記録です。  
use crate::config::PV_BUFFER;
use crate::cosmic::playing::{MovegenPhase, PosNums};
use crate::cosmic::pos_hash::pos_hash::*;
use crate::cosmic::recording::History;
use crate::cosmic::recording::Phase;
use crate::cosmic::recording::{FireAddress, HandAddress, Movement};
use crate::cosmic::smart::features::{
    DoubleFacedPiece, DoubleFacedPieceType, PieceType, PHYSICAL_PIECE_TYPE_LEN,
};
use crate::cosmic::smart::square::AbsoluteAddress2D;
use crate::cosmic::smart::square::BOARD_MEMORY_AREA;
use crate::cosmic::smart::square::RANK10U8;
use crate::cosmic::smart::square::RANK1U8;
use crate::cosmic::toy_box::*;
use crate::law::generate_move::Area;
use crate::law::speed_of_light::Nine299792458;
use crate::look_and_model::piece::Piece;
use crate::spaceship::equipment::DestinationDisplay;
use crate::LogExt;
use casual_logger::Log;
use num_traits::FromPrimitive;
use std::fmt;

/// Position. A record of the game used to suspend or resume it.  
/// 局面。 ゲームを中断したり、再開したりするときに使うゲームの記録です。  
pub struct Position {
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
    pub movegen_phase: MovegenPhase,

    // Principal variation(読み筋)☆（＾～＾）
    pub pv: PrincipalVariation,
}
impl Default for Position {
    fn default() -> Position {
        Position {
            history: History::default(),
            starting_table: GameTable::default(),
            hash_seed: GameHashSeed::default(),
            table: GameTable::default(),
            info: DestinationDisplay::default(),
            movegen_phase: MovegenPhase::default(),
            pv: PrincipalVariation::default(),
        }
    }
}
impl Position {
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット
        self.hash_seed.big_bang();
    }

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
    pub fn do_move(&mut self, turn: Phase, move_: &Movement) {
        // 局面ハッシュを作り直す
        self.hash_seed
            .update_by_do_move(&mut self.history, &self.table, move_);

        // 移動元のマスにある駒をポップすることは確定。
        let src_piece_num = self.table.pop_piece(turn, &move_.source);

        // 持ち駒は成ることは無いので、成るなら盤上の駒であることが確定。
        if move_.promote {
            // 成ったのなら、元のマスの駒を成らすぜ☆（＾～＾）
            if let Some(piece_num) = src_piece_num {
                self.table.promote(piece_num);
            } else {
                panic!(Log::panic(
                    "(Err.248) 成ったのに、元の升に駒がなかった☆（＾～＾）"
                ));
            }
        }
        // 移動先升に駒があるかどうか
        // あれば　盤の相手の駒を先後反転して、自分の駒台に置きます。
        self.table.rotate_piece_board_to_hand(turn, &move_);

        // 移動先升に駒を置く
        self.table
            .push_piece(turn, &move_.destination, src_piece_num);

        // // 局面ハッシュを作り直す
        // let ky_hash = self.hash_seed.current_position(&self);
        // self.history.set_position_hash(ky_hash);

        self.history.ply += 1;
        self.pv.push(&move_);
    }

    /// 逆順に指します。
    pub fn undo_move(&mut self) -> bool {
        self.pv.pop();
        if 0 < self.history.ply {
            // 棋譜から読取、手目も減る
            self.history.ply -= 1;
            let move_ = &self.history.get_move();
            // 移動先にある駒をポップするのは確定。
            let moveing_piece_num = self
                .table
                .pop_piece(self.history.get_turn(), &move_.destination);
            match move_.source {
                FireAddress::Board(_src_sq) => {
                    // 盤上の移動なら
                    if move_.promote {
                        // 成ったなら、成る前へ
                        if let Some(source_piece_num) = moveing_piece_num {
                            self.table.demote(source_piece_num);
                        } else {
                            panic!(Log::panic(
                                "(Err.305) 成ったのに移動先に駒が無いぜ☆（＾～＾）！"
                            ))
                        }
                    }

                    // 打でなければ、移動元升に、動かした駒を置く☆（＾～＾）打なら何もしないぜ☆（＾～＾）
                    self.table.push_piece(
                        self.history.get_turn(),
                        &move_.source,
                        moveing_piece_num,
                    );
                }
                FireAddress::Hand(_src_drop_type) => {
                    // 打なら
                    // 打った場所に駒があるはずだぜ☆（＾～＾）
                    let piece_num = moveing_piece_num.unwrap();
                    // 自分の持ち駒を増やそうぜ☆（＾～＾）！
                    let turn = self.table.get_phase(piece_num);
                    // TODO この駒を置くことになる場所は☆（＾～＾）？
                    self.table.push_piece(
                        turn,
                        &FireAddress::Hand(HandAddress::new(
                            self.table.get_double_faced_piece_type(piece_num),
                            AbsoluteAddress2D::default(),
                        )),
                        moveing_piece_num,
                    );
                }
            }

            // 取った駒が有ったか。
            // あれば、指し手で取った駒の先後をひっくり返せば、自分の駒台にある駒を取り出せるので取り出して、盤の上に指し手の取った駒のまま駒を置きます。
            self.table
                .rotate_piece_hand_to_board(self.history.get_turn(), &move_);

            // TODO 局面ハッシュを作り直したいぜ☆（＾～＾）

            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }
}

/// 卓☆（＾～＾）
/// でかいのでコピーもクローンも不可☆（＾～＾）！
/// 10の位を筋、1の位を段とする。
/// 0筋、0段は未使用
pub struct GameTable {
    /// 盤に、駒が紐づくぜ☆（＾～＾）
    board: [Option<PieceNum>; BOARD_MEMORY_AREA as usize],
    hand_king1_cur: isize,
    hand_rook1_cur: isize,
    hand_bishop1_cur: isize,
    hand_gold1_cur: isize,
    hand_silver1_cur: isize,
    hand_knight1_cur: isize,
    hand_lance1_cur: isize,
    hand_pawn1_cur: isize,
    hand_king2_cur: isize,
    hand_rook2_cur: isize,
    hand_bishop2_cur: isize,
    hand_gold2_cur: isize,
    hand_silver2_cur: isize,
    hand_knight2_cur: isize,
    hand_lance2_cur: isize,
    hand_pawn2_cur: isize,
    /// 背番号付きの駒に、番地が紐づいているぜ☆（＾～＾）
    address_list: [FireAddress; NAMED_PIECES_LEN],
    /// 駒の背番号に、駒が紐づくぜ☆（＾～＾）
    piece_list: [Piece; NAMED_PIECES_LEN],
    /// 駒の背番号を付けるのに使うぜ☆（＾～＾）
    double_faced_piece_type_index: [usize; PHYSICAL_PIECE_TYPE_LEN],
    /// 持ち駒☆（＾～＾）TODO 固定長サイズのスタックを用意したいぜ☆（＾～＾）
    phase_classification: PhaseClassification,
    /// 指し手生成に利用☆（＾～＾）
    pub area: Area,
}
impl Default for GameTable {
    fn default() -> Self {
        GameTable {
            // 盤上
            board: [None; BOARD_MEMORY_AREA],
            /// 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
            address_list: [FireAddress::default(); NAMED_PIECES_LEN],
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
            phase_classification: PhaseClassification::default(),
            area: Area::default(),
            hand_king1_cur: GameTable::hand_start(DoubleFacedPiece::King1),
            hand_rook1_cur: GameTable::hand_start(DoubleFacedPiece::Rook1),
            hand_bishop1_cur: GameTable::hand_start(DoubleFacedPiece::Bishop1),
            hand_gold1_cur: GameTable::hand_start(DoubleFacedPiece::Gold1),
            hand_silver1_cur: GameTable::hand_start(DoubleFacedPiece::Silver1),
            hand_knight1_cur: GameTable::hand_start(DoubleFacedPiece::Knight1),
            hand_lance1_cur: GameTable::hand_start(DoubleFacedPiece::Lance1),
            hand_pawn1_cur: GameTable::hand_start(DoubleFacedPiece::Pawn1),
            hand_king2_cur: GameTable::hand_start(DoubleFacedPiece::King2),
            hand_rook2_cur: GameTable::hand_start(DoubleFacedPiece::Rook2),
            hand_bishop2_cur: GameTable::hand_start(DoubleFacedPiece::Bishop2),
            hand_gold2_cur: GameTable::hand_start(DoubleFacedPiece::Gold2),
            hand_silver2_cur: GameTable::hand_start(DoubleFacedPiece::Silver2),
            hand_knight2_cur: GameTable::hand_start(DoubleFacedPiece::Knight2),
            hand_lance2_cur: GameTable::hand_start(DoubleFacedPiece::Lance2),
            hand_pawn2_cur: GameTable::hand_start(DoubleFacedPiece::Pawn2),
        }
    }
}
impl GameTable {
    pub fn clear(&mut self) {
        self.board = [None; BOARD_MEMORY_AREA];
        // 初期値はゴミ値だぜ☆（＾～＾）上書きして消せだぜ☆（＾～＾）
        self.address_list = [FireAddress::default(); NAMED_PIECES_LEN];
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
        self.phase_classification = PhaseClassification::default();
    }

    /// 開始盤面を、現盤面にコピーしたいときに使うぜ☆（＾～＾）
    pub fn copy_from(&mut self, table: &GameTable) {
        self.board = table.board.clone();
        self.address_list = table.address_list.clone();
        self.piece_list = table.piece_list.clone();
        self.double_faced_piece_type_index = table.double_faced_piece_type_index.clone();
        self.phase_classification = table.phase_classification.clone();
    }

    pub fn get_phase(&self, num: PieceNum) -> Phase {
        self.piece_list[num as usize].phase()
    }
    pub fn get_type(&self, num: PieceNum) -> PieceType {
        self.piece_list[num as usize].type_()
    }
    pub fn get_double_faced_piece(&self, num: PieceNum) -> DoubleFacedPiece {
        self.piece_list[num as usize].double_faced_piece()
    }
    pub fn get_double_faced_piece_type(&self, num: PieceNum) -> DoubleFacedPieceType {
        self.piece_list[num as usize].double_faced_piece().type_()
    }
    fn new_piece_num(&mut self, piece: Piece, num: PieceNum) -> PieceNum {
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
    pub fn rotate_piece_board_to_hand(&mut self, turn: Phase, move_: &Movement) {
        if let Some(collision_piece_num_val) = self.pop_piece(turn, &move_.destination) {
            // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
            // 先後ひっくり返す。
            self.turn_phase(collision_piece_num_val);
            self.push_piece(
                turn,
                &FireAddress::Hand(HandAddress::new(
                    self.get_double_faced_piece_type(collision_piece_num_val),
                    AbsoluteAddress2D::default(),
                )),
                Some(collision_piece_num_val),
            );
        }
    }

    /// アンドゥ時の動き。
    /// あれば、指し手で取った駒の先後をひっくり返せば、自分の駒台にある駒を取り出せるので取り出して、盤の上に指し手の取った駒のまま駒を置きます。
    pub fn rotate_piece_hand_to_board(&mut self, turn: Phase, move_: &Movement) {
        if let Some(move2_val) = move_.captured {
            // アンドゥだから盤にまだない。
            let piece_type = self.last_hand_type(turn, &move2_val.destination).unwrap();

            // 取った方の駒台の先後に合わせるぜ☆（＾～＾）
            // 取った方の持ち駒を減らす
            let piece_num = {
                // TODO テスト中☆（＾～＾）
                let double_faced_piece = DoubleFacedPiece::from_phase_and_type(
                    turn,
                    piece_type.double_faced_piece_type(),
                );
                let fire1 = FireAddress::Hand(HandAddress::new(
                    double_faced_piece.type_(),
                    AbsoluteAddress2D::default(),
                ));
                self.pop_piece(turn, &fire1).unwrap()
            };
            // 先後をひっくり返す。
            self.turn_phase(piece_num);
            if piece_type.promoted() {
                // 成り駒にします。
                self.promote(piece_num);
            } else {
                // 成っていない駒にします。
                self.demote(piece_num);
            }
            // 取られた方に、駒を返すぜ☆（＾～＾）置くのは指し手の移動先☆（＾～＾）
            self.push_piece(turn, &move_.destination, Some(piece_num));
        }
    }
    /// 駒を置く。
    pub fn push_piece(&mut self, turn: Phase, fire: &FireAddress, piece_num: Option<PieceNum>) {
        match fire {
            FireAddress::Board(sq) => {
                if let Some(piece_num_val) = piece_num {
                    // マスに駒を置きます。
                    self.board[sq.serial_number() as usize] = piece_num;
                    // データベース
                    self.push_hand(turn, &FireAddress::Board(*sq), piece_num_val);
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] = FireAddress::Board(*sq);
                } else {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                }
            }
            FireAddress::Hand(drop_type) => {
                if let Some(piece_num_val) = piece_num {
                    // 持ち駒を１つ増やします。
                    self.push_hand(turn, &FireAddress::Hand(*drop_type), piece_num_val);
                    // 背番号に番地を紐づけます。
                    self.address_list[piece_num_val as usize] = *fire;
                }
            }
        }
    }
    /// 駒を取りのぞく。
    pub fn pop_piece(&mut self, turn: Phase, fire: &FireAddress) -> Option<PieceNum> {
        match fire {
            FireAddress::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    // マスを空にします。
                    self.board[sq.serial_number() as usize] = None;
                    // データベース
                    self.pop_hand(turn, &fire);
                    // TODO 背番号の番地を、ゴミ値で塗りつぶすが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                    self.address_list[piece_num_val as usize] = FireAddress::default();
                }
                piece_num
            }
            FireAddress::Hand(_drop_type) => {
                // 場所で指定します。
                // 台から取りのぞきます。
                let piece_num = self.pop_hand(turn, &fire);
                // TODO 背番号の番地に、ゴミ値を入れて消去するが、できれば pop ではなく swap にしろだぜ☆（＾～＾）
                self.address_list[piece_num as usize] = FireAddress::default();
                Some(piece_num)
            }
        }
    }

    /// 散らばっている駒に、背番号を付けて、駒台に置くぜ☆（＾～＾）
    pub fn init_hand(&mut self, turn: Phase, piece_type: PieceType) {
        // 駒に背番号を付けるぜ☆（＾～＾）
        let piece_num = self.numbering_piece(turn, piece_type);
        // 駒台に置くぜ☆（＾～＾）
        let drop = FireAddress::Hand(HandAddress::new(
            self.get_double_faced_piece_type(piece_num),
            AbsoluteAddress2D::default(),
        ));
        self.push_piece(turn, &drop, Some(piece_num));
    }

    /// 駒の新しい背番号を生成します。
    pub fn numbering_piece(&mut self, turn: Phase, piece_type: PieceType) -> PieceNum {
        let piece = Piece::from_phase_and_piece_type(turn, piece_type);
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
    pub fn exists_pawn_on_file(&self, turn: Phase, file: u8) -> bool {
        for rank in RANK1U8..RANK10U8 {
            if let Some(piece_num) =
                self.piece_num_board_at(&FireAddress::Board(AbsoluteAddress2D::new(file, rank)))
            {
                if self.get_phase(piece_num) == turn && self.get_type(piece_num) == PieceType::Pawn
                {
                    return true;
                }
            }
        }
        false
    }
    /// ハッシュを作るときに利用。盤上専用。
    pub fn get_piece_board_hash_index(&self, addr: &FireAddress) -> Option<usize> {
        match addr {
            FireAddress::Board(sq) => {
                if let Some(piece_num) = self.board[sq.serial_number() as usize] {
                    Some(self.piece_list[piece_num as usize] as usize)
                } else {
                    None
                }
            }
            FireAddress::Hand(_drop_type) => {
                panic!(Log::panic(&format!("(Err.345) 駒台は非対応☆（＾～＾）！",)))
            }
        }
    }
    /// TODO Piece をカプセル化したい。外に出したくないぜ☆（＾～＾）
    /// 升で指定して駒を取得。
    /// 駒台には対応してない。 -> 何に使っている？
    pub fn piece_num_at(&self, turn: Phase, fire: &FireAddress) -> Option<PieceNum> {
        match fire {
            FireAddress::Board(sq) => self.board[sq.serial_number() as usize],
            _ => {
                self.last_hand_num(turn, fire)
                /*
                    panic!(Log::panic(&format!(
                    "(Err.254) まだ駒台は実装してないぜ☆（＾～＾）！",
                )))
                */
            }
        }
    }
    pub fn piece_num_board_at(&self, addr: &FireAddress) -> Option<PieceNum> {
        match addr {
            FireAddress::Board(sq) => self.board[sq.serial_number() as usize],
            _ => panic!(Log::panic(&format!(
                "(Err.254) まだ駒台は実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 通常盤表示用。
    pub fn piece_info_at1(&self, addr: &FireAddress) -> Option<PieceInfo> {
        match addr {
            FireAddress::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    Some(PieceInfo::new(
                        &format!("{}", self.piece_list[piece_num_val as usize]),
                        piece_num_val,
                    ))
                } else {
                    None
                }
            }
            _ => panic!(Log::panic(&format!(
                "(Err.321) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 盤2表示用。
    pub fn piece_info_num_at(&self, addr: &FireAddress) -> Option<PieceInfo> {
        match addr {
            FireAddress::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    Some(PieceInfo::new(&format!("{}", piece_num_val), piece_num_val))
                } else {
                    None
                }
            }
            _ => panic!(Log::panic(&format!(
                "(Err.321) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 盤2表示用。
    pub fn piece_info_address_at(&self, piece_num: PieceNum) -> Option<PieceInfo> {
        Some(PieceInfo::new(
            &format!("{}", self.address_list[piece_num as usize]),
            piece_num,
        ))
    }
    /// 盤2表示用。
    pub fn piece_info_piece_at(&self, piece_num: PieceNum) -> Option<PieceInfo> {
        Some(PieceInfo::new(
            &format!("{}", self.piece_list[piece_num as usize]),
            piece_num,
        ))
    }
    pub fn promotion_value_at(&self, table: &GameTable, fire: &FireAddress) -> isize {
        match fire {
            FireAddress::Board(sq) => {
                let piece_num = self.board[sq.serial_number() as usize];
                if let Some(piece_num_val) = piece_num {
                    table
                        .get_double_faced_piece_type(piece_num_val)
                        .promotion_value()
                } else {
                    // 打なら成りは無いぜ☆（＾～＾）
                    0
                }
            }
            FireAddress::Hand(_drop_type) => panic!(Log::panic(&format!(
                "(Err.254) まだ実装してないぜ☆（＾～＾）！",
            ))),
        }
    }
    /// 指し手生成で使うぜ☆（＾～＾）有無を調べるぜ☆（＾～＾）
    pub fn last_hand_type(&self, turn: Phase, fire: &FireAddress) -> Option<PieceType> {
        if let Some(piece_num) = self.last_hand_num(turn, &fire) {
            Some(self.get_type(piece_num))
        } else {
            None
        }
    }
    pub fn count_hand(&self, turn: Phase, fire: &FireAddress) -> usize {
        match fire {
            FireAddress::Board(_sq) => {
                panic!(Log::panic(&format!("(Err.3266) 未対応☆（＾～＾）！",)))
            }
            FireAddress::Hand(drop_type) => self.len_hand(turn, &FireAddress::Hand(*drop_type)),
        }
    }

    /// 表示に使うだけ☆（＾～＾）
    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    pub fn for_all_pieces_on_table<F>(&self, piece_get: &mut F)
    where
        F: FnMut(usize, Option<&AbsoluteAddress2D>, Option<PieceInfo>),
    {
        for (i, fire) in self.address_list.iter().enumerate() {
            match fire {
                FireAddress::Board(sq) => {
                    // 盤上の駒☆（＾～＾）
                    let piece_info = self.piece_info_at1(&fire).unwrap();
                    piece_get(i, Some(&sq), Some(piece_info));
                }
                FireAddress::Hand(_drop) => {
                    // TODO 持ち駒☆（＾～＾）
                    piece_get(i, None, None);
                }
            }
        }
    }

    /// 盤上を検索するのではなく、４０個の駒を検索するぜ☆（＾～＾）
    /// TODO 自分、相手で分けて持っておけば２倍ぐらい短縮できないか☆（＾～＾）？
    /// TODO できれば、「自分の盤上の駒」「自分の持ち駒」「相手の盤上の駒」「相手の持ち駒」の４チャンネルで分けておけないか☆（＾～＾）？
    pub fn for_some_pieces_on_list40<F>(&self, turn: Phase, piece_get: &mut F)
    where
        F: FnMut(&FireAddress),
    {
        for piece_num in Nine299792458::piece_numbers().iter() {
            // 盤上の駒だけを調べようぜ☆（＾～＾）
            let fire = self.address_list[*piece_num as usize];
            match fire {
                FireAddress::Board(_sq) => {
                    if self.get_phase(*piece_num) == turn {
                        piece_get(&fire);
                    }
                }
                FireAddress::Hand(_drop) => {
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
        for drop in &FIRST_SECOND[turn as usize] {
            let fire =
                &FireAddress::Hand(HandAddress::new(drop.type_(), AbsoluteAddress2D::default()));
            if !self.is_empty_hand(turn, fire) {
                piece_get(&FireAddress::Hand(HandAddress::new(
                    drop.type_(),
                    AbsoluteAddress2D::default(),
                ))); // TODO この fire は使い回せないのかだぜ☆（＾～＾）？
            }
        }
    }

    /// 開始地点。
    fn hand_start(double_faced_piece: DoubleFacedPiece) -> isize {
        match double_faced_piece {
            DoubleFacedPiece::King1 => 2,
            DoubleFacedPiece::Rook1 => 103,
            DoubleFacedPiece::Bishop1 => 101,
            DoubleFacedPiece::Gold1 => 6,
            DoubleFacedPiece::Silver1 => 10,
            DoubleFacedPiece::Knight1 => 50,
            DoubleFacedPiece::Lance1 => 90,
            DoubleFacedPiece::Pawn1 => 121,
            DoubleFacedPiece::King2 => 1,
            DoubleFacedPiece::Rook2 => 102,
            DoubleFacedPiece::Bishop2 => 100,
            DoubleFacedPiece::Gold2 => 3,
            DoubleFacedPiece::Silver2 => 7,
            DoubleFacedPiece::Knight2 => 20,
            DoubleFacedPiece::Lance2 => 60,
            DoubleFacedPiece::Pawn2 => 104,
        }
    }
    /// 向き。
    fn hand_direction(double_faced_piece: DoubleFacedPiece) -> isize {
        match double_faced_piece {
            DoubleFacedPiece::King1 => -1,
            DoubleFacedPiece::Rook1 => -1,
            DoubleFacedPiece::Bishop1 => -1,
            DoubleFacedPiece::Gold1 => -1,
            DoubleFacedPiece::Silver1 => -1,
            DoubleFacedPiece::Knight1 => -10,
            DoubleFacedPiece::Lance1 => -10,
            DoubleFacedPiece::Pawn1 => -1,
            DoubleFacedPiece::King2 => 1,
            DoubleFacedPiece::Rook2 => 1,
            DoubleFacedPiece::Bishop2 => 1,
            DoubleFacedPiece::Gold2 => 1,
            DoubleFacedPiece::Silver2 => 1,
            DoubleFacedPiece::Knight2 => 10,
            DoubleFacedPiece::Lance2 => 10,
            DoubleFacedPiece::Pawn2 => 1,
        }
    }

    /// 駒の先後を ひっくり返してから入れてください。
    pub fn push_hand(&mut self, turn: Phase, fire: &FireAddress, num: PieceNum) {
        match fire {
            FireAddress::Board(_sq) => {
                // TODO 現在未実装だが、あとで使う☆（＾～＾）
            }
            FireAddress::Hand(drop_type) => {
                let drop = DoubleFacedPiece::from_phase_and_type(turn, drop_type.old);
                // 駒台に駒を置くぜ☆（＾～＾）
                self.board[self.hand_cur(drop) as usize] = Some(num);
                // 位置を増減するぜ☆（＾～＾）
                self.add_hand_cur(drop, GameTable::hand_direction(drop));
            }
        }
    }
    pub fn pop_hand(&mut self, turn: Phase, fire: &FireAddress) -> PieceNum {
        match fire {
            FireAddress::Board(_sq) => {
                // TODO 先端の要素をポップ。
                // let peak = self.items[self.board_cur(turn) as usize];
                // TODO 中段の要素をポップ。
                // let middle =
                // TODO さっき抜いた先端の要素を、中段の要素のところへスワップ。
                PieceNum::King1 // ゴミ値を返しとくぜ☆（＾～＾）
            }
            FireAddress::Hand(drop_type) => {
                let drop = DoubleFacedPiece::from_phase_and_type(turn, drop_type.old);
                // 位置を増減するぜ☆（＾～＾）
                self.add_hand_cur(drop, -GameTable::hand_direction(drop));
                // 駒台の駒をはがすぜ☆（＾～＾）
                let num = self.board[self.hand_cur(drop) as usize].unwrap();
                self.board[self.hand_cur(drop) as usize] = None;
                num
            }
        }
    }

    /// 指し手生成で使うぜ☆（＾～＾）
    pub fn last_hand(&self, turn: Phase, fire: &FireAddress) -> Option<(PieceType, FireAddress)> {
        match fire {
            FireAddress::Board(_sq) => {
                panic!(Log::panic(&format!("(Err.3251) 未対応☆（＾～＾）！",)))
            }
            FireAddress::Hand(drop_type) => {
                if let Some(piece_num) = self.last_hand_num(
                    turn,
                    &FireAddress::Hand(HandAddress::new(
                        drop_type.old,
                        AbsoluteAddress2D::default(),
                    )),
                ) {
                    let piece = self.piece_list[piece_num as usize];
                    Some((
                        piece.type_(),
                        FireAddress::Hand(HandAddress::new(
                            piece.double_faced_piece().type_(),
                            AbsoluteAddress2D::default(),
                        )),
                    ))
                } else {
                    None
                }
            }
        }
    }
    pub fn last_hand_num(&self, turn: Phase, fire: &FireAddress) -> Option<PieceNum> {
        match fire {
            FireAddress::Board(_sq) => panic!(Log::panic("(Err.3431) 未対応☆（＾～＾）")),
            FireAddress::Hand(drop_type) => {
                let drop = DoubleFacedPiece::from_phase_and_type(turn, drop_type.old);
                let direction = GameTable::hand_direction(drop);
                if direction < 0 {
                    // 先手
                    if self.hand_cur(drop) < GameTable::hand_start(drop) {
                        self.board[(self.hand_cur(drop) - direction) as usize]
                    } else {
                        None
                    }
                } else {
                    if GameTable::hand_start(drop) < self.hand_cur(drop) {
                        self.board[(self.hand_cur(drop) - direction) as usize]
                    } else {
                        None
                    }
                }
            }
        }
    }

    /// 指し手生成で使うぜ☆（＾～＾）有無を調べるぜ☆（＾～＾）
    pub fn is_empty_hand(&self, turn: Phase, fire: &FireAddress) -> bool {
        match fire {
            FireAddress::Board(_sq) => panic!(Log::panic("(Err.3431) 未対応☆（＾～＾）")),
            FireAddress::Hand(drop_type) => {
                let drop = DoubleFacedPiece::from_phase_and_type(turn, drop_type.old);
                if GameTable::hand_direction(drop) < 0 {
                    // 先手
                    if self.hand_cur(drop) < GameTable::hand_start(drop) {
                        false
                    } else {
                        true
                    }
                } else {
                    if GameTable::hand_start(drop) < self.hand_cur(drop) {
                        false
                    } else {
                        true
                    }
                }
            }
        }
    }

    fn len_hand(&self, turn: Phase, fire: &FireAddress) -> usize {
        match fire {
            FireAddress::Board(_sq) => panic!(Log::panic("(Err.3431) 未対応☆（＾～＾）")),
            FireAddress::Hand(drop_type) => {
                let drop = DoubleFacedPiece::from_phase_and_type(turn, drop_type.old);
                if GameTable::hand_direction(drop) < 0 {
                    // 先手
                    (GameTable::hand_start(drop) - self.hand_cur(drop)) as usize
                } else {
                    (self.hand_cur(drop) - GameTable::hand_start(drop)) as usize
                }
            }
        }
    }

    fn hand_cur(&self, double_faced_piece: DoubleFacedPiece) -> isize {
        match double_faced_piece {
            DoubleFacedPiece::King1 => self.hand_king1_cur,
            DoubleFacedPiece::Rook1 => self.hand_rook1_cur,
            DoubleFacedPiece::Bishop1 => self.hand_bishop1_cur,
            DoubleFacedPiece::Gold1 => self.hand_gold1_cur,
            DoubleFacedPiece::Silver1 => self.hand_silver1_cur,
            DoubleFacedPiece::Knight1 => self.hand_knight1_cur,
            DoubleFacedPiece::Lance1 => self.hand_lance1_cur,
            DoubleFacedPiece::Pawn1 => self.hand_pawn1_cur,
            DoubleFacedPiece::King2 => self.hand_king2_cur,
            DoubleFacedPiece::Rook2 => self.hand_rook2_cur,
            DoubleFacedPiece::Bishop2 => self.hand_bishop2_cur,
            DoubleFacedPiece::Gold2 => self.hand_gold2_cur,
            DoubleFacedPiece::Silver2 => self.hand_silver2_cur,
            DoubleFacedPiece::Knight2 => self.hand_knight2_cur,
            DoubleFacedPiece::Lance2 => self.hand_lance2_cur,
            DoubleFacedPiece::Pawn2 => self.hand_pawn2_cur,
        }
    }
    fn add_hand_cur(&mut self, double_faced_piece: DoubleFacedPiece, direction: isize) {
        match double_faced_piece {
            DoubleFacedPiece::King1 => self.hand_king1_cur += direction,
            DoubleFacedPiece::Rook1 => self.hand_rook1_cur += direction,
            DoubleFacedPiece::Bishop1 => self.hand_bishop1_cur += direction,
            DoubleFacedPiece::Gold1 => self.hand_gold1_cur += direction,
            DoubleFacedPiece::Silver1 => self.hand_silver1_cur += direction,
            DoubleFacedPiece::Knight1 => self.hand_knight1_cur += direction,
            DoubleFacedPiece::Lance1 => self.hand_lance1_cur += direction,
            DoubleFacedPiece::Pawn1 => self.hand_pawn1_cur += direction,
            DoubleFacedPiece::King2 => self.hand_king2_cur += direction,
            DoubleFacedPiece::Rook2 => self.hand_rook2_cur += direction,
            DoubleFacedPiece::Bishop2 => self.hand_bishop2_cur += direction,
            DoubleFacedPiece::Gold2 => self.hand_gold2_cur += direction,
            DoubleFacedPiece::Silver2 => self.hand_silver2_cur += direction,
            DoubleFacedPiece::Knight2 => self.hand_knight2_cur += direction,
            DoubleFacedPiece::Lance2 => self.hand_lance2_cur += direction,
            DoubleFacedPiece::Pawn2 => self.hand_pawn2_cur += direction,
        }
    }
}

#[derive(Clone)]
pub struct PrincipalVariation {
    text: String,
    ply: usize,
}
impl Default for PrincipalVariation {
    fn default() -> Self {
        PrincipalVariation {
            // ゴミの値で埋めるぜ☆（＾～＾）
            text: String::with_capacity(PV_BUFFER),
            ply: 0,
        }
    }
}
impl PrincipalVariation {
    pub fn push(&mut self, movement: &Movement) {
        if self.text.is_empty() {
            self.text.push_str(&movement.to_string());
        } else {
            self.text.push_str(&format!(" {}", movement));
        }
        self.ply += 1;
    }

    pub fn pop(&mut self) {
        // None か スペースが出てくるまで削除しようぜ☆（＾～＾）
        loop {
            if let Some(ch) = self.text.pop() {
                if ch == ' ' {
                    break;
                }
            } else {
                break;
            }
        }

        if 0 < self.ply {
            self.ply -= 1;
        }
    }

    pub fn len(&self) -> usize {
        self.ply
    }
}
impl fmt::Display for PrincipalVariation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
