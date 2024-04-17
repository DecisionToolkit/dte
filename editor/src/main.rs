mod editor;
mod keys;
mod utils;

use crate::editor::Editor;
use crossterm::terminal;
use std::fs::read_to_string;
use std::io::Result;

fn start(content: String) -> Result<()> {
  Editor::new(content)?.start()
}

fn main() -> Result<()> {
  let content = read_to_string("./examples/e2.dtb").expect("Failed to load file");
  terminal::enable_raw_mode()?;
  let _ = start(content);
  terminal::disable_raw_mode()?;
  Ok(())
}
