use crate::command_line_seek::CommandLineSeek;
use crate::config::*;
use crate::cosmic::playing::PosNums;
use crate::cosmic::recording::Movement;
use crate::cosmic::recording::Phase;
use crate::cosmic::smart::square::AbsoluteAddress2D;
use crate::cosmic::smart::square::FILE1U8;
use crate::cosmic::universe::Universe;
use crate::law::cryptographic::*;
use crate::law::generate_move::MoveGen;
use crate::law::usi::*;
use crate::log::LogExt;
use crate::look_and_model::facility::{CommandRoom, GameRoom, Kitchen, TheaterRoom1, TheaterRoom2};
use crate::look_and_model::search::Search;
use crate::spaceship::engine;
use crate::spaceship::equipment::{PvString, Telescope};
use casual_logger::{Log, Table};
use rand::Rng;

/// 船長：きふわらべ
///
/// 対局で許されている命令だけをするぜ☆（＾～＾）
pub struct Kifuwarabe {}
impl Kifuwarabe {
    /// bestmoveコマンドを送るぜ☆（＾～＾） 思考するのもこの中だぜ☆（＾～＾）
    pub fn go(universe: &mut Universe, p: &mut CommandLineSeek) {
        // go btime 40000 wtime 50000 binc 10000 winc 10000
        let go1 = engine::Go::parse(p);
        Log::debug(&format!("Debug   | go=|{}|", go1));
        let mut tree = Search::new(
            universe.option_many_ways_weight,
            universe.option_komawari_weight,
            universe.option_promotion_weight,
            universe.option_depth_not_to_give_up,
        );

        // 残り時間と、追加時間☆（＾～＾）
        fn margined_msec(msec: u64) -> u64 {
            if 2000 < msec {
                msec - 2000
            } else {
                0
            }
        }
        let (msec, _minc) = match universe.game.history.get_turn() {
            // 2秒余裕を見ておけば、探索を中断できるだろ……☆（＾～＾）負の数になったらエラーな☆（＾～＾）
            Phase::First => (margined_msec(go1.btime), go1.binc),
            Phase::Second => (margined_msec(go1.wtime), go1.winc),
        };
        tree.think_msec = if universe.option_max_think_msec < msec {
            // 残り時間が、最大思考時間より長ければ充分だぜ☆（＾～＾）
            rand::thread_rng().gen_range(
                universe.option_min_think_msec,
                universe.option_max_think_msec,
            ) as u128
        } else if universe.option_min_think_msec < msec {
            // 残り時間が、最小思考時間より長いが、最長思考時間まで考えてられないな☆（＾～＾）
            rand::thread_rng().gen_range(universe.option_min_think_msec, msec) as u128
        } else if 3000 < msec {
            // 持ち時間が、最小思考時間未満、3秒より多いになったら☆（＾～＾）
            // 第一引数が負の数にならないように注意☆（＾～＾）
            rand::thread_rng().gen_range(0, msec - 2000) as u128
        } else {
            // ヤケクソの 1msec 指しだぜ☆（＾～＾）
            1
        };

        let ts = tree.iteration_deeping(universe);
        // その手を選んだ理由☆（＾～＾）
        Log::print_info(&Search::info_str(
            None,
            Some((tree.nodes, tree.nps())),
            Some(ts.bestmove.value),
            ts.bestmove.movement,
            &Some(PvString::String(ts.bestmove.reason.to_string())),
        ));
        // 例: bestmove 7g7f
        // 例: bestmove resign
        Log::print_notice(&format!(
            "bestmove {}",
            if let Some(bestmove) = ts.bestmove.movement {
                format!("{}", bestmove)
            } else {
                "resign".to_string()
            }
        ));
    }
    pub fn isready() {
        Log::print_notice("readyok");
    }
    pub fn position(universe: &mut Universe, line: &str) {
        // positionコマンドの読取を丸投げ
        set_position(&mut universe.game, &mut CommandLineSeek::new(line));
    }
    pub fn setoption_name(universe: &mut Universe, p: &mut CommandLineSeek) {
        // Example: setoption name USI_Ponder value true
        p.go_next_to("setoption name ");
        if let Some(name_len) = p.line()[p.current()..].find(' ') {
            let name = p.line()[p.current()..(p.current() + name_len)].to_string();
            p.go_next_to(&name);
            p.go_next_to(" value ");
            let value = &p.line()[(p.current())..];
            Log::debug_t(
                "SetOption",
                Table::default().str("Name", &name).str("Value", value),
            );
            match name.as_str() {
                "MaxPly" => {
                    universe.option_max_ply = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => std::panic::panic_any(Log::print_fatal(&format!(
                            "Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "ManyWaysPer1000" => {
                    universe.option_many_ways_weight = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => std::panic::panic_any(Log::print_fatal(&format!(
                            "Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "DepthNotToGiveUp" => {
                    universe.option_depth_not_to_give_up = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => std::panic::panic_any(Log::print_fatal(&format!(
                            "Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "KomawariWeightPer1000" => {
                    universe.option_komawari_weight = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => std::panic::panic_any(Log::print_fatal(&format!(
                            "Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "PromotionWeightPer1000" => {
                    universe.option_promotion_weight = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => std::panic::panic_any(Log::print_fatal(&format!(
                            "Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "MaxDepth" => {
                    universe.option_max_depth = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => std::panic::panic_any(Log::print_fatal(&format!(
                            "Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "MinThinkMsec" => {
                    universe.option_min_think_msec = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => std::panic::panic_any(Log::print_fatal(&format!(
                            "Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                "MaxThinkMsec" => {
                    universe.option_max_think_msec = match value.parse() {
                        Result::Ok(val) => val,
                        Result::Err(e) => std::panic::panic_any(Log::print_fatal(&format!(
                            "Invalid value=|{}|",
                            e
                        ))),
                    };
                }
                _ => {}
            }
        };
    }
    pub fn usi() {
        Log::print_notice(&format!("id name {}", ENGINE_NAME));
        Log::print_notice(&format!("id author {}", ENGINE_AUTHOR));
        /*
        IO::writeln("option name BookFile type string default public.bin");
        IO::writeln("option name UseBook type check default true");
        IO::writeln("option name Selectivity type spin default 2 min 0 max 4");
        IO::writeln(
            "option name Style type combo default Normal var Solid var Normal var Risky",
        );
        IO::writeln("option name ResetLearning type button");
        IO::writeln("option name LearningFile type filename default <empty>");
        */
        // アルファベット順ではなく、将棋所のダイアログボックスが見やすくなるように並べろだぜ☆（＾～＾）
        // 大会ルール関連☆（＾～＾）
        Log::print_notice("option name MaxPly type spin default 320 min 1 max 10000");
        // 読みの深さ関連☆（＾～＾）
        Log::print_notice("option name DepthNotToGiveUp type spin default 4 min 1 max 8");
        Log::print_notice("option name MaxDepth type spin default 7 min 1 max 15");
        // 思考時間関連☆（＾～＾）
        Log::print_notice("option name MinThinkMsec type spin default 5000 min 0 max 599000");
        Log::print_notice("option name MaxThinkMsec type spin default 17000 min 1000 max 600000");
        // 評価値関連☆（＾～＾）
        Log::print_notice(
            "option name KomawariWeightPer1000 type spin default 1000 min -100000 max 100000",
        );
        Log::print_notice(
            "option name ManyWaysPer1000 type spin default 1000 min -100000 max 100000",
        );
        Log::print_notice(
            "option name PromotionWeightPer1000 type spin default 1000 min -100000 max 100000",
        );
        Log::print_notice("usiok");
    }
    pub fn usinewgame(universe: &mut Universe) {
        universe.game.clear();
    }
}

/// 副船長：ちゆり
///
/// 対局でやっちゃいかん命令なら任せろだぜ☆（＾～＾）
pub struct Chiyuri {}
impl Chiyuri {
    pub fn do_(universe: &mut Universe, p: &mut CommandLineSeek) {
        // コマンド読取。棋譜に追加され、手目も増える
        if read_sasite(&mut universe.game, p) {
            // 手目を戻す
            universe.game.history.ply -= 1;
            // 入っている指し手の通り指すぜ☆（＾～＾）
            let ply = universe.game.history.ply;
            let move_ = universe.game.history.movements[ply as usize];
            universe
                .game
                .redo_move(universe.game.history.get_turn(), &move_);
        }
    }
    pub fn genmove(universe: &Universe) {
        // Generation move.
        // FIXME 合法手とは限らない
        let mut ways = Vec::<Movement>::new();
        MoveGen::make_move(
            &universe.game,
            match universe.game.history.get_turn() {
                Phase::First => &universe.game.movegen_phase.first_movegen,
                Phase::Second => &universe.game.movegen_phase.second_movegen,
            },
            &mut |way| {
                ways.push(way);
            },
        );
        Log::print_notice("----指し手生成(合法手とは限らない) ここから----");
        Kitchen::print_ways(
            universe.game.history.get_turn(),
            &universe.game.table,
            &ways,
        );
        Log::print_notice("----指し手生成(合法手とは限らない) ここまで----");
    }
    pub fn hash(universe: &Universe) {
        Log::print_notice("局面ハッシュ表示");
        let s = universe.game.get_positions_hash_text();
        Log::print_notice(&s);
    }
    pub fn how_much(line: &str) {
        // Example: how-much 7g7f
        let bestmove = &line[9..];
        Log::print_notice(&format!("Debug   | bestmove=|{}|", bestmove));
    }
    pub fn kifu(universe: &Universe) {
        Log::print_notice("棋譜表示");
        let s = universe.game.get_moves_history_text();
        Log::print_notice(&s);
    }
    /// 表示するだけ☆（＾～＾）
    pub fn list40(universe: &Universe) {
        Log::print_notice("----駒リスト40表示 ここから----");
        universe
            .game
            .table
            .for_all_pieces_on_table(&mut |i, adr, piece| {
                Log::print_notice(&format!(
                    "[{}]{}{}",
                    i,
                    if let Some(adr_val) = adr {
                        format!(" {:?}", adr_val)
                    } else {
                        " --".to_string()
                    },
                    if let Some(piece_val) = piece {
                        format!(" {} {:?}", piece_val.text1, piece_val.num)
                    } else {
                        " --".to_string()
                    }
                ));
            });
        Log::print_notice("----駒リスト40表示 ここまで----");
    }
    pub fn len0(universe: &mut Universe) {
        Log::debug("info string EnterEmpty.");
        if !&universe.dialogue_mode {
            // 空打ち１回目なら、対話モードへ☆（＾～＾）
            universe.dialogue_mode = true;
            // タイトル表示
            // １画面は２５行だが、最後の２行は開けておかないと、
            // カーソルが２行分場所を取るんだぜ☆（＾～＾）
            CommandRoom::print_title();
        } else {
            // 局面表示
            let s = GameRoom::to_string(&universe.game, PosNums::Current);
            Log::print_notice(&s);
        }
    }
    pub fn pos(universe: &Universe) {
        // 現局面表示
        let s = GameRoom::to_string(&universe.game, PosNums::Current);
        Log::print_notice(&s);
    }
    pub fn pos2(universe: &Universe) {
        // 現局面表示
        let s = format!(
            "{}{}",
            TheaterRoom1::to_string(&universe.game, PosNums::Current),
            TheaterRoom2::to_string(&universe.game, PosNums::Current)
        );
        Log::print_notice(&s);
    }
    pub fn pos0(universe: &Universe) {
        // 初期局面表示
        let s = GameRoom::to_string(&universe.game, PosNums::Start);
        Log::print_notice(&s);
    }
    pub fn rand() {
        // 乱数の試し
        let secret_number = rand::thread_rng().gen_range(1, 101); //1~100
        Log::print_notice(&format!("乱数={}", secret_number));
    }
    pub fn same(universe: &Universe) {
        let count = universe.game.count_same_position();
        Log::print_notice(&format!("同一局面調べ count={}", count));
    }
    pub fn startpos(universe: &mut Universe) {
        // 平手初期局面
        set_position(
            &mut universe.game,
            &mut CommandLineSeek::new(&POS_1.to_string()),
        );
    }
    pub fn teigi_conv() {
        Log::print_notice("teigi::convのテスト");

        for ms in 1..9 {
            for hash in 0..10 {
                let sq = AbsoluteAddress2D::new(FILE1U8, ms);
                let next = push_sq_to_hash(hash, Some(&sq));
                let (hash_orig, square_orig) = pop_sq_from_hash(next);
                Log::print_notice( &format!("push_ms_to_hash(0b{:4b},0b{:5b})=0b{:11b} pop_sq_from_hash(...)=(0b{:4b},0b{:5b})"
                    ,hash
                    ,ms
                    ,next
                    ,hash_orig
                    ,if let Some(square_orig_val) = square_orig{ square_orig_val.serial_number()}else{0}
                ));
            }
        }
    }
    pub fn undo(universe: &mut Universe) {
        if !universe.game.undo_move() {
            Log::print_notice(&format!(
                "info string ply={} を、これより戻せません",
                universe.game.history.ply
            ));
        }
    }
}

/// 乗組員：夢美
pub struct Yumemi {}
impl Yumemi {
    /// 望遠鏡を覗き込みましょう。
    pub fn look_into_the_telescope() {
        Telescope::look();
    }
}
