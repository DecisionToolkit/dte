//! # Decision table editor

use crate::trigger::{read_trigger, Trigger};
use crate::utils::*;
use crossterm::style::{Print, Stylize};
use crossterm::{execute, queue};
use dtee::{Controller, Region, SPACE};
use std::cmp::min;
use std::io::{Result, Stdout, Write};

/// Minimal terminal width before `locking` the screen.
const MINIMAL_TERMINAL_WIDTH: usize = 30;

/// Minimal terminal height before `locking` the screen.
const MINIMAL_TERMINAL_HEIGHT: usize = 10;

/// Decision table editor.
pub struct Editor {
  stdout: Stdout,
  controller: Controller,
  locked: bool,
}

impl Editor {
  /// Creates a new editor populated with specified text.
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
      let key = read_trigger();
      if matches!(key, Trigger::Exit) {
        break;
      }
      if self.locked {
        self.process_trigger_when_locked_screen(key)?;
      } else {
        self.process_trigger_when_unlocked_screen(key)?;
      }
    }
    execute!(self.stdout, t_leave_alternate_screen(), c_default_user_shape(), c_show())?;
    Ok(())
  }

  /// Processes a trigger when the screen is locked (too small).
  fn process_trigger_when_locked_screen(&mut self, trigger: Trigger) -> Result<()> {
    if let Trigger::Resize(width, height) = trigger {
      self.action_resize(width, height)?
    }
    Ok(())
  }

  /// Processes a trigger when the screen is unlocked (normal state).
  fn process_trigger_when_unlocked_screen(&mut self, trigger: Trigger) -> Result<()> {
    match trigger {
      Trigger::Backspace => self.action_delete(true)?,
      Trigger::Char(ch) => self.action_insert_char(ch)?,
      Trigger::Delete => self.action_delete(false)?,
      Trigger::Down => self.action_cursor_move_down()?,
      Trigger::End => self.action_cursor_move_cell_end()?,
      Trigger::Enter => self.action_split_line()?,
      Trigger::F1 => self.action_show_help()?,
      Trigger::Home => self.action_cursor_move_cell_start()?,
      Trigger::Insert => self.action_cursor_toggle_bar_block()?,
      Trigger::Left => self.action_cursor_move_left()?,
      Trigger::Resize(width, height) => self.action_resize(width, height)?,
      Trigger::Right => self.action_cursor_move_right()?,
      Trigger::ShiftEnd => self.action_cursor_move_row_end()?,
      Trigger::ShiftHome => self.action_cursor_move_row_start()?,
      Trigger::ShiftTab => self.action_cursor_move_cell_prev()?,
      Trigger::Tab => self.action_cursor_move_cell_next()?,
      Trigger::Up => self.action_cursor_move_up()?,
      _ => {}
    };
    Ok(())
  }

  fn action_show_help(&mut self) -> Result<()> {
    //TODO Implement displaying a screen with the help.
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
    if self.controller.cursor_is_caret() {
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

  fn action_delete(&mut self, backspace: bool) -> Result<()> {
    if backspace {
      self.controller.delete_char_before_cursor();
    } else {
      self.controller.delete_char_under_cursor();
    }
    self.repaint_all()?;
    self.update_cursor_position()?;
    Ok(())
  }

  fn action_resize(&mut self, new_width: usize, new_height: usize) -> Result<()> {
    if new_width < MINIMAL_TERMINAL_WIDTH || new_height < MINIMAL_TERMINAL_HEIGHT {
      execute!(self.stdout, c_hide(), t_clear_all(), c_move(0, 0), Print(" I'm squeezed! 🍋".yellow().bold()))?;
      self.locked = true;
    } else {
      self.repaint_all()?;
      if self.locked {
        execute!(self.stdout, c_show())?;
      }
      let (col, row) = self.controller.cursor_position();
      let (left, top) = self.controller.viewport().offset();
      execute!(self.stdout, c_move(col.saturating_sub(left), row.saturating_sub(top)))?;
      self.locked = false;
    }
    Ok(())
  }

  fn action_insert_char(&mut self, ch: char) -> Result<()> {
    self.controller.insert_char(ch);
    self.repaint_all()?;
    self.update_cursor_position()?;
    Ok(())
  }

  /// Splits the line at the cursor position.
  fn action_split_line(&mut self) -> Result<()> {
    self.controller.split_line();
    self.repaint_all()?;
    self.update_cursor_position()?;
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
        let mut last_row_index = 0;
        for (row_index, row) in self.controller.content().iter().skip(top).take(height).enumerate() {
          let mut last_column_index = 0;
          for (col_index, ch) in row.iter().skip(left).take(width).enumerate() {
            last_column_index = col_index;
            let x = left.saturating_add(col_index).saturating_sub(offset_left);
            let y = top.saturating_add(row_index).saturating_sub(offset_top);
            queue!(self.stdout, c_move(x, y), Print(ch))?;
          }
          for col_index in last_column_index + 1..min(width, text_width + 1) {
            // trick with additional space to clear lines while shrinking width ---^^
            let x = left.saturating_add(col_index).saturating_sub(offset_left);
            let y = top.saturating_add(row_index).saturating_sub(offset_top);
            queue!(self.stdout, c_move(x, y), Print(SPACE))?;
          }
          last_row_index += 1;
        }
        // trick with additional row to clear lines while shrinking height
        for x in 0..min(width, text_width + 1) {
          queue!(self.stdout, c_move(x, last_row_index), Print(SPACE))?;
        }
      }
      queue!(self.stdout, c_show())?;
      self.stdout.flush()?;
    }
    Ok(())
  }

  /// Repaints the whole viewport.
  fn repaint_all(&mut self) -> Result<()> {
    self.repaint(&[self.controller.viewport()])
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
    let (col_index, row_index) = self.controller.cursor_position();
    let (offset_left, offset_top) = self.controller.viewport().offset();
    execute!(self.stdout, c_move(col_index.saturating_sub(offset_left), row_index.saturating_sub(offset_top)))
  }
}
