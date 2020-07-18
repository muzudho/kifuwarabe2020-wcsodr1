//! 宇宙船の備品だぜ☆（＾～＾）
use crate::computer_player::daydream::Value;
use crate::cosmic::recording::Movement;
use crate::cosmic::smart::square::test_rotation;
use crate::log::LogExt;
use casual_logger::Log;
use std::time::{Duration, Instant};

/// ちゆり「望遠鏡だぜ☆」
/// 夢見　「何も見えないんだけど？」
/// ちゆり「そうか、残念だな……☆」
pub struct Telescope {}
impl Telescope {
    pub fn look() {
        test_rotation();
    }
}

/// PV表示、または 文字列表示だぜ☆（＾～＾）
pub enum PvString {
    /// 思考を開始してからのミリ秒と、読み筋。
    PV(u128, String),
    String(String),
}

/// 行き先表示案内板だぜ☆（＾～＾）
/// 読み筋とか表示されてるぜ☆（＾～＾）
pub struct DestinationDisplay {
    /// 情報用のストップウォッチ
    stopwatch: Instant,
    previous: Duration,
    first: bool,
}
impl Default for DestinationDisplay {
    fn default() -> Self {
        let stopwatch1 = Instant::now();
        DestinationDisplay {
            stopwatch: stopwatch1,
            previous: stopwatch1.elapsed(),
            first: true,
        }
    }
}
impl DestinationDisplay {
    /// ストップウォッチを初期化します。
    pub fn clear(&mut self) {
        self.stopwatch = Instant::now();
        self.previous = self.stopwatch.elapsed();
        self.first = true;
    }

    pub fn is_printable(&self) -> bool {
        // 初回か、前回より1秒以上経過していれば。
        self.first || self.previous.as_secs() + 1 < self.stopwatch.elapsed().as_secs()
    }
    /// 情報表示
    pub fn print(
        &mut self,
        cur_depth: Option<usize>,
        state_nodes_nps: Option<(u64, u64)>,
        value: Option<Value>,
        movement: Option<Movement>,
        pv_string: &Option<PvString>,
    ) {
        // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
        Log::println(&format!(
            "info{}{}{}{}{}{}",
            // 1. 思考を開始してからのミリ秒☆（＾～＾）
            if let Some(pv_string_val) = pv_string {
                match pv_string_val {
                    PvString::PV(msec, _pv) => format!(" time {}", msec),
                    PvString::String(_x) => "".to_string(),
                }
            } else {
                "".to_string()
            },
            // 2.
            if let Some(num) = cur_depth {
                // 単に読み筋の長さ☆（＾～＾）
                format!(" depth {}", num)
            } else {
                "".to_string()
            },
            // 3.
            if let Some((state_node, nps)) = state_nodes_nps {
                format!(" nodes {} nps {}", state_node, nps)
            } else {
                "".to_string()
            },
            // 4.
            if let Some(value_val) = value {
                match value_val {
                    Value::Win => {
                        // 自分が勝つ
                        " score mate +".to_string()
                    }
                    Value::Lose => {
                        // 自分が負ける
                        " score mate -".to_string()
                    }
                    Value::CentiPawn(num) => format!(" score cp {}", num),
                }
            } else {
                "".to_string()
            },
            // 5.
            if let Some(movement_val) = movement {
                format!(" currmove {}", movement_val)
            } else {
                "".to_string()
            },
            // 6.
            if let Some(pv_string_val) = pv_string {
                match pv_string_val {
                    PvString::PV(_sec, pv) => format!(" pv {}", pv),
                    PvString::String(x) => format!(" string {}", x),
                }
            } else {
                "".to_string()
            }
        ));
        self.first = false;
        self.previous = self.stopwatch.elapsed();
    }
}
