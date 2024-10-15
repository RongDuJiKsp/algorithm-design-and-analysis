use crate::types::{BackpackContext, Solver, SolverContext};

pub fn run() {
    use crate::solve_by_backtracking::test::BackTrackingSolver;
    use crate::solve_by_branch_and_bound::test::BranchAndBoundSolver;
    use crate::solve_by_dynamic_programming::test::DynamicProgrammingSolver;
    use crate::solve_by_random_algorithm::test::RandomSolver;
    SolverContext::<BranchAndBoundSolver>::make(BackpackContext::make(
        70,
        vec![71, 69, 1],
        vec![100, 1, 2],
    ))
    .print();
}
