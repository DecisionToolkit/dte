mod editor;
mod keys;
mod utils;

use crate::editor::Editor;
use clap::{arg, command, ArgMatches};
use crossterm::terminal;
use std::fs;
use std::io::Result;

fn get_matches() -> ArgMatches {
  command!()
    .arg(arg!(<INPUT_FILE>).help("File containing decision table to edit").required(true).index(1))
    .get_matches()
}

fn start(content: String) -> Result<()> {
  Editor::new(content)?.start()
}

fn main() -> Result<()> {
  let matches = get_matches();
  let file_name = matches.get_one::<String>("INPUT_FILE").unwrap().to_string();
  if let Ok(content) = fs::read_to_string(file_name) {
    terminal::enable_raw_mode()?;
    let _ = start(content);
    terminal::disable_raw_mode()?;
  }
  Ok(())
}
