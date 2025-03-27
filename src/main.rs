mod commands;
mod constants;
mod database;
mod errors;
mod parser;

use clap::{arg, command, ArgAction, Command};

mod utils;

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("store")
                .about("store data as a key value pair")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue))
                .arg(arg!(-s --sync "backup to a remote database").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("generate")
                .about("generate a new project or project files")
                .subcommand(
                    Command::new("readme")
                        .about("create readme for a project")
                        .arg(arg!( -f --force "Overwrite existing "))
                        .arg(arg!( -b --backup "backup existing"))
                        .arg(arg!( -p --path <PATH> "desired path")),
                )
                .subcommand(
                    Command::new("git-ignore")
                        .about("create readme for a project")
                        .arg(arg!( -f --force "Overwrite existing "))
                        .arg(arg!( -b --backup "backup existing"))
                        .arg(arg!( -p --path <PATH> "desired path")),
                )
                .subcommand(
                    Command::new("service")
                        .about("create a new service")
                        .arg(arg!( -p --path <PATH> "desired path").required(true)),
                ),
        )
        .subcommand(Command::new("update").about("self update the CLI"))
        .subcommand(Command::new("uninstall").about("Uninstall the CLI"))
        .get_matches();

    parser::parse_commands(matches);
}
