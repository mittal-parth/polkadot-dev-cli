## Polkadot Dev CLI

A CLI tool for Polkadot developers to include different crates and tools for developing on Polkadot like linting, formatting, version management, etc.

## Usage

```
CLI tool for Polkadot developers bundling linting, formatting, and version management

Usage: polkadot-dev <COMMAND>

Commands:
  help-contribute  Show a checklist for contributing to the project
  format           Format code using the correct Rust nightly version
  flint            Analyze, Fix and Lint features in your Rust workspace via Zepter [aliases: feature-lint, f-lint]
  version          Manage Polkadot SDK versions via psvm
  prdoc            Generate, check and load PRDoc files via prdoc
  checkup          Runs format, flint and version altogether
  install          Install all the required dependencies for polkadot-sdk development
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Command wise usage

In general the `--help` or `-h` flag can be used to get help for any command.

#### `polkadot-dev format`

```
Format code using the correct Rust nightly version

Usage: polkadot-dev format [OPTIONS]

Options:
  -q, --quiet
          No output printed to stdout
  -v, --verbose
          Use verbose output
      --version
          Print rustfmt version and exit
  -p, --package <package>
          Specify Package to format
      --manifest-path <manifest-path>
          Specify path to the Cargo.toml file
      --message-format <message-format>
          Specify message-format: short|json|human
      --all
          Format all packages, and also their local path-based dependencies
      --check
          Run rustfmt in check mode
  -h, --help
          Print help
```

#### `polkadot-dev flint`

*Only the top level commands are mentioned here. For individual subcommands, please refer to the help message. Eg: `polkadot-dev fint trace --help`*

```
Analyze, Fix and Lint features in your Rust workspace via Zepter

Usage: polkadot-dev flint [OPTIONS] [COMMAND]

Commands:
  run              Run a workflow from the config file. Uses `default` if none is specified.
  format-features  Format features layout and remove duplicates [aliases: ff]
  trace            Trace dependencies paths.
  lint             Lint your feature usage by analyzing crate metadata
  debug            Just for quick debugging some stuff.
  transpose        Transpose dependencies in the workspace
  help             Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet                          Only print errors. Supersedes `--log`
      --log <log>                      Log level to use [default: info]
      --color                          Use ANSI terminal colors
      --exit-code-zero                 Try to exit with code zero if the intended check failed.
      --fix-hint <fix-hint>            Don't print any hints on how to fix the error. [default: on] [possible values: on, off]
      --manifest-path <manifest-path>  Manually set the location of the manifest file. Must point directly to a file and not a directory.
  -h, --help                           Print help
```

#### `polkadot-dev version`

```
Manage Polkadot SDK versions via psvm

Usage: polkadot-dev version [OPTIONS]

Options:
  -l, --list               List all available versions
  -v, --version <version>  Specifies the Polkadot SDK version
  -p, --path <path>        Path to a crate folder or Cargo.toml file [default: Cargo.toml]
  -o, --overwrite          Overwrite local dependencies (using path) with same name as the ones in the Polkadot SDK
  -c, --check              Check if the dependencies versions match the Polkadot SDK version. Does not update the Cargo.toml
  -O, --orml               To either list available ORML version or update the Cargo.toml file with the corresponding ORML version
  -h, --help               Print help
```

#### `polkadot-dev prdoc`

*Note: Only the top level command is mentioned here. Use the `--help` flag to get help for individual subcommands. Eg: `polkadot-dev prdoc scan --help`*

```
Generate, check and load PRDoc files via prdoc

Usage: polkadot-dev prdoc [OPTIONS] [COMMAND]

Commands:
  generate  Generate a new file. It will be saved by default unless you provide --dry-run.
                            The command will fail if the target file already exists.
  check     Check one ore more prdoc files for validity
  scan      Scan a directory for prdoc files based on their name
  load      Load one or more prdoc
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <config>                [env: PRDOC_CONFIG=]
  -d, --prdoc-folders <prdoc-folders>  [env: PRDOC_FOLDERS=]
  -v, --version                        Show the version
  -j, --json                           Output as JSON
  -h, --help                           Print help
```

#### `polkadot-dev checkup`

```
Runs format, flint and version altogether

Usage: polkadot-dev checkup

Options:
  -v, --version <version>  Specify the Polkadot SDK version to check versions against
  -h, --help               Print help
```

#### `polkadot-dev install`

```
Install all the required dependencies for polkadot-sdk development

Usage: polkadot-dev install

Options:
  -h, --help  Print help
```

#### `polkadot-dev help-contribute`

```
Show a checklist for contributing to the project

Usage: polkadot-dev help-contribute

Options:
  -h, --help  Print help
```

## Installation

1. Clone the repository
2. Run `cargo build --release` in the root directory of the repository
3. Run `cargo install --path <path>` where `<path>` is the path to the root directory of this repository
  Eg: If you are using this for the polkadot-sdk:
    1. Head to the root directory of the polkadot-sdk
    2. Run `cargo install --path <path to the root directory of polkadot-dev-cli>`

## Contributing

We welcome contributions in the form of pull requests, issues and documentation. Feel free to help us in any way! ❤️

- Please read and abide by our [Code of Conduct](/CODE_OF_CONDUCT.md). Our community aspires to be a respectful place both during online and in-person interactions.
- Please follow the [installation guide](#installation) to contribute.

## References

The polkadot-dev CLI is powered by a number of tools and libraries. Here are their individual documentations. Big thanks to the developers of these tools! 🎉:

1. `format`: [rustfmt](https://github.com/rust-lang/rustfmt)
2. `flint`: [Zepter](https://github.com/ggwpez/zepter)
3. `version`: [psvm](https://github.com/paritytech/psvm)
4. `prdoc`: [prdoc](https://github.com/paritytech/prdoc)
