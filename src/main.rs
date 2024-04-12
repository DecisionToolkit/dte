mod editor;
mod keys;

use crate::editor::Editor;
use crate::keys::{read_key, Key};
use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, SetCursorStyle};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, terminal};
use std::io;

fn run() -> io::Result<()> {
  let mut editor = Editor;
  let mut stdout = io::stdout();
  execute!(stdout, Clear(ClearType::All), SetCursorStyle::BlinkingBar, MoveTo(0, 0))?;
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
  let _ = run();
  // return to canonical mode, process input after pressing Enter
  terminal::disable_raw_mode()?;
  Ok(())
}
