use crate::types;

pub fn run() {
    let ctx = types::BackpackContext::make(4, vec![2, 2, 3, 4], vec![999, 2222, 1, 3]).unwrap();
    crate::solve_by_backtracking::test::run(ctx.clone());
}
