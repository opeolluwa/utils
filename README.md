# Utils

<div style="display:flex; justify-content:center; align-items:center">
  <img alt=app-icon src=app-icon.png/>
</div>

Compilation of utility scripts for everyday use

- [Description](#description)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)
- [License](#license)

## Demo

![Demo](./utils-demo.gif)

## Description

This repository contains a collection of useful scripts that can be used in any software project and in a desktop environment. The main purpose is to provide abstracts on some common tasks such as adding readme to a project, sending email and SMS, etc.

## Getting Started

### Dependencies

- [Rust](https://rust-lang.org) >= v1.70.0

### Installing

- Install from the [Cargo](https://crates.io) repository

  ```sh
  cargo install utils
  ```

- Clone the repository and build the application

  ```sh
  git clone https://github.com/opeolluwa/utils.git
  cd utils
  cargo build --release
  cargo install --path .
  ```

- Install from NPM
  
  ```sh
  npm install -g @opeolluwa/utils
  ```

### Executing program

To run the application locally

```sh
cargo run -- --help
```


## License

This project is licensed under the Apache License
Version 2.0 License - see the [LICENSE](./LICENSE) file for details
