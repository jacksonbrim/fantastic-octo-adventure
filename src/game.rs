use crate::expected_values::ExpectedValues;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fmt;
use tracing::{debug, info};

#[derive(Debug)]
pub struct Game {
    seed: u64,
    num_sides: i32,
    num_turns: usize,
    rolls: usize,
    bankroll: i32,
    die_result: i32,
    expected_values: ExpectedValues,
    rng: StdRng,
}

impl Game {
    pub fn new(num_turns: usize, num_sides: i32, seed: u64) -> Self {
        let mut expected_values = ExpectedValues::new(num_turns, num_sides);
        expected_values.calculate();
        let rng = StdRng::seed_from_u64(seed);
        Self {
            seed,
            num_sides,
            num_turns,
            rolls: num_turns,
            bankroll: 0,
            die_result: 1,
            expected_values,
            rng,
        }
    }
    pub fn game_loop(&mut self) {
        while self.rolls > 0 {
            if self.should_roll() {
                self.roll();
            } else {
                self.take();
            }
            info!("strat1: {}", self);
        }
    }

    pub fn game_loop_twenty_only_strategy(&mut self) {
        while self.rolls > 0 {
            if self.die_result == 20 {
                self.take();
            } else {
                self.roll();
            }
            info!("strat2: {}", self);
        }
    }
    fn take_rest(&mut self) {
        while self.rolls > 0 {
            self.take();
        }
    }

    fn roll(&mut self) {
        self.rolls -= 1;
        self.die_result = self.rng.gen_range(1..=self.num_sides);
    }

    fn take(&mut self) {
        self.bankroll += self.die_result;
        self.rolls -= 1;
    }

    pub fn is_game_over(&self) -> bool {
        self.rolls == 0
    }

    fn should_roll(&self) -> bool {
        self.expected_values.get(self.rolls, self.die_result) > self.die_result as f32
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seed: {}, rolls: {}, bankroll: {}, die_result: {}, ev: {}, should_roll: {}",
            self.seed,
            self.rolls,
            self.bankroll,
            self.die_result,
            self.expected_values.get(self.rolls, self.die_result),
            self.should_roll(),
        )
    }
}
