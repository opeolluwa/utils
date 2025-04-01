# Toolbox

Toolbox is a lightweight extensible, command line toolchain to streamline a
developer workflow

## Installation

As of this time, Utils CLI only be built from source

```sh
git clone https://github.com/opeolluwa/toolbox.git
cd utils
cargo build --release
cargo install --path .
```

### Executing program

To run the application locally

```sh
cargo run -- --help

A set of extensible tools to accelerate software development

Usage: toolbox [COMMAND]

Commands:
  store      store and manage a key value pair
  generate   generate a new project or project files
  update     self update the CLI
  uninstall  Uninstall the CLI
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## License

This project is licensed under the Apache License Version 2.0 License - see the
[LICENSE](./LICENSE) file for details
