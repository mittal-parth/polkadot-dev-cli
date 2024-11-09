mod contribute;
mod format;
mod lint;
mod psvm;

use clap::{Arg, Command};

fn main() {
    let cmd = Command::new("polkadot-dev-cli")
        .bin_name("polkadot-dev-cli")
        .version("1.0")
        .about(
            "CLI tool for Polkadot developers bundling linting, formatting, and version management",
        )
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
                .about("Analyze, Fix and Lint features in your Rust workspace via Zepter")
                .arg(
                    Arg::new("fix")
                        .short('f')
                        .long("fix")
                        .help("Apply available fixes")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .subcommand(
                    Command::new("features").about("Lint features layout and remove duplicates"),
                )
                .subcommand(
                    Command::new("trace")
                        .about("Trace dependencies paths.")
                        .arg(Arg::new("from").help("From crate").required(true).index(1))
                        .arg(Arg::new("to").help("To crate").required(true).index(2)),
                ),
        );

    // Get matches for the command-line arguments
    let matches = cmd.get_matches();

    // Handle the subcommands
    match matches.subcommand() {
        Some(("help-contribute", _)) => contribute::contribute_help(),
        Some(("format", sub_matches)) => format::run_format(sub_matches),
        Some(("lint", sub_matches)) => {
            let fix = sub_matches.get_flag("fix");
            match sub_matches.subcommand() {
                Some(("features", _)) => lint::run_lint_features(fix),
                Some(("trace", trace_matches)) => lint::run_lint_trace(fix, trace_matches),
                _ => unreachable!("clap should ensure we don't get here"),
            }
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
