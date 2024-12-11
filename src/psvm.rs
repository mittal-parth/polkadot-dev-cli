use crate::logged_command::LoggedCommand;
use std::io::{self};
use std::process::Command;

// Run `psvm` with the specified version and options
pub fn handle_version_command(sub_matches: &clap::ArgMatches) {
    // Ensure psvm is installed
    if Command::new("psvm").arg("--help").output().is_err() {
        // if `psvm --help` fails, attempt to install it
        if let Err(e) = install_psvm() {
            eprintln!("Error installing psvm: {}", e);
            return;
        }
    }

    // Prepare `psvm` command
    let mut cmd = LoggedCommand::new("psvm");

    // Add optional arguments to the command
    let list = sub_matches.get_flag("list");
    let path = sub_matches.get_one::<String>("path").map(|s| s.as_str());
    let version = sub_matches.get_one::<String>("version").map(|s| s.as_str());
    let overwrite = sub_matches.get_flag("overwrite");
    let check = sub_matches.get_flag("check");
    let orml = sub_matches.get_flag("orml");

    if list {
        cmd.arg("--list");
    }

    if let Some(path) = path {
        cmd.arg("--path").arg(path);
    }

    if let Some(version) = version {
        cmd.arg("--version").arg(version);
    }

    if overwrite {
        cmd.arg("--overwrite");
    }

    if check {
        cmd.arg("--check");
    }

    if orml {
        cmd.arg("--orml");
    }

    // Run the command and capture the output
    if let Err(e) = cmd.status() {
        eprintln!("Error running version command: {}", e);
    }
}

// Helper method to install psvm
fn install_psvm() -> io::Result<()> {
    LoggedCommand::new("cargo")
        .arg("install")
        .arg("psvm")
        .status()?;

    Ok(())
}
