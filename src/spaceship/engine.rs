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
    pub fn parse(line: &str) -> Go {
        let re = Regex::new(r"^go btime (\d+) wtime (\d+) binc (\d+) winc (\d+)$").unwrap();
        if let Some(cap) = re.captures(line) {
            Go {
                btime: cap[1].parse().unwrap(),
                wtime: cap[2].parse().unwrap(),
                binc: cap[3].parse().unwrap(),
                winc: cap[4].parse().unwrap(),
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
