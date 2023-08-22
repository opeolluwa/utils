use serde::Serialize;

#[derive(clap::Args, Debug, Serialize)]
pub struct ReadmeCommands {
    #[clap(short, long, value_parser)]
    ///path to the directory where the application will be created
    pub path: String, //path to create the readme
    #[clap(short, long, value_parser)]
    ///the name of the application
    pub name: String, //name of the application
    #[clap(short, long, value_parser)]
    ///the description of the application
    pub description: String, //description of the application
    #[clap(short, long, value_parser)]
    /// overwrite existing readme
    pub force: bool,
    #[clap(short, long, value_parser)]
    /// backup existing readme
    pub backup: bool,
}

impl ReadmeCommands {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            name: String::new(),
            description: String::new(),
            force: false,
            backup: false,
        }
    }

  

    fn get_template() -> &'static str {
        r#" ## Title 

## Description 

- [Description](#description)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)
- [Help](#help)
- [Authors](#authors)
- [Version History](#version-history)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Description

An in-depth paragraph about your project and overview of use.

## Getting Started

### Dependencies

- Describe any prerequisites, libraries, OS version, etc., needed before installing program.
- ex. Windows 10

### Installing

- How/where to download your program
- Any modifications needed to be made to files/folders

### Executing program

- How to run the program
- Step-by-step bullets

```
code blocks for commands
```

## Documentation

Describe any special instructions that are necessary to install a software package on your computer (if applicable).

## Help

Any advise for common problems or issues.

```
command to run if program contains helper info
```

## Authors

Contributors names and contact info

ex. Dominique Pizzie  
ex. [@DomPizzie](https://twitter.com/dompizzie)

## Version History

- 0.2
  - Various bug fixes and optimizations
  - See [commit change]() or See [release history]()
- 0.1
  - Initial Release

## License

This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.
        "#
    }
}
