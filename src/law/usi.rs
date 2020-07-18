//!
//! USIプロトコル
//!
use crate::command_line_seek::CommandLineSeek;
use crate::cosmic::recording::{CapturedMove, FireAddress, HandAddress, Movement, Phase};
use crate::cosmic::smart::features::{DoubleFacedPieceType, PieceType};
use crate::cosmic::smart::square::AbsoluteAddress2D;
use crate::cosmic::smart::square::FILE9U8;
use crate::cosmic::smart::square::RANK1U8;
use crate::log::LogExt;
use crate::position::Position;
use atoi::atoi;
use casual_logger::Log;

// 局面の最多合法手５９３手
//pub const MAX_WAYS: usize = 593;

/*
/// USIプロトコル表記: 最多合法手５９３手の局面
/// https://ameblo.jp/professionalhearts/entry-10001031814.html
pub const POS_593: &str = "position sfen R8/2K1S1SSk/4B4/9/9/9/9/9/1L1L1L3 w RBGSNLP3g3n17p 1";
*/

/// USIプロトコル表記: 飛角落ち初期局面
/// http://www.geocities.jp/shogidokoro/usi.html
pub const POS_1: &str = "position startpos";

/*
/// USIプロトコル表記: 飛角落ち初期局面
/// http://www.geocities.jp/shogidokoro/usi.html
pub const POS_2: &str =
  "position sfen lnsgkgsnl/9/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1 moves 5a6b 7g7f 3a3b";
*/

/// USIプロトコル表記: 平手初期局面（の盤上の駒配置部分のみ）
//pub const STARTPOS_LN: usize = 57;
pub const STARTPOS: &str = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL";

/// 指し手読取
/// 例: 7g7f
///
/// 読み取った指し手は、棋譜に入れる。
/// 現在の手目のところに入れ、手目のカウントアップも行う。
pub fn read_sasite(pos: &mut Position, p: &mut CommandLineSeek) -> bool {
    // 4文字か5文字あるはず。
    if (p.len() - p.current()) < 4 {
        // 指し手読取終了時にここを通るぜ☆（＾～＾）
        // 残り４文字もない。
        return false;
    }

    let mut buffer = Movement::default();

    let turn = pos.history.get_turn();

    // 1文字目と2文字目。盤上の移動元か、ドロップする駒種類。
    buffer.source = match &p.line()[p.current()..=p.current()] {
        // 1文字目が駒だったら打。2文字目は必ず「*」なはずなので読み飛ばす。
        "R" => {
            p.go_next_to("R*");
            FireAddress::Hand(HandAddress::new(
                DoubleFacedPieceType::Rook,
                AbsoluteAddress2D::default(),
            ))
        }
        "B" => {
            p.go_next_to("B*");
            FireAddress::Hand(HandAddress::new(
                DoubleFacedPieceType::Bishop,
                AbsoluteAddress2D::default(),
            ))
        }
        "G" => {
            p.go_next_to("G*");
            FireAddress::Hand(HandAddress::new(
                DoubleFacedPieceType::Gold,
                AbsoluteAddress2D::default(),
            ))
        }
        "S" => {
            p.go_next_to("S*");
            FireAddress::Hand(HandAddress::new(
                DoubleFacedPieceType::Silver,
                AbsoluteAddress2D::default(),
            ))
        }
        "N" => {
            p.go_next_to("N*");
            FireAddress::Hand(HandAddress::new(
                DoubleFacedPieceType::Knight,
                AbsoluteAddress2D::default(),
            ))
        }
        "L" => {
            p.go_next_to("L*");
            FireAddress::Hand(HandAddress::new(
                DoubleFacedPieceType::Lance,
                AbsoluteAddress2D::default(),
            ))
        }
        "P" => {
            p.go_next_to("P*");
            FireAddress::Hand(HandAddress::new(
                DoubleFacedPieceType::Pawn,
                AbsoluteAddress2D::default(),
            ))
        }
        _ => {
            // 残りは「筋の数字」、「段のアルファベット」のはず。
            // 数字じゃないものが入ったら強制終了するんじゃないか☆（＾～＾）
            let file = if let Some(num) = atoi::<u8>(p.line()[p.current()..=p.current()].as_bytes())
            {
                num
            } else {
                panic!(Log::print_fatal(&format!(
                    "(Err.72)  '{}' だった。",
                    &p.line()[p.current()..=p.current()]
                )))
            };
            p.go_next_to("1"); // 1桁の数。

            match &p.line()[p.current()..=p.current()] {
                "a" => {
                    p.go_next_to("a");
                    FireAddress::Board(AbsoluteAddress2D::new(file, 1))
                }
                "b" => {
                    p.go_next_to("b");
                    FireAddress::Board(AbsoluteAddress2D::new(file, 2))
                }
                "c" => {
                    p.go_next_to("c");
                    FireAddress::Board(AbsoluteAddress2D::new(file, 3))
                }
                "d" => {
                    p.go_next_to("d");
                    FireAddress::Board(AbsoluteAddress2D::new(file, 4))
                }
                "e" => {
                    p.go_next_to("e");
                    FireAddress::Board(AbsoluteAddress2D::new(file, 5))
                }
                "f" => {
                    p.go_next_to("f");
                    FireAddress::Board(AbsoluteAddress2D::new(file, 6))
                }
                "g" => {
                    p.go_next_to("g");
                    FireAddress::Board(AbsoluteAddress2D::new(file, 7))
                }
                "h" => {
                    p.go_next_to("h");
                    FireAddress::Board(AbsoluteAddress2D::new(file, 8))
                }
                "i" => {
                    p.go_next_to("i");
                    FireAddress::Board(AbsoluteAddress2D::new(file, 9))
                }
                _ => {
                    panic!(Log::print_fatal(&format!(
                        "(Err.90)  '{}' だった。",
                        &p.line()[p.current()..=p.current()]
                    )));
                }
            }
        }
    };

    // 残りは「筋の数字」、「段のアルファベット」のはず。

    // 3文字目
    let file = if let Some(num) = atoi::<u8>(&p.line()[p.current()..=p.current()].as_bytes()) {
        num
    } else {
        panic!(Log::print_fatal(&format!(
            "(Err.118)  '{}' だった。",
            &p.line()[p.current()..=p.current()]
        )));
    };
    p.go_next_to("1"); // 1桁の数。
                       // 4文字目
    let rank = match &p.line()[p.current()..=p.current()] {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        _ => {
            panic!(Log::print_fatal(&format!(
                "(Err.136)  '{}' だった。",
                &p.line()[p.current()..=p.current()]
            )));
        }
    };
    p.go_next_to("a"); // 1文字。

    // 行き先。
    buffer.destination = FireAddress::Board(AbsoluteAddress2D::new(file, rank));

    // 5文字に「+」があれば成り。
    buffer.promote = if 0 < (p.len() - p.current()) && &p.line()[p.current()..=p.current()] == "+" {
        p.go_next_to("+");
        true
    } else {
        false
    };

    // 続きにスペース「 」が１つあれば読み飛ばす
    if 0 < (p.len() - p.current()) && &p.line()[p.current()..=p.current()] == " " {
        p.go_next_to(" ");
    }

    // 取られる駒を事前に調べてセットするぜ☆（＾～＾）！
    let captured_piece_num = pos.table.piece_num_at(turn, &buffer.destination);
    buffer.captured = if let Some(captured_piece_num_val) = captured_piece_num {
        Some(CapturedMove::new(
            buffer.destination,
            FireAddress::Hand(HandAddress::new(
                pos.table
                    .get_double_faced_piece_type(captured_piece_num_val),
                AbsoluteAddress2D::default(),
            )),
        ))
    } else {
        None
    };

    // 確定。
    pos.set_move(&buffer);

    pos.history.ply += 1;
    true
}

/// position コマンド 盤上部分のみ 読取
/// 初期化は既に終わらせてあります。
pub fn read_board(pos: &mut Position, p: &mut CommandLineSeek) {
    // 初期盤面
    let table = pos.mut_starting();
    let mut file = FILE9U8; //９筋から右方向へ読取
    let mut rank = RANK1U8;

    // `/` か、`+`か、1桁の数か、1文字のアルファベットのいずれかだぜ☆（＾～＾）それ以外なら盤パート終了☆（＾～＾）
    enum BoardPart {
        /// 改行のようなものだぜ☆（＾～＾）
        NewLine,
        /// スペース数☆（＾～＾）
        Number(u8),
        /// 駒☆（＾～＾）+で始まるものもこっちだぜ☆（＾～＾）
        Alphabet((Phase, PieceType)),
    }

    'ban: while 0 < (p.len() - p.current()) {
        let board_part = match &p.line()[p.current()..=p.current()] {
            "/" => BoardPart::NewLine,
            "1" => BoardPart::Number(1),
            "2" => BoardPart::Number(2),
            "3" => BoardPart::Number(3),
            "4" => BoardPart::Number(4),
            "5" => BoardPart::Number(5),
            "6" => BoardPart::Number(6),
            "7" => BoardPart::Number(7),
            "8" => BoardPart::Number(8),
            "9" => BoardPart::Number(9),
            "K" => BoardPart::Alphabet((Phase::First, PieceType::King)),
            "R" => BoardPart::Alphabet((Phase::First, PieceType::Rook)),
            "B" => BoardPart::Alphabet((Phase::First, PieceType::Bishop)),
            "G" => BoardPart::Alphabet((Phase::First, PieceType::Gold)),
            "S" => BoardPart::Alphabet((Phase::First, PieceType::Silver)),
            "N" => BoardPart::Alphabet((Phase::First, PieceType::Knight)),
            "L" => BoardPart::Alphabet((Phase::First, PieceType::Lance)),
            "P" => BoardPart::Alphabet((Phase::First, PieceType::Pawn)),
            "k" => BoardPart::Alphabet((Phase::Second, PieceType::King)),
            "r" => BoardPart::Alphabet((Phase::Second, PieceType::Rook)),
            "b" => BoardPart::Alphabet((Phase::Second, PieceType::Bishop)),
            "g" => BoardPart::Alphabet((Phase::Second, PieceType::Gold)),
            "s" => BoardPart::Alphabet((Phase::Second, PieceType::Silver)),
            "n" => BoardPart::Alphabet((Phase::Second, PieceType::Knight)),
            "l" => BoardPart::Alphabet((Phase::Second, PieceType::Lance)),
            "p" => BoardPart::Alphabet((Phase::Second, PieceType::Pawn)),
            "+" => {
                p.go_next_to("+"); // 1文字。
                                   // 次に必ず１文字が来るぜ☆（＾～＾）
                match &p.line()[p.current()..=p.current()] {
                    "R" => BoardPart::Alphabet((Phase::First, PieceType::Dragon)),
                    "B" => BoardPart::Alphabet((Phase::First, PieceType::Horse)),
                    "S" => BoardPart::Alphabet((Phase::First, PieceType::PromotedSilver)),
                    "N" => BoardPart::Alphabet((Phase::First, PieceType::PromotedKnight)),
                    "L" => BoardPart::Alphabet((Phase::First, PieceType::PromotedLance)),
                    "P" => BoardPart::Alphabet((Phase::First, PieceType::PromotedPawn)),
                    "r" => BoardPart::Alphabet((Phase::Second, PieceType::Dragon)),
                    "b" => BoardPart::Alphabet((Phase::Second, PieceType::Horse)),
                    "s" => BoardPart::Alphabet((Phase::Second, PieceType::PromotedSilver)),
                    "n" => BoardPart::Alphabet((Phase::Second, PieceType::PromotedKnight)),
                    "l" => BoardPart::Alphabet((Phase::Second, PieceType::PromotedLance)),
                    "p" => BoardPart::Alphabet((Phase::Second, PieceType::PromotedPawn)),
                    _ => {
                        panic!(Log::print_fatal(&format!(
                            "(Err.235)  盤部(0) '{}' だった。",
                            &p.line()[p.current()..=p.current()]
                        )));
                    }
                }
            }
            _ => {
                break 'ban;
            } // 盤部正常終了
        };

        match board_part {
            BoardPart::Alphabet((turn, piece_type)) => {
                p.go_next_to("K"); // 1文字。
                let fire = FireAddress::Board(AbsoluteAddress2D::new(file, rank));

                // 駒に背番号を付けるぜ☆（＾～＾）
                let piece_num = table.numbering_piece(turn, piece_type);
                // 盤に置くぜ☆（＾～＾）
                table.push_piece(turn, &fire, Some(piece_num));

                file -= 1;
            }
            BoardPart::Number(space_num) => {
                p.go_next_to("1"); // 1文字。
                                   // もともと空升なんで、飛ばそうぜ☆（＾～＾）
                file -= space_num;
            }
            BoardPart::NewLine => {
                p.go_next_to("/");
                file = FILE9U8;
                rank += 1;
            }
        }
    }

    // 初期局面ハッシュを作り直す
    let ky_hash = pos.hash_seed.starting_position(pos);
    pos.history.starting_position_hash = ky_hash;
}

/// position コマンド読取
pub fn set_position(pos: &mut Position, p: &mut CommandLineSeek) {
    // 局面をクリアー。手目も 0 に戻します。
    pos.clear();

    if p.starts_with("position startpos") {
        // 'position startpos' を読み飛ばし
        p.go_next_to("position startpos");

        // 別途用意した平手初期局面文字列を読取
        let mut p2 = CommandLineSeek::new(STARTPOS);
        read_board(pos, &mut p2);

        // 元のパーサーで続行。
        if p.starts_with(" ") {
            // ' ' を読み飛ばした。
            p.go_next_to(" ");
        }
    } else if p.starts_with("position sfen ") {
        // 'position sfen ' を読み飛ばし
        p.go_next_to("position sfen ");
        read_board(pos, p);

        if p.starts_with(" ") {
            p.go_next_to(" ");
        }

        if p.starts_with("w") || p.starts_with("b") {
            p.go_next_to("w"); // 文字の長さは "b" も同じ。
        }

        if p.starts_with(" ") {
            p.go_next_to(" ");
        }

        // 持ち駒の読取
        if p.starts_with("-") {
            p.go_next_to("-");
        } else {
            enum HandCount {
                // 数字なし
                N0Digit,
                // 1桁の数
                N1Digit(isize),
                // 2桁の数
                N2Digit(isize),
            }
            'mg: loop {
                if 0 < (p.len() - p.current()) {
                    // 数字か、数字でないかで大きく分かれるぜ☆（＾～＾）
                    // let mut count = 1;
                    let hand_count = match &p.line()[p.current()..=p.current()] {
                        "1" => {
                            // 1枚のときは数字は付かないので、10～18 と確定☆
                            match &p.line()[p.current()..=p.current()] {
                                "0" => HandCount::N2Digit(10),
                                "1" => HandCount::N2Digit(11),
                                "2" => HandCount::N2Digit(12),
                                "3" => HandCount::N2Digit(13),
                                "4" => HandCount::N2Digit(14),
                                "5" => HandCount::N2Digit(15),
                                "6" => HandCount::N2Digit(16),
                                "7" => HandCount::N2Digit(17),
                                "8" => HandCount::N2Digit(18),
                                _ => {
                                    panic!(Log::print_fatal(&format!(
                                        "(Err.346)  持駒部(0) '{}' だった。",
                                        &p.line()[p.current()..(p.current() + 2)]
                                    )));
                                }
                            }
                        }
                        "2" => HandCount::N1Digit(2),
                        "3" => HandCount::N1Digit(3),
                        "4" => HandCount::N1Digit(4),
                        "5" => HandCount::N1Digit(5),
                        "6" => HandCount::N1Digit(6),
                        "7" => HandCount::N1Digit(7),
                        "8" => HandCount::N1Digit(8),
                        "9" => HandCount::N1Digit(9),
                        _ => HandCount::N0Digit, // 駒の名前か、エラーなら次へ
                    };

                    let hand_num = match hand_count {
                        HandCount::N0Digit => {
                            // 持ち駒が１枚のときは数は付かないぜ☆（＾～＾）
                            1
                        }
                        HandCount::N1Digit(hand_num) => {
                            p.go_next_to("1"); // 1桁の数。
                            hand_num
                        }
                        HandCount::N2Digit(hand_num) => {
                            p.go_next_to("10"); // 2桁の数。
                            hand_num
                        }
                    };

                    let (turn, piece_type) = match &p.line()[p.current()..=p.current()] {
                        "R" => (Phase::First, PieceType::Rook),
                        "B" => (Phase::First, PieceType::Bishop),
                        "G" => (Phase::First, PieceType::Gold),
                        "S" => (Phase::First, PieceType::Silver),
                        "N" => (Phase::First, PieceType::Knight),
                        "L" => (Phase::First, PieceType::Lance),
                        "P" => (Phase::First, PieceType::Pawn),
                        "r" => (Phase::Second, PieceType::Rook),
                        "b" => (Phase::Second, PieceType::Bishop),
                        "g" => (Phase::Second, PieceType::Gold),
                        "s" => (Phase::Second, PieceType::Silver),
                        "n" => (Phase::Second, PieceType::Knight),
                        "l" => (Phase::Second, PieceType::Lance),
                        "p" => (Phase::Second, PieceType::Pawn),
                        _ => {
                            break 'mg;
                        } // 持駒部 正常終了
                    };
                    p.go_next_to("R"); // 1文字。

                    for _i in 0..hand_num {
                        // 散らばっている駒に、背番号を付けて、駒台に置くぜ☆（＾～＾）
                        pos.mut_starting().init_hand(turn, piece_type);
                    }
                } //if
            } //loop
        } //else

        if p.starts_with(" 1 ") {
            p.go_next_to(" 1 ");
        }
    } else {
        Log::print_info("'position startpos' でも、'position sfen ' でも始まらなかった。");
        return;
    }

    if p.starts_with("moves") {
        p.go_next_to("moves");
    }

    if p.starts_with(" ") {
        p.go_next_to(" ");
    }

    // 初期局面を、現局面にコピーします
    pos.table.copy_from(&pos.starting_table);

    // 指し手を全部読んでいくぜ☆（＾～＾）手目のカウントも増えていくぜ☆（＾～＾）
    while read_sasite(pos, p) {
        // 手目を戻す
        pos.history.ply -= 1;
        // 入っている指し手の通り指すぜ☆（＾～＾）
        let ply = pos.history.ply;

        let move_ = pos.history.movements[ply as usize];
        pos.do_move(pos.history.get_turn(), &move_);
    }
}
