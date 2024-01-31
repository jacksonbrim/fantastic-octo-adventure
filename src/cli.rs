use clap::{Arg, ArgAction, ArgMatches, Command};
pub fn parse_args() -> (i64, Option<String>, u8, Option<(f32, String, String)>) {
    let matches = Command::new("Dice Game Simulations & Unit Conversion")
        .author("Jackson Brim")
        .version("0.1.0")
        .about("Dice Game Simulation & Unit Conversion")
        .arg(
            Arg::new("num-sims")
                .short('n')
                .long("num-sims")
                .value_parser(clap::value_parser!(i64))
                .help("The number of simulations to run."),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .action(ArgAction::Set)
                .help("Specify the output file for generated numbers"),
        )
        .arg(
            Arg::new("unit-conversion")
                .short('u')
                .long("unit-conversion")
                .action(ArgAction::SetTrue)
                .help("Perform a unit conversion, requires -v <value> -f <from> -t <to>"),
        )
        .arg(
            Arg::new("value")
                .index(1)
                .value_parser(clap::value_parser!(f32))
                .requires("unit-conversion")
                .help("The value to convert"),
        )
        .arg(
            Arg::new("from")
                .index(2)
                .value_parser(clap::builder::ValueParser::new(
                    clap::builder::PossibleValuesParser::new(["m", "in", "ft", "hr", "s"]),
                ))
                .requires("unit-conversion")
                .help("The unit to convert from ('m', 'in', 'ft', 'hr', 's')"),
        )
        .arg(
            Arg::new("to")
                .index(3)
                .value_parser(clap::builder::ValueParser::new(
                    clap::builder::PossibleValuesParser::new(["m", "in", "ft", "hr", "s"]),
                ))
                .requires("unit-conversion")
                .help("The unit to convert to ('m', 'in', 'ft', 'hr', 's')"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::Count)
                .help(
                    "Use the -v flag to set the verbosity level:
                OFF (default),
                -v => INFO,
                -vv => DEBUG,
                -vvv => TRACE",
                ),
        )
        .get_matches();

    let num_sims = *matches.get_one::<i64>("num-sims").unwrap_or(&0);
    let output = matches.get_one::<String>("output").cloned();
    let verbosity = *matches.get_one::<u8>("verbose").unwrap_or(&0);
    let unit_conversion_match = *matches.get_one::<bool>("unit-conversion").unwrap_or(&false);
    let unit_conversion = if unit_conversion_match {
        let value = *matches.get_one::<f32>("value").unwrap();
        let from = matches.get_one::<String>("from").cloned().unwrap();
        let to = matches.get_one::<String>("to").cloned().unwrap();
        Some((value, from, to))
    } else {
        None
    };

    (num_sims, output, verbosity, unit_conversion)
}
