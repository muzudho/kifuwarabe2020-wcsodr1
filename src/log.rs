//! Extend the functionality of the log.  
//! ログの機能を拡張します。  
use casual_logger::Log;

/// Extend the functionality of the log.  
/// ログの機能を拡張します。  
pub trait LogExt {
    fn println(s: &str);
    fn panic(s: &str) -> String;
}
impl LogExt for Log {
    /// Info level logging and add print to stdout.
    fn println(s: &str) {
        println!("{}", s);
        Log::infoln(s);
    }
    /// panic! で強制終了する前に、ヤケクソで読み筋欄に表示できないかトライするぜ☆（＾～＾）
    fn panic(s: &str) -> String {
        let s2 = format!("info string panic! {}", s);
        Log::fatalln(&s2);
        s2
    }
}
