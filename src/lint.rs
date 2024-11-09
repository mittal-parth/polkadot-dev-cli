use clap::ArgMatches;
use std::io::{self};
use std::process::Command;

// Function to run zepter lint features
pub fn run_lint_features(fix: bool) {
    // Check if zepter is installed by running `zepter --version`
    if Command::new("zepter")
        .arg("--version")
        .output()
        .is_err()
    {
        // If `zepter --version` fails, attempt to install it
        if let Err(e) = install_zepter() {
            eprintln!("Error installing zepter: {}", e);
            return;
        }
    }

    // Prepare command
    let mut cmd = Command::new("zepter");
    cmd.arg("format").arg("features");

    if fix {
        cmd.arg("--fix");
    }

    // Run command
    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint features: {}", e);
    }
}

// Function to run zepter lint trace
pub fn run_lint_trace(fix: bool, sub_matches: &ArgMatches) {
    // Check if zepter is installed by running `zepter --version`
    if Command::new("zepter")
        .arg("--version")
        .output()
        .is_err()
    {
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
    if let Some(from) = sub_matches.get_one::<String>("from") {
        cmd.arg(from);

        if let Some(to) = sub_matches.get_one::<String>("to") {
            cmd.arg(to);
        }
    }

    if fix {
        cmd.arg("--fix");
    }

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
