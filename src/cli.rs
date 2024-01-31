use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn parse_args() -> (i64, Option<String>, u8) {
    let matches = Command::new("Dice Game Simulations")
        .author("Jackson Brim")
        .version("0.1.0")
        .about("Dice Game Simulations")
        .arg(
            Arg::new("num-sims")
                .short('n')
                .long("num-sims")
                .required(true)
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

    (num_sims, output, verbosity)
}
