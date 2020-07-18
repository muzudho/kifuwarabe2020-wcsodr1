//! Square is shogi coordinate. file*10+rank.
//!
//!           North
//!   91 81 71 61 51 41 31 21 11
//!   92 82 72 62 52 42 32 22 12
//! W 93 83 73 63 53 43 33 23 13 E
//! E 94 84 74 64 54 44 34 24 14 A
//! S 95 85 75 65 55 45 35 25 15 S
//! T 96 86 76 66 56 46 36 26 16 T
//!   97 87 77 67 57 47 37 27 17
//!   98 88 78 68 58 48 38 28 18
//!   99 89 79 69 59 49 39 29 19
//!           Source
//!
//!
//!              North
//!   00 01 02 03 04 05 06 07 08 09
//!   10 11 12 13 14 15 16 17 18 19
//!   20 21 22 23 24 25 26 27 28 29
//! E 30 31 32 33 34 35 36 37 38 39 W
//! A 40 41 42 43 44 45 46 47 48 49 E
//! S 50 51 51 53 54 55 56 57 58 59 S
//! T 60 61 62 63 64 65 66 67 68 69 T
//!   70 71 72 73 74 75 76 77 78 79
//!   80 81 82 83 84 85 86 87 88 89
//!   90 91 92 93 94 95 96 97 98 99
//!              Source
//!
//! None is 0.
use crate::law::cryptographic::num_to_lower_case;
use crate::law::speed_of_light::Nine299792458;
use std::cmp::max;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::fmt;
use std::hash::Hash;

///
/// 打はテストできない
///
pub fn _assert_in_board_as_absolute(ab_adr: &AbsoluteAddress2D, hint: &str) {
    let adr = ab_adr.serial_number();
    debug_assert!(
        (10 < adr && adr < 20)
            || (20 < adr && adr < 30)
            || (30 < adr && adr < 40)
            || (40 < adr && adr < 50)
            || (50 < adr && adr < 60)
            || (60 < adr && adr < 70)
            || (70 < adr && adr < 80)
            || (80 < adr && adr < 90)
            || (90 < adr && adr < 100),
        "abs-adr=|{}| hint={}",
        adr,
        hint
    );
}

fn test_dort(test_name: &str, expected: &str, actual: &DictOrthant) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}
fn test_d45ort(test_name: &str, expected: &str, actual: &Degree45Orthant) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}
fn test_rsq(test_name: &str, expected: &str, actual: &RelAdr2D) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}

pub fn test_rotation() {
    // 辞書象限のテスト
    {
        let mut ort = DictOrthant::from_file_and_rank(0, -1);
        test_dort("e1", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(1, -1);
        test_dort("e2", "IV", &ort);
        ort = DictOrthant::from_file_and_rank(1, 0);
        test_dort("e3", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(1, 1);
        test_dort("e4", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(0, 1);
        test_dort("e5", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(-1, 1);
        test_dort("e6", "II", &ort);
        ort = DictOrthant::from_file_and_rank(-1, 0);
        test_dort("e7", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(-1, -1);
        test_dort("e8", "IOrIII", &ort);
    }
    // 45°回転象限のテスト
    {
        // TODO speed_of_light に West とか相対座標を入れておきたい。
        let mut ort = Degree45Orthant::new(&RelAdr2D::new(0, -1));
        test_d45ort("f1", "CoIIIOrCoIV", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(1, -1));
        test_d45ort("f2", "IVOrI", &ort);
        ort = Degree45Orthant::new(&Nine299792458::west());
        test_d45ort("f3", "IVOrI", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(1, 1));
        test_d45ort("f4", "IVOrI", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(0, 1));
        test_d45ort("f5", "CoIOrCoII", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(-1, 1));
        test_d45ort("f6", "IIOrIII", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(-1, 0));
        test_d45ort("f7", "IIOrIII", &ort);
        ort = Degree45Orthant::new(&RelAdr2D::new(-1, -1));
        test_d45ort("f8", "IIOrIII", &ort);
    }
    // 相対番地のテスト
    {
        test_rsq("b1", "(0x -1y -1adr)", &RelAdr2D::new(0, -1));
        test_rsq("b2", "(1x -1y 9adr)", &RelAdr2D::new(1, -1));
        test_rsq("b3", "(1x 0y 10adr)", &Nine299792458::west());
        test_rsq("b4", "(1x 1y 11adr)", &RelAdr2D::new(1, 1));
        test_rsq("b5", "(0x 1y 1adr)", &RelAdr2D::new(0, 1));
        test_rsq("b6", "(-1x 1y -9adr)", &RelAdr2D::new(-1, 1));
        test_rsq("b7", "(-1x 0y -10adr)", &RelAdr2D::new(-1, 0));
        test_rsq("b8", "(-1x -1y -11adr)", &RelAdr2D::new(-1, -1));
    }
    // 45°回転のテスト
    {
        let mut r = RelAdr2D::new(0, -1);
        test_rsq("a1", "(0x -1y -1adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a2", "(1x -1y 9adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a3", "(1x 0y 10adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a4", "(1x 1y 11adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a5", "(0x 1y 1adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a6", "(-1x 1y -9adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a7", "(-1x 0y -10adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a8", "(-1x -1y -11adr)", &r);
        r.rotate_45_ccw();
        test_rsq("a9", "(0x -1y -1adr)", &r);
    }
    // 90°回転のテスト＜その１＞
    {
        let mut r = RelAdr2D::new(0, -1);
        test_rsq("c1", "(0x -1y -1adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c2", "(1x 0y 10adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c3", "(0x 1y 1adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c4", "(-1x 0y -10adr)", &r);
        r.rotate_90_ccw();
        test_rsq("c5", "(0x -1y -1adr)", &r);
    }
    // 90°回転のテスト＜その２＞
    {
        let mut r = RelAdr2D::new(1, -1);
        test_rsq("d1", "(1x -1y 9adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d2", "(1x 1y 11adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d3", "(-1x 1y -9adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d4", "(-1x -1y -11adr)", &r);
        r.rotate_90_ccw();
        test_rsq("d5", "(1x -1y 9adr)", &r);
    }
    // 桂馬のテスト
    {
        let mut r = RelAdr2D::new(0, -1);
        test_rsq("g1", "(0x -1y -1adr)", &r);
        r.rotate(Angle::Ccw45);
        test_rsq("g2", "(1x -1y 9adr)", &r);
        r.double_rank();
        test_rsq("g3", "(1x -2y 8adr)", &r);

        let mut r = RelAdr2D::new(0, -1);
        test_rsq("g4", "(0x -1y -1adr)", &r);
        r.rotate(Angle::Ccw315);
        test_rsq("g5", "(-1x -1y -11adr)", &r);
        r.double_rank();
        test_rsq("g6", "(-1x -2y -12adr)", &r);

        let mut r = RelAdr2D::new(0, 1);
        test_rsq("g7", "(0x 1y 1adr)", &r);
        r.rotate(Angle::Ccw45);
        test_rsq("g8", "(-1x 1y -9adr)", &r);
        r.double_rank();
        test_rsq("g9", "(-1x 2y -8adr)", &r);

        let mut r = RelAdr2D::new(0, 1);
        test_rsq("g10", "(0x 1y 1adr)", &r);
        r.rotate(Angle::Ccw315);
        test_rsq("g11", "(1x 1y 11adr)", &r);
        r.double_rank();
        test_rsq("g12", "(1x 2y 12adr)", &r);
    }
    // 角度指定回転のテスト(北から)
    {
        // 0
        let mut r = RelAdr2D::new(0, -1);
        test_rsq("h1", "(0x -1y -1adr)", &r);
        r.rotate(Angle::Ccw0);
        test_rsq("h2", "(0x -1y -1adr)", &r);

        // 45
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw45);
        test_rsq("h3", "(1x -1y 9adr)", &r);

        // 90
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw90);
        test_rsq("h4", "(1x 0y 10adr)", &r);

        // 135
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw135);
        test_rsq("h5", "(1x 1y 11adr)", &r);

        // 180
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw180);
        test_rsq("h6", "(0x 1y 1adr)", &r);

        // 225
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw225);
        test_rsq("h7", "(-1x 1y -9adr)", &r);

        // 270
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw270);
        test_rsq("h8", "(-1x 0y -10adr)", &r);

        // 315
        r = RelAdr2D::new(0, -1);
        r.rotate(Angle::Ccw315);
        test_rsq("h9", "(-1x -1y -11adr)", &r);
    }
    // 角度指定回転のテスト(南から)
    {
        // 0
        let mut r = RelAdr2D::new(0, 1);
        test_rsq("h1", "(0x 1y 1adr)", &r);
        r.rotate(Angle::Ccw0);
        test_rsq("h2", "(0x 1y 1adr)", &r);

        // 45
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw45);
        test_rsq("h3", "(-1x 1y -9adr)", &r);

        // 90
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw90);
        test_rsq("h4", "(-1x 0y -10adr)", &r);

        // 135
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw135);
        test_rsq("h5", "(-1x -1y -11adr)", &r);

        // 180
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw180);
        test_rsq("h6", "(0x -1y -1adr)", &r);

        // 225
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw225);
        test_rsq("h7", "(1x -1y 9adr)", &r);

        // 270
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw270);
        test_rsq("h8", "(1x 0y 10adr)", &r);

        // 315
        r = RelAdr2D::new(0, 1);
        r.rotate(Angle::Ccw315);
        test_rsq("h9", "(1x 1y 11adr)", &r);
    }
}

//
// 盤、升、筋、段
//

// #[allow(non_camel_case_types)]
// pub type isquare = isize;

/// 持ち駒を置くところも付いている盤だぜ☆（＾～＾）添付の図を参照してくれだぜ☆（＾～＾）
pub const BOARD_MEMORY_AREA: usize = 122;

/// 筋、段は 1 から始まる、という明示。
/// usize が速い☆（＾～＾）
// pub const FILE_0: usize = 0;
pub const FILE0U8: u8 = 0;
// pub const FILE_1: usize = 1;
pub const FILE1U8: u8 = 1;
// pub const FILE_9: usize = 9;
pub const FILE9U8: u8 = 9;
// pub const FILE_10: usize = 10;
pub const FILE10U8: u8 = 10;
// pub const FILE_11: usize = 11;
// pub const FILE11U8: u8 = 11;
pub const FILE13U8: u8 = 13;
// pub const RANK_0: usize = 0;
pub const RANK0U8: u8 = 0;
// pub const RANK_1: usize = 1;
pub const RANK1U8: u8 = 1;
// pub const RANK_2: usize = 2;
pub const RANK2U8: u8 = 2;
// pub const RANK_3: usize = 3;
pub const RANK3U8: u8 = 3;
// pub const RANK_4: usize = 4;
pub const RANK4U8: u8 = 4;
// pub const RANK_5: usize = 5;
// pub const RANK_6: usize = 6;
pub const RANK6U8: u8 = 6;
// pub const RANK_7: usize = 7;
pub const RANK7U8: u8 = 7;
// pub const RANK_8: usize = 8; //うさぎの打てる段の上限
// pub const RANK8U8: u8 = 8;
// pub const RANK_9: usize = 9;
pub const RANK9U8: u8 = 9;
// pub const RANK_10: usize = 10;
pub const RANK10U8: u8 = 10;
// pub const RANK_11: usize = 11;
// pub const RANK11U8: u8 = 11;

/// 升の検索等で、該当なしの場合
pub const SQUARE_NONE: usize = 0;

#[derive(Debug)]
pub enum DictOrthant {
    /// 第２象限。x=0, y=0 ともに含みません。
    II,
    /// 第４象限。x=0, y=0 ともに含みません。
    IV,
    /// 第１象限と第三象限。区別しません。x=0, y=0 ともに含みます。
    IOrIII,
}
impl DictOrthant {
    pub fn from_file_and_rank(file: isize, rank: isize) -> Self {
        if 0 <= file * rank {
            DictOrthant::IOrIII
        } else if file < 0 {
            DictOrthant::II
        } else {
            DictOrthant::IV
        }
    }
}

#[derive(Debug)]
pub enum Degree45Orthant {
    /// 正第４象限と、正第１象限☆（＾～＾）
    IVOrI,
    /// コ第１象限と、コ第２象限☆（＾～＾）
    CoIOrCoII,
    /// 正第２象限と、正第３象限☆（＾～＾）
    IIOrIII,
    /// コ第３象限と、コ第４象限☆（＾～＾）
    CoIIIOrCoIV,
}
impl Degree45Orthant {
    /// Arguments
    /// ---------
    /// * `r` - (Relative file, relative rank).
    pub fn new(r: &RelAdr2D) -> Self {
        let range = max(r.file.abs(), r.rank.abs());
        if r.file == range {
            Degree45Orthant::IVOrI
        } else if r.file == -range {
            Degree45Orthant::IIOrIII
        } else if r.rank == range {
            Degree45Orthant::CoIOrCoII
        } else {
            Degree45Orthant::CoIIIOrCoIV
        }
    }
}

pub const ANGLE_LEN: usize = 8;
/// Counterclockwise(反時計回り)での回転方向。 45°ずつ☆（＾～＾）
#[derive(Clone, Copy, Debug)]
pub enum Angle {
    /// 西。
    Ccw0,
    /// 南西。
    Ccw45,
    /// 南。
    Ccw90,
    /// 南東。
    Ccw135,
    /// 東。
    Ccw180,
    /// 北東。
    Ccw225,
    /// 北。
    Ccw270,
    /// 北西。
    Ccw315,
}

/// 相対番地。絶対番地と同じだが、回転の中心を原点に固定した操作が行われるぜ☆（＾～＾）
///
/// 18  8  -2 -12 -22
/// 19  9  -1 -11 -21
/// 20 10   0 -10 -20
/// 21 11   1 - 9 -19
/// 22 12   2 - 8 -18
///
/// file, rank から 相対番地は作れますが、相対番地から file, rank を作ることはできません(不定)。
/// そこから、 file, rank で持ちます。
///
/// メモリを使わないようにしようぜ☆（＾～＾）
#[derive(Clone, Copy)]
pub struct RelAdr2D {
    file: isize,
    rank: isize,
}
impl RelAdr2D {
    pub fn new(file: isize, rank: isize) -> Self {
        RelAdr2D {
            file: file,
            rank: rank,
        }
    }

    /// Arguments
    /// ---------
    /// * `r` - (Relative file, relative rank).
    pub fn get_address(&self) -> isize {
        10 * self.file + self.rank
    }

    /// Arguments
    /// ---------
    /// * `r` - (Relative file, relative rank).
    pub fn rotate_180(&mut self) -> &mut Self {
        self.file *= -1;
        self.rank *= -1;
        self
    }

    /// Counterclockwise
    ///
    /// Arguments
    /// ---------
    /// * `r` - (Relative file, relative rank).
    pub fn rotate_90_ccw(&mut self) -> &mut Self {
        // 象限は、何度回転するかによって境界線の位置が変わってくるので、回転の直前で調べるしかないぜ☆（＾～＾）
        // でも、 90°回転のときは 象限は１つしかないけどな☆（＾～＾）全象限同じ式だぜ☆（*＾～＾*）
        let new_file = -self.rank;
        let new_rank = self.file;
        self.file = new_file;
        self.rank = new_rank;
        self
    }

    /// Counterclockwise
    ///
    /// Arguments
    /// ---------
    /// * `r` - (Relative file, relative rank).
    pub fn rotate_45_ccw(&mut self) -> &mut Self {
        // 象限は、何度回転するかによって境界線の位置が変わってくるので、回転の直前で調べるしかないぜ☆（＾～＾）
        let orthant = Degree45Orthant::new(self);
        match orthant {
            Degree45Orthant::IVOrI => {
                let distance = self.file;
                let mut file2 = self.file;
                let mut rank2 = self.rank + distance;
                let over = rank2.abs() - distance.abs();
                if 0 < over {
                    rank2 = distance;
                    file2 -= over;
                }
                self.file = file2;
                self.rank = rank2;
                self
            }
            Degree45Orthant::IIOrIII => {
                let distance = self.file;
                let mut file2 = self.file;
                let mut rank2 = self.rank + distance;
                let over = rank2.abs() - distance.abs();
                if 0 < over {
                    rank2 = distance;
                    file2 += over;
                }
                self.file = file2;
                self.rank = rank2;
                self
            }
            Degree45Orthant::CoIOrCoII => {
                let distance = self.rank;
                let mut file2 = self.file - distance;
                let mut rank2 = self.rank;
                let over = rank2.abs() - distance.abs();
                if 0 < over {
                    file2 = distance;
                    rank2 -= over;
                }
                self.file = file2;
                self.rank = rank2;
                self
            }
            Degree45Orthant::CoIIIOrCoIV => {
                let distance = self.rank;
                let mut file2 = self.file - distance;
                let mut rank2 = self.rank;
                let over = rank2.abs() - distance.abs();
                if 0 < over {
                    file2 = distance;
                    rank2 -= over;
                }
                self.file = file2;
                self.rank = rank2;
                self
            }
        }
    }

    /// Arguments
    /// ---------
    /// * `r` - (Relative file, relative rank).
    pub fn rotate(&mut self, angle: Angle) -> &mut Self {
        use crate::cosmic::smart::square::Angle::*;
        match angle {
            Ccw0 => self,
            Ccw45 => self.rotate_45_ccw(),
            Ccw90 => self.rotate_90_ccw(),
            Ccw135 => self.rotate_45_ccw().rotate_90_ccw(),
            Ccw180 => self.rotate_180(),
            Ccw225 => self.rotate_45_ccw().rotate_180(),
            Ccw270 => self.rotate_90_ccw().rotate_180(),
            Ccw315 => self.rotate_45_ccw().rotate_90_ccw().rotate_180(),
        }
    }

    /// 段を２倍にします。桂馬に使います。
    ///
    /// Arguments
    /// ---------
    /// * `r` - (Relative file, relative rank).
    pub fn double_rank(&mut self) -> &mut Self {
        let rank2 = 2 * self.rank;
        let carry = rank2 / 10;
        let file2 = if carry != 0 {
            self.file + carry
        } else {
            self.file
        };
        self.file = file2;
        self.rank = rank2;
        self
    }
}
/// 回転してみるまで象限は分からないので、出せるのは筋、段、相対番地だけだぜ☆（＾～＾）
impl fmt::Debug for RelAdr2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}x {}y {}adr)",
            self.file,
            self.rank,
            self.get_address()
        )
    }
}

/// このオブジェクトは大量に生成されるから、容量を小さく抑えた方がいいんだろうか☆（＾～＾）？
/// 1～9 は 4 byte.
/// 11～99 は 8 byte.
///
/// 絶対番地☆（＾～＾）相対番地と同じだが、回転の操作は座標 55 が中心になるぜ☆（＾～＾）
/// きふわらべでは 辞書象限 を採用している☆（＾～＾）
/// これは、file, rank は別々に持ち、しかも軸毎にプラス・マイナスを持つぜ☆（＾～＾）
///
/// Copy: 配列の要素の初期化時に使う☆（＾～＾）
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AbsoluteAddress2D {
    /// Square is shogi coordinate. file*10+rank.
    ///
    ///           North
    ///   91 81 71 61 51 41 31 21 11
    ///   92 82 72 62 52 42 32 22 12
    /// W 93 83 73 63 53 43 33 23 13 E
    /// E 94 84 74 64 54 44 34 24 14 A
    /// S 95 85 75 65 55 45 35 25 15 S
    /// T 96 86 76 66 56 46 36 26 16 T
    ///   97 87 77 67 57 47 37 27 17
    ///   98 88 78 68 58 48 38 28 18
    ///   99 89 79 69 59 49 39 29 19
    ///           Source
    serial: u8,
}
impl Default for AbsoluteAddress2D {
    /// ゴミの値を作るぜ☆（＾～＾）
    fn default() -> Self {
        AbsoluteAddress2D { serial: 11 }
    }
}
impl AbsoluteAddress2D {
    pub fn new(file: u8, rank: u8) -> Self {
        debug_assert!(FILE0U8 <= file && file < FILE13U8, format!("file={}", file));
        debug_assert!(
            RANK0U8 <= rank && rank < RANK10U8 as u8,
            format!("rank={}", rank)
        );
        AbsoluteAddress2D {
            serial: 10 * file as u8 + rank as u8,
        }
    }

    pub fn from_absolute_address(serial: u8) -> Option<AbsoluteAddress2D> {
        let file = serial / 10; // 12 列まであるぜ☆（＾～＾） // (serial / 10) % 10
        let rank = serial % 10;
        if serial == 0 {
            None
        } else {
            debug_assert!(FILE0U8 <= file && file < FILE13U8, format!("file={}", file));
            debug_assert!(RANK0U8 <= rank && rank < RANK10U8, format!("rank={}", rank));
            Some(AbsoluteAddress2D::new(file, rank))
        }
    }

    /// 列番号。いわゆる筋。右から 1, 2, 3 ...
    pub fn file(&self) -> u8 {
        self.serial / 10 // 12 列まであるぜ☆（＾～＾） // (self.serial / 10) % 10
    }

    /// 行番号。いわゆる段。上から 1, 2, 3 ...
    pub fn rank(&self) -> u8 {
        self.serial % 10
    }

    pub fn to_file_rank(&self) -> (u8, u8) {
        (self.file(), self.rank())
    }

    /*
    pub fn rotate_180(&self) -> Self {
        let file = FILE10U8 - self.file();
        let rank = RANK10U8 - self.rank();
        debug_assert!(FILE0U8 < file && file < FILE10U8, format!("file={}", file));
        debug_assert!(RANK0U8 < rank && rank < RANK10U8, format!("rank={}", rank));
        AbsoluteAddress2D::new(file, rank)
    }
    */

    /// 壁の中にいる☆（＾～＾）
    pub fn wall(&self) -> bool {
        self.file() % 10 == 0 || self.rank() % 10 == 0
    }

    /// 連番
    pub fn serial_number(&self) -> u8 {
        self.serial
    }

    pub fn offset(&mut self, r: &RelAdr2D) -> &mut Self {
        // TODO rankの符号はどうだったか……☆（＾～＾） 絶対番地の使い方をしてれば問題ないだろ☆（＾～＾）
        // TODO sum は負数になることもあり、そのときは明らかにイリーガルだぜ☆（＾～＾）
        self.serial = (self.serial_number() as isize + r.get_address()) as u8;
        self
    }
}
/// USI向け。
impl fmt::Display for AbsoluteAddress2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.file(),
            num_to_lower_case(self.rank() as usize),
        )
    }
}
impl fmt::Debug for AbsoluteAddress2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}x {}y {}adr)",
            self.file(),
            self.rank(),
            self.serial_number()
        )
    }
}
