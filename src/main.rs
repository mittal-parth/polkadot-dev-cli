mod contribute;
mod flint;
mod format;
mod install;
mod prdoc;
mod psvm;

use clap::{Arg, Command};

fn main() {
    let cmd = Command::new("polkadot-dev")
        .bin_name("polkadot-dev")
        .version("1.0")
        .about(
            "CLI tool for Polkadot developers bundling linting, formatting, and version management",
        )
        .subcommand_required(true)
        // `help-contribute` command to show a checklist for contributing to the project
        .subcommand(
            Command::new("help-contribute")
                .about("Show a checklist for contributing to the project"),
        )
        // `format` command to format code using the correct Rust nightly version
        .subcommand(
            Command::new("format")
                .about("Format code using the correct Rust nightly version")
                .arg(
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .help("No output printed to stdout")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Use verbose output")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("version")
                        .long("version")
                        .help("Print rustfmt version and exit")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("package")
                        .short('p')
                        .long("package")
                        .help("Specify Package to format")
                        .global(true),
                )
                .arg(
                    Arg::new("manifest-path")
                        .long("manifest-path")
                        .help("Specify path to the Cargo.toml file")
                        .global(true),
                )
                .arg(
                    Arg::new("message-format")
                        .long("message-format")
                        .help("Specify message-format: short|json|human")
                        .global(true),
                )
                .arg(
                    Arg::new("all")
                        .long("all")
                        .help("Format all packages, and also their local path-based dependencies")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("check")
                        .long("check")
                        .help("Run rustfmt in check mode")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
        )
        // `flint` command to analyze, fix and lint features in your Rust workspace via Zepter
        .subcommand(
            Command::new("flint")
                .about("Analyze, Fix and Lint features in your Rust workspace via Zepter")
                .visible_aliases(["feature-lint", "f-lint"])
                .arg(
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .help("Only print errors. Supersedes `--log`")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("log")
                        .long("log")
                        .help("Log level to use [default: info]")
                        .global(true),
                )
                .arg(
                    Arg::new("color")
                        .long("color")
                        .help("Use ANSI terminal colors")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("exit-code-zero")
                        .long("exit-code-zero")
                        .help("Try to exit with code zero if the intended check failed.")
                        .global(true)
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("fix-hint")
                        .long("fix-hint")
                        .help("Don't print any hints on how to fix the error. [default: on] [possible values: on, off]")
                        .global(true)
                )
                .arg(
                    Arg::new("manifest-path")
                        .long("manifest-path")
                        .help("Manually set the location of the manifest file. Must point directly to a file and not a directory.")
                        .global(true),
                )
                // `run` command
                .subcommand(
                    Command::new("run")
                        .about("Run a workflow from the config file. Uses `default` if none is specified.")
                        .arg(
                            Arg::new("config")
                                .long("config")
                                .short('c')
                                .help("Path to configuration file to use"),
                        )
                        .arg(
                            Arg::new("check-cfg-compatibility")
                                .long("check-cfg-compatibility")
                                .help("Whether to check if the config file is compatible with the current version of Zepter | [on,off]")
                        ),
                )
                // `format-features` command
                .subcommand(
                    Command::new("format-features")
                    .about("Format features layout and remove duplicates")
                    .visible_aliases(["ff"])
                    .arg(
                        Arg::new("workspace")
                            .long("workspace")
                            .help("Whether to only consider workspace crates")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("offline")
                            .long("offline")
                            .help("Whether to use offline mode")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("locked")
                            .long("locked")
                            .help("Whether to use all the locked dependencies from the `Cargo.lock`. 
                                    Otherwise it may update some dependencies. 
                                    For CI usage its a good idea to use it.")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("all-features")
                            .long("all-features")
                            .help("Modify all features")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("no-workspace")
                            .long("no-workspace")
                            .help("Include dependencies in the formatting check. They will not be modified, unless their path is included in `--modify-paths`.")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("modify-paths")
                            .long("modify-paths")
                            .help("Paths that are allowed to be modified by the formatter"),
                    )
                    .arg(
                        Arg::new("line-width")
                            .long("line-width")
                            .help("The maximal length of a line for a feature [default: 80]"),
                    )
                    .arg(
                        Arg::new("mode-per-feature")
                            .long("mode-per-feature")
                            .help("Set the formatting mode for a specific feature.
                                    Can be specified multiple times. Example:
                                    `--mode-per-feature default:sort,default:canonicalize`"),
                    )
                    .arg(
                        Arg::new("ignore-feature")
                            .long("ignore-feature")
                            .help(" Ignore a specific feature across all crates.
                                    This is equivalent to `--mode-per-feature FEATURE:none`"),
                    )
                    .arg(
                        Arg::new("print-paths")
                            .long("print-paths")
                            .help("Also print the paths of the offending Cargo.toml files")
                            .action(clap::ArgAction::SetTrue),
                    )
                    ,
                )
                // `trace` command
                .subcommand(
                    Command::new("trace")
                        .about("Trace dependencies paths.")
                        .arg(Arg::new("from").help("From crate").required(true).index(1))
                        .arg(Arg::new("to").help("To crate").required(true).index(2))
                        .arg(
                            Arg::new("workspace")
                                .long("workspace")
                                .help("Whether to only consider workspace crates")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("offline")
                                .long("offline")
                                .help("Whether to use offline mode")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("locked")
                                .long("locked")
                                .help("Whether to use all the locked dependencies from the `Cargo.lock`. 
                                        Otherwise it may update some dependencies. 
                                        For CI usage its a good idea to use it.")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("all-features")
                                .long("all-features")
                                .help("Modify all features")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("show-source")
                                .long("show-source")
                                .help("Show the source location of crates in the output")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("show-version")
                                .long("show-version")
                                .help("Show the version of crates in the output")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("path-delimiter")
                                .long("path-delimiter")
                                .help("Delimiter for rendering dependency paths [ default: ' -> ' ]"),
                        )
                        .arg(
                            Arg::new("unique-versions")
                                .long("unique-versions")
                                .help("Do not unify versions but treat `(id, version)` as a unique crate in the dependency graph.
                                        Unifying the versions would mean that they are factored out and only `id` is used 
                                        to identify a crate.")
                                .action(clap::ArgAction::SetTrue),
                        )
                )
                // `lint` command
                .subcommand(
                    Command::new("lint")
                    .about("Lint your feature usage by analyzing crate metadata")
                    .arg(
                        Arg::new("workspace")
                            .long("workspace")
                            .help("Whether to only consider workspace crates")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("offline")
                            .long("offline")
                            .help("Whether to use offline mode")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("locked")
                            .long("locked")
                            .help("Whether to use all the locked dependencies from the `Cargo.lock`. 
                                    Otherwise it may update some dependencies. 
                                    For CI usage its a good idea to use it.")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("all-features")
                            .long("all-features")
                            .help("Modify all features")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .subcommand(
                        Command::new("propagate-feature")
                        .about("Check that features are passed down")
                        .arg(
                            Arg::new("features")
                                .long("features")
                                .help("The feature to check")
                                .required(true),
                        )
                        .arg(
                            Arg::new("packages")
                                .long("packages")
                                .short('p')
                                .help("The packages to check. If empty, all packages are checked")
                        )
                        .arg(
                            Arg::new("feature-enables-dep")
                                .long("feature-enables-dep")
                                .help("The auto-fixer will enables the feature of the dependencies as non-optional.
                                        This can be used in case that a dependency should not be enabled like `dep?/feature` but
                                        like `dep/feature` instead. In this case you would pass `--feature-enables-dep
                                        feature:dep`. The option can be passed multiple times, or multiple key-value pairs can be
                                        passed at once by separating them with a comma like: `--feature-enables-dep
                                        feature:dep,feature2:dep2`. (TODO: Duplicate entries are undefined)."
                                    )
                        )
                        .arg(
                            Arg::new("left-side-feature-missing")
                                .long("left-side-feature-missing")
                                .help("Overwrite the behaviour when the left side dependency is missing the feature.
                                        This can be used to ignore missing features, treat them as warning or error. A 'missing
                                        feature' here means that if `A` has a dependency `B` which has a feature `F`, and the
                                        propagation is checked then normally it would error if `A` is not forwarding `F` to `B`.
                                        Now this option modifies the behaviour if `A` does not have the feature in the first place.
                                        The default behaviour is to require `A` to also have `F`.
                                        
                                        [default: fix]

                                        Possible values:
                                        - ignore: Ignore this behaviour
                                        - report: Only report but do not fix
                                        - fix:    Fix if `--fix` is passed"
                                    )
                        )
                        .arg(
                            Arg::new("ignore-missing-propagate")
                                .long("ignore-missing-propagate")
                                .help("Ignore single missing links in the feature propagation chain. 
                                Usage --ignore-missing-propagate <CRATE/FEATURE:DEP/DEP_FEATURE>")
                        )
                        .arg(
                            Arg::new("dep-kinds")
                                .long("dep-kinds")
                                .help("How to handle dev-dependencies. 
                                Usage: --dep-kinds <KIND/MUTE_SETTING>
                                [default: normal:check,dev:check,build:check]")
                        )
                        .arg(
                            Arg::new("show-version")
                                .long("show-version")
                                .help("Show crate versions in the output")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("show-path")
                                .long("show-path")
                                .help("Show crate manifest paths in the output")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("fix")
                                .long("fix")
                                .help("Try to automatically fix the problems")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("modify-paths")
                                .long("modify-paths")
                                .help("Paths that are allowed to be modified by the formatter"),
                        )
                        .arg(
                            Arg::new("fix-dependency")
                            .long("fix-dependency")
                            .help("Fix only issues with this package as a dependency")
                        )
                        .arg(
                            Arg::new("fix-package")
                            .long("fix-package")
                            .help("Fix only issues with this package as a feature source")
                        )
                    )
                    .subcommand(
                        Command::new("never-enables")
                        .about("A feature should never enable another one")
                        .arg(
                            Arg::new("precondition")
                                .long("precondition")
                                .help("The left side of the feature implication. 
                                    Can be set to `default` for the default feature set.")
                                .required(true),
                        )
                        .arg(
                            Arg::new("stays-disabled")
                                .long("stays-disabled")
                                .help("The right side of the feature implication. 
                                    If [precondition] is enabled, this stays disabled.")
                                .required(true),
                        )
                    )
                    .subcommand(
                        Command::new("never-implies")
                        .about("A feature should never transitively imply another one.")
                        .arg(
                            Arg::new("precondition")
                                .long("precondition")
                                .help("The left side of the feature implication. 
                                    Can be set to `default` for the default feature set.")
                                .required(true),
                        )
                        .arg(
                            Arg::new("stays-disabled")
                                .long("stays-disabled")
                                .help("The right side of the feature implication. 
                                    If [precondition] is enabled, this stays disabled.")
                                .required(true),
                        )
                        .arg(
                            Arg::new("show-source")
                                .long("show-source")
                                .help("Show the source location of crates in the output")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("show-version")
                                .long("show-version")
                                .help("Show the version of crates in the output")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("path-delimiter")
                                .long("path-delimiter")
                                .help("Delimiter for rendering dependency paths [ default: ' -> ' ]"),
                        )
                    )
                    .subcommand(
                        Command::new("only-enables")
                        .about("A feature should exclusively enable another one")
                        .arg(
                            Arg::new("precondition")
                                .long("precondition")
                                .help("The left side of the feature implication. 
                                    Can be set to `default` for the default feature set.")
                                .required(true),
                        )
                        .arg(
                            Arg::new("only-enables")
                                .long("only-enables")
                                .help("The right side of the feature implication. 
                                    If [precondition] is enabled, this stays disabled.")
                                .required(true),
                        )
                    )
                    .subcommand(
                        Command::new("why-enabled")
                        .about("Find out why a specific feature is enabled")
                        .arg(
                            Arg::new("package")
                                .long("package")
                                .short('p')
                                .help("Name of the package")
                                .required(true),
                        )
                        .arg(
                            Arg::new("feature")
                                .long("feature")
                                .help("Name of the feature")
                                .required(true),
                        )
                    )
                )
                .subcommand(
                    Command::new("debug")
                    .about("Just for quick debugging some stuff.")
                    .arg(
                        Arg::new("workspace")
                            .long("workspace")
                            .help("Whether to only consider workspace crates")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("offline")
                            .long("offline")
                            .help("Whether to use offline mode")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("locked")
                            .long("locked")
                            .help("Whether to use all the locked dependencies from the `Cargo.lock`. 
                                    Otherwise it may update some dependencies. 
                                    For CI usage its a good idea to use it.")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("all-features")
                            .long("all-features")
                            .help("Modify all features")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("no-benchmark")
                            .long("no-benchmark")
                            .help("Do not include benchmark dependencies in the output")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("no-root")
                            .long("no-root")
                            .help("Do not include the root crate in the output")
                            .action(clap::ArgAction::SetTrue),
                    )
                )
                .subcommand(
                    Command::new("transpose")
                    .about("Transpose dependencies in the workspace")
                    .arg(
                        Arg::new("workspace")
                            .long("workspace")
                            .help("Whether to only consider workspace crates")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("offline")
                            .long("offline")
                            .help("Whether to use offline mode")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("locked")
                            .long("locked")
                            .help("Whether to use all the locked dependencies from the `Cargo.lock`. 
                                    Otherwise it may update some dependencies. 
                                    For CI usage its a good idea to use it.")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .arg(
                        Arg::new("all-features")
                            .long("all-features")
                            .help("Modify all features")
                            .action(clap::ArgAction::SetTrue),
                    )
                    .subcommand(
                        Command::new("dependency")
                        .subcommand(
                            Command::new("lift-to-workspace")
                            .about("Lifts crate dependencies to the workspace")
                        )
                        .arg(
                            Arg::new("fix")
                                .long("fix")
                                .help("Instead of dry-running, actually modify the files")
                                .action(clap::ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("version-selector")
                                .long("version-selector")
                                .help("How to determine which version to use for the whole workspace
                                        [default: unambiguous]

                                        Possible values:
                                        - unambiguous: The version must be unambiguous - eg. there is only one version in the workspace
                                        - exact:       A specific version
                                        - highest:     The latest version that was seen in the workspace")
                        )
                        .arg(
                            Arg::new("skip-package")
                                .long("skip-package")
                                .help("Do not try to modify this package")
                        )
                        .arg(
                            Arg::new("source-location")
                                .long("source-location")
                                .help("Optionally only check dependencies with this source location

                                    Possible values:
                                    - local:  The dependency is referenced via a `path`
                                    - remote: Either git or a registry")
                        )
                        .arg(
                            Arg::new("exact-version")
                                .long("exact-version")
                                .help("The exact version to use for the whole workspace")
                        )
                        .arg(
                            Arg::new("ignore-errors")
                                .long("ignore-errors")
                                .help("Ignore errors and continue with the next dependency")
                                .action(clap::ArgAction::SetTrue),
                        )
                    )
                ),
        )
        // `version` command to manage Polkadot SDK versions via psvm
        .subcommand(
            Command::new("version")
            .about("Manage Polkadot SDK versions via psvm")
            .arg(
                Arg::new("list")
                    .short('l')
                    .long("list")
                    .help("List all available versions")
                    .global(true)
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("version")
                .short('v')
                .long("version")
                .help("Specifies the Polkadot SDK version")
            )
            .arg(
                Arg::new("path")
                    .short('p')
                    .long("path")
                    .help("Path to a crate folder or Cargo.toml file [default: Cargo.toml]")
                    .global(true)
            )
            .arg(
                Arg::new("overwrite")
                .short('o')
                .long("overwrite")
                .help("Overwrite local dependencies (using path) with same name as the ones in the Polkadot SDK")
                .global(true)
                .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("check")
                .short('c')
                .long("check")
                .help("Check if the dependencies versions match the Polkadot SDK version. Does not update the Cargo.toml")
                .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("orml")
                .short('O')
                .long("orml")
                .help("To either list available ORML version or update the Cargo.toml file with the corresponding ORML version")
                .action(clap::ArgAction::SetTrue),
            )
        )
        // `prdoc` command to generate, check and load PRDoc files via prdoc
        .subcommand(
            Command::new("prdoc")
            .about("Generate, check and load PRDoc files via prdoc")
            .arg(
                Arg::new("config")
                    .long("config")
                    .short('c')
                    .help("[env: PRDOC_CONFIG=]")
            )
            .arg(
                Arg::new("prdoc-folders")
                    .long("prdoc-folders")
                    .short('d')
                    .help("[env: PRDOC_FOLDERS=]")
            )
            .arg(
                Arg::new("version")
                    .long("version")
                    .short('v')
                    .help("Show the version")
            )
            .arg(
                Arg::new("json")
                    .long("json")
                    .short('j')
                    .help("Output as JSON")
            )
            .subcommand(
                Command::new("generate")
                .about("Generate a new file. It will be saved by default unless you provide --dry-run.
                The command will fail if the target file already exists.")
                .arg(
                    Arg::new("number")
                        .long("number")
                        .short('n')
                        .help("The PR number")
                        .required(true)
                )
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .help("Do not save the generated document to file with the proper naming, show the content instead")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("output-dir")
                        .long("output-dir")
                        .short('o')
                        .help("Optional output directory. It not passed, the default PRDOC_DIR will be used under the root of the current project")
                )
            )
            .subcommand(
                Command::new("check")
                .about("Check one ore more prdoc files for validity")
                .arg(
                    Arg::new("file")
                        .long("file")
                        .short('f')
                        .help("Directly specify the file to be checked. It can be relative to the base directory")
                )
                .arg(
                    Arg::new("number")
                        .long("number")
                        .short('n')
                        .help("The PR number")
                )
                .arg(
                    Arg::new("list")
                        .long("list")
                        .short('l')
                        .help("Get the list of PR numbers from a file")
                )
                .arg(
                    Arg::new("schema")
                        .long("schema")
                        .short('s')
                        .help("Schema to be used. Passing this flag/ENV overrides the value from the config [env: PRDOC_SCHEMA=]")
                )
            )
            .subcommand(
                Command::new("scan")
                .about("Scan a directory for prdoc files based on their name")
                .arg(
                    Arg::new("all")
                        .long("all")
                        .short('a')
                        .help("Also return invalid files")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("sort")
                        .long("sort")
                        .short('s')
                        .help("Sort the output")
                        .action(clap::ArgAction::SetTrue)
                )
            )
            .subcommand(
                Command::new("load")
                .about("Load one or more prdoc")
                .arg(
                    Arg::new("file")
                        .long("file")
                        .short('f')
                        .help("Directly specify the file to be loaded. It can be relative to the base directory")
                )
                .arg(
                    Arg::new("number")
                        .long("number")
                        .short('n')
                        .help("One or more PR numbers. Depending on the host OS, the max length of a command may differ. 
                        If you run into issues, make sure to check the --list option instead")
                )
                .arg(
                    Arg::new("list")
                        .long("list")
                        .short('l')
                        .help("Get the list of PR numbers from a file")
                )
            )
        )
        // `install` command to install all the required dependencies for polkadot-sdk development
        .subcommand(
            Command::new("install")
            .about("Install all the required dependencies for polkadot-sdk development")
        );

    // Get matches for the command-line arguments
    let matches = cmd.get_matches();

    // Handle the subcommands
    match matches.subcommand() {
        Some(("help-contribute", _)) => contribute::contribute_help(),
        Some(("install", _)) => install::run_install(),
        Some(("format", sub_matches)) => {
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

            format::run_format(
                quiet,
                verbose,
                version,
                package,
                manifest_path,
                message_format,
                all,
                check,
            );
        }
        Some(("flint", sub_matches)) => flint::handle_flint_command(sub_matches),
        Some(("prdoc", sub_matches)) => prdoc::handle_prdoc_command(sub_matches),
        Some(("version", sub_matches)) => {
            let list = sub_matches.get_flag("list");
            let path = sub_matches.get_one::<String>("path").map(|s| s.as_str());
            let version = sub_matches.get_one::<String>("version").map(|s| s.as_str());
            let overwrite = sub_matches.get_flag("overwrite");
            let check = sub_matches.get_flag("check");
            let orml = sub_matches.get_flag("orml");

            psvm::run_version(list, overwrite, check, orml, version, path)
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
