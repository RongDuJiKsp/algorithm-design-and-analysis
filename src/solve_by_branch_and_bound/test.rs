use crate::types::{BackpackContext, Solver};
use std::collections::VecDeque;
use std::iter::zip;

pub struct BranchAndBoundSolver {
    ctx: BackpackContext,
}
impl BranchAndBoundSolver {
    fn bound(&self, elm_idx: usize, now_value: i32, now_weigh: i32) -> i32 {
        let (w, v) = (self.ctx.weighs[elm_idx], self.ctx.value[elm_idx]);
        now_value + (self.ctx.capacity - now_weigh) * v / w
    }
    fn sort(&mut self) {
        let mut ele = zip(
            self.ctx.weighs.iter().cloned(),
            self.ctx.value.iter().cloned(),
        )
        .collect::<Vec<_>>();
        ele.sort_by(|a, b| b.1.cmp(&a.1));
        for (idx, (w, v)) in ele.into_iter().enumerate() {
            self.ctx.weighs[idx] = w;
            self.ctx.value[idx] = v;
        }
    }
}
impl Solver for BranchAndBoundSolver {
    fn make(ctx: BackpackContext) -> Self {
        Self { ctx }
    }

    fn name() -> &'static str {
        "BranchAndBoundSolver"
    }

    fn solve(&mut self) -> i32 {
        self.sort();
        let mut ans = 0;
        let max_deep = self.ctx.value.len();
        let mut queue = VecDeque::<SearchCtx>::new();
        queue.push_back(SearchCtx::new(0, 0, 0, 0));
        while let Some(this) = queue.pop_front() {
            if this.deep == max_deep {
                ans = ans.max(this.v);
                continue;
            }
            //for left node
            if self.bound(this.deep, this.w, this.v) >= ans {
                queue.push_back(SearchCtx::new(
                    this.select_status,
                    this.deep + 1,
                    this.w,
                    this.v,
                ))
            }
            //for right
            if this.w + self.ctx.weighs[this.deep] <= self.ctx.capacity
                && self.bound(this.deep, this.w, this.v) >= ans
            {
                queue.push_back(SearchCtx::new(
                    this.select_status | (1 << this.deep),
                    this.deep + 1,
                    this.w + self.ctx.weighs[this.deep],
                    this.v + self.ctx.value[this.deep],
                ))
            }
        }
        ans
    }
}
struct SearchCtx {
    select_status: u128,
    deep: usize,
    w: i32,
    v: i32,
}
impl SearchCtx {
    fn new(select_status: u128, deep: usize, w: i32, v: i32) -> SearchCtx {
        SearchCtx {
            select_status,
            deep,
            w,
            v,
        }
    }
}
