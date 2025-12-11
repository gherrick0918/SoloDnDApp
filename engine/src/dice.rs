use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Clone, Debug)]
pub struct DiceRoller {
    rng: StdRng,
}

impl DiceRoller {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn d20(&mut self) -> i32 {
        self.rng.gen_range(1..=20)
    }

    pub fn roll(&mut self, count: u8, sides: u8) -> i32 {
        (0..count)
            .map(|_| self.rng.gen_range(1..=sides as i32))
            .sum()
    }
}
