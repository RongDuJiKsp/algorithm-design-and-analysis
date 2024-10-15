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
pub struct SolverContext<T: Solver>(T, BackpackContext);
impl<T: Solver> SolverContext<T> {
    pub fn make(t: BackpackContext) -> SolverContext<T> {
        Self(T::make(t.clone()), t)
    }
    pub fn print(&mut self) {
        println!(
            "Solve {:?} \n By {} \n Result is {}",
            self.1,
            T::name(),
            self.0.solve()
        );
    }
}
