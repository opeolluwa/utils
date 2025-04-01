mod commands;
mod constants;
mod database;
mod errors;
mod helpers;
mod parser;
mod ui;

use clap::{arg, command, Command};
use errors::app::AppError;
use rusqlite::Connection;

fn main() -> Result<(), AppError> {
    let matches = command!()
        .subcommand(
            Command::new("store")
                .about("store and manage a key value pair")
                .subcommand(
                    Command::new("save")
                        .about("Save a new key value pair")
                        .arg(arg!(-k --key  <KEY> "key"))
                        .arg(arg!(-v --value <VALUE> "value"))
                        .arg(arg!(-s --sensitive  "mark data as sensitive data, used for encrypting data when returning them")),

                )
                .subcommand(
                    Command::new("list")
                        .about("retrieve the stored entries")
                        .arg(arg!( -s --sort <SORT> "sort by Acending(ASC)  decending(DSC), sort by key (KEY)")),
                )
                .subcommand(Command::new("find").about("find one or more entries").arg(arg!(-e --exact "find exact keyword, againt the stored keys")))
                .subcommand(Command::new("remove"))
                .subcommand(Command::new("export").about("export to HTML or PDF "))
                .arg(arg!(-s --sync <REMOTE_SERVER> "backup to a remote database SQL")),
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
                        .about("create a .gitignore file for a project")
                        .arg(arg!( -f --force "Overwrite existing .gitignore file "))
                        .arg(arg!( -b --backup "backup existing .gitignore file before overwrite"))
                        .arg(arg!( -p --path <PATH> "desired path")),
                )
                .subcommand(
                    Command::new("service")
                        .about("create a new backend service using convectional structure or as specified in the toolbox.toml file")
                        .arg(arg!( -p --path <PATH> "desired path").required(true)),
                ),
        )
        .subcommand(Command::new("update").about("self update the CLI"))
        .subcommand(Command::new("uninstall").about("Uninstall the CLI"))
        .get_matches();

    let os_default_home_dir = dirs::home_dir().unwrap();
    let db_path = format!(
        "{home_dir}/{upload_dir}",
        home_dir = os_default_home_dir.display(),
        upload_dir = ".toolbox"
    );
    let _ = std::fs::create_dir_all(&db_path);
    let database_path = format!("{db_path}/toolbox.db");

    let connection =
        Connection::open(&database_path).map_err(|err| AppError::OperationFailed(err.to_string()))?;
    connection
        .execute(
            r#"
    CREATE TABLE IF NOT EXISTS data_store (
    id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    sensitive INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
    )
    "#,
            (),
        )
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;

    parser::parse_commands(matches, connection);

    Ok(())
}
