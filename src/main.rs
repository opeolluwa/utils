use clap::{arg, command, ArgAction, Command};

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

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
        .subcommand(Command::new("generate").about("generate a new project or project files"))
        .arg(arg!( -n --name "project of file name ").action(ArgAction::SetTrue))
        .get_matches();

    match matches.subcommand() {
        Some(("store", _sub_matches)) => {
            let _ = run_store_tui();
            println!("store")
        }
        Some(("uninstall", _)) => {
            println!("uninstall")
        }
        Some(("upgrade", _)) => {
            println!("upgrade")
        }
        Some(("generate", _sub_matches)) => {
            println!("generate")
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    // println!("{:#?}", matches)
    // Continued program logic goes here...
}

fn run_store_tui() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}
