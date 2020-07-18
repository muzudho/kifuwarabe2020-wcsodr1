use crate::law::generate_move::{FirstOperation, PhaseOperation, SecondOperation};

/// 指し手生成で、先手、後手で処理が変わるやつを吸収するぜ☆（＾～＾）
pub struct MovegenPhase {
    pub first_movegen: Box<dyn PhaseOperation>,
    pub second_movegen: Box<dyn PhaseOperation>,
}
impl Default for MovegenPhase {
    fn default() -> Self {
        MovegenPhase {
            /// 指し手生成
            first_movegen: Box::new(FirstOperation::default()),
            second_movegen: Box::new(SecondOperation::default()),
        }
    }
}

/// 局面
pub enum PosNums {
    // 現局面
    Current,
    // 初期局面
    Start,
}
