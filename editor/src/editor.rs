//! # Decision table editor

use crate::trigger::{read_trigger, Trigger};
use crate::utils::*;
use crossterm::style::{Print, Stylize};
use crossterm::{execute, queue};
use dtee::{Char, Controller, CursorShape, Updates, SPACE};
use std::cmp::max;
use std::io::{Result, Stdout, Write};

/// Minimal terminal width before `locking` the screen.
const MINIMAL_TERMINAL_WIDTH: usize = 40;

/// Minimal terminal height before `locking` the screen.
const MINIMAL_TERMINAL_HEIGHT: usize = 10;

/// Decision table editor.
pub struct Editor {
  stdout: Stdout,
  controller: Controller,
  locked: bool,
}

impl Editor {
  /// Creates a new editor populated with the provided text.
  pub fn new(text: String) -> Result<Self> {
    Ok(Self {
      stdout: std::io::stdout(),
      controller: Controller::new(text),
      locked: true,
    })
  }

  /// Starts text editing loop.
  pub fn start(&mut self) -> Result<()> {
    execute!(self.stdout, t_enter_alternate_screen())?;
    execute!(self.stdout, t_clear_all())?;
    execute!(self.stdout, c_blinking_bar(), c_show())?;
    let (width, height) = t_size()?;
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
      Trigger::AltInsert => self.action_cursor_toggle_caret_under_score()?,
      Trigger::Backspace => self.action_delete(true)?,
      Trigger::Char(ch) => self.action_insert_char(ch)?,
      Trigger::CtrlEnd => self.action_cursor_move_row_end()?,
      Trigger::CtrlHome => self.action_cursor_move_row_start()?,
      Trigger::CtrlPageDown => self.action_cursor_move_col_end()?,
      Trigger::CtrlPageUp => self.action_cursor_move_col_start()?,
      Trigger::Delete => self.action_delete(false)?,
      Trigger::Down => self.action_cursor_move_down()?,
      Trigger::End => self.action_cursor_move_cell_end()?,
      Trigger::Enter => self.action_split_line()?,
      Trigger::F1 => self.action_show_help()?,
      Trigger::Home => self.action_cursor_move_cell_start()?,
      Trigger::Insert => self.action_cursor_toggle_caret_block()?,
      Trigger::Left => self.action_cursor_move_left()?,
      Trigger::PageDown => self.action_cursor_move_cell_bottom()?,
      Trigger::PageUp => self.action_cursor_move_cell_top()?,
      Trigger::Resize(width, height) => self.action_resize(width, height - 1)?,
      Trigger::Right => self.action_cursor_move_right()?,
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
    let updates = self.controller.cursor_move_right();
    self.process_updates(updates)
  }

  fn action_cursor_move_left(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_left();
    self.process_updates(updates)
  }

  fn action_cursor_move_up(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_up();
    self.process_updates(updates)
  }

  fn action_cursor_move_down(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_down();
    self.process_updates(updates)
  }

  fn action_cursor_move_cell_start(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_cell_start();
    self.process_updates(updates)
  }

  fn action_cursor_move_cell_end(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_cell_end();
    self.process_updates(updates)
  }

  fn action_cursor_move_cell_top(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_cell_top();
    self.process_updates(updates)
  }

  fn action_cursor_move_cell_bottom(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_cell_bottom();
    self.process_updates(updates)
  }

  fn action_cursor_move_row_start(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_row_start();
    self.process_updates(updates)
  }

  fn action_cursor_move_row_end(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_row_end();
    self.process_updates(updates)
  }

  fn action_cursor_move_col_start(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_col_start();
    self.process_updates(updates)
  }

  fn action_cursor_move_col_end(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_col_end();
    self.process_updates(updates)
  }

  fn action_cursor_move_cell_next(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_cell_next();
    self.process_updates(updates)
  }

  fn action_cursor_move_cell_prev(&mut self) -> Result<()> {
    let updates = self.controller.cursor_move_cell_prev();
    self.process_updates(updates)
  }

  fn action_cursor_toggle_caret_block(&mut self) -> Result<()> {
    self.controller.cursor_toggle_caret_block();
    self.update_cursor_shape()?;
    self.update_cursor_position()
  }

  fn action_cursor_toggle_caret_under_score(&mut self) -> Result<()> {
    self.controller.cursor_toggle_caret_under_score();
    self.update_cursor_shape()?;
    self.update_cursor_position()
  }

  fn action_delete(&mut self, backspace: bool) -> Result<()> {
    let updates = if backspace {
      self.controller.delete_char_before_cursor()
    } else {
      self.controller.delete_char_under_cursor()
    };
    self.process_updates(updates)
  }

  fn action_insert_char(&mut self, ch: char) -> Result<()> {
    let updates = self.controller.insert_char(ch);
    self.process_updates(updates)
  }

  /// Splits the line at the cursor position.
  fn action_split_line(&mut self) -> Result<()> {
    let updates = self.controller.split_line();
    self.process_updates(updates)
  }

  fn action_resize(&mut self, width: usize, height: usize) -> Result<()> {
    if width < MINIMAL_TERMINAL_WIDTH || height < MINIMAL_TERMINAL_HEIGHT {
      self.controller.resize(max(width, MINIMAL_TERMINAL_WIDTH), max(height, MINIMAL_TERMINAL_HEIGHT));
      execute!(self.stdout, c_hide(), t_clear_all(), c_move(2, 1), Print("I'm squeezed! 🍋".yellow().bold()))?;
      self.locked = true;
    } else {
      self.controller.resize(width, height);
      if self.locked {
        execute!(self.stdout, t_clear_all())?;
      }
      self.repaint_all()?;
      if self.locked {
        execute!(self.stdout, c_show())?;
      }
      let (column, row) = self.controller.cursor().pos();
      let (left, top) = self.controller.viewport().offset();
      execute!(self.stdout, c_move(column.saturating_sub(left), row.saturating_sub(top)))?;
      self.locked = false;
    }
    Ok(())
  }

  /// Repaints the viewport.
  fn repaint_all(&mut self) -> Result<()> {
    queue!(self.stdout, c_hide())?;
    let f = |col_index, row_index, chr: &Char| {
      let _ = queue!(self.stdout, c_move(col_index, row_index), Print(chr));
    };
    self.controller.visit_visible_content(f, Some(SPACE.into()), Some(1), Some(1));
    queue!(self.stdout, c_show())?;
    self.stdout.flush()?;
    Ok(())
  }

  /// Processes all pending updates.
  fn process_updates(&mut self, updates: Updates) -> Result<()> {
    if updates.viewport_pos_changed() || updates.content_changed() {
      self.repaint_all()?;
      self.update_cursor_position()?;
    } else if updates.cursor_pos_changed() {
      self.update_cursor_position()?;
    }
    Ok(())
  }

  fn update_cursor_position(&mut self) -> Result<()> {
    let (col_index, row_index) = self.controller.cursor().pos();
    let (offset_left, offset_top) = self.controller.viewport().offset();
    execute!(self.stdout, c_move(col_index.saturating_sub(offset_left), row_index.saturating_sub(offset_top)))
  }

  fn update_cursor_shape(&mut self) -> Result<()> {
    match self.controller.cursor().shape() {
      CursorShape::Caret => execute!(self.stdout, c_blinking_bar()),
      CursorShape::Block => execute!(self.stdout, c_blinking_block()),
      CursorShape::UnderScore => execute!(self.stdout, c_blinking_under_score()),
    }
  }
}
