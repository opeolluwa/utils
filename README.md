# Utils CLI

Utils CLI is a powerful command-line interface (CLI) tool designed to
supercharge your productivity by providing essential utilities for software
projects. This tool supports storing key-value pairs, generating `.gitignore`
files, managing README files, and more. Utils CLI is lightweight, easy to use,
and integrates seamlessly with your projects.

## Installation

As of this time, Utils CLI only be built from source

```sh
git clone https://github.com/opeolluwa/utils.git
cd utils
cargo build --release
cargo install --path .
```

### Executing program

To run the application locally

```sh
cargo run -- --help
```

## Usage

```sh
utils <COMMAND>
```

### Commands

- **store**: Store and retrieve data as key-value pairs.

```sh
utils store --key=<KEY> --value=<Value> --secure
```

- **ignore**: Generate a .gitignore file for your project tooling.

```sh
utils ignore <tool-name>
```

- **readme**: Add a README.md to your project.

```sh
utils readme <project-name>
```

- **upgrade**: Upgrade the CLI tool to the latest version.

```sh
utils upgrade
```

- **sync**: Synchronize the data with a remote database.

```sh
utils sync --database-url=<REMOTE_DATABASE_CONNECTION>
```

- **config**: Configure CLI behavior using a config file at
  `$HOME/.utils/utils.conf`

```sh
utils config | vi
```

- **help**: Display the help message for available commands.

```sh
utils help
```

### Options

The basic usage of Utils CLI involves passing a command followed by optional
arguments.

## License

This project is licensed under the Apache License Version 2.0 License - see the
[LICENSE](./LICENSE) file for details
