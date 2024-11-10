use std::process::Command;

pub fn run_format(
    quiet: bool,
    verbose: bool,
    version: bool,
    package: Option<&str>,
    manifest_path: Option<&str>,
    message_format: Option<&str>,
    all: bool,
    check: bool,
) {
    let mut cmd = Command::new("cargo");

    cmd.arg("+nightly").arg("fmt");

    add_optional_args(
        &mut cmd,
        quiet,
        verbose,
        version,
        package,
        manifest_path,
        message_format,
        all,
        check,
    );

    if let Err(e) = cmd.status() {
        eprintln!("Error running zepter lint features: {}", e);
    }
}

fn add_optional_args(
    cmd: &mut Command,
    quiet: bool,
    verbose: bool,
    version: bool,
    package: Option<&str>,
    manifest_path: Option<&str>,
    message_format: Option<&str>,
    all: bool,
    check: bool,
) {
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
}
