mod editor;
mod keys;
mod states;

use crate::editor::Editor;
use crossterm::terminal;
use std::{fs, io};

fn main() -> io::Result<()> {
  terminal::enable_raw_mode()?;
  let content = fs::read_to_string("./examples/e2.dtb").expect("Failed to load file");
  if let Ok(mut editor) = Editor::new(content) {
    let _ = editor.run();
  }
  terminal::disable_raw_mode()?;
  Ok(())
}
