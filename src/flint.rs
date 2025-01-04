// Module for handling the `flint` command and its subcommands
use crate::logged_command::LoggedCommand;
use std::io::{self};
use std::process::Command;

// Function to handle the main `flint` command
pub fn handle_flint_command(sub_matches: &clap::ArgMatches) {
    let quiet = sub_matches.get_flag("quiet");
    let color = sub_matches.get_flag("color");
    let exit_code_zero = sub_matches.get_flag("exit-code-zero");
    let log = sub_matches.get_one::<String>("log").map(|s| s.as_str());
    let fix_hint = sub_matches
        .get_one::<String>("fix-hint")
        .map(|s| s.as_str());
    let manifest_path = sub_matches
        .get_one::<String>("manifest-path")
        .map(|s| s.as_str());

    if Command::new("zepter").arg("--version").output().is_err() {
        // If `zepter --version` fails, attempt to install it
        if let Err(e) = install_zepter() {
            eprintln!("Error installing zepter: {}", e);
            return;
        }
    }

    let mut cmd = LoggedCommand::new("zepter");

    add_optional_args(
        &mut cmd,
        quiet,
        color,
        exit_code_zero,
        log,
        fix_hint,
        manifest_path,
    );

    // Default behavior if no subcommand is specified
    if sub_matches.subcommand_name().is_none() {
        handle_run(&mut cmd, sub_matches);
        return;
    }

    // Match each subcommand and call the appropriate function
    match sub_matches.subcommand() {
        Some(("format-features", sub_matches)) => handle_format_features(&mut cmd, sub_matches),
        Some(("trace", trace_matches)) => handle_trace_command(&mut cmd, trace_matches),
        Some(("lint", lint_matches)) => handle_lint_command(&mut cmd, lint_matches),
        Some(("debug", debug_matches)) => handle_debug_command(&mut cmd, debug_matches),
        Some(("transpose", transpose_matches)) => {
            handle_transpose_command(&mut cmd, transpose_matches)
        }
        _ => println!("Invalid command or subcommand combination. Please refer to the help by running `polkadot-dev-cli flint --help`"),
    }
}

pub fn handle_run(cmd: &mut LoggedCommand, sub_matches: &clap::ArgMatches) {
    cmd.arg("run");

    let config = sub_matches.get_one::<String>("config");
    let check_cfg_compatibility = sub_matches.get_one::<String>("check-cfg-compatibility");

    if let Some(config) = config {
        cmd.arg("--config").arg(config);
    }
    if let Some(check_cfg_compatibility) = check_cfg_compatibility {
        cmd.arg("--check-cfg-compatibility")
            .arg(check_cfg_compatibility);
    }

    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint features: {}", e);
    }
}

// Function to run zepter lint features
pub fn handle_format_features(cmd: &mut LoggedCommand, sub_matches: &clap::ArgMatches) {
    cmd.arg("format").arg("features");

    let no_workspace = sub_matches.get_flag("no-workspace");
    let modify_paths = sub_matches.get_one::<String>("only-enables");
    let line_width = sub_matches.get_one::<String>("line-width");
    let mode_per_feature = sub_matches.get_one::<String>("mode-per-feature");
    let ignore_feature = sub_matches.get_one::<String>("ignore-feature");
    let print_paths = sub_matches.get_flag("print-paths");

    if no_workspace {
        cmd.arg("--no-workspace");
    }
    if let Some(modify_paths) = modify_paths {
        cmd.arg("--modify-paths").arg(modify_paths);
    }
    if let Some(line_width) = line_width {
        cmd.arg("--line-width").arg(line_width);
    }
    if let Some(mode_per_feature) = mode_per_feature {
        cmd.arg("--mode-per-feature").arg(mode_per_feature);
    }
    if let Some(ignore_feature) = ignore_feature {
        cmd.arg("--ignore-feature").arg(ignore_feature);
    }
    if print_paths {
        cmd.arg("--print-paths");
    }

    add_optional_args_for_flint_except_run(cmd, sub_matches);

    // Run command
    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint features: {}", e);
    }
}

// Function to handle the `trace` subcommand
fn handle_trace_command(cmd: &mut LoggedCommand, trace_matches: &clap::ArgMatches) {
    cmd.arg("trace");

    // Add the from and to arguments if they exist
    cmd.arg("trace")
        .arg(trace_matches.get_one::<String>("from").unwrap())
        .arg(trace_matches.get_one::<String>("to").unwrap());

    add_optional_args_for_flint_except_run(cmd, trace_matches);

    let show_source = trace_matches.get_flag("show-source");
    let show_version = trace_matches.get_flag("show-version");
    let path_delimiter = trace_matches
        .get_one::<String>("path-delimiter")
        .map(|s| s.as_str());
    let unique_versions = trace_matches.get_flag("unique-versions");

    if show_source {
        cmd.arg("--show-source");
    }
    if show_version {
        cmd.arg("--show-version");
    }
    if let Some(path_delimiter) = path_delimiter {
        cmd.arg("--path-delimiter").arg(path_delimiter);
    }
    if unique_versions {
        cmd.arg("--unique-versions");
    }

    match cmd.status() {
        Err(e) => {
            eprintln!("Error running zepter trace: {}", e);
        }
        Ok(_) => {}
    }
}

// Function to handle the `lint` subcommand with nested subcommands
fn handle_lint_command(cmd: &mut LoggedCommand, lint_matches: &clap::ArgMatches) {
    cmd.arg("lint");

    add_optional_args_for_flint_except_run(cmd, lint_matches);

    // Handle lint subcommands
    match lint_matches.subcommand() {
        Some(("propagate-feature", propagate_feature_matches)) => {
            handle_propagate_feature(cmd, propagate_feature_matches);
        }
        Some(("never-enables", never_enables_matches)) => {
            handle_never_enables(cmd, never_enables_matches);
        }
        Some(("never-implies", never_implies_matches)) => {
            handle_never_implies(cmd, never_implies_matches);
        }
        Some(("only-enables", only_enables_matches)) => {
            handle_only_enables(cmd, only_enables_matches);
        }
        Some(("why-enabled", why_enabled_matches)) => {
            handle_why_enabled(cmd, why_enabled_matches);
        }
        _ => println!("Invalid lint subcommand. Please refer to the help by running `polkadot-dev-cli flint lint --help`"),
    }
}

fn handle_propagate_feature(cmd: &mut LoggedCommand, matches: &clap::ArgMatches) {
    let features = matches.get_one::<String>("features").expect("required");
    let packages = matches.get_one::<String>("packages");
    let features_enables_dep = matches.get_one::<String>("features-enables-dep");
    let left_side_feat_missing = matches.get_one::<String>("left-side-feat-missing");
    let ignore_missing_propagate = matches.get_one::<String>("ignore-missing-propagate");
    let dep_kinds = matches.get_one::<String>("dep-kinds");
    let show_version = matches.get_flag("show-version");
    let show_path = matches.get_flag("show-path");
    let fix = matches.get_flag("fix");
    let modify_paths = matches.get_one::<String>("modify-paths");
    let fix_dependency = matches.get_one::<String>("fix-dependency");
    let fix_package = matches.get_one::<String>("fix-package");

    cmd.arg("propagate-feature");

    // Add the required arguments
    cmd.arg("--features").arg(features);

    // Add optional arguments
    if let Some(packages) = packages {
        cmd.arg("--packages").arg(packages);
    }
    if let Some(features_enables_dep) = features_enables_dep {
        cmd.arg("--features-enables-dep").arg(features_enables_dep);
    }
    if let Some(left_side_feat_missing) = left_side_feat_missing {
        cmd.arg("--left-side-feat-missing")
            .arg(left_side_feat_missing);
    }
    if let Some(ignore_missing_propagate) = ignore_missing_propagate {
        cmd.arg("--ignore-missing-propagate")
            .arg(ignore_missing_propagate);
    }
    if let Some(dep_kinds) = dep_kinds {
        cmd.arg("--dep-kinds").arg(dep_kinds);
    }
    if show_version {
        cmd.arg("--show-version");
    }
    if show_path {
        cmd.arg("--show-path");
    }
    if fix {
        cmd.arg("--fix");
    }
    if let Some(modify_paths) = modify_paths {
        cmd.arg("--modify-paths").arg(modify_paths);
    }
    if let Some(fix_dependency) = fix_dependency {
        cmd.arg("--fix-dependency").arg(fix_dependency);
    }
    if let Some(fix_package) = fix_package {
        cmd.arg("--fix-package").arg(fix_package);
    }

    // Run command
    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint propagate-feature: {}", e);
    }
}

// Nested subcommand handling for `never-enables`
fn handle_never_enables(cmd: &mut LoggedCommand, matches: &clap::ArgMatches) {
    let precondition = matches.get_one::<String>("precondition").expect("required");
    let stays_disabled = matches
        .get_one::<String>("stays-disabled")
        .expect("required");

    cmd.arg("never-enables");

    // Add the precondition and stays-disabled arguments
    cmd.arg("--precondition").arg(precondition);
    cmd.arg("--stays-disabled").arg(stays_disabled);

    // Run command
    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint never-enables: {}", e);
    }
}

// Nested subcommand handling for `never-implies`
fn handle_never_implies(cmd: &mut LoggedCommand, matches: &clap::ArgMatches) {
    let precondition = matches.get_one::<String>("precondition").expect("required");
    let stays_disabled = matches
        .get_one::<String>("stays-disabled")
        .expect("required");
    let show_source = matches.get_flag("show-source");
    let show_version = matches.get_flag("show-version");
    let path_delimiter = matches
        .get_one::<String>("path-delimiter")
        .map(|s| s.as_str());

    cmd.arg("never-implies");

    cmd.arg("--precondition").arg(precondition);
    cmd.arg("--stays-disabled").arg(stays_disabled);

    // Add optional flags
    if show_source {
        cmd.arg("--show-source");
    }
    if show_version {
        cmd.arg("--show-version");
    }
    if let Some(path_delimiter) = path_delimiter {
        cmd.arg("--path-delimiter").arg(path_delimiter);
    }

    // Run command
    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint never-implies: {}", e);
    }
}

fn handle_only_enables(cmd: &mut LoggedCommand, matches: &clap::ArgMatches) {
    let precondition = matches.get_one::<String>("precondition").expect("required");
    let only_enables = matches.get_one::<String>("only-enables").expect("required");
    let path_delimiter = matches
        .get_one::<String>("path-delimiter")
        .map(|s| s.as_str());

    cmd.arg("only-enables");

    cmd.arg("--precondition").arg(precondition);
    cmd.arg("--only-enables").arg(only_enables);

    if let Some(path_delimiter) = path_delimiter {
        cmd.arg("--path-delimiter").arg(path_delimiter);
    }

    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint only-enables: {}", e);
    }
}

fn handle_why_enabled(cmd: &mut LoggedCommand, matches: &clap::ArgMatches) {
    let package = matches.get_one::<String>("package").expect("required");
    let feature = matches.get_one::<String>("feature").expect("required");

    cmd.arg("why-enabled");

    cmd.arg("--package").arg(package);
    cmd.arg("--feature").arg(feature);

    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint why-enabled: {}", e);
    }
}

fn handle_debug_command(cmd: &mut LoggedCommand, debug_matches: &clap::ArgMatches) {
    cmd.arg("debug");

    add_optional_args_for_flint_except_run(cmd, debug_matches);

    let no_benchmark = debug_matches.get_flag("no-benchmark");
    let no_root = debug_matches.get_flag("no-root");

    if no_benchmark {
        cmd.arg("--no-benchmark");
    }
    if no_root {
        cmd.arg("--no-root");
    }

    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter debug: {}", e);
    }
}

fn handle_transpose_command(cmd: &mut LoggedCommand, transpose_matches: &clap::ArgMatches) {
    cmd.arg("transpose");

    add_optional_args_for_flint_except_run(cmd, transpose_matches);

    match transpose_matches.subcommand() {
        Some(("dependency", dependency_matches)) => match dependency_matches.subcommand() {
            Some(("lift-to-workspace", sub_matches)) => {
                cmd.arg("dependency").arg("lift-to-workspace");

                let fix = sub_matches.get_flag("fix");
                let version_selector = sub_matches.get_one::<String>("version-selector");
                let skip_package = sub_matches.get_one::<String>("skip-package");
                let source_location = sub_matches.get_one::<String>("source-location");
                let exact_version = sub_matches.get_flag("exact-version");
                let ignore_errors = sub_matches.get_flag("ignore-errors");

                if fix {
                    cmd.arg("--fix");
                }
                if let Some(version_selector) = version_selector {
                    cmd.arg("--version-selector").arg(version_selector);
                }
                if let Some(skip_package) = skip_package {
                    cmd.arg("--skip-package").arg(skip_package);
                }
                if let Some(source_location) = source_location {
                    cmd.arg("--source-location").arg(source_location);
                }
                if exact_version {
                    cmd.arg("--exact-version");
                }
                if ignore_errors {
                    cmd.arg("--ignore-errors");
                }

                if let Err(e) = cmd.status() {
                    eprintln!(
                        "Error running zepter transpose dependency lift-to-workspace: {}",
                        e
                    );
                }
            }
            _ => println!("Invalid subcommand for zepter transpose dependency. Please refer to the help by running `polkadot-dev-cli flint transpose dependency --help`"),
        },
        _ => println!("Invalid subcommand for zepter transpose. Please refer to the help by running `polkadot-dev-cli flint transpose --help`"),
    }
    // Run command
    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter transpose: {}", e);
    }
}

// Helper function to ensure zepter is installed
fn install_zepter() -> io::Result<()> {
    LoggedCommand::new("cargo")
        .arg("install")
        .arg("zepter")
        .arg("-f")
        .arg("--locked")
        .status()?;

    Ok(())
}

// Helper function to add the flags and arguments common to all zepter commands
fn add_optional_args(
    cmd: &mut LoggedCommand,
    quiet: bool,
    color: bool,
    exit_code_zero: bool,
    log: Option<&str>,
    fix_hint: Option<&str>,
    manifest_path: Option<&str>,
) {
    // Add flags
    if quiet {
        cmd.arg("--quiet");
    }
    if color {
        cmd.arg("--color");
    }
    if exit_code_zero {
        cmd.arg("--exit-code-zero");
    }

    // Add arguments with values
    if let Some(log_level) = log {
        cmd.arg("--log").arg(log_level);
    }
    if let Some(hint) = fix_hint {
        cmd.arg("--fix-hint").arg(hint);
    }
    if let Some(path) = manifest_path {
        cmd.arg("--manifest-path").arg(path);
    }
}

fn add_optional_args_for_flint_except_run(cmd: &mut LoggedCommand, sub_matches: &clap::ArgMatches) {
    let workspace = sub_matches.get_flag("workspace");
    let offline = sub_matches.get_flag("offline");
    let locked = sub_matches.get_flag("locked");
    let all_features = sub_matches.get_flag("all-features");

    if workspace {
        cmd.arg("--workspace");
    }
    if offline {
        cmd.arg("--offline");
    }
    if locked {
        cmd.arg("--locked");
    }
    if all_features {
        cmd.arg("--all-features");
    }
}
