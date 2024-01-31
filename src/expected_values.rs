use std::fmt;
#[derive(Debug)]
pub struct ExpectedValues {
    values: Vec<Vec<f32>>,
}

impl ExpectedValues {
    pub fn new(num_turns: usize, num_sides: i32) -> Self {
        ExpectedValues {
            values: vec![vec![0.0; num_sides as usize + 1]; num_turns + 1],
        }
    }

    pub fn calculate(&mut self) {
        let num_sides = self.values[0].len() as i32 - 1; // Assuming num_sides is the length of the inner vector - 1
        for turn in (0..self.values.len() - 1).rev() {
            for die_value in 1..=num_sides {
                let ev_reroll: f32 = (1..=num_sides)
                    .map(|v| self.values[turn + 1][v as usize])
                    .sum::<f32>()
                    / num_sides as f32;
                self.values[turn][die_value as usize] = ev_reroll.max(die_value as f32);
            }
        }
    }

    // Method to get the expected value for a given turn and die value
    pub fn get(&self, turn: usize, die_value: i32) -> f32 {
        self.values[turn][die_value as usize]
    }
}

impl fmt::Display for ExpectedValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (turn, row) in self.values.iter().enumerate() {
            writeln!(f, "Turn {}: {:?}", turn, row)?;
        }
        Ok(())
    }
}
