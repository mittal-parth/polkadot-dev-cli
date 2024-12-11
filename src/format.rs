use crate::logged_command::LoggedCommand;

pub fn run_format(sub_matches: &clap::ArgMatches) {
    let mut cmd = LoggedCommand::new("cargo");

    cmd.arg("+nightly").arg("fmt");

    // Add optional arguments to the command
    let quiet = sub_matches.get_flag("quiet");
    let verbose = sub_matches.get_flag("verbose");
    let version = sub_matches.get_flag("version");
    let package = sub_matches.get_one::<String>("package").map(|s| s.as_str());
    let manifest_path = sub_matches
        .get_one::<String>("manifest-path")
        .map(|s| s.as_str());
    let message_format = sub_matches
        .get_one::<String>("message-format")
        .map(|s| s.as_str());
    let all = sub_matches.get_flag("all");
    let check = sub_matches.get_flag("check");

    if quiet {
        cmd.arg("--quiet");
    }

    if verbose {
        cmd.arg("--verbose");
    }

    if version {
        cmd.arg("--version");
    }

    if let Some(package) = package {
        cmd.arg("--package").arg(package);
    }

    if let Some(manifest_path) = manifest_path {
        cmd.arg("--manifest-path").arg(manifest_path);
    }

    if let Some(message_format) = message_format {
        cmd.arg("--message-format").arg(message_format);
    }
    if all {
        cmd.arg("--all");
    }
    if check {
        cmd.arg("--check");
    }

    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint features: {}", e);
    }
}
