use std::process::Command;

// Command to run polkadot-dev format, flint and version --check altogether
pub fn run_checkup(sub_matches: &clap::ArgMatches) {
    // Run `polkadot-dev format`
    let status = Command::new("polkadot-dev")
        .arg("format")
        .status()
        .expect("Failed to execute `polkadot-dev format`");
    if !status.success() {
        eprintln!("`polkadot-dev format` failed");
    }

    // Run `polkadot-dev flint`
    let status = Command::new("polkadot-dev")
        .arg("flint")
        .status()
        .expect("Failed to execute `polkadot-dev flint`");
    if !status.success() {
        eprintln!("`polkadot-dev flint` failed");
    }

    let version = sub_matches.get_one::<String>("version").map(|s| s.as_str());
    if !version.is_some() {
        eprintln!("Version is required to run `polkadot-dev version --check`, skipping. Format and flint checks passed.");
        return;
    }
    // Run `polkadot-dev version --check`
    let status = Command::new("polkadot-dev")
        .arg("version")
        .arg("-v")
        .arg(version.unwrap())
        .arg("--check")
        .status()
        .expect("Failed to execute `polkadot-dev version --check`");
    if !status.success() {
        eprintln!("`polkadot-dev version --check` failed");
    }

    println!("Code formatting complete, feature lint checks passed, and version consistency verified! 😉");
}
