use crate::types::{BackpackContext, SolverContext};
use rand::{thread_rng, Rng};
#[allow(dead_code)]
fn gen_full_backpack(sz: i32) -> BackpackContext {
    let (w, v) = ((1..=sz).collect::<Vec<_>>(), (1..=sz).collect::<Vec<_>>());
    BackpackContext::make((1..=sz).sum::<i32>(), w, v)
}
#[allow(dead_code)]
fn gen_rand_data(sz: i32, top_w: i32, top_v: i32, top_cap: i32) -> BackpackContext {
    let mut rng = thread_rng();
    let (w, v) = (
        (1..=sz)
            .map(|_x| rng.random::<i32>().abs() % top_w + 1)
            .collect::<Vec<_>>(),
        (1..=sz)
            .map(|_x| rng.random::<i32>().abs() % top_v + 1)
            .collect::<Vec<_>>(),
    );
    BackpackContext::make(rng.random::<i32>().abs() % top_cap, w, v)
}
#[allow(dead_code)]
fn get_strong_data() -> BackpackContext {
    gen_rand_data(25, 800, 320, 15000)
}

pub fn run() {
    use crate::solve_by_backtracking::test::BackTrackingSolver;
    use crate::solve_by_branch_and_bound::test::BranchAndBoundSolver;
    use crate::solve_by_dynamic_programming::test::DynamicProgrammingSolver;
    use crate::solve_by_random_algorithm::test::SimulatedAnnealingStochasticAlgorithmRandomSolver;
    let ctx = get_strong_data();
    println!("------Data-------");
    println!("{:?}", &ctx);
    println!("------Baseline-------");
    SolverContext::<DynamicProgrammingSolver>::make(ctx.clone()).print();
    println!("--------tests---------");
    SolverContext::<BackTrackingSolver>::make(ctx.clone()).print();
    SolverContext::<BranchAndBoundSolver>::make(ctx.clone()).print();
    SolverContext::<SimulatedAnnealingStochasticAlgorithmRandomSolver>::make(ctx.clone()).print();
}
