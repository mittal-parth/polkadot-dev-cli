use std::io::{self};
use std::process::Command;

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

// Function to run zepter lint features
pub fn run_lint_features() {
    // Ensure zepter is installed
    if let Err(e) = install_zepter() {
        eprintln!("Error installing zepter: {}", e);
        return;
    }

    // Run `zepter lint features`
    let status = Command::new("zepter").arg("format").arg("features").status();

    match status {
        Ok(status) if status.success() => {
            println!("Zepter lint features executed successfully.");
        }
        Ok(status) => {
            eprintln!("Zepter lint features failed with status: {}", status);
        }
        Err(e) => {
            eprintln!("Error running zepter lint features: {}", e);
        }
    }
}
