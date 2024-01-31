use chrono::Local;
use rand::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fmt;
use std::io::{BufWriter, Result, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fs, fs::File};
use tracing::{debug, info};

const NUM_SIDES: i32 = 20;
const NUM_TURNS: usize = 100;

pub fn simulate_dice_games(num_sims: i64, path_name: Option<&str>) -> Result<String> {
    let now = Local::now();
    let original_path_name = path_name.map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(format!(
            "./output/{}_simulations_{}.txt",
            num_sims,
            now.format("%Y-%m-%d_%H-%M-%S")
        ))
    });

    let f = File::create(&original_path_name)?;
    let mut writer = BufWriter::new(f);

    let running = Arc::new(AtomicBool::new(false));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    println!("Running...");
    let mut rng = rand::thread_rng();
    let mut strat1_wins = 0;
    let mut strat1_sum = 0;
    let mut strat2_wins = 0;
    let mut strat2_sum = 0;
    let mut num_sims = num_sims;
    for i in 1..=num_sims {
        if running.load(Ordering::SeqCst) {
            println!("Exit command received. Finishing up...");
            num_sims = i - 1;
            break;
        }
        let seed = rng.gen();
        let mut strat1 = Game::new(NUM_TURNS, NUM_SIDES, seed);
        let mut strat2 = Game::new(NUM_TURNS, NUM_SIDES, seed);

        strat1.game_loop();
        strat2.game_loop_twenty_only_strategy();
        // write output to file
        write!(
            writer,
            "Seed: {}, Final Bankrolls: {}, {}\n",
            seed, strat1.bankroll, strat2.bankroll
        )?;
        if strat1.bankroll > strat2.bankroll {
            info!(
                "strat1 > strat2 - Seed: {}, strat1 Bankroll: {}, strat2 Bankroll: {}",
                seed, strat1.bankroll, strat2.bankroll
            );

            strat1_wins += 1;
        } else if strat1.bankroll < strat2.bankroll {
            info!(
                "strat1 < strat2 - Seed: {}, strat1 Bankroll: {}, strat2 Bankroll: {}",
                seed, strat1.bankroll, strat2.bankroll
            );
            strat2_wins += 1;
        }
        strat1_sum += strat1.bankroll;
        strat2_sum += strat2.bankroll;
    }

    let strat1_avg = strat1_sum as i64 / num_sims;
    let strat2_avg = strat2_sum as i64 / num_sims;

    println!(
        "Results: {} simulations, {} strat1 wins, {} strat2 wins, {} equal outcomes",
        num_sims,
        strat1_wins,
        strat2_wins,
        num_sims - strat2_wins - strat1_wins
    );
    println!("Strat1 avg bankroll: {}", strat1_avg);
    println!("Strat2 avg bankroll: {}", strat2_avg);
    // Close the writer to ensure all data is written
    writer.flush()?;
    drop(writer);

    // change file name if the process was interrupted
    let final_path_name = if running.load(Ordering::SeqCst) {
        debug!("Process was interrupted: renaming file path to reflect updated num_sims, run.");
        let mut new_path = original_path_name.clone();
        new_path.set_file_name(format!(
            "{}_early_exit_{}_simulations_completed",
            new_path.file_stem().unwrap().to_str().unwrap(),
            num_sims
        ));
        new_path.set_extension(
            original_path_name
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap(),
        );
        fs::rename(&original_path_name, &new_path)?;
        new_path
    } else {
        original_path_name
    };
    // Rename the file if needed
    let output_fp: String = final_path_name.to_string_lossy().into_owned();
    println!("Results saved to : {}", &output_fp);

    Ok(output_fp)
}

#[derive(Debug)]
struct Game {
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
    fn new(num_turns: usize, num_sides: i32, seed: u64) -> Self {
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
    fn game_loop(&mut self) {
        while self.rolls > 0 {
            if self.should_roll() {
                self.roll();
            } else {
                self.take();
            }
            info!("strat1: {}", self);
        }
    }

    fn game_loop_twenty_only_strategy(&mut self) {
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

    fn is_game_over(&self) -> bool {
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
#[derive(Debug)]
struct ExpectedValues {
    values: Vec<Vec<f32>>,
}

impl ExpectedValues {
    fn new(num_turns: usize, num_sides: i32) -> Self {
        ExpectedValues {
            values: vec![vec![0.0; num_sides as usize + 1]; num_turns + 1],
        }
    }

    fn calculate(&mut self) {
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
    fn get(&self, turn: usize, die_value: i32) -> f32 {
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
