use crate::model::Model;
use crate::region::Region;

pub struct Controller {
  /// Edited textual content.
  model: Model,
  /// Visible content viewport.
  viewport: Region,
}

impl Controller {
  pub fn new(text: String, width: usize, height: usize) -> Self {
    let model = Model::new(text);
    let viewport = Region::new(0, 0, width, height);
    Self { model, viewport }
  }

  pub fn viewport(&self) -> &Region {
    &self.viewport
  }

  pub fn resize(&mut self, width: usize, height: usize) -> Vec<Region> {
    let mut dirties = vec![];
    if width > self.viewport.width() {
      dirties.push(Region::new(self.viewport.width(), 0, width.saturating_sub(self.viewport.width()), height));
    }
    if height > self.viewport.height() {
      dirties.push(Region::new(0, self.viewport.height(), width, height.saturating_sub(self.viewport.height())));
    }
    self.viewport.resize(width, height);
    dirties
  }

  /// Returns the cursor position in the text coordinates.
  pub fn cursor_position(&self) -> (usize, usize) {
    self.model.cursor_position()
  }

  pub fn text(&self) -> &[Vec<char>] {
    self.model.text()
  }

  pub fn cursor_move_right(&mut self) -> Option<(usize, usize)> {
    if self.model.cursor_move_right() {
      return Some(self.cursor_position());
    }
    None
  }

  pub fn cursor_move_left(&mut self) -> Option<(usize, usize)> {
    if self.model.cursor_move_left() {
      return Some(self.cursor_position());
    }
    None
  }

  pub fn cursor_move_up(&mut self) -> Option<(usize, usize)> {
    if self.model.cursor_move_up() {
      return Some(self.cursor_position());
    }
    None
  }

  pub fn cursor_move_down(&mut self) -> Option<(usize, usize)> {
    if self.model.cursor_move_down() {
      return Some(self.cursor_position());
    }
    None
  }

  pub fn cursor_move_cell_start(&mut self) -> Option<(usize, usize)> {
    if self.model.cursor_move_cell_start() {
      return Some(self.cursor_position());
    }
    None
  }

  pub fn cursor_move_cell_end(&mut self) -> Option<(usize, usize)> {
    if self.model.cursor_move_cell_end() {
      return Some(self.cursor_position());
    }
    None
  }

  pub fn cursor_move_cell_next(&mut self) -> Option<(usize, usize)> {
    if self.model.cursor_move_cell_next() {
      return Some(self.cursor_position());
    }
    None
  }

  pub fn cursor_move_cell_prev(&mut self) -> Option<(usize, usize)> {
    if self.model.cursor_move_cell_prev() {
      return Some(self.cursor_position());
    }
    None
  }

  pub fn cursor_move_row_start(&mut self) -> Option<(usize, usize)> {
    if self.model.cursor_move_row_start() {
      return Some(self.cursor_position());
    }
    None
  }

  pub fn cursor_move_row_end(&mut self) -> Option<(usize, usize)> {
    if self.model.cursor_move_row_end() {
      return Some(self.cursor_position());
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
}
