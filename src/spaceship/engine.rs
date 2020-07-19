use crate::command_line_seek::CommandLineSeek;
use crate::log::LogExt;
use casual_logger::Log;
use regex::Regex;
use std::fmt;

pub struct Go {
    pub btime: u64,
    pub wtime: u64,
    pub binc: u64,
    pub winc: u64,
}
impl fmt::Display for Go {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "go btime {} wtime {} binc {} winc {}",
            self.btime, self.wtime, self.binc, self.winc
        )
    }
}
impl Go {
    /// Example
    /// -------
    /// go btime 40000 wtime 50000 binc 10000 winc 10000
    pub fn parse(p: &mut CommandLineSeek) -> Go {
        let re = match Regex::new(r"^go btime (\d+) wtime (\d+) binc (\d+) winc (\d+)$") {
            Result::Ok(val) => val,
            Result::Err(e) => panic!(Log::print_fatal(&format!("Invalid regex=|{}|", e))),
        };
        if let Some(cap) = re.captures(p.line()) {
            Go {
                btime: match cap[1].parse() {
                    Result::Ok(val) => val,
                    Result::Err(e) => panic!(Log::print_fatal(&format!("Invalid cap1=|{}|", e))),
                },
                wtime: match cap[2].parse() {
                    Result::Ok(val) => val,
                    Result::Err(e) => panic!(Log::print_fatal(&format!("Invalid cap2=|{}|", e))),
                },
                binc: match cap[3].parse() {
                    Result::Ok(val) => val,
                    Result::Err(e) => panic!(Log::print_fatal(&format!("Invalid cap3=|{}|", e))),
                },
                winc: match cap[4].parse() {
                    Result::Ok(val) => val,
                    Result::Err(e) => panic!(Log::print_fatal(&format!("Invalid cap4=|{}|", e))),
                },
            }
        } else {
            // デバッグ時に `go` のみ打鍵した場合など。小さな数にします。
            Go {
                btime: 500,
                wtime: 500,
                binc: 0,
                winc: 0,
            }
        }
    }
}
