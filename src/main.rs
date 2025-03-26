use clap::{arg, command, ArgAction, Command};

use commands::generator::Generator;
use include_dir::{include_dir, Dir};

pub const SOURCE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

lazy_static::lazy_static! {
    pub static ref DB_URL: std::string::String = {
        let os_default_home_dir = dirs::home_dir().unwrap();
        let db_path = format!(
            "{home_dir}/{upload_dir}",
            home_dir = os_default_home_dir.display(),
            upload_dir = ".utils"
        );
        let _ = std::fs::create_dir_all(&db_path);
    format!("sqlite://{db_path}/utils.db")
    };

    // the path to the config file
    pub static ref CONFIG_FILE: std::string::String = {
        let os_default_home_dir = dirs::home_dir().unwrap();
        let config_path = format!(
            "{home_dir}/{upload_dir}",
            home_dir = os_default_home_dir.display(),
            upload_dir = ".utils"
        );

        // create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&config_path);
        format!("{config_path}/utils.conf")
    };
}

mod commands;
mod database;
mod errors;
mod utils;
fn main() {
    let matches = command!()
        .subcommand(
            Command::new("store")
                .about("store data as a key value pair")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue))
                .arg(arg!(-s --sync "backup to a remote database").action(ArgAction::SetTrue)),
        )
        .subcommand(Command::new("update").about("self update the CLI"))
        .subcommand(Command::new("uninstall").about("Uninstall the CLI"))
        .subcommand(
            Command::new("generate")
                .about("generate a new project or project files")
                .subcommand(
                    Command::new("readme")
                        .about("create readme for a project")
                        .arg(arg!( -f --force "Overwrite existing "))
                        // .arg(arg!( -b -back "backup existing")),
                )
                .subcommand(Command::new("git-ignore")),
        )
        .arg(arg!( -n --name "project of file name ").action(ArgAction::SetTrue))
        .get_matches();

    match matches.subcommand() {
        Some(("store", _sub_matches)) => {
            // let _ = run_store_tui();
            println!("store")
        }
        Some(("uninstall", _)) => {
            println!("uninstall")
        }
        Some(("upgrade", _)) => {
            println!("upgrade")
        }
        Some(("generate", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("readme", _)) => {
                    let base_path = "";
                    let force = false;
                    let back_up = false;

                    let _ = Generator::new(force, base_path.into(), back_up).generate_readme();
                }
                Some(("git-ignore", _)) => println!("git-ignore"),
                Some(("service", _)) => println!("service"),
                _ => std::process::exit(1),
            }
            // println!("generate {:#?}", sub_matches.)
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    // println!("{:#?}", matches)
    // Continued program logic goes here...
}
