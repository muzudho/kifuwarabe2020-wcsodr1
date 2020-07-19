use crate::computer_player::search::Value;
use crate::config::INFO_INTERVAL_MSEC;
use crate::cosmic::recording::Movement;
use crate::cosmic::smart::evaluator::Evaluation;
use crate::spaceship::equipment::PvString;
use std::time::{Duration, Instant};

/// ツリーは探索中に１つしか生成しないぜ☆（＾～＾）
pub struct Search {
    /// Number of state nodes searched.  
    /// 探索した状態ノード数。  
    pub nodes: u64,
    /// Start the stopwatch when this structure is created.  
    /// この構造体を生成した時点からストップ・ウォッチを開始します。  
    pub stopwatch: Instant,

    // 思考時間（ミリ秒）をランダムにすることで、指し手を変えるぜ☆（＾～＾）
    pub think_msec: u128,

    pub evaluation: Evaluation,

    // 反復深化探索の１回目だけ真☆（＾～＾）
    pub depth_not_to_give_up: usize,
    // 読みの深さの上限☆（＾～＾）１手を読み切るなら 0 を指定しろだぜ☆（＾～＾）
    pub max_depth0: usize,
    /// 情報表示担当
    pub info: InfoDisplay,
}

impl Search {
    pub fn new(
        many_ways_weight: isize,
        komawari_weight: isize,
        promotion_weight: isize,
        depth_not_to_give_up: usize,
    ) -> Self {
        Search {
            stopwatch: Instant::now(),
            nodes: 0,
            think_msec: 0,
            evaluation: Evaluation::new(many_ways_weight, komawari_weight, promotion_weight),
            depth_not_to_give_up: depth_not_to_give_up,
            max_depth0: 0,
            info: InfoDisplay::default(),
        }
    }

    /// 情報表示
    pub fn info_str(
        cur_depth: Option<usize>,
        state_nodes_nps: Option<(u64, u64)>,
        value: Option<Value>,
        movement: Option<Movement>,
        pv_string: &Option<PvString>,
    ) -> String {
        // TODO 評価値が自分のか相手のか調べてないぜ☆（＾～＾）
        let s = format!(
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
        );
        s.to_string()
    }

    /// ストップウォッチを初期化します。
    pub fn remake_info_display(&mut self) {
        self.info = InfoDisplay::default();
    }
}

/// 行き先表示案内板だぜ☆（＾～＾）
/// 読み筋とか表示されてるぜ☆（＾～＾）
pub struct InfoDisplay {
    /// 情報表示。現在の経過時間。
    info_stopwatch: Instant,
    /// 情報表示。前回表示したタイム。
    info_previous: Duration,
    /// 情報表示。初回は必ず表示。
    info_first: bool,
}
impl Default for InfoDisplay {
    fn default() -> Self {
        let stopwatch1 = Instant::now();
        InfoDisplay {
            info_stopwatch: stopwatch1,
            info_previous: stopwatch1.elapsed(),
            info_first: true,
        }
    }
}
impl InfoDisplay {
    pub fn is_printable(&self) -> bool {
        // 初回か、前回より1秒以上経過していれば。
        self.info_first
            || self.info_previous.as_millis() + INFO_INTERVAL_MSEC
                < self.info_stopwatch.elapsed().as_millis()
    }

    /// 表示間隔を開けます。
    pub fn set_interval(&mut self) {
        self.info_first = false;
        self.info_previous = self.info_stopwatch.elapsed();
    }
}
