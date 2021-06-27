//!
//! きふわらべＷＣＳＣ３０
//!
//! これは、最初に実行されるファイルだぜ☆（＾～＾）
//!

// extern crate は、 main.rs か lib.rs の冒頭にまとめろだぜ☆（＾～＾）
extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate atoi;
extern crate num_derive;
extern crate num_traits;
extern crate regex;

// Rust言語の mod や ソース置き場の説明
//     「Rust のモジュールシステム」
//      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
//
// 使いたい ディレクトリー名を pub mod しろだぜ☆（＾～＾）
// 別のアプリにも見えるようにしたけりゃ pub mod にしろだぜ☆（＾～＾）
mod command_line_seek;
mod computer_player;
mod config;
mod cosmic;
mod law;
mod log;
mod look_and_model;
mod performance_measurement;
mod position;
mod spaceship;

use crate::command_line_seek::CommandLineSeek;
use crate::config::LOG_FILE;
use crate::cosmic::universe::Universe;
use crate::log::LogExt;
use crate::spaceship::crew::{Chiyuri, Kifuwarabe, Yumemi};
use casual_logger::{Log, Table};

fn main() {
    Log::set_file_name(LOG_FILE);
    // Log::set_level(Level::Notice);
    Log::remove_old_logs();
    // 宇宙☆（＾～＾）変化するぜ☆（＾～＾）
    let mut universe: Universe = Universe::default();

    // ビッグバン
    universe.big_bang();

    // 「何が見えんの？」
    Yumemi::look_into_the_telescope();

    main_loop(&mut universe);
    // [Ctrl]+[C] で強制終了
}

fn main_loop(universe: &mut Universe) {
    // End the loop with 'quit'. Forced termination with [Ctrl]+[C].
    // 'quit' でループを終了。 [Ctrl]+[C] で強制終了。
    loop {
        let mut line: String = String::new();
        // Wait for command line input from standard input.
        // 標準入力からのコマンドライン入力を待機します。
        match std::io::stdin().read_line(&mut line) {
            Ok(_n) => {}
            // Tips. You can separate error numbers by simply specifying the line number.
            // テクニック。 エラー番号は行番号を振っておくだけで少しはばらけます。
            Err(e) => std::panic::panic_any(Log::print_fatal(&format!(
                "(Err.373) Failed to read line. / {}",
                e
            ))),
        };

        // Write input to log.
        // 入力をログに書きます。
        Log::notice_t(&line, Table::default().str("Description", "Input."));

        if line.chars().count() == 26 && line == "position startpos moves *0" {
            // 将棋所の連続対局中に
            // 相手が 時間切れを知らずに bestmove を返すと、
            // 将棋所は `isready` など次の対局が始まっている最中に
            // `position startpos moves *0` を返してくる。
            // この `*0` をパースできずに落ちることがあるので、無視するぜ（＾～＾）
            continue;
        }

        // p is the acronym for parser.
        // p は parser の頭文字。
        let mut p = CommandLineSeek::new(&line);

        if p.len() == 0 {
            // 任せろだぜ☆（＾～＾）
            Chiyuri::len0(universe);
        // 文字数の長いものからチェック
        } else if p.starts_with("usinewgame") {
            Kifuwarabe::usinewgame(universe);
        } else if p.starts_with("position") {
            Kifuwarabe::position(universe, &line);
        } else if p.starts_with("isready") {
            Kifuwarabe::isready();
        } else if p.starts_with("quit") {
            // ループを抜けて終了
            break;
        } else if p.starts_with("setoption name ") {
            Kifuwarabe::setoption_name(universe, &mut CommandLineSeek::new(&line));
        } else if p.starts_with("usi") {
            Kifuwarabe::usi();
        } else if p.starts_with("go") {
            Kifuwarabe::go(universe, &mut p);
        } else {
            help_chiyuri(universe, &mut p);
        }
    } //loop

    Log::flush();
}

/// 独自コマンド☆（＾～＾）
fn help_chiyuri(universe: &mut Universe, p: &mut CommandLineSeek) {
    // D
    if p.starts_with("do ") {
        p.go_next_to("do ");
        Chiyuri::do_(universe, p);
    // G
    } else if p.starts_with("genmove") {
        Chiyuri::genmove(&universe);
    // H
    } else if p.starts_with("how-much") {
        Chiyuri::how_much(p.line());
    } else if p.starts_with("hash") {
        Chiyuri::hash(universe);
    } else if p.starts_with("kifu") {
        Chiyuri::kifu(universe);
    // L
    } else if p.starts_with("list40") {
        Chiyuri::list40(universe);
    // P
    } else if p.starts_with("pos0") {
        Chiyuri::pos0(universe);
    } else if p.starts_with("pos2") {
        Chiyuri::pos2(universe);
    } else if p.starts_with("pos") {
        Chiyuri::pos(universe);
    // S
    } else if p.starts_with("startpos") {
        Chiyuri::startpos(universe);
    // R
    } else if p.starts_with("rand") {
        Chiyuri::rand();
    // S
    } else if p.starts_with("same") {
        Chiyuri::same(universe);
    // T
    } else if p.starts_with("teigi::conv") {
        Chiyuri::teigi_conv();
    // U
    } else if p.starts_with("undo") {
        Chiyuri::undo(universe);
    }
}
