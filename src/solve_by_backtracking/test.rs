use crate::types::{BackpackContext, Solver};

pub struct BackTrackingSolver {}
impl BackTrackingSolver {}
impl Solver for BackTrackingSolver {
    fn make(ctx: BackpackContext) -> Self {
        todo!()
    }

    fn name() -> &'static str {
        "BackTrackingSolver"
    }

    fn solve(&mut self) -> i32 {
        todo!()
    }
}