use crate::types::{BackpackContext, Solver};

pub struct DynamicProgrammingSolver {
    ctx: BackpackContext,
}

impl Solver for DynamicProgrammingSolver {
    fn make(ctx: BackpackContext) -> Self {
        Self { ctx }
    }

    fn name() -> &'static str {
        "DynamicProgrammingSolver"
    }

    fn solve(&mut self) -> i32 {
        const INF: i32 = 0x3f3f3f3f;
        let n = self.ctx.weighs.len();
        let mut dp = vec![-INF; self.ctx.capacity as usize + 1];
        dp[0] = 0;
        for idx in 0..n {
            let (w, v) = (self.ctx.weighs[idx] as usize, self.ctx.value[idx]);
            for weigh in (w..=self.ctx.capacity as usize).rev() {
                dp[weigh] = dp[weigh].max(dp[weigh - w] + v);
            }
        }
        dp.iter().max().cloned().unwrap()
    }
}
