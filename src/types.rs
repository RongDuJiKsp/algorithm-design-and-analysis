#[derive(Debug, Clone)]
pub struct BackpackContext {
    pub capacity: i32,
    pub weighs: Vec<i32>,
    pub value: Vec<i32>,
}
impl BackpackContext {
    pub fn make(
        capacity: i32,
        weighs: Vec<i32>,
        value: Vec<i32>,
    ) -> Result<BackpackContext, &'static str> {
        if weighs.len() == value.len()
            && weighs.iter().all(|x| *x >= 0)
            && value.iter().all(|x| *x >= 0)
        {
            Ok(BackpackContext {
                capacity,
                weighs,
                value,
            })
        } else {
            Err("Can't Be Solve")
        }
    }
}
pub trait Solver {
    fn make(ctx: BackpackContext) -> Self;
    fn name() -> &'static str;
    fn solve(&self) -> i32;
}
