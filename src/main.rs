mod commands;
mod constants;
mod database;
mod errors;
mod helpers;
mod parser;
mod parsers;
mod ui;
use errors::app::AppError;

use clap::{arg, command, Command};
use rusqlite::Connection;

fn main() -> Result<(), AppError> {
    // Self subcommands
    let self_uninstall_cmd = Command::new("uninstall").about("uninstall toolbox");
    let self_upgrade_cmd = Command::new("upgrade").about("upgrade toolbox");
    let self_configure_cmd = Command::new("configure").about("configure the toolbox");

    let self_cmd = Command::new("self")
        .about("manage and configure the toolbox")
        .subcommand(self_uninstall_cmd)
        .subcommand(self_upgrade_cmd)
        .subcommand(self_configure_cmd);

    // Store subcommands
    let store_save_cmd = Command::new("save")
    .about("Save a new key value pair")
    .arg(arg!(-k --key <KEY> "key"))
    .arg(arg!(-v --value <VALUE> "value"))
    .arg(arg!(-s --sensitive "mark data as sensitive data, used for encrypting data when returning them"));

    let store_list_cmd = Command::new("list")
        .about("retrieve the stored entries")
        .arg(arg!(-s --sort <SORT> "sort by Ascending(ASC)  descending(DSC), sort by key (KEY)"));

    let store_find_cmd = Command::new("find")
        .about("find one or more entries")
        .arg(arg!(-e --exact "find exact keyword, against the stored keys"));

    let store_remove_cmd = Command::new("remove").about("delete an entry from the database");

    let store_export_cmd = Command::new("export").about("export to HTML or PDF");

    let store_cmd = Command::new("store")
        .visible_aliases(["s", "-s"])
        .about("store and manage a key value pair")
        .subcommand(store_save_cmd)
        .subcommand(store_list_cmd)
        .subcommand(store_find_cmd)
        .subcommand(store_remove_cmd)
        .subcommand(store_export_cmd)
        .arg(arg!(-s --sync <REMOTE_SERVER> "backup to a remote database SQL"));

    // Generate subcommands
    let generate_readme_cmd = Command::new("readme")
        .about("create readme for a project")
        .arg(arg!(-f --force "Overwrite existing "))
        .arg(arg!(-b --backup "backup existing"))
        .arg(arg!(-p --path <PATH> "desired path"));

    let generate_gitignore_cmd = Command::new("git-ignore")
        .about("create a .gitignore file for a project")
        .arg(arg!(-f --force "Overwrite existing .gitignore file "))
        .arg(arg!(-b --backup "backup existing .gitignore file before overwrite"))
        .arg(arg!(-p --path <PATH> "desired path"));

    let generate_service_cmd = Command::new("service")
    .about("create a new backend service using convectional structure or as specified in the toolbox.toml file")
    .arg(arg!(-p --path <PATH> "desired path").required(true));

    let generate_cmd = Command::new("generate")
        .visible_aliases(["g", "-g"])
        .about("generate a new project or project files")
        .subcommand(generate_readme_cmd)
        .subcommand(generate_gitignore_cmd)
        .subcommand(generate_service_cmd);

    // Top-level commands
    let update_cmd = Command::new("update").about("self update the CLI");
    let uninstall_cmd = Command::new("uninstall").about("Uninstall the CLI");

    // Main command
    let matches = command!()
        .subcommand(self_cmd)
        .subcommand(store_cmd)
        .subcommand(generate_cmd)
        .subcommand(update_cmd)
        .subcommand(uninstall_cmd)
        .get_matches();

    let os_default_home_dir = dirs::home_dir().unwrap();
    let db_path = format!(
        "{home_dir}/{upload_dir}",
        home_dir = os_default_home_dir.display(),
        upload_dir = ".toolbox"
    );
    let _ = std::fs::create_dir_all(&db_path);
    let database_path = format!("{db_path}/toolbox.db");

    let connection = Connection::open(&database_path)
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;
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
