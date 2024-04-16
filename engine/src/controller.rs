use crate::model::Model;

struct Viewport {
  left: usize,
  top: usize,
  width: usize,
  height: usize,
}

impl Viewport {
  fn new(left: usize, top: usize, width: usize, height: usize) -> Self {
    Self { left, top, width, height }
  }
}

pub struct Controller {
  /// Model containing edited text.
  model: Model,
  /// Text viewport.
  viewport: Viewport,
}

impl Controller {
  pub fn new(text: String, width: usize, height: usize) -> Self {
    Self {
      model: Model::new(text),
      viewport: Viewport::new(0, 0, width, height),
    }
  }

  pub fn resize(&mut self, width: usize, height: usize) {
    self.viewport.width = width;
    self.viewport.height = height;
  }

  /// Returns the position of the cursor in viewport's coordinates.
  pub fn cursor_position(&self) -> (usize, usize) {
    let (column, row) = self.model.cursor_position();
    (column.saturating_sub(self.viewport.left), row.saturating_sub(self.viewport.top))
  }

  pub fn rows(&self) -> &[Vec<char>] {
    self.model.rows()
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
