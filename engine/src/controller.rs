//! # Controller

use crate::model::Model;
use crate::region::Region;

pub struct Controller {
  /// Edited textual content.
  model: Model,
  /// Visible content viewport.
  viewport: Region,
}

impl Controller {
  /// Creates a new controller with specified content and viewing area.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Controller;
  ///
  /// # let decision_table = "...".to_string();
  /// let controller = Controller::new(decision_table, 200, 100);
  /// assert!(!controller.content().is_empty());
  /// ```
  pub fn new(content: String, width: usize, height: usize) -> Self {
    let model = Model::new(content);
    let viewport = Region::new(0, 0, width, height);
    Self { model, viewport }
  }

  pub fn viewport(&self) -> &Region {
    &self.viewport
  }

  pub fn resize(&mut self, new_width: usize, new_height: usize) -> Vec<Region> {
    let mut dirties = vec![];
    if new_width > self.viewport.width() {
      dirties.push(Region::new(self.viewport.width(), 0, new_width.saturating_sub(self.viewport.width()), new_height));
    }
    if new_height > self.viewport.height() {
      dirties.push(Region::new(0, self.viewport.height(), new_width, new_height.saturating_sub(self.viewport.height())));
    }
    self.viewport.resize(new_width, new_height);
    dirties
  }

  /// Returns the cursor position in model coordinates.
  pub fn cursor_position(&self) -> (usize, usize) {
    self.model.cursor_position()
  }

  pub fn content(&self) -> &[Vec<char>] {
    self.model.content()
  }

  pub fn content_region(&mut self) -> Region {
    self.model.content_region()
  }

  pub fn cursor_move_left(&mut self) -> Option<bool> {
    if self.model.cursor_move_left() {
      return self.cursor_moved_left();
    }
    None
  }

  pub fn cursor_move_right(&mut self) -> Option<bool> {
    if self.model.cursor_move_right() {
      return self.cursor_moved_right();
    }
    None
  }

  pub fn cursor_move_up(&mut self) -> Option<bool> {
    if self.model.cursor_move_up() {
      return self.cursor_moved_up();
    }
    None
  }

  pub fn cursor_move_down(&mut self) -> Option<bool> {
    if self.model.cursor_move_down() {
      return self.cursor_moved_down();
    }
    None
  }

  pub fn cursor_move_cell_start(&mut self) -> Option<bool> {
    if self.model.cursor_move_cell_start() {
      return self.cursor_moved_left();
    }
    None
  }

  pub fn cursor_move_cell_end(&mut self) -> Option<bool> {
    if self.model.cursor_move_cell_end() {
      return self.cursor_moved_right();
    }
    None
  }

  pub fn cursor_move_cell_next(&mut self) -> Option<bool> {
    if self.model.cursor_move_cell_next() {
      return self.cursor_moved_right();
    }
    None
  }

  pub fn cursor_move_cell_prev(&mut self) -> Option<bool> {
    if self.model.cursor_move_cell_prev() {
      return self.cursor_moved_left();
    }
    None
  }

  pub fn cursor_move_row_start(&mut self) -> Option<bool> {
    if self.model.cursor_move_row_start() {
      return self.cursor_moved_left();
    }
    None
  }

  pub fn cursor_move_row_end(&mut self) -> Option<bool> {
    if self.model.cursor_move_row_end() {
      return self.cursor_moved_right();
    }
    None
  }

  pub fn cursor_toggle(&mut self) {
    self.model.cursor_toggle();
  }

  pub fn cursor_toggle_bar_block(&mut self) {
    self.model.cursor_toggle_bar_block();
  }

  pub fn cursor_is_bar(&self) -> bool {
    self.model.cursor_is_bar()
  }

  pub fn cursor_is_block(&self) -> bool {
    self.model.cursor_is_block()
  }

  pub fn cursor_is_underscore(&self) -> bool {
    self.model.cursor_is_underscore()
  }

  /// Returns the character pointed by cursor.
  pub fn cursor_char(&self) -> Option<char> {
    self.model.cursor_char()
  }

  fn cursor_moved_left(&mut self) -> Option<bool> {
    let (x, _) = self.cursor_position();
    let minimum = self.content_region().left();
    if self.viewport.move_left(x, minimum) {
      return Some(true);
    }
    Some(false)
  }

  fn cursor_moved_right(&mut self) -> Option<bool> {
    let (x, _) = self.cursor_position();
    let maximum = self.content_region().right();
    if self.viewport.move_right(x, maximum) {
      return Some(true);
    }
    Some(false)
  }

  fn cursor_moved_up(&mut self) -> Option<bool> {
    let (_, y) = self.cursor_position();
    let minimum = self.content_region().top();
    if self.viewport.move_up(y, minimum) {
      return Some(true);
    }
    Some(false)
  }

  fn cursor_moved_down(&mut self) -> Option<bool> {
    let (_, y) = self.cursor_position();
    let maximum = self.content_region().bottom();
    if self.viewport.move_down(y, maximum) {
      return Some(true);
    }
    Some(false)
  }
}
