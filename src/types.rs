use std::time::Instant;

#[derive(Debug, Clone)]
pub struct BackpackContext {
    pub capacity: i32,
    pub weighs: Vec<i32>,
    pub value: Vec<i32>,
}
impl BackpackContext {
    pub fn make(capacity: i32, weighs: Vec<i32>, value: Vec<i32>) -> BackpackContext {
        if weighs.len() == value.len() && weighs.iter().all(|x| *x >= 0) {
            BackpackContext {
                capacity,
                weighs,
                value,
            }
        } else {
            panic!("Data Can't Be Solve")
        }
    }
}
pub trait Solver {
    fn make(ctx: BackpackContext) -> Self;
    fn name() -> &'static str;
    fn solve(&mut self) -> i32;
}
pub struct SolverContext<T: Solver>(T);
impl<T: Solver> SolverContext<T> {
    pub fn make(t: BackpackContext) -> SolverContext<T> {
        Self(T::make(t.clone()))
    }
    pub fn print(&mut self) {
        let start = Instant::now();
        let ans = self.0.solve();
        let duration = start.elapsed();
        println!(
            "Solve By {} | Result is {} | Spend {}s",
            T::name(),
            ans,
            duration.as_secs_f32()
        );
    }
}
