use clap::ArgMatches;
use std::io::{self};
use std::process::Command;

pub fn run(
    quiet: bool,
    color: bool,
    exit_code_zero: bool,
    log: Option<&str>,
    fix_hint: Option<&str>,
) {
    if Command::new("zepter").arg("--version").output().is_err() {
        if let Err(e) = install_zepter() {
            eprintln!("Error installing zepter: {}", e);
            return;
        }
    }

    let mut cmd = Command::new("zepter");
    cmd.arg("run");

    add_optional_args(&mut cmd, quiet, color, exit_code_zero, log, fix_hint);

    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint features: {}", e);
    }
}

// Function to run zepter lint features
pub fn run_features(
    quiet: bool,
    color: bool,
    exit_code_zero: bool,
    log: Option<&str>,
    fix_hint: Option<&str>,
) {
    // Check if zepter is installed by running `zepter --version`
    if Command::new("zepter").arg("--version").output().is_err() {
        // If `zepter --version` fails, attempt to install it
        if let Err(e) = install_zepter() {
            eprintln!("Error installing zepter: {}", e);
            return;
        }
    }

    // Prepare command
    let mut cmd = Command::new("zepter");
    cmd.arg("format").arg("features");

    // Add common arguments
    add_optional_args(&mut cmd, quiet, color, exit_code_zero, log, fix_hint);

    // Run command
    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint features: {}", e);
    }
}

// Function to run zepter lint trace
pub fn run_trace(
    quiet: bool,
    color: bool,
    exit_code_zero: bool,
    log: Option<&str>,
    fix_hint: Option<&str>,
    sub_matches: &ArgMatches,
) {
    // Check if zepter is installed by running `zepter --version`
    if Command::new("zepter").arg("--version").output().is_err() {
        // If `zepter --version` fails, attempt to install it
        if let Err(e) = install_zepter() {
            eprintln!("Error installing zepter: {}", e);
            return;
        }
    }

    // Prepare command
    let mut cmd = Command::new("zepter");
    cmd.arg("trace");

    // Add the from and to arguments if they exist
    cmd.arg("trace")
        .arg(sub_matches.get_one::<String>("from").unwrap())
        .arg(sub_matches.get_one::<String>("to").unwrap());

    // Add common arguments
    add_optional_args(&mut cmd, quiet, color, exit_code_zero, log, fix_hint);

    match cmd.status() {
        Ok(status) if !status.success() => {
            eprintln!("zepter trace failed with status: {}", status);
        }
        Err(e) => {
            eprintln!("Error running zepter trace: {}", e);
        }
        Ok(_) => {}
    }
}

// Function to ensure zepter is installed
fn install_zepter() -> io::Result<()> {
    let status = Command::new("cargo")
        .arg("install")
        .arg("zepter")
        .arg("-f")
        .arg("--locked")
        .status()?;

    if status.success() {
        println!("zepter installed successfully.");
    } else {
        eprintln!("Failed to install zepter.");
    }

    Ok(())
}

// Helper function to add the flags and arguments common to all zepter commands
fn add_optional_args(
    cmd: &mut Command,
    quiet: bool,
    color: bool,
    exit_code_zero: bool,
    log: Option<&str>,
    fix_hint: Option<&str>,
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
}
