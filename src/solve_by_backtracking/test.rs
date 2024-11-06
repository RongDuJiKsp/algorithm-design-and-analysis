use crate::types::{BackpackContext, Solver};
use std::cmp;

pub struct BackTrackingSolver {
    ctx: BackpackContext,
}
impl BackTrackingSolver {
    //params vis 每个物品是否被选中 采用二进制状态压缩
    fn dfs(&self, deep: usize, vis: u128, weights: i32, values: i32) -> i32 {
        if weights > self.ctx.capacity {
            return -1;
        }
        if self.ctx.weighs.len() == deep {
            return values;
        }
        cmp::max(
            self.dfs(deep + 1, vis, weights, values),
            self.dfs(
                deep + 1,
                vis | (1 << deep),
                weights + self.ctx.weighs[deep],
                values + self.ctx.value[deep],
            ),
        )
    }
}
impl Solver for BackTrackingSolver {
    fn make(ctx: BackpackContext) -> Self {
        if ctx.weighs.len() > 128 {
            panic!("Hard To Resolve!");
        }
        Self { ctx }
    }

    fn name() -> &'static str {
        "BackTrackingSolver"
    }

    fn solve(&mut self) -> i32 {
        self.dfs(0, 0, 0, 0)
    }
}
