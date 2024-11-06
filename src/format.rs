// src/format.rs

use clap::ArgMatches;
use std::process::Command as ProcessCommand;

pub fn run_format(sub_matches: &ArgMatches) {
    // Version of Rust nightly to use
    let rust_version = "nightly-2024-04-14";

    // If the user requests the version, show it
    if sub_matches.contains_id("show-version") {
        println!("Using Rust version: {}", rust_version);
        return;
    }

    // Run the command with the specific version of Rust
    let status = ProcessCommand::new("cargo")
        .arg(format!("+{}", rust_version)) // Specify the nightly version
        .arg("fmt") // Run cargo fmt
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("Code formatted successfully!");
        }
        Ok(status) => {
            eprintln!(
                "Failed to format code. Cargo fmt returned non-zero status: {}",
                status
            );
        }
        Err(e) => {
            eprintln!("Error executing cargo fmt: {}", e);
        }
    }
}
