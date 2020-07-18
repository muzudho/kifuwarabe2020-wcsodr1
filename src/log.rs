//! Extend the functionality of the log.  
//! ログの機能を拡張します。  
use casual_logger::Log;

/// Extend the functionality of the log.  
/// ログの機能を拡張します。  
pub trait LogExt {
    fn print_info(s: &str);
    fn print_fatal(s: &str) -> String;
}
impl LogExt for Log {
    /// Info level logging and add print to stdout.
    fn print_info(s: &str) {
        println!("{}", s);
        Log::info(s);
    }
    /// panic! で強制終了する前に、ヤケクソで読み筋欄に表示できないかトライするぜ☆（＾～＾）
    fn print_fatal(s: &str) -> String {
        Log::fatal(&format!("info string panic! {}", s))
    }
}
