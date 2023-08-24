use include_dir::{include_dir, Dir};
use parser::Utils;
pub const SOURCE_DIR: Dir = include_dir!("src/templates");

mod commands;
mod parser;
mod utils;

fn main() {
    Utils::run();
}
