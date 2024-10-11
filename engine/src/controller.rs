//! # Controller

use crate::model::{Plane, Row};
use crate::region::Region;
use crate::updates::Updates;
use crate::{Char, Cursor, NO_UPDATES};

/// Handler for edit operations.
pub struct Controller {
  /// Edited textual content.
  plane: Plane,
  /// Visible content viewport.
  viewport: Region,
  margin_left: usize,
  margin_right: usize,
  margin_top: usize,
  margin_bottom: usize,
}

impl Controller {
  /// Creates a new controller with specified content.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Controller;
  ///
  /// let controller = Controller::new("…");
  /// assert!(!controller.content().is_empty());
  /// ```
  pub fn new<T: ToString>(content: T) -> Self {
    let mut plane = Plane::new(content);
    let viewport = *plane.region();
    Self {
      plane,
      viewport,
      margin_left: 1,
      margin_right: 2,
      margin_top: 1,
      margin_bottom: 2,
    }
  }

  pub fn with_viewport(mut self, width: usize, height: usize) -> Self {
    self.viewport = Region::new(0, 0, width, height);
    self
  }

  pub fn with_margins(mut self, top: usize, right: usize, bottom: usize, left: usize) -> Self {
    self.margin_left = left;
    self.margin_right = right;
    self.margin_top = top;
    self.margin_bottom = bottom;
    self
  }

  pub fn viewport(&self) -> &Region {
    &self.viewport
  }

  /// Resizes the viewport.
  pub fn resize(&mut self, width: usize, height: usize) -> Updates {
    let cursor_before = *self.cursor();
    self.viewport.resize(width, height);
    self.updates(cursor_before)
  }

  /// Returns a reference to the cursor.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Controller;
  ///
  /// let controller = Controller::new("…");
  /// let (column, row) = controller.cursor().pos();
  /// assert_eq!(1, column);
  /// assert_eq!(1, row);
  /// ```
  pub fn cursor(&self) -> &Cursor {
    self.plane.cursor()
  }

  pub fn content(&self) -> &[Row] {
    self.plane.content()
  }

  pub fn visit_visible_content<F>(&self, mut f: F, fill: Option<Char>, ext_width: Option<usize>, ext_height: Option<usize>)
  where
    F: FnMut(usize, usize, &Char),
  {
    let (left, top, width, height) = self.viewport.rect();
    let mut last_row_index = 0;
    for row in self.plane.content().iter().skip(top).take(height) {
      let mut last_col_index = 0;
      for chr in row.iter().skip(left).take(width) {
        f(last_col_index, last_row_index, chr);
        last_col_index += 1;
      }
      if let Some(chr_fill) = fill.as_ref() {
        let extended_width = width + ext_width.unwrap_or(0);
        for ix_col in last_col_index..extended_width {
          f(ix_col, last_row_index, chr_fill);
        }
      }
      last_row_index += 1;
    }
    if let Some(chr_fill) = fill.as_ref() {
      let extended_height = height + ext_height.unwrap_or(0);
      for ix_row in last_row_index..extended_height {
        for ix_col in 0..width {
          f(ix_col, ix_row, chr_fill);
        }
      }
    }
  }

  pub fn content_region(&mut self) -> &Region {
    self.plane.region()
  }

  pub fn cursor_move_left(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_left() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_right(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_right() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_up(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_up() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_down(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_down() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_cell_start(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_cell_start() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_cell_end(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_cell_end() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_cell_top(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_cell_top() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_cell_bottom(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_cell_bottom() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_cell_next(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_cell_next() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_cell_prev(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_cell_prev() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_row_start(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_row_start() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_row_end(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    if self.plane.cursor_move_row_end() {
      self.updates(cursor_before)
    } else {
      NO_UPDATES
    }
  }

  pub fn cursor_move_col_start(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    self.plane.cursor_move_col_start();
    self.updates(cursor_before)
  }

  pub fn cursor_move_col_end(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    self.plane.cursor_move_col_end();
    self.updates(cursor_before)
  }

  pub fn cursor_toggle_caret_block(&mut self) {
    self.plane.cursor_toggle_caret_block();
  }

  pub fn cursor_toggle_caret_under_score(&mut self) {
    self.plane.cursor_toggle_caret_under_score();
  }

  /// Returns the character pointed by the cursor.
  pub fn cursor_char(&self) -> Option<char> {
    self.plane.cursor_char().map(|ch| ch.char())
  }

  /// Inserts a single character.
  pub fn insert_char(&mut self, ch: char) -> Updates {
    let cursor_before = *self.plane.cursor();
    let content_changed = if cursor_before.insert_mode() {
      self.plane.insert_char(ch)
    } else if cursor_before.override_mode() {
      self.plane.override_char(ch)
    } else {
      false
    };
    self.updates(cursor_before).with_content_changed(content_changed)
  }

  /// Splits the line at the cursor position.
  pub fn split_line(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    let changed = self.plane.split_line();
    self.updates(cursor_before).with_content_changed(changed)
  }

  /// Deletes a single character before the cursor.
  pub fn delete_char_before_cursor(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    let changed = self.plane.delete_char_before_cursor();
    self.updates(cursor_before).with_content_changed(changed)
  }

  /// Deletes a single character under the cursor.
  pub fn delete_char_under_cursor(&mut self) -> Updates {
    let cursor_before = *self.plane.cursor();
    let changed = self.plane.delete_char_under_cursor();
    self.updates(cursor_before).with_content_changed(changed)
  }

  fn updates(&mut self, cursor_before: Cursor) -> Updates {
    let updates = Updates::new()
      .with_cursor_pos(self.cursor().pos() != cursor_before.pos())
      .with_cursor_shape(self.cursor().shape() != cursor_before.shape())
      .with_viewport_pos(
        self.viewport.shift_left_when_needed(self.cursor().col(), self.margin_left)
          || self.viewport.shift_right_when_needed(self.cursor().col(), self.margin_right)
          || self.viewport.shift_up_when_needed(self.cursor().row(), self.margin_top)
          || self.viewport.shift_down_when_needed(self.cursor().row(), self.margin_bottom),
      )
      .with_viewport_size(false);
    updates
  }
}
