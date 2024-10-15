use std::collections::VecDeque;
use crate::types::{BackpackContext, Solver};

pub struct BranchAndBoundSolver {
    ctx: BackpackContext,
}
impl BranchAndBoundSolver {
    fn bound(&self, elm_idx: usize, now_value: i32, now_weigh: i32) {
        let (w, v) = (self.ctx.weighs[elm_idx], self.ctx.value[elm_idx]);
        now_value + (self.ctx.capacity - now_weigh) * v / w;
    }
}
impl Solver for BranchAndBoundSolver {
    fn make(ctx: BackpackContext) -> Self {
        Self {
            ctx
        }
    }

    fn name() -> &'static str {
        "BranchAndBoundSolver"
    }

    fn solve(&self) -> i32 {
        let mut queue = VecDeque::new();
    }
}