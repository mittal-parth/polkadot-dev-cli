mod contribute;
mod format;
mod install;
mod flint;
mod psvm;

use clap::{Arg, Command};

fn main() {
    let cmd = Command::new("polkadot-dev")
        .bin_name("polkadot-dev")
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
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .help("No output printed to stdout")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Use verbose output")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("version")
                        .long("version")
                        .help("Print rustfmt version and exit")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("package")
                        .short('p')
                        .long("package")
                        .help("Specify Package to format")
                        .global(true),
                )
                .arg(
                    Arg::new("manifest-path")
                        .long("manifest-path")
                        .help("Specify path to the Cargo.toml file")
                        .global(true),
                )
                .arg(
                    Arg::new("message-format")
                        .long("message-format")
                        .help("Specify message-format: short|json|human")
                        .global(true),
                )
                .arg(
                    Arg::new("all")
                        .long("all")
                        .help("Format all packages, and also their local path-based dependencies")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("check")
                        .long("check")
                        .help("Run rustfmt in check mode")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
        )
        .subcommand(
            Command::new("flint")
                .about("Analyze, Fix and Lint features in your Rust workspace via Zepter")
                .visible_aliases(["feature-lint", "f-lint"])
                .arg(
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .help("Only print errors. Supersedes `--log`")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("log")
                        .long("log")
                        .help("Log level to use [default: info]")
                        .global(true),
                )
                .arg(
                    Arg::new("color")
                        .long("color")
                        .help("Use ANSI terminal colors")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("exit-code-zero")
                        .long("exit-code-zero")
                        .help("Try to exit with code zero if the intended check failed.")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("fix-hint")
                        .long("fix-hint")
                        .help("Dont print any hints on how to fix the error. [default: on] [possible values: on, off]")
                        .global(true)
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
        )
        .subcommand(
            Command::new("version")
            .about("Manage Polkadot SDK versions via psvm")
            .arg(
                Arg::new("list")
                    .short('l')
                    .long("list")
                    .help("List all available versions")
                    .global(true)
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("version")
                .short('v')
                .long("version")
                .help("Specifies the Polkadot SDK version")
            )
            .arg(
                Arg::new("path")
                    .short('p')
                    .long("path")
                    .help("Path to a crate folder or Cargo.toml file [default: Cargo.toml]")
                    .global(true)
            )
            .arg(
                Arg::new("overwrite")
                .short('o')
                .long("overwrite")
                .help("Overwrite local dependencies (using path) with same name as the ones in the Polkadot SDK")
                .global(true)
                .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("check")
                .short('c')
                .long("check")
                .help("Check if the dependencies versions match the Polkadot SDK version. Does not update the Cargo.toml")
                .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("orml")
                .short('O')
                .long("orml")
                .help("To either list available ORML version or update the Cargo.toml file with the corresponding ORML version")
                .action(clap::ArgAction::SetTrue),
            )
        )
        .subcommand(
            Command::new("install")
            .about("Install all the required dependencies for polkadot-sdk development")
        );

    // Get matches for the command-line arguments
    let matches = cmd.get_matches();

    // Handle the subcommands
    match matches.subcommand() {
        Some(("help-contribute", _)) => contribute::contribute_help(),
        Some(("install", _)) => install::run_install(),
        Some(("format", sub_matches)) => {
            let quiet = sub_matches.get_flag("quiet");
            let verbose = sub_matches.get_flag("verbose");
            let version = sub_matches.get_flag("version");
            let package = sub_matches.get_one::<String>("package").map(|s| s.as_str());
            let manifest_path = sub_matches
                .get_one::<String>("manifest-path")
                .map(|s| s.as_str());
            let message_format = sub_matches
                .get_one::<String>("message-format")
                .map(|s| s.as_str());
            let all = sub_matches.get_flag("all");
            let check = sub_matches.get_flag("check");

            format::run_format(
                quiet,
                verbose,
                version,
                package,
                manifest_path,
                message_format,
                all,
                check,
            );
        }
        Some(("flint", sub_matches)) => {
            let fix = sub_matches.get_flag("fix");
            let quiet = sub_matches.get_flag("quiet");
            let color = sub_matches.get_flag("color");
            let exit_code_zero = sub_matches.get_flag("exit-code-zero");
            let log = sub_matches.get_one::<String>("log").map(|s| s.as_str());
            let fix_hint = sub_matches
                .get_one::<String>("fix-hint")
                .map(|s| s.as_str());

            if sub_matches.subcommand_name().is_none() {
                flint::run(quiet, color, exit_code_zero, log, fix_hint);
            }
            match sub_matches.subcommand() {
                Some(("features", _)) => {
                    flint::run_features(quiet, color, exit_code_zero, log, fix_hint)
                }
                Some(("trace", trace_matches)) => flint::run_trace(
                    quiet,
                    color,
                    exit_code_zero,
                    log,
                    fix_hint,
                    trace_matches,
                ),
                _ => unreachable!("clap should ensure we don't get here"),
            }
        }
        Some(("version", sub_matches)) => {
            let list = sub_matches.get_flag("list");
            let path = sub_matches.get_one::<String>("path").map(|s| s.as_str());
            let version = sub_matches.get_one::<String>("version").map(|s| s.as_str());
            let overwrite = sub_matches.get_flag("overwrite");
            let check = sub_matches.get_flag("check");
            let orml = sub_matches.get_flag("orml");

            psvm::run_version(list, overwrite, check, orml, version, path)
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
