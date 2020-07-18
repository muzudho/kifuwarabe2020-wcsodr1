use crate::cosmic::playing::PosNums;
use crate::cosmic::recording::{FireAddress, HandAddress, Movement, Phase};
use crate::cosmic::smart::features::DoubleFacedPieceType;
use crate::cosmic::smart::square::AbsoluteAddress2D;
use crate::cosmic::toy_box::PieceNum;
use crate::look_and_model::piece::PIECE_WHITE_SPACE;
use crate::position::GameTable;
use crate::position::Position;
use crate::LogExt;
use casual_logger::Log;

/// 指令室はこちらだぜ☆（＾～＾）！
pub struct CommandRoom {}
impl CommandRoom {
    // 対話モードのタイトル画面
    pub fn print_title() {
        // 横幅は 半角79文字使えるぜ☆（＾～＾）
        // 80文字目を使うと、次の行が改行で空行になってしまう☆（＾～＾）
        Log::println(
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
    /// 表示
    pub fn to_string(pos: &Position, pos_nums: PosNums) -> String {
        let table = pos.get_table(pos_nums);
        let ply = pos.history.ply;
        let phase = pos.history.get_turn();
        let same_pos_count = pos.count_same_position();

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
            Self::to_string3(table, 9, 1),
            Self::to_string3(table, 8, 1),
            Self::to_string3(table, 7, 1),
            Self::to_string3(table, 6, 1),
            Self::to_string3(table, 5, 1),
            Self::to_string3(table, 4, 1),
            Self::to_string3(table, 3, 1),
            Self::to_string3(table, 2, 1),
            Self::to_string3(table, 1, 1),
            Self::to_string3(table, 9, 2),
            Self::to_string3(table, 8, 2),
            Self::to_string3(table, 7, 2),
            Self::to_string3(table, 6, 2),
            Self::to_string3(table, 5, 2),
            Self::to_string3(table, 4, 2),
            Self::to_string3(table, 3, 2),
            Self::to_string3(table, 2, 2),
            Self::to_string3(table, 1, 2),
            Self::to_string3(table, 9, 3),
            Self::to_string3(table, 8, 3),
            Self::to_string3(table, 7, 3),
            Self::to_string3(table, 6, 3),
            Self::to_string3(table, 5, 3),
            Self::to_string3(table, 4, 3),
            Self::to_string3(table, 3, 3),
            Self::to_string3(table, 2, 3),
            Self::to_string3(table, 1, 3),
            Self::to_string3(table, 9, 4),
            Self::to_string3(table, 8, 4),
            Self::to_string3(table, 7, 4),
            Self::to_string3(table, 6, 4),
            Self::to_string3(table, 5, 4),
            Self::to_string3(table, 4, 4),
            Self::to_string3(table, 3, 4),
            Self::to_string3(table, 2, 4),
            Self::to_string3(table, 1, 4),
            Self::to_string3(table, 9, 5),
            Self::to_string3(table, 8, 5),
            Self::to_string3(table, 7, 5),
            Self::to_string3(table, 6, 5),
            Self::to_string3(table, 5, 5),
            Self::to_string3(table, 4, 5),
            Self::to_string3(table, 3, 5),
            Self::to_string3(table, 2, 5),
            Self::to_string3(table, 1, 5),
            Self::to_string3(table, 9, 6),
            Self::to_string3(table, 8, 6),
            Self::to_string3(table, 7, 6),
            Self::to_string3(table, 6, 6),
            Self::to_string3(table, 5, 6),
            Self::to_string3(table, 4, 6),
            Self::to_string3(table, 3, 6),
            Self::to_string3(table, 2, 6),
            Self::to_string3(table, 1, 6),
            Self::to_string3(table, 9, 7),
            Self::to_string3(table, 8, 7),
            Self::to_string3(table, 7, 7),
            Self::to_string3(table, 6, 7),
            Self::to_string3(table, 5, 7),
            Self::to_string3(table, 4, 7),
            Self::to_string3(table, 3, 7),
            Self::to_string3(table, 2, 7),
            Self::to_string3(table, 1, 7),
            Self::to_string3(table, 9, 8),
            Self::to_string3(table, 8, 8),
            Self::to_string3(table, 7, 8),
            Self::to_string3(table, 6, 8),
            Self::to_string3(table, 5, 8),
            Self::to_string3(table, 4, 8),
            Self::to_string3(table, 3, 8),
            Self::to_string3(table, 2, 8),
            Self::to_string3(table, 1, 8),
            Self::to_string3(table, 9, 9),
            Self::to_string3(table, 8, 9),
            Self::to_string3(table, 7, 9),
            Self::to_string3(table, 6, 9),
            Self::to_string3(table, 5, 9),
            Self::to_string3(table, 4, 9),
            Self::to_string3(table, 3, 9),
            Self::to_string3(table, 2, 9),
            Self::to_string3(table, 1, 9),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Rook,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Bishop,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Gold,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Silver,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Knight,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Lance,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::First,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Pawn,
                    AbsoluteAddress2D::default()
                ))
            ),
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Rook,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Bishop,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Gold,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Silver,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Knight,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Lance,
                    AbsoluteAddress2D::default()
                ))
            ),
            table.count_hand(
                Phase::Second,
                &FireAddress::Hand(HandAddress::new(
                    DoubleFacedPieceType::Pawn,
                    AbsoluteAddress2D::default()
                ))
            ),
            ply,
            phase,
            same_pos_count
        )
    }
    fn to_string3(table: &GameTable, file: u8, rank: u8) -> String {
        if let Some(piece_info_val) =
            table.piece_info_at1(&FireAddress::Board(AbsoluteAddress2D::new(file, rank)))
        {
            format!("{}", piece_info_val.text1)
        } else {
            PIECE_WHITE_SPACE.to_string()
        }
    }
}

/// シアター・ルーム１はこちらだぜ☆（＾～＾）！
pub struct TheaterRoom1 {}
impl TheaterRoom1 {
    /// 表示
    pub fn to_string(pos: &Position, pos_nums: PosNums) -> String {
        let table = pos.get_table(pos_nums);
        let ply = pos.history.ply;
        let phase = pos.history.get_turn();
        let same_pos_count = pos.count_same_position();

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
                Self::to_string3(table, 0),
                Self::to_string3(table, 1),
                Self::to_string3(table, 2),
                Self::to_string3(table, 3),
                Self::to_string3(table, 4),
                Self::to_string3(table, 10),
                Self::to_string3(table, 11),
                Self::to_string3(table, 12),
                Self::to_string3(table, 13),
                Self::to_string3(table, 14),
                Self::to_string3(table, 20),
                Self::to_string3(table, 21),
                Self::to_string3(table, 22),
                Self::to_string3(table, 23),
                Self::to_string3(table, 24),
                Self::to_string3(table, 30),
                Self::to_string3(table, 31),
                Self::to_string3(table, 32),
                Self::to_string3(table, 33),
                Self::to_string3(table, 34),
                Self::to_string3(table, 40),
                Self::to_string3(table, 41),
                Self::to_string3(table, 42),
                Self::to_string3(table, 43),
                Self::to_string3(table, 44),
                Self::to_string3(table, 50),
                Self::to_string3(table, 51),
                Self::to_string3(table, 52),
                Self::to_string3(table, 53),
                Self::to_string3(table, 54),
                Self::to_string3(table, 60),
                Self::to_string3(table, 61),
                Self::to_string3(table, 62),
                Self::to_string3(table, 63),
                Self::to_string3(table, 64),
                Self::to_string3(table, 70),
                Self::to_string3(table, 71),
                Self::to_string3(table, 72),
                Self::to_string3(table, 73),
                Self::to_string3(table, 74),
                Self::to_string3(table, 80),
                Self::to_string3(table, 81),
                Self::to_string3(table, 82),
                Self::to_string3(table, 83),
                Self::to_string3(table, 84),
                Self::to_string3(table, 90),
                Self::to_string3(table, 91),
                Self::to_string3(table, 92),
                Self::to_string3(table, 93),
                Self::to_string3(table, 94),
                Self::to_string3(table, 100),
                Self::to_string3(table, 101),
                Self::to_string3(table, 102),
                Self::to_string3(table, 103),
                Self::to_string3(table, 104),
                Self::to_string3(table, 110),
                Self::to_string3(table, 111),
                Self::to_string3(table, 112),
                Self::to_string3(table, 113),
                Self::to_string3(table, 114),
                Self::to_string3(table, 120),
                Self::to_string3(table, 121),
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
     +----+----+----+----+----+----+----+----+----+----+----+----+
",
                Self::to_string3(table, 5),
                Self::to_string3(table, 6),
                Self::to_string3(table, 7),
                Self::to_string3(table, 8),
                Self::to_string3(table, 9),
                Self::to_string3(table, 15),
                Self::to_string3(table, 16),
                Self::to_string3(table, 17),
                Self::to_string3(table, 18),
                Self::to_string3(table, 19),
                Self::to_string3(table, 25),
                Self::to_string3(table, 26),
                Self::to_string3(table, 27),
                Self::to_string3(table, 28),
                Self::to_string3(table, 29),
                Self::to_string3(table, 35),
                Self::to_string3(table, 36),
                Self::to_string3(table, 37),
                Self::to_string3(table, 38),
                Self::to_string3(table, 39),
                Self::to_string3(table, 45),
                Self::to_string3(table, 46),
                Self::to_string3(table, 47),
                Self::to_string3(table, 48),
                Self::to_string3(table, 49),
                Self::to_string3(table, 55),
                Self::to_string3(table, 56),
                Self::to_string3(table, 57),
                Self::to_string3(table, 58),
                Self::to_string3(table, 59),
                Self::to_string3(table, 65),
                Self::to_string3(table, 66),
                Self::to_string3(table, 67),
                Self::to_string3(table, 68),
                Self::to_string3(table, 69),
                Self::to_string3(table, 75),
                Self::to_string3(table, 76),
                Self::to_string3(table, 77),
                Self::to_string3(table, 78),
                Self::to_string3(table, 79),
                Self::to_string3(table, 85),
                Self::to_string3(table, 86),
                Self::to_string3(table, 87),
                Self::to_string3(table, 88),
                Self::to_string3(table, 89),
                Self::to_string3(table, 95),
                Self::to_string3(table, 96),
                Self::to_string3(table, 97),
                Self::to_string3(table, 98),
                Self::to_string3(table, 99),
                Self::to_string3(table, 105),
                Self::to_string3(table, 106),
                Self::to_string3(table, 107),
                Self::to_string3(table, 108),
                Self::to_string3(table, 109),
                Self::to_string3(table, 115),
                Self::to_string3(table, 116),
                Self::to_string3(table, 117),
                Self::to_string3(table, 118),
                Self::to_string3(table, 119),
            )
        )
    }
    fn to_string3(table: &GameTable, serial: u8) -> String {
        if let Some(sq) = AbsoluteAddress2D::from_absolute_address(serial) {
            if let Some(piece_info_val) = table.piece_info_num_at(&FireAddress::Board(sq)) {
                format!("{}", piece_info_val.text1).to_string()
            } else {
                "    ".to_string()
            }
        } else {
            // 0 は None.
            "    ".to_string()
        }
    }
}
/// シアター・ルーム２はこちらだぜ☆（＾～＾）！
pub struct TheaterRoom2 {}
impl TheaterRoom2 {
    /// 表示
    pub fn to_string(pos: &Position, pos_nums: PosNums) -> String {
        let table = pos.get_table(pos_nums);

        // 局面表示
        // フォーマットの引数は 98個まで。
        format!(
            "{}{}{}{}",
            format!(
                " 01K  02K  03G  04G  05G  06G  07S  08S  09S  10S  11N  12N  13N  14N  15L  16L  17L  18L  19B  20B
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
",
                Self::to_string3(table, PieceNum::King1),
                Self::to_string3(table, PieceNum::King2),
                Self::to_string3(table, PieceNum::Gold3),
                Self::to_string3(table, PieceNum::Gold4),
                Self::to_string3(table, PieceNum::Gold5),
                Self::to_string3(table, PieceNum::Gold6),
                Self::to_string3(table, PieceNum::Silver7),
                Self::to_string3(table, PieceNum::Silver8),
                Self::to_string3(table, PieceNum::Silver9),
                Self::to_string3(table, PieceNum::Silver10),
                Self::to_string3(table, PieceNum::Knight11),
                Self::to_string3(table, PieceNum::Knight12),
                Self::to_string3(table, PieceNum::Knight13),
                Self::to_string3(table, PieceNum::Knight14),
                Self::to_string3(table, PieceNum::Lance15),
                Self::to_string3(table, PieceNum::Lance16),
                Self::to_string3(table, PieceNum::Lance17),
                Self::to_string3(table, PieceNum::Lance18),
                Self::to_string3(table, PieceNum::Bishop19),
                Self::to_string3(table, PieceNum::Bishop20),
            ),
            format!(
                "\
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
",
                Self::to_string4(table, PieceNum::King1),
                Self::to_string4(table, PieceNum::King2),
                Self::to_string4(table, PieceNum::Gold3),
                Self::to_string4(table, PieceNum::Gold4),
                Self::to_string4(table, PieceNum::Gold5),
                Self::to_string4(table, PieceNum::Gold6),
                Self::to_string4(table, PieceNum::Silver7),
                Self::to_string4(table, PieceNum::Silver8),
                Self::to_string4(table, PieceNum::Silver9),
                Self::to_string4(table, PieceNum::Silver10),
                Self::to_string4(table, PieceNum::Knight11),
                Self::to_string4(table, PieceNum::Knight12),
                Self::to_string4(table, PieceNum::Knight13),
                Self::to_string4(table, PieceNum::Knight14),
                Self::to_string4(table, PieceNum::Lance15),
                Self::to_string4(table, PieceNum::Lance16),
                Self::to_string4(table, PieceNum::Lance17),
                Self::to_string4(table, PieceNum::Lance18),
                Self::to_string4(table, PieceNum::Bishop19),
                Self::to_string4(table, PieceNum::Bishop20),
            ),
            format!(
                " 21R  22R  23P  24P  25P  26P  27P  28P  29P  30P  31P  32P  33P  34P  35P  36P  37P  38P  39P  40P
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
",
                Self::to_string3(table, PieceNum::Rook21),
                Self::to_string3(table, PieceNum::Rook22),
                Self::to_string3(table, PieceNum::Pawn23),
                Self::to_string3(table, PieceNum::Pawn24),
                Self::to_string3(table, PieceNum::Pawn25),
                Self::to_string3(table, PieceNum::Pawn26),
                Self::to_string3(table, PieceNum::Pawn27),
                Self::to_string3(table, PieceNum::Pawn28),
                Self::to_string3(table, PieceNum::Pawn29),
                Self::to_string3(table, PieceNum::Pawn30),
                Self::to_string3(table, PieceNum::Pawn31),
                Self::to_string3(table, PieceNum::Pawn32),
                Self::to_string3(table, PieceNum::Pawn33),
                Self::to_string3(table, PieceNum::Pawn34),
                Self::to_string3(table, PieceNum::Pawn35),
                Self::to_string3(table, PieceNum::Pawn36),
                Self::to_string3(table, PieceNum::Pawn37),
                Self::to_string3(table, PieceNum::Pawn38),
                Self::to_string3(table, PieceNum::Pawn39),
                Self::to_string3(table, PieceNum::Pawn40),
            ),
            format!(
                "\
|{0 }|{1 }|{2 }|{3 }|{4 }|{5 }|{6 }|{7 }|{8 }|{9 }|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|{18}|{19}|
+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+
",
                Self::to_string4(table, PieceNum::Rook21),
                Self::to_string4(table, PieceNum::Rook22),
                Self::to_string4(table, PieceNum::Pawn23),
                Self::to_string4(table, PieceNum::Pawn24),
                Self::to_string4(table, PieceNum::Pawn25),
                Self::to_string4(table, PieceNum::Pawn26),
                Self::to_string4(table, PieceNum::Pawn27),
                Self::to_string4(table, PieceNum::Pawn28),
                Self::to_string4(table, PieceNum::Pawn29),
                Self::to_string4(table, PieceNum::Pawn30),
                Self::to_string4(table, PieceNum::Pawn31),
                Self::to_string4(table, PieceNum::Pawn32),
                Self::to_string4(table, PieceNum::Pawn33),
                Self::to_string4(table, PieceNum::Pawn34),
                Self::to_string4(table, PieceNum::Pawn35),
                Self::to_string4(table, PieceNum::Pawn36),
                Self::to_string4(table, PieceNum::Pawn37),
                Self::to_string4(table, PieceNum::Pawn38),
                Self::to_string4(table, PieceNum::Pawn39),
                Self::to_string4(table, PieceNum::Pawn40),
            ),
        )
    }
    fn to_string3(table: &GameTable, piece_num: PieceNum) -> String {
        if let Some(piece_info_val) = table.piece_info_address_at(piece_num) {
            format!("{: >4}", piece_info_val.text1).to_string()
        } else {
            "    ".to_string()
        }
    }
    fn to_string4(table: &GameTable, piece_num: PieceNum) -> String {
        if let Some(piece_info_val) = table.piece_info_piece_at(piece_num) {
            format!("{: >4}", piece_info_val.text1).to_string()
        } else {
            "    ".to_string()
        }
    }
}

/// 台所はこちらだぜ☆（＾～＾）！指し手の一覧が見れるぜ☆（＾～＾）！
pub struct Kitchen {}
impl Kitchen {
    /// 現在の局面での、指し手の一覧を表示するぜ☆（＾～＾）
    pub fn print_ways(turn: Phase, table: &GameTable, ways: &Vec<Movement>) {
        Log::println(&format!("Moves count={}", ways.len()));
        // 辞書順ソート
        let mut move_names = Vec::new();
        for move_ in ways {
            let ss_str = format!(
                "{}{}",
                format!("{}", move_),
                if let Some(captured_move) = move_.captured {
                    let piece_type =
                        table.get_type(table.piece_num_at(turn, &captured_move.source).unwrap());
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
            Log::println(&format!("[{}] {}", i, move_name));
        }
    }
}
