use crate::keys::{read_key, Key};
use crossterm::cursor::{MoveTo, SetCursorStyle};
use crossterm::style::Print;
use crossterm::{execute, queue, terminal};
use dtee::Controller;
use std::io;
use std::io::{Stdout, Write};

const MIN_TERMINAL_WIDTH: usize = 20;
const MIN_TERMINAL_HEIGHT: usize = 5;

enum ResizeState {
  Normal,
  TooSmall,
}

impl From<(usize, usize)> for ResizeState {
  fn from(value: (usize, usize)) -> Self {
    if value.0 < MIN_TERMINAL_WIDTH || value.1 < MIN_TERMINAL_HEIGHT {
      Self::TooSmall
    } else {
      Self::Normal
    }
  }
}

pub struct Editor {
  stdout: Stdout,
  controller: Controller,
  resize_state: ResizeState,
}

impl Editor {
  /// Creates a new editor initialized with specified text.
  pub fn new(text: String) -> io::Result<Self> {
    let stdout = io::stdout();
    let (terminal_width, terminal_height) = terminal::size()?;
    let controller = Controller::new(text, terminal_width as usize, terminal_height as usize);
    let resize_state: ResizeState = (terminal_width as usize, terminal_height as usize).into();
    Ok(Self { stdout, controller, resize_state })
  }

  /// Starts text editing loop.
  pub fn run(&mut self) -> io::Result<()> {
    execute!(self.stdout, terminal::EnterAlternateScreen)?;
    execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
    self.repaint()?;
    execute!(self.stdout, SetCursorStyle::BlinkingBar, MoveTo(1, 1))?;
    loop {
      match read_key() {
        Key::Right => self.action_cursor_move_right()?,
        Key::Left => self.action_cursor_move_left()?,
        Key::Up => self.action_cursor_move_up()?,
        Key::Down => self.action_cursor_move_down()?,
        Key::Home => self.action_cursor_move_cell_start()?,
        Key::End => self.action_cursor_move_cell_end()?,
        Key::ShiftHome => self.action_cursor_move_row_start()?,
        Key::ShiftEnd => self.action_cursor_move_row_end()?,
        Key::Tab => self.action_cursor_move_cell_next()?,
        Key::ShiftTab => self.action_cursor_move_cell_prev()?,
        Key::Insert => self.action_cursor_toggle_bar_block()?,
        Key::Char(ch) => self.action_write(ch)?,
        Key::Resize(width, height) => self.action_resize(width, height)?,
        Key::CtrlQ => break,
        _ => {}
      };
    }
    execute!(self.stdout, terminal::LeaveAlternateScreen)?;
    Ok(())
  }

  fn action_cursor_move_right(&mut self) -> io::Result<()> {
    if let Some((column, row)) = self.controller.cursor_move_right() {
      execute!(self.stdout, MoveTo(column as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_cursor_move_left(&mut self) -> io::Result<()> {
    if let Some((column, row)) = self.controller.cursor_move_left() {
      execute!(self.stdout, MoveTo(column as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_cursor_move_up(&mut self) -> io::Result<()> {
    if let Some((col, row)) = self.controller.cursor_move_up() {
      execute!(self.stdout, MoveTo(col as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_cursor_move_down(&mut self) -> io::Result<()> {
    if let Some((col, row)) = self.controller.cursor_move_down() {
      execute!(self.stdout, MoveTo(col as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_cursor_move_cell_start(&mut self) -> io::Result<()> {
    if let Some((col, row)) = self.controller.cursor_move_cell_start() {
      execute!(self.stdout, MoveTo(col as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_cursor_move_cell_end(&mut self) -> io::Result<()> {
    if let Some((col, row)) = self.controller.cursor_move_cell_end() {
      execute!(self.stdout, MoveTo(col as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_cursor_move_row_start(&mut self) -> io::Result<()> {
    if let Some((col, row)) = self.controller.cursor_move_row_start() {
      execute!(self.stdout, MoveTo(col as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_cursor_move_row_end(&mut self) -> io::Result<()> {
    if let Some((col, row)) = self.controller.cursor_move_row_end() {
      execute!(self.stdout, MoveTo(col as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_cursor_move_cell_next(&mut self) -> io::Result<()> {
    if let Some((col, row)) = self.controller.cursor_move_cell_next() {
      execute!(self.stdout, MoveTo(col as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_cursor_move_cell_prev(&mut self) -> io::Result<()> {
    if let Some((col, row)) = self.controller.cursor_move_cell_prev() {
      execute!(self.stdout, MoveTo(col as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_cursor_toggle_bar_block(&mut self) -> io::Result<()> {
    self.controller.cursor_toggle_bar_block();
    if self.controller.cursor_is_bar() {
      execute!(self.stdout, SetCursorStyle::BlinkingBar)?;
    }
    if self.controller.cursor_is_block() {
      execute!(self.stdout, SetCursorStyle::BlinkingBlock)?;
    }
    // refresh the character under the cursor that has changed
    if let Some(ch) = self.controller.cursor_char() {
      let (column, row) = self.controller.cursor_position();
      execute!(self.stdout, Print(ch), MoveTo(column as u16, row as u16))?;
    }
    Ok(())
  }

  fn action_resize(&mut self, width: usize, height: usize) -> io::Result<()> {
    self.controller.resize(width, height);
    self.repaint()?;
    let (col, row) = self.controller.cursor_position();
    let (left, top) = self.controller.offset();
    execute!(self.stdout, MoveTo(col.saturating_sub(left) as u16, row.saturating_sub(top) as u16))?;
    Ok(())
  }

  fn action_write(&mut self, _ch: char) -> io::Result<()> {
    //TODO implement editing action
    Ok(())
  }

  /// Refreshes dirty regions.
  fn repaint(&mut self) -> io::Result<()> {
    if self.controller.is_dirty() {
      let (offset_left, offset_top) = self.controller.offset();
      for rect in self.controller.dirties() {
        let top = rect.top().saturating_sub(offset_left);
        let left = rect.left().saturating_sub(offset_top);
        for (row_index, row) in self.controller.text().iter().skip(top).take(rect.height()).enumerate() {
          for (col_index, ch) in row.iter().skip(left).take(rect.width()).enumerate() {
            let _ = queue!(self.stdout, MoveTo((left + col_index) as u16, (top + row_index) as u16), Print(ch));
          }
        }
      }
      return self.stdout.flush();
    }
    Ok(())
  }

  fn is_resize_normal(&self) -> bool {
    matches!(self.resize_state, ResizeState::Normal)
  }
}
