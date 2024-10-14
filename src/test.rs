use crate::solve_by_backtracking::test::BackTrackingSolver;
use crate::types::{BackpackContext, Solver};

pub fn run() {
    let ctx = BackpackContext::make(4, vec![2, 2, 3, 4], vec![999, 2222, 1, 3]).unwrap();
    let mut solver: Box<dyn Solver> = Box::new(BackTrackingSolver::make(ctx));
    println!("Solve {:?} By {} Result is {}", solver.solve(), solver.name(), solver.solve());
}
