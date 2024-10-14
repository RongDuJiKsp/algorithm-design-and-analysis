use crate::solve_by_backtracking::test::BackTrackingSolver;
use crate::types::{BackpackContext, Solver, SolverContext};

pub fn run() {
    SolverContext::<BackTrackingSolver>::make(
        BackpackContext::make(4, vec![2, 2, 3, 4], vec![999, 2222, 1, 3])
    ).print();
}

