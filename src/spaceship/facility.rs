use crate::cosmic::playing::{Game, PosNums};
use crate::cosmic::recording::{AddressPos, Movement};
use crate::cosmic::smart::features::{PhysicalPiece, PIECE_WHITE_SPACE};
use crate::cosmic::smart::square::*;
use crate::cosmic::toy_box::*;
use crate::spaceship::equipment::Beam;

/// 指令室はこちらだぜ☆（＾～＾）！
pub struct CommandRoom {}
impl CommandRoom {
    // 対話モードのタイトル画面
    pub fn print_title() {
        // 横幅は 半角79文字使えるぜ☆（＾～＾）
        // 80文字目を使うと、次の行が改行で空行になってしまう☆（＾～＾）
        Beam::shoot(
            &"\
+--------- --------- --------- --------- --------- --------- --------- -------+
| KifuWarabe Shogi 2020                                                       |
+---------+--------- --------- --------- --------- --------- --------- -------+
          | Created by Muzudho (Doujin Circle Grayscale)                      |
          +--------- --------- --------- --------- --------- --------- -------+
05
          [Enter]
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23\
"
            .to_string(),
        );
    }
}

/// ゲームルームはこちらだぜ☆（＾～＾）！
pub struct GameRoom {}
impl GameRoom {
    fn to_string3(table: &GameTable, file: usize, rank: usize) -> String {
        if let Some(piece_info_val) =
            table.piece_info_at(&AddressPos::Board(AbsoluteAddress2D::new(file, rank)))
        {
            format!("{}", piece_info_val.meaning)
        } else {
            PIECE_WHITE_SPACE.to_string()
        }
    }
    /// 表示
    pub fn to_string(game: &Game, pos_nums: PosNums) -> String {
        let table = game.get_table(pos_nums);
        let ply = game.history.ply;
        let phase = game.history.get_friend();
        let same_pos_count = game.count_same_position();

        // 局面表示
        format!(
            "\
[{95} ply. {96} phase. {97} repeats.]

         9    8    7    6    5    4    3    2    1
        +----+----+----+----+----+----+----+----+----+
▲       |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}| a1   ▽
        +----+----+----+----+----+----+----+----+----+
R x{81:2}   |{9}|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}| b2   r x{88:2}
        +----+----+----+----+----+----+----+----+----+
B x{82:2}   |{18}|{19}|{20}|{21}|{22}|{23}|{24}|{25}|{26}| c3   b x{89:2}
        +----+----+----+----+----+----+----+----+----+
G x{83:2}   |{27}|{28}|{29}|{30}|{31}|{32}|{33}|{34}|{35}| d4   g x{90:2}
        +----+----+----+----+----+----+----+----+----+
S x{84:2}   |{36}|{37}|{38}|{39}|{40}|{41}|{42}|{43}|{44}| e5   s x{91:2}
        +----+----+----+----+----+----+----+----+----+
N x{85:2}   |{45}|{46}|{47}|{48}|{49}|{50}|{51}|{52}|{53}| f6   n x{92:2}
        +----+----+----+----+----+----+----+----+----+
L x{86:2}   |{54}|{55}|{56}|{57}|{58}|{59}|{60}|{61}|{62}| g7   l x{93:2}
        +----+----+----+----+----+----+----+----+----+
P x{87:2}   |{63}|{64}|{65}|{66}|{67}|{68}|{69}|{70}|{71}| h8   p x{94:2}
        +----+----+----+----+----+----+----+----+----+
        |{72}|{73}|{74}|{75}|{76}|{77}|{78}|{79}|{80}| i9
        +----+----+----+----+----+----+----+----+----+\
",
            GameRoom::to_string3(table, 9, 1),
            GameRoom::to_string3(table, 8, 1),
            GameRoom::to_string3(table, 7, 1),
            GameRoom::to_string3(table, 6, 1),
            GameRoom::to_string3(table, 5, 1),
            GameRoom::to_string3(table, 4, 1),
            GameRoom::to_string3(table, 3, 1),
            GameRoom::to_string3(table, 2, 1),
            GameRoom::to_string3(table, 1, 1),
            GameRoom::to_string3(table, 9, 2),
            GameRoom::to_string3(table, 8, 2),
            GameRoom::to_string3(table, 7, 2),
            GameRoom::to_string3(table, 6, 2),
            GameRoom::to_string3(table, 5, 2),
            GameRoom::to_string3(table, 4, 2),
            GameRoom::to_string3(table, 3, 2),
            GameRoom::to_string3(table, 2, 2),
            GameRoom::to_string3(table, 1, 2),
            GameRoom::to_string3(table, 9, 3),
            GameRoom::to_string3(table, 8, 3),
            GameRoom::to_string3(table, 7, 3),
            GameRoom::to_string3(table, 6, 3),
            GameRoom::to_string3(table, 5, 3),
            GameRoom::to_string3(table, 4, 3),
            GameRoom::to_string3(table, 3, 3),
            GameRoom::to_string3(table, 2, 3),
            GameRoom::to_string3(table, 1, 3),
            GameRoom::to_string3(table, 9, 4),
            GameRoom::to_string3(table, 8, 4),
            GameRoom::to_string3(table, 7, 4),
            GameRoom::to_string3(table, 6, 4),
            GameRoom::to_string3(table, 5, 4),
            GameRoom::to_string3(table, 4, 4),
            GameRoom::to_string3(table, 3, 4),
            GameRoom::to_string3(table, 2, 4),
            GameRoom::to_string3(table, 1, 4),
            GameRoom::to_string3(table, 9, 5),
            GameRoom::to_string3(table, 8, 5),
            GameRoom::to_string3(table, 7, 5),
            GameRoom::to_string3(table, 6, 5),
            GameRoom::to_string3(table, 5, 5),
            GameRoom::to_string3(table, 4, 5),
            GameRoom::to_string3(table, 3, 5),
            GameRoom::to_string3(table, 2, 5),
            GameRoom::to_string3(table, 1, 5),
            GameRoom::to_string3(table, 9, 6),
            GameRoom::to_string3(table, 8, 6),
            GameRoom::to_string3(table, 7, 6),
            GameRoom::to_string3(table, 6, 6),
            GameRoom::to_string3(table, 5, 6),
            GameRoom::to_string3(table, 4, 6),
            GameRoom::to_string3(table, 3, 6),
            GameRoom::to_string3(table, 2, 6),
            GameRoom::to_string3(table, 1, 6),
            GameRoom::to_string3(table, 9, 7),
            GameRoom::to_string3(table, 8, 7),
            GameRoom::to_string3(table, 7, 7),
            GameRoom::to_string3(table, 6, 7),
            GameRoom::to_string3(table, 5, 7),
            GameRoom::to_string3(table, 4, 7),
            GameRoom::to_string3(table, 3, 7),
            GameRoom::to_string3(table, 2, 7),
            GameRoom::to_string3(table, 1, 7),
            GameRoom::to_string3(table, 9, 8),
            GameRoom::to_string3(table, 8, 8),
            GameRoom::to_string3(table, 7, 8),
            GameRoom::to_string3(table, 6, 8),
            GameRoom::to_string3(table, 5, 8),
            GameRoom::to_string3(table, 4, 8),
            GameRoom::to_string3(table, 3, 8),
            GameRoom::to_string3(table, 2, 8),
            GameRoom::to_string3(table, 1, 8),
            GameRoom::to_string3(table, 9, 9),
            GameRoom::to_string3(table, 8, 9),
            GameRoom::to_string3(table, 7, 9),
            GameRoom::to_string3(table, 6, 9),
            GameRoom::to_string3(table, 5, 9),
            GameRoom::to_string3(table, 4, 9),
            GameRoom::to_string3(table, 3, 9),
            GameRoom::to_string3(table, 2, 9),
            GameRoom::to_string3(table, 1, 9),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            table.count_hand(PhysicalPiece::Rook1),
            table.count_hand(PhysicalPiece::Bishop1),
            table.count_hand(PhysicalPiece::Gold1),
            table.count_hand(PhysicalPiece::Silver1),
            table.count_hand(PhysicalPiece::Knight1),
            table.count_hand(PhysicalPiece::Lance1),
            table.count_hand(PhysicalPiece::Pawn1),
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            table.count_hand(PhysicalPiece::Rook2),
            table.count_hand(PhysicalPiece::Bishop2),
            table.count_hand(PhysicalPiece::Gold2),
            table.count_hand(PhysicalPiece::Silver2),
            table.count_hand(PhysicalPiece::Knight2),
            table.count_hand(PhysicalPiece::Lance2),
            table.count_hand(PhysicalPiece::Pawn2),
            ply,
            phase,
            same_pos_count
        )
    }
}

/// 台所はこちらだぜ☆（＾～＾）！指し手の一覧が見れるぜ☆（＾～＾）！
pub struct Kitchen {}
impl Kitchen {
    /// 現在の局面での、指し手の一覧を表示するぜ☆（＾～＾）
    pub fn print_ways(table: &GameTable, ways: &Vec<Movement>) {
        Beam::shoot(&format!("Moves count={}", ways.len()));
        // 辞書順ソート
        let mut move_names = Vec::new();
        for move_ in ways {
            let ss_str = format!(
                "{}{}",
                format!("{}", move_),
                if let Some(captured_move) = move_.captured {
                    format!(" ({})", captured_move.piece_type)
                } else {
                    "".to_string()
                }
            );
            move_names.push(ss_str);
        }
        // move_names.sort();
        move_names.sort_by(|y_str, x_str| {
            let y_arr: Vec<_> = y_str.chars().collect();
            let x_arr: Vec<_> = x_str.chars().collect();
            use std::cmp::min;
            let len = min(y_arr.len(), x_arr.len());

            use std::cmp::Ordering;
            for i in 0..len {
                match x_arr[i].cmp(&y_arr[i]) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => {}
                }
            }

            // Returns Ordering::Greater, Ordering::Less, Ordering::Equal.
            x_arr.len().cmp(&y_arr.len())
        });
        move_names.reverse();

        for (i, move_name) in move_names.into_iter().enumerate() {
            Beam::shoot(&format!("[{}] {}", i, move_name));
        }
    }
}
