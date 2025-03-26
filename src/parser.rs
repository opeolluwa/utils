use clap::ArgMatches;

use crate::commands::generator::Generator;

pub fn parse_commands(matches: ArgMatches) {
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
}
