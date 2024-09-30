use utils_style::style::LogMessage;
use ::std::io::BufRead;
use anyhow::Result;
use online::check;
use std::env;
use std::fs;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;
use std::str;
pub struct CliCommands;

impl CliCommands {
    /// upadate the cli
    /*
    to upgrade the CLI,
    see if internet connection exists
    ....

    see if cargo exist, if yes
    do cargo install utils-cli, this will automatically make the upgrade
    // else, log an error that cargo is required.
    //FIXME: stabilize npm support

     */
    pub async fn upgrade() -> anyhow::Result<()> {
        let internet_connection_exist: bool = check(None).is_ok();
        if !internet_connection_exist {
            LogMessage::error("Please connect to the internet and retry");
            std::process::exit(1)
        }

        // internet exist, see if cargo or npm exist
        let cargo_exist = is_program_in_path("cargo");
        let npm_exists = is_program_in_path("npm");

        if !cargo_exist && !npm_exists {
            LogMessage::error("Cargo or NPM is required");
            std::process::exit(1)
        }

        // upgrade with cargo
        //https://stackoverflow.com/questions/72750736/run-command-stream-stdout-stderr-and-capture-results

        let mut output = Command::new("cargo")
            .args(["install", "utils-cli"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stream_output = output.stdout.take().unwrap();

        // Stream output.
        let lines = BufReader::new(stream_output).lines();
        for line in lines {
            println!("{}", line.unwrap());
        }

        Ok(())

        // upgrade with npm
    }

    /// self destruct the cli
    pub async fn uninstall() -> Result<()> {
        // internet exist, see if cargo or npm exist
        let cargo_exist = is_program_in_path("cargo");
        let npm_exists = is_program_in_path("npm");

        if !cargo_exist && !npm_exists {
            LogMessage::error("Cargo or NPM is required");
            std::process::exit(1)
        }

        // upgrade with cargo
        //https://stackoverflow.com/questions/72750736/run-command-stream-stdout-stderr-and-capture-results

        let mut output = Command::new("cargo")
            .args(["uninstall", "utils-cli"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stream_output = output.stdout.take().unwrap();

        // Stream output.
        let lines = BufReader::new(stream_output).lines();
        for line in lines {
            println!("{}", line.unwrap());
        }

        Ok(())
    }

    /// back up the data in the store to a remote server
    pub async fn sync() {
        println!("synchrinizing data")
    }
}

/// see if a program exist in the path
/// solution adapted from https://stackoverflow.com/questions/35045996/check-if-a-command-is-in-path-executable-as-process
/// supports windows, and unix based system

#[cfg(target_family = "unix")] // for Linux and MacOs

fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

#[cfg(target_family = "windows")] // for windows
fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            // window uses ";" as path seperator
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}
