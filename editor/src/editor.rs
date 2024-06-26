use crate::keys::{read_key, Key};
use crate::utils::*;
use crossterm::style::{Print, Stylize};
use crossterm::{execute, queue};
use dtee::{Controller, Region};
use std::cmp::min;
use std::io::{Result, Stdout, Write};

/// Minimal terminal width before locking.
const MIN_TERMINAL_WIDTH: usize = 30;

/// Minimal terminal height before locking.
const MIN_TERMINAL_HEIGHT: usize = 10;

/// Whitespace character used for filling regions.
const WS: char = ' ';

pub struct Editor {
  stdout: Stdout,
  controller: Controller,
  locked: bool,
}

impl Editor {
  /// Creates a new editor initialized with specified text.
  pub fn new(text: String) -> Result<Self> {
    let stdout = std::io::stdout();
    let (width, height) = t_size()?;
    let controller = Controller::new(text, width, height);
    Ok(Self { stdout, controller, locked: true })
  }

  /// Starts text editing loop.
  pub fn start(&mut self) -> Result<()> {
    execute!(self.stdout, t_enter_alternate_screen())?;
    execute!(self.stdout, t_clear_all())?;
    execute!(self.stdout, c_blinking_bar(), c_show())?;
    let (width, height) = self.controller.viewport().size();
    self.action_resize(width, height)?;
    loop {
      let key = read_key();
      if matches!(key, Key::CtrlQ) {
        break;
      }
      if self.locked {
        self.process_key_when_locked(key)?;
      } else {
        self.process_key_when_unlocked(key)?;
      }
    }
    execute!(self.stdout, t_leave_alternate_screen(), c_default_user_shape(), c_show())?;
    Ok(())
  }

  fn process_key_when_locked(&mut self, event: Key) -> Result<()> {
    if let Key::Resize(width, height) = event {
      self.action_resize(width, height)?
    }
    Ok(())
  }

  fn process_key_when_unlocked(&mut self, event: Key) -> Result<()> {
    match event {
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
      _ => {}
    };
    Ok(())
  }

  fn action_cursor_move_right(&mut self) -> Result<()> {
    let res = self.controller.cursor_move_right();
    self.cursor_move(res)
  }

  fn action_cursor_move_left(&mut self) -> Result<()> {
    let res = self.controller.cursor_move_left();
    self.cursor_move(res)
  }

  fn action_cursor_move_up(&mut self) -> Result<()> {
    let res = self.controller.cursor_move_up();
    self.cursor_move(res)
  }

  fn action_cursor_move_down(&mut self) -> Result<()> {
    let res = self.controller.cursor_move_down();
    self.cursor_move(res)
  }

  fn action_cursor_move_cell_start(&mut self) -> Result<()> {
    let res = self.controller.cursor_move_cell_start();
    self.cursor_move(res)
  }

  fn action_cursor_move_cell_end(&mut self) -> Result<()> {
    let res = self.controller.cursor_move_cell_end();
    self.cursor_move(res)
  }

  fn action_cursor_move_row_start(&mut self) -> Result<()> {
    let res = self.controller.cursor_move_row_start();
    self.cursor_move(res)
  }

  fn action_cursor_move_row_end(&mut self) -> Result<()> {
    let res = self.controller.cursor_move_row_end();
    self.cursor_move(res)
  }

  fn action_cursor_move_cell_next(&mut self) -> Result<()> {
    let res = self.controller.cursor_move_cell_next();
    self.cursor_move(res)
  }

  fn action_cursor_move_cell_prev(&mut self) -> Result<()> {
    let res = self.controller.cursor_move_cell_prev();
    self.cursor_move(res)
  }

  fn action_cursor_toggle_bar_block(&mut self) -> Result<()> {
    self.controller.cursor_toggle_bar_block();
    if self.controller.cursor_is_bar() {
      execute!(self.stdout, c_blinking_bar())?;
    }
    if self.controller.cursor_is_block() {
      execute!(self.stdout, c_blinking_block())?;
    }
    // refresh the character under the cursor that has changed
    if let Some(ch) = self.controller.cursor_char() {
      let (col, row) = self.controller.cursor_position();
      let (left, top) = self.controller.viewport().offset();
      execute!(self.stdout, Print(ch), c_move(col.saturating_sub(left), row.saturating_sub(top)))?;
    }
    Ok(())
  }

  fn action_resize(&mut self, width: usize, height: usize) -> Result<()> {
    let dirties = self.controller.resize(width, height);
    if width < MIN_TERMINAL_WIDTH || height < MIN_TERMINAL_HEIGHT {
      execute!(self.stdout, c_hide(), t_clear_all(), c_move(0, 0), Print("I'm squeezed!".yellow().bold()))?;
      self.locked = true;
    } else {
      if self.locked {
        self.repaint_all()?;
        execute!(self.stdout, c_show())?;
      } else {
        self.repaint(&dirties)?;
      }
      let (col, row) = self.controller.cursor_position();
      let (left, top) = self.controller.viewport().offset();
      execute!(self.stdout, c_move(col.saturating_sub(left), row.saturating_sub(top)))?;
      self.locked = false;
    }
    Ok(())
  }

  fn action_write(&mut self, _ch: char) -> Result<()> {
    //TODO implement editing action
    Ok(())
  }

  /// Repaints specified regions.
  fn repaint(&mut self, regions: &[Region]) -> Result<()> {
    if !regions.is_empty() {
      queue!(self.stdout, c_hide())?;
      let (offset_left, offset_top) = self.controller.viewport().offset();
      let text_width = self.controller.content_region().width();
      for region in regions {
        let (left, top) = region.offset();
        let (width, height) = region.size();
        for (row_index, row) in self.controller.content().iter().skip(top).take(height).enumerate() {
          let mut last_column_index = 0;
          for (col_index, ch) in row.iter().skip(left).take(width).enumerate() {
            last_column_index = col_index;
            let x = left.saturating_add(col_index).saturating_sub(offset_left);
            let y = top.saturating_add(row_index).saturating_sub(offset_top);
            queue!(self.stdout, c_move(x, y), Print(ch))?;
          }
          for col_index in last_column_index + 1..min(width, text_width) {
            let x = left.saturating_add(col_index).saturating_sub(offset_left);
            let y = top.saturating_add(row_index).saturating_sub(offset_top);
            queue!(self.stdout, c_move(x, y), Print(WS))?;
          }
        }
      }
      queue!(self.stdout, c_show())?;
      self.stdout.flush()?;
    }
    Ok(())
  }

  /// Repaints whole viewport.
  fn repaint_all(&mut self) -> Result<()> {
    self.repaint(&[*self.controller.viewport()])
  }

  fn cursor_move(&mut self, repaint: Option<bool>) -> Result<()> {
    if let Some(repaint) = repaint {
      if repaint {
        self.repaint_all()?;
      }
      self.update_cursor_position()?;
    }
    Ok(())
  }

  fn update_cursor_position(&mut self) -> Result<()> {
    let (col, row) = self.controller.cursor_position();
    let (left, top) = self.controller.viewport().offset();
    execute!(self.stdout, c_move(col.saturating_sub(left), row.saturating_sub(top)))
  }
}
