use regex::Regex;
use std::fmt;

pub struct Go {
    pub btime: i64,
    pub wtime: i64,
    pub binc: i64,
    pub winc: i64,
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
    pub fn parse(line: &String) -> Go {
        let re = Regex::new(r"^go btime (\d+) wtime (\d+) binc (\d+) winc (\d+)$").unwrap();
        let cap = re.captures(line).unwrap();
        Go {
            btime: cap[1].parse().unwrap(),
            wtime: cap[2].parse().unwrap(),
            binc: cap[3].parse().unwrap(),
            winc: cap[4].parse().unwrap(),
        }
    }
}
