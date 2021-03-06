//!
//! 設定
//!

/// USI対応コンピューター将棋ソフトの名前☆（＾～＾）
pub const ENGINE_NAME: &str = "KifuwarabeDR1 bld272";

/// 作者の名前。姓・名の順にしたいぜ☆（＾～＾）異文化に通じる表記方法はないものか☆（＾～＾）
pub const ENGINE_AUTHOR: &str = "TAKAHASHI, Satoshi";

/// ログ
pub const LOG_FILE: &str = "kifuwarabe2020_dr1";

pub const PV_BUFFER: usize = 2048;

/// info 表示の間隔（ミリ秒）
pub const INFO_INTERVAL_MSEC: u128 = 1000;
