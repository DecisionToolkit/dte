use crate::model::Model;
use crate::rect::Rect;
use std::cmp::min;

pub struct Controller {
  /// Model containing edited text.
  model: Model,
  /// Text viewport.
  viewport: Rect,
  /// Dirty parts of the viewport to be refreshed in view.
  dirty: Vec<Rect>,
}

impl Controller {
  pub fn new(text: String, width: usize, height: usize) -> Self {
    let model = Model::new(text);
    let viewport = Rect::new(0, 0, width, height);
    let (columns, rows) = model.size();
    let dirty = vec![Rect::new(0, 0, min(width, columns), min(height, rows))];
    Self { model, viewport, dirty }
  }

  pub fn dirty(&self) -> &[Rect] {
    &self.dirty
  }

  pub fn resize(&mut self, width: usize, height: usize) {
    self.dirty.clear();
    let old_width = self.viewport.width();
    let old_height = self.viewport.height();
    if width > old_width {
      self.dirty.push(Rect::new(old_width, 0, width - old_width, height));
    }
    if height > old_height {
      self.dirty.push(Rect::new(0, old_height, width, height - old_height));
    }
    self.viewport.resize(width, height);
  }

  /// Returns the position of the cursor in viewport's coordinates.
  pub fn cursor_position(&self) -> (usize, usize) {
    let (column, row) = self.model.cursor_position();
    (column.saturating_sub(self.viewport.left()), row.saturating_sub(self.viewport.top()))
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
