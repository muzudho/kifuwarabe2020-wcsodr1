use crate::cosmic::playing::{Game, PosNums};
use crate::cosmic::recording::{FireAddress, Movement, Phase};
use crate::cosmic::smart::features::DoubleFacedPieceType;
use crate::cosmic::smart::square::AbsoluteAddress2D;
use crate::cosmic::toy_box::{GameTable, PIECE_WHITE_SPACE};
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
    fn to_string3(table: &GameTable, file: u8, rank: u8) -> String {
        if let Some(piece_info_val) =
            table.piece_info_at1(&FireAddress::Board(AbsoluteAddress2D::new(file, rank)))
        {
            format!("{}", piece_info_val.piece)
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
            table.count_hand(Phase::First, &FireAddress::Hand(DoubleFacedPieceType::Rook)),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(DoubleFacedPieceType::Bishop)
            ),
            table.count_hand(Phase::First, &FireAddress::Hand(DoubleFacedPieceType::Gold)),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(DoubleFacedPieceType::Silver)
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(DoubleFacedPieceType::Knight)
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(DoubleFacedPieceType::Lance)
            ),
            table.count_hand(Phase::First, &FireAddress::Hand(DoubleFacedPieceType::Pawn)),
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(DoubleFacedPieceType::Rook)
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(DoubleFacedPieceType::Bishop)
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(DoubleFacedPieceType::Gold)
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(DoubleFacedPieceType::Silver)
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(DoubleFacedPieceType::Knight)
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(DoubleFacedPieceType::Lance)
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(DoubleFacedPieceType::Pawn)
            ),
            ply,
            phase,
            same_pos_count
        )
    }
}

/// シアター・ルームはこちらだぜ☆（＾～＾）！
pub struct TheaterRoom {}
impl TheaterRoom {
    fn to_string3(table: &GameTable, serial: u8) -> String {
        if let Some(sq) = AbsoluteAddress2D::from_absolute_address(serial) {
            if let Some(piece_info_val) = table.piece_info_at1(&FireAddress::Board(sq)) {
                format!("{}", piece_info_val.piece).to_string()
            } else {
                "    ".to_string()
            }
        } else {
            // 0 は None.
            "    ".to_string()
        }
    }
    /// 表示
    pub fn to_string(game: &Game, pos_nums: PosNums) -> String {
        let table = game.get_table(pos_nums);
        let ply = game.history.ply;
        let phase = game.history.get_friend();
        let same_pos_count = game.count_same_position();

        // 局面表示
        // フォーマットの引数は 98個まで。
        format!(
            "{}{}{}",
            format!(
                "[{0} ply. {1} phase. {2} repeats.]

",
                ply, phase, same_pos_count
            ),
            format!(
                "  12   11   10    9    8    7    6    5    4    3    2    1    0
+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{60} {55}|{50}|{45} {40} {35} {30}|{25} {20} {15} {10}|{5 }|{0 }| z
+    +    +    +----+----+----+----+----+----+----+----+----+----+
|{61} {56}|{51}|{46}|{41}|{36}|{31}|{26}|{21}|{16}|{11}|{6 }|{1 }| a
+----+    +----+----+----+----+----+----+----+----+----+----+    +
     |{57}|{52}|{47}|{42}|{37}|{32}|{27}|{22}|{17}|{12}|{7 }|{2 }| b
     +    +    +----+----+----+----+----+----+----+----+----+----+
     |{58}|{53}|{48}|{43}|{38}|{33}|{28}|{23}|{18}|{13}|{8 }|{3 }| c
     +    +----+----+----+----+----+----+----+----+----+----+    +
     |{59} {54}|{49}|{44}|{39}|{34}|{29}|{24}|{19}|{14}|{9 }|{4 }| d
",
                TheaterRoom::to_string3(table, 0),
                TheaterRoom::to_string3(table, 1),
                TheaterRoom::to_string3(table, 2),
                TheaterRoom::to_string3(table, 3),
                TheaterRoom::to_string3(table, 4),
                TheaterRoom::to_string3(table, 10),
                TheaterRoom::to_string3(table, 11),
                TheaterRoom::to_string3(table, 12),
                TheaterRoom::to_string3(table, 13),
                TheaterRoom::to_string3(table, 14),
                TheaterRoom::to_string3(table, 20),
                TheaterRoom::to_string3(table, 21),
                TheaterRoom::to_string3(table, 22),
                TheaterRoom::to_string3(table, 23),
                TheaterRoom::to_string3(table, 24),
                TheaterRoom::to_string3(table, 30),
                TheaterRoom::to_string3(table, 31),
                TheaterRoom::to_string3(table, 32),
                TheaterRoom::to_string3(table, 33),
                TheaterRoom::to_string3(table, 34),
                TheaterRoom::to_string3(table, 40),
                TheaterRoom::to_string3(table, 41),
                TheaterRoom::to_string3(table, 42),
                TheaterRoom::to_string3(table, 43),
                TheaterRoom::to_string3(table, 44),
                TheaterRoom::to_string3(table, 50),
                TheaterRoom::to_string3(table, 51),
                TheaterRoom::to_string3(table, 52),
                TheaterRoom::to_string3(table, 53),
                TheaterRoom::to_string3(table, 54),
                TheaterRoom::to_string3(table, 60),
                TheaterRoom::to_string3(table, 61),
                TheaterRoom::to_string3(table, 62),
                TheaterRoom::to_string3(table, 63),
                TheaterRoom::to_string3(table, 64),
                TheaterRoom::to_string3(table, 70),
                TheaterRoom::to_string3(table, 71),
                TheaterRoom::to_string3(table, 72),
                TheaterRoom::to_string3(table, 73),
                TheaterRoom::to_string3(table, 74),
                TheaterRoom::to_string3(table, 80),
                TheaterRoom::to_string3(table, 81),
                TheaterRoom::to_string3(table, 82),
                TheaterRoom::to_string3(table, 83),
                TheaterRoom::to_string3(table, 84),
                TheaterRoom::to_string3(table, 90),
                TheaterRoom::to_string3(table, 91),
                TheaterRoom::to_string3(table, 92),
                TheaterRoom::to_string3(table, 93),
                TheaterRoom::to_string3(table, 94),
                TheaterRoom::to_string3(table, 100),
                TheaterRoom::to_string3(table, 101),
                TheaterRoom::to_string3(table, 102),
                TheaterRoom::to_string3(table, 103),
                TheaterRoom::to_string3(table, 104),
                TheaterRoom::to_string3(table, 110),
                TheaterRoom::to_string3(table, 111),
                TheaterRoom::to_string3(table, 112),
                TheaterRoom::to_string3(table, 113),
                TheaterRoom::to_string3(table, 114),
                TheaterRoom::to_string3(table, 120),
                TheaterRoom::to_string3(table, 121),
            ),
            format!(
                "     +    +    +----+----+----+----+----+----+----+----+----+    +
     |{55} {50}|{45}|{40}|{35}|{30}|{25}|{20}|{15}|{10}|{5 }|{0 }| e
     +    +    +----+----+----+----+----+----+----+----+----+    +
     |{56} {51}|{46}|{41}|{36}|{31}|{26}|{21}|{16}|{11}|{6 }|{1 }| f
     +    +    +----+----+----+----+----+----+----+----+----+----+
     |{57} {52}|{47}|{42}|{37}|{32}|{27}|{22}|{17}|{12}|{7 }|{2 }| g
     +    +    +----+----+----+----+----+----+----+----+----+    +
     |{58} {53}|{48}|{43}|{38}|{33}|{28}|{23}|{18}|{13}|{8 }|{3 }| h
     +    +    +----+----+----+----+----+----+----+----+----+    +
     |{59} {54}|{49}|{44}|{39}|{34}|{29}|{24}|{19}|{14}|{9 }|{4 }| i
     +----+----+----+----+----+----+----+----+----+----+----+----+\
",
                TheaterRoom::to_string3(table, 5),
                TheaterRoom::to_string3(table, 6),
                TheaterRoom::to_string3(table, 7),
                TheaterRoom::to_string3(table, 8),
                TheaterRoom::to_string3(table, 9),
                TheaterRoom::to_string3(table, 15),
                TheaterRoom::to_string3(table, 16),
                TheaterRoom::to_string3(table, 17),
                TheaterRoom::to_string3(table, 18),
                TheaterRoom::to_string3(table, 19),
                TheaterRoom::to_string3(table, 25),
                TheaterRoom::to_string3(table, 26),
                TheaterRoom::to_string3(table, 27),
                TheaterRoom::to_string3(table, 28),
                TheaterRoom::to_string3(table, 29),
                TheaterRoom::to_string3(table, 35),
                TheaterRoom::to_string3(table, 36),
                TheaterRoom::to_string3(table, 37),
                TheaterRoom::to_string3(table, 38),
                TheaterRoom::to_string3(table, 39),
                TheaterRoom::to_string3(table, 45),
                TheaterRoom::to_string3(table, 46),
                TheaterRoom::to_string3(table, 47),
                TheaterRoom::to_string3(table, 48),
                TheaterRoom::to_string3(table, 49),
                TheaterRoom::to_string3(table, 55),
                TheaterRoom::to_string3(table, 56),
                TheaterRoom::to_string3(table, 57),
                TheaterRoom::to_string3(table, 58),
                TheaterRoom::to_string3(table, 59),
                TheaterRoom::to_string3(table, 65),
                TheaterRoom::to_string3(table, 66),
                TheaterRoom::to_string3(table, 67),
                TheaterRoom::to_string3(table, 68),
                TheaterRoom::to_string3(table, 69),
                TheaterRoom::to_string3(table, 75),
                TheaterRoom::to_string3(table, 76),
                TheaterRoom::to_string3(table, 77),
                TheaterRoom::to_string3(table, 78),
                TheaterRoom::to_string3(table, 79),
                TheaterRoom::to_string3(table, 85),
                TheaterRoom::to_string3(table, 86),
                TheaterRoom::to_string3(table, 87),
                TheaterRoom::to_string3(table, 88),
                TheaterRoom::to_string3(table, 89),
                TheaterRoom::to_string3(table, 95),
                TheaterRoom::to_string3(table, 96),
                TheaterRoom::to_string3(table, 97),
                TheaterRoom::to_string3(table, 98),
                TheaterRoom::to_string3(table, 99),
                TheaterRoom::to_string3(table, 105),
                TheaterRoom::to_string3(table, 106),
                TheaterRoom::to_string3(table, 107),
                TheaterRoom::to_string3(table, 108),
                TheaterRoom::to_string3(table, 109),
                TheaterRoom::to_string3(table, 115),
                TheaterRoom::to_string3(table, 116),
                TheaterRoom::to_string3(table, 117),
                TheaterRoom::to_string3(table, 118),
                TheaterRoom::to_string3(table, 119),
            )
        )
    }
}

/// 台所はこちらだぜ☆（＾～＾）！指し手の一覧が見れるぜ☆（＾～＾）！
pub struct Kitchen {}
impl Kitchen {
    /// 現在の局面での、指し手の一覧を表示するぜ☆（＾～＾）
    pub fn print_ways(friend: Phase, table: &GameTable, ways: &Vec<Movement>) {
        Beam::shoot(&format!("Moves count={}", ways.len()));
        // 辞書順ソート
        let mut move_names = Vec::new();
        for move_ in ways {
            let ss_str = format!(
                "{}{}",
                format!("{}", move_),
                if let Some(captured_move) = move_.captured {
                    let piece_type =
                        table.get_type(table.piece_num_at(friend, &captured_move.source).unwrap());
                    format!(" ({})", piece_type)
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
