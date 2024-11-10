use std::io::{self};
use std::process::Command;

// Run `psvm` with the specified version and options
pub fn run_version(
    list: bool,
    overwrite: bool,
    check: bool,
    orml: bool,
    version: Option<&str>,
    path: Option<&str>,
) {
    // Ensure psvm is installed
    if Command::new("psvm").arg("--help").output().is_err() {
        // if `psvm --help` fails, attempt to install it
        if let Err(e) = install_psvm() {
            eprintln!("Error installing psvm: {}", e);
            return;
        }
    }

    // Prepare `psvm` command
    let mut cmd = Command::new("psvm");

    // Add arguments to the command
    add_optional_args(&mut cmd, list, overwrite, check, orml, version, path);

    // Run the command and capture the output
    if let Err(e) = cmd.status() {
        eprintln!("Error running version command: {}", e);
    }
}

// Helper method to install psvm
fn install_psvm() -> io::Result<()> {
    let status = Command::new("cargo").arg("install").arg("psvm").status()?;

    if status.success() {
        println!("psvm installed successfully.");
    } else {
        eprintln!("Failed to install psvm.");
    }

    Ok(())
}

fn add_optional_args(
    cmd: &mut Command,
    list: bool,
    overwrite: bool,
    check: bool,
    orml: bool,
    version: Option<&str>,
    path: Option<&str>,
) {
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
}
