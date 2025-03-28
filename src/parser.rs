use std::{path::Path, process};

use clap::ArgMatches;
use rusqlite::Connection;

use crate::{
    commands::{generator::GeneratorConfig, store::StoreConfig},
    utils::{console::LogMessage, parser::extract_command_argument},
};

pub fn parse_commands(matches: ArgMatches, database_connection: Connection) {
    match matches.subcommand() {
        Some(("store", sub_matches)) => parse_store_options(sub_matches, database_connection),
        Some(("uninstall", sub_matches)) => parse_uinstall_options(sub_matches),
        Some(("upgrade", sub_matches)) => parse_upgrade_options(sub_matches),
        Some(("generate", sub_matches)) => parse_generator_options(sub_matches),
        _ => std::process::exit(1),
    }
}

fn parse_uinstall_options(_sub_matches: &ArgMatches) {}
fn parse_upgrade_options(_sub_matches: &ArgMatches) {}
fn parse_store_options(sub_matches: &ArgMatches, database_connection: Connection) {
    let store_engine = StoreConfig::new(database_connection);
    match sub_matches.subcommand() {
        Some(("list", _)) => store_engine.list(),
        Some(("save", command_arguments)) => {
            let Some(key) = extract_command_argument::<String>(command_arguments, "key") else {
                LogMessage::error("key is required");
                process::exit(1);
            };

            let Some(value) = extract_command_argument::<String>(command_arguments, "value") else {
                LogMessage::error("key is required");
                process::exit(1);
            };

            let sensitive = extract_command_argument::<bool>(command_arguments, "sensitive")
                .unwrap_or_default();

            store_engine.save(&key, &value, sensitive)
        }
        //Some(("remove", _)) => store_engine.list(),
        //Some(("export", _)) => store_engine.list(),
        // Some(("sync", _)) => store_engine.list(),
        _ => process::exit(1),
    }
}
fn parse_generator_options(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("readme", command_arguments)) => {
            let GeneratorConfig {
                force,
                base_path,
                back_up,
            } = GeneratorConfig::parse_options(command_arguments);
            let _ = GeneratorConfig::new(force, base_path, back_up).generate_readme();
        }
        Some(("git-ignore", command_arguments)) => {
            let GeneratorConfig {
                force,
                base_path,
                back_up,
            } = GeneratorConfig::parse_options(command_arguments);
            let _ = GeneratorConfig::new(force, base_path, back_up).generate_ignore_file();
        }
        Some(("service", command_arguments)) => {
            let mut base_path = String::from(".");

            if let Some(base_path_flag) = command_arguments.get_one::<String>("path") {
                base_path = base_path_flag.trim().to_string();
            };

            GeneratorConfig::generate_service(&Path::new(&base_path).to_path_buf());
        }
        _ => std::process::exit(1),
    }
}
