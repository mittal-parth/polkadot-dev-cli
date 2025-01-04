use crate::logged_command::LoggedCommand;
use std::process::Command;

pub fn handle_prdoc_command(sub_matches: &clap::ArgMatches) {
    let config = sub_matches.get_one::<String>("config");
    let prdoc_folders = sub_matches.get_one::<String>("prdoc-folders");
    let version = sub_matches.get_flag("version");
    let json = sub_matches.get_flag("json");

    // Ensure prdoc is installed
    if Command::new("prdoc").arg("--version").status().is_err() {
        install_prdoc().unwrap();
    }

    let mut cmd = LoggedCommand::new("prdoc");

    if let Some(config) = config {
        cmd.arg("--config").arg(config);
    }
    if let Some(prdoc_folders) = prdoc_folders {
        cmd.arg("--prdoc-folders").arg(prdoc_folders);
    }
    if version {
        cmd.arg("--version");
    }
    if json {
        cmd.arg("--json");
    }

    // Match each subcommand and call the appropriate function
    match sub_matches.subcommand() {
        Some(("generate", generate_matches)) => handle_generate(&mut cmd, generate_matches),
        Some(("check", check_matches)) => handle_check(&mut cmd, check_matches),
        Some(("scan", scan_matches)) => handle_scan(&mut cmd, scan_matches),
        Some(("load", load_matches)) => handle_load(&mut cmd, load_matches),
        _ => println!("Invalid subcommand. Please refer to the help by running `polkadot-dev-cli prdoc --help`."),
    }
}

fn handle_generate(cmd: &mut LoggedCommand, generate_matches: &clap::ArgMatches) {
    cmd.arg("generate");

    let number = generate_matches
        .get_one::<String>("number")
        .expect("required");
    let dry_run = generate_matches.get_flag("dry-run");
    let output_dir = generate_matches.get_one::<String>("output-dir");

    cmd.arg(number);

    if dry_run {
        cmd.arg("--dry-run");
    }
    if let Some(output_dir) = output_dir {
        cmd.arg("--output-dir").arg(output_dir);
    }

    if let Err(e) = cmd.status() {
        eprintln!("Error running prdoc generate: {}", e);
    }
}

fn handle_check(cmd: &mut LoggedCommand, check_matches: &clap::ArgMatches) {
    cmd.arg("check");

    let file = check_matches.get_one::<String>("file");
    let number = check_matches.get_one::<String>("number");
    let list = check_matches.get_one::<String>("list");
    let schema = check_matches.get_one::<String>("schema");

    if let Some(file) = file {
        cmd.arg("--file").arg(file);
    }
    if let Some(number) = number {
        cmd.arg("--number").arg(number);
    }
    if let Some(list) = list {
        cmd.arg("--list").arg(list);
    }
    if let Some(schema) = schema {
        cmd.arg("--schema").arg(schema);
    }

    if let Err(e) = cmd.status() {
        eprintln!("Error running prdoc check: {}", e);
    }
}

fn handle_scan(cmd: &mut LoggedCommand, scan_matches: &clap::ArgMatches) {
    cmd.arg("scan");

    let all = scan_matches.get_flag("all");
    let sort = scan_matches.get_flag("sort");

    if all {
        cmd.arg("--all");
    }
    if sort {
        cmd.arg("--sort");
    }

    if let Err(e) = cmd.status() {
        eprintln!("Error running prdoc scan: {}", e);
    }
}

fn handle_load(cmd: &mut LoggedCommand, load_matches: &clap::ArgMatches) {
    cmd.arg("load");

    let file = load_matches.get_one::<String>("file");
    let number = load_matches.get_one::<String>("number");
    let list = load_matches.get_one::<String>("list");

    if let Some(file) = file {
        cmd.arg("--file").arg(file);
    }
    if let Some(number) = number {
        cmd.arg("--number").arg(number);
    }
    if let Some(list) = list {
        cmd.arg("--list").arg(list);
    }

    if let Err(e) = cmd.status() {
        eprintln!("Error running prdoc load: {}", e);
    }
}

fn install_prdoc() -> std::io::Result<()> {
    let status = Command::new("cargo")
        .arg("install")
        .arg("parity-prdoc")
        .status()?;

    if status.success() {
        println!("prdoc installed successfully.");
    } else {
        eprintln!("Failed to install prdoc.");
    }

    Ok(())
}
