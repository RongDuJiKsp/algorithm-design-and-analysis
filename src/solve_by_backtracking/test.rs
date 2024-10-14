use crate::types::{BackpackContext, Solver};

pub struct BackTrackingSolver {
    ctx: BackpackContext,
}
impl BackTrackingSolver {
    //params vis 每个物品是否被选中 采用二进制状态压缩
    fn dfs(deep: i32, vis: u128, values: i32, weights: i32) {}
}
impl Solver for BackTrackingSolver {
    fn make(ctx: BackpackContext) -> Self {
        //时间复杂太大。直接放弃
        if ctx.weighs.len() > 128 {
            panic!("Hard To Resolve!");
        }
        Self { ctx }
    }

    fn name() -> &'static str {
        "BackTrackingSolver"
    }

    fn solve(&self) -> i32 {}
}
