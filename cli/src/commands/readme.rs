use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use serde::Serialize;
use utils_style::style::LogMessage;

/// the readme utils is used to for generating project readme
/// # Example
/// the basic examples are listed thus:
/// ```sh
///
/// utils readme #generate readme in the current directory
/// utils readme -p ~/Desktop # generate readme in the provide path
/// utils readme --path ~/Desktop # generate readme in the provide path
/// ```
///
#[derive(clap::Args, Debug, Serialize)]
pub struct ReadmeCommands {
    ///path to the directory where the readme will be created
    #[clap(short, long, value_parser, default_value = ".")]
    pub path: String,
    /// the project title
    #[clap(short, long, value_parser, default_value = "Title")]
    pub title: String,
    /// the project description
    #[clap(short, long, value_parser, default_value = "Description")]
    pub description: String,
    /// overwrite existing readme
    #[clap(short, long, value_parser)]
    pub force: bool,
    /// backup existing readme
    #[clap(short, long, value_parser)]
    pub backup: bool,
}

impl ReadmeCommands {
    /// parse the commands parameters
    ///  create a  new readme in the provide path,
    /// if the path is not provided the readme should be created in the current working directory
    ///
    /// ### Example
    /// ```sh
    /// utils readme  -d "het" -t "hh" -p .
    ///  ```
    pub fn parse(&self) {
        // see if the readme already exist
        let binding = Path::new(&self.path).join("README.md");
        let path = binding.as_path();

        if path.exists() {
            // if the readme exist and the force flag is not set, exit
            if !self.force {
                LogMessage::error("the readme already exist, use the --force flag to overwrite it");
                return;
            }
            if self.force && !self.backup {
                // ask if the current readme should be over written
                let override_readme = Confirm::new()
                    .with_prompt(
                        "The current readme would not be backed up, do you wish to continue?",
                    )
                    .interact()
                    .unwrap();

                if !override_readme {
                    return;
                } else {
                    // remove the current readme and create a new one
                    fs::remove_file(path).unwrap();
                    ReadmeCommands::create_new(path);
                }
            }
            // if the readme exist and the force flag is set, backup the existing readme
            if self.backup {
                // create a backup of the existing readme
                let backup_path = Path::new(&self.path).join("README.md.bak");
                fs::copy(path, backup_path).expect("failed to create backup");
                // create the readme
                ReadmeCommands::create_new(path);
            }
        } else {
            // if the readme does not exist, create it
            ReadmeCommands::create_new(path);
        }
    }

    ///  create a  new readme
    fn create_new(read_me_path: &Path) {
        // prompt for project name, description and confirm the path,
        let project_title: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Title")
            .validate_with({
                let mut force = None;
                move |input: &String| -> Result<(), &str> {
                    if input.len() >= 3 || force.as_ref().map_or(false, |old| old == input) {
                        Ok(())
                    } else {
                        force = Some(input.clone());
                        Err("Please provide a project title, at least 3 characters long")
                    }
                }
            })
            .interact_text()
            .unwrap();

        let project_description : String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project description")
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                if input.len() >= 10 || force.as_ref().map_or(false, |old| old == input) {
                    Ok(())
                } else {
                    force = Some(input.clone());
                    Err("Please provide a  valid project description, at least 10 characters long")
                }
            }
        })
        .interact_text()
        .unwrap();

        // // the file path
        //     let read_me_path: String = Input::with_theme(&ColorfulTheme::default())
        //     .with_prompt("Your planet")
        //     .default(".".to_string())
        //     .interact_text()
        //     .unwrap();

        // write the provided data to the file
        let mut file = File::create(read_me_path).unwrap();
        file.write_all(
            ReadmeCommands::get_template(project_title.trim(), project_description.trim())
                .as_bytes(),
        )
        .unwrap();
    }

    /// return the template as string
    fn get_template(title: &str, description: &str) -> String {
        format!(
            r#" # {title}

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

{description}

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
        )
    }
}
