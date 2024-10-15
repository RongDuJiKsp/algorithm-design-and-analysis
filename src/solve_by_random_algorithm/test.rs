use crate::types::{BackpackContext, Solver};

pub struct RandomSolver {
    ctx: BackpackContext,
}
impl Solver for RandomSolver {
    fn make(ctx: BackpackContext) -> Self {
        todo!()
    }

    fn name() -> &'static str {
        todo!()
    }

    fn solve(&mut self) -> i32 {
        todo!()
    }
}
