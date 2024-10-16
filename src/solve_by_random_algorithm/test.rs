use crate::types::{BackpackContext, Solver};
use std::ops::Sub;
use rand::{random, thread_rng, Rng};
use rand::rngs::ThreadRng;

const START_TMP: f64 = 300.0;
const END_TMP: f64 = 2.0;
const TMP_DOWN: f64 = 0.85;
const EXP: f64 = 1e-2;
const REV_BITS: i32 = 2;
const BALANCE_TIME: i32 = 100;

pub struct RandomSolver {
    ctx: BackpackContext,
    tmp: f64,
    rand: ThreadRng,
}
impl RandomSolver {
    fn af(&mut self) -> bool {
        self.tmp *= TMP_DOWN;
        !(self.tmp < END_TMP + EXP)
    }
    fn zero(&self) -> u128 {
        (0..self.ctx.value.len()).fold(0u128, |g, _x| (g << 1) | 1)
    }
    fn weigh_of(&self, v: u128) -> i32 {
        let mut res = 0;
        let mut mov = v;
        let mut idx = 0usize;
        while mov > 0 {
            if mov & 1 == 1 {
                res += self.ctx.weighs[idx];
            }
            mov >>= 1;
            idx += 1;
        }
        res
    }
    fn value_of(&self, v: u128) -> i32 {
        let mut res = 0;
        let mut mov = v;
        let mut idx = 0usize;
        while mov > 0 {
            if mov & 1 == 1 {
                res += self.ctx.value[idx];
            }
            mov >>= 1;
            idx += 1;
        }
        res
    }
    fn rand_rev(&mut self, v: u128) -> u128 {
        let mut r = v;
        for _time in 0..REV_BITS {
            let idx = self.rand.gen_range(0..self.ctx.value.len() as i32);
            r = (r & !(1 << idx)) | ((!r) & (1 << idx));
        }
        r
    }
}
impl Solver for RandomSolver {
    fn make(ctx: BackpackContext) -> Self {
        RandomSolver {
            ctx,
            tmp: START_TMP,
            rand: thread_rng(),
        }
    }

    fn name() -> &'static str {
        "SimulatedAnnealingStochasticAlgorithmRandomSolver"
    }

    fn solve(&mut self) -> i32 {
        let mut res = 0;
        let mut top_of = 0;
        let mut select = random::<u128>() & self.zero();
        let mut range = 0;
        loop {
            select = self.rand_rev(select);
            if self.weigh_of(select) <= self.ctx.capacity {
                let v = self.value_of(select);
                if v >= top_of || random::<f64>() < ((v - top_of) as f64 / self.tmp).exp() {
                    top_of = v;
                }
                res = res.max(top_of);
            }
            if range + 1 == BALANCE_TIME {
                range = 0;
                if !self.af() {
                    break;
                }
            } else { range += 1; }
        }
        res
    }
}
