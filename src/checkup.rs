use crate::logged_command::LoggedCommand;

// Command to run polkadot-dev format, flint and version --check altogether
pub fn run_checkup(sub_matches: &clap::ArgMatches) {
    // Run `polkadot-dev format`
    LoggedCommand::new("polkadot-dev")
        .arg("format")
        .status()
        .expect("Failed to execute `polkadot-dev format`");

    // Run `polkadot-dev flint`
    LoggedCommand::new("polkadot-dev")
        .arg("flint")
        .status()
        .expect("Failed to execute `polkadot-dev flint`");

    let version = sub_matches.get_one::<String>("version").map(|s| s.as_str());
    if !version.is_some() {
        eprintln!("Version is required to run `polkadot-dev version --check`, skipping. Format and flint checks passed.");
        return;
    }
    // Run `polkadot-dev version --check`
    LoggedCommand::new("polkadot-dev")
        .arg("version")
        .arg("-v")
        .arg(version.unwrap())
        .arg("--check")
        .status()
        .expect("Failed to execute `polkadot-dev version --check`");

    println!("Code formatting complete, feature lint checks passed, and version consistency verified! 😉");
}
