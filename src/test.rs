use crate::types::{BackpackContext, Solver, SolverContext};

pub fn run() {
    use crate::solve_by_backtracking::test::BackTrackingSolver;
    use crate::solve_by_dynamic_programming::test::DynamicProgrammingSolver;
    SolverContext::<DynamicProgrammingSolver>::make(
        BackpackContext::make(70, vec![71, 69, 1], vec![100, 1, 2])
    ).print();
}

