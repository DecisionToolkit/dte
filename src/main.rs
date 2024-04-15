mod ed;
mod keys;

use crate::ed::CursorType;
use crate::keys::{read_key, Key};
use crossterm::cursor::{MoveTo, SetCursorStyle};
use crossterm::style::Print;
use crossterm::terminal::{size, Clear, ClearType};
use crossterm::{execute, queue, terminal};
use ed::Editor;
use std::io::{Stdout, Write};
use std::{fs, io};

fn repaint(stdout: &mut Stdout, content: &Vec<Vec<char>>) -> io::Result<()> {
  for (row_index, row) in content.iter().enumerate() {
    for (col_index, ch) in row.iter().enumerate() {
      let _ = queue!(stdout, MoveTo(col_index as u16, row_index as u16), Print(ch));
    }
  }
  stdout.flush()
}

fn run(content: String) -> io::Result<()> {
  let mut stdout = io::stdout();
  execute!(stdout, Clear(ClearType::All))?;
  let (cols, rows) = size()?;
  let mut editor = Editor::new(content, cols as usize, rows as usize);
  repaint(&mut stdout, editor.content())?;
  execute!(stdout, SetCursorStyle::BlinkingBar, MoveTo(1, 1))?;
  loop {
    match read_key() {
      Key::Right => {
        if let Some((col, row)) = editor.cursor_move_right() {
          execute!(stdout, MoveTo(col as u16, row as u16))?;
        }
      }
      Key::Left => {
        if let Some((col, row)) = editor.cursor_move_left() {
          execute!(stdout, MoveTo(col as u16, row as u16))?;
        }
      }
      Key::Up => {
        if let Some((col, row)) = editor.cursor_move_up() {
          execute!(stdout, MoveTo(col as u16, row as u16))?;
        }
      }
      Key::Down => {
        if let Some((col, row)) = editor.cursor_move_down() {
          execute!(stdout, MoveTo(col as u16, row as u16))?;
        }
      }
      Key::Home => {
        if let Some((col, row)) = editor.cursor_move_cell_start() {
          execute!(stdout, MoveTo(col as u16, row as u16))?;
        }
      }
      Key::ShiftHome => {
        if let Some((col, row)) = editor.cursor_move_row_start() {
          execute!(stdout, MoveTo(col as u16, row as u16))?;
        }
      }
      Key::End => {
        if let Some((col, row)) = editor.cursor_move_cell_end() {
          execute!(stdout, MoveTo(col as u16, row as u16))?;
        }
      }
      Key::ShiftEnd => {
        if let Some((col, row)) = editor.cursor_move_row_end() {
          execute!(stdout, MoveTo(col as u16, row as u16))?;
        }
      }
      Key::Tab => {
        if let Some((col, row)) = editor.cursor_move_cell_next() {
          execute!(stdout, MoveTo(col as u16, row as u16))?;
        }
      }
      Key::ShiftTab => {
        if let Some((col, row)) = editor.cursor_move_cell_prev() {
          execute!(stdout, MoveTo(col as u16, row as u16))?;
        }
      }
      Key::CtrlQ => break,
      Key::Insert => {
        match editor.toggle_cursor() {
          CursorType::Bar => execute!(stdout, SetCursorStyle::BlinkingBar)?,
          CursorType::Block => execute!(stdout, SetCursorStyle::BlinkingBlock)?,
          CursorType::UnderScore => execute!(stdout, SetCursorStyle::BlinkingUnderScore)?,
        }
        // refresh character under cursor
        if let Some(ch) = editor.char_under_cursor() {
          let (col, row) = editor.cursor();
          execute!(stdout, Print(ch), MoveTo(col as u16, row as u16))?;
        }
      }
      Key::Char(ch) => execute!(stdout, Print(ch))?,
      Key::Resize(width, height) => editor.resize(width, height),
      _ => {}
    };
  }
  execute!(stdout, Clear(ClearType::All), SetCursorStyle::DefaultUserShape, MoveTo(0, 0))?;
  Ok(())
}

fn main() -> io::Result<()> {
  terminal::enable_raw_mode()?;
  let content = fs::read_to_string("./examples/e2.dtb").expect("Failed to load file");
  let _ = run(content);
  terminal::disable_raw_mode()?;
  Ok(())
}
