mod ed;
mod keys;

use crate::keys::{read_key, Key};
use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, SetCursorStyle};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, queue, terminal};
use ed::Editor;
use std::io::Write;
use std::{fs, io};

fn run(content: String) -> io::Result<()> {
  let mut editor = Editor::new(content);
  let mut stdout = io::stdout();
  execute!(stdout, Clear(ClearType::All), SetCursorStyle::BlinkingBar, MoveTo(0, 0))?;
  editor.repaint(|row_index, row| {
    for (col_index, ch) in row.iter().enumerate() {
      queue!(stdout, MoveTo(col_index as u16, row_index as u16), Print(ch))?;
    }
    Ok(())
  })?;
  queue!(stdout, MoveTo(1, 1))?;
  stdout.flush()?;
  loop {
    match read_key() {
      Key::Right => execute!(stdout, MoveRight(1))?,
      Key::Left => execute!(stdout, MoveLeft(1))?,
      Key::Up => execute!(stdout, MoveUp(1))?,
      Key::Down => execute!(stdout, MoveDown(1))?,
      Key::CtrlQ => break,
      Key::Char(ch) => execute!(stdout, Print(ch))?,
      Key::Resize(width, height) => editor.resize(width, height),
      _ => {}
    };
  }
  execute!(stdout, Clear(ClearType::All), SetCursorStyle::DefaultUserShape, MoveTo(0, 0))?;
  Ok(())
}

fn main() -> io::Result<()> {
  // enter raw mode, directly process each key press
  terminal::enable_raw_mode()?;
  let content = fs::read_to_string("./examples/e1.dtb").expect("Failed to load file");
  let _ = run(content);
  // return to canonical mode, process input after pressing Enter
  terminal::disable_raw_mode()?;
  Ok(())
}
