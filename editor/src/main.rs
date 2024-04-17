mod editor;
mod keys;
mod states;

use crate::editor::Editor;
use crossterm::terminal;
use std::{fs, io};

fn run(content: String) -> io::Result<()> {
  Editor::new(content)?.run()
}

fn main() -> io::Result<()> {
  let content = fs::read_to_string("./examples/e2.dtb").expect("Failed to load file");
  terminal::enable_raw_mode()?;
  let _ = run(content);
  terminal::disable_raw_mode()?;
  Ok(())
}
