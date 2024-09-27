//! # Decision table editor

mod editor;
mod keys;
mod utils;

use crate::editor::Editor;
use clap::{arg, command, ArgMatches};
use crossterm::terminal;
use std::fs;

/// Returns argument matches for command-line arguments.
fn get_matches() -> ArgMatches {
  command!()
    .arg(arg!(<INPUT_FILE>).help("File containing decision table to edit").required(true).index(1))
    .get_matches()
}

/// Starts editing provided decision table.
fn start(content: String) -> std::io::Result<()> {
  Editor::new(content)?.start()
}

/// Main entrypoint of the application.
fn main() -> std::io::Result<()> {
  // get command-line argument matches
  let matches = get_matches();
  // get the name of the file to be edited
  let file_name = matches.get_one::<String>("INPUT_FILE").unwrap().to_string();
  // read the file content as Unicode string
  if let Ok(content) = fs::read_to_string(file_name) {
    // switch the terminal to raw mode, we take the over the full control
    terminal::enable_raw_mode()?;
    // start the editor...
    let _ = start(content);
    // ... and when the user is done with editing, switch back to normal mode
    terminal::disable_raw_mode()?;
  }
  Ok(())
}
