mod cli;
mod dice_game;
mod expected_values;
mod game;
mod unit_conversion;

use cli::parse_args;
use std::error::Error;
use tracing::{info, level_filters::LevelFilter};

fn main() -> Result<(), Box<dyn Error>> {
    let (num_sims, output, verbosity) = parse_args();

    setup_logging(verbosity);
    info!("verbosity level: {:?}", verbosity);

    if num_sims > 0 {
        let output_fp = dice_game::simulate_dice_games(num_sims, output.as_deref())?;
        println!("Output written to file: {}", output_fp);
    }

    Ok(())
}

fn setup_logging(verbosity: u8) {
    let tracing_level = match verbosity {
        0 => LevelFilter::OFF,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing_level)
        .init();
}
