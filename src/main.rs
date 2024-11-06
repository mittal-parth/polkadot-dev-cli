mod contribute;
mod format;
mod lint;

use clap::{Arg, Command};

fn main() {
    let cmd = Command::new("polkadot-dev-cli")
        .bin_name("polkadot-dev-cli")
        .version("1.0")
        .about("CLI tool for Polkadot developers")
        .subcommand_required(true)
        .subcommand(
            Command::new("help-contribute")
                .about("Show a checklist for contributing to the project"),
        )
        .subcommand(
            Command::new("format")
                .about("Format code using the correct Rust nightly version")
                .arg(
                    Arg::new("show-version")
                        .long("show-version")
                        .help("Displays the Rust nightly version to be used"),
                ),
        )
        .subcommand(
            Command::new("lint")
                .about("Lint features using zepter")
                .subcommand(
                    Command::new("features").about("Lint features layout and remove duplicates"),
                ),
        );

    // Get matches for the command-line arguments
    let matches = cmd.get_matches();

    // Handle the subcommands
    match matches.subcommand() {
        Some(("help-contribute", _)) => contribute::contribute_help(),
        Some(("format", sub_matches)) => format::run_format(sub_matches),
        Some(("lint", sub_matches)) => {
            if let Some(("features", _)) = sub_matches.subcommand() {
                lint::run_lint_features();
            }
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
