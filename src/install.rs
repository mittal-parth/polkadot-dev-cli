
use crate::logged_command::LoggedCommand;

pub fn run_install() {
    let url = "https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/heads/master/scripts/getting-started.sh";

    LoggedCommand::new("bash")
        .arg("-c")
        .arg(format!("curl {} | bash", url))
        .status()
        .expect("Failed to install dependencies for polkadot-sdk development");
}
