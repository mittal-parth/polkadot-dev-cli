use std::process::Command;

pub fn run_install() {
    let url = "https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/heads/master/scripts/getting-started.sh";

    Command::new("bash")
        .arg("-c")
        .arg(format!("curl {} | bash", url))
        .status()
        .expect("Failed to install dependencies for polkadot-sdk development");
}
