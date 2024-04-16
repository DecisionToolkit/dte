use crate::cursor::CursorShape;
use crate::plane::Plane;

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

pub struct Editor {
  /// Plane containing the edited content.
  plane: Plane,
  /// Editor's viewport.
  viewport: Viewport,
}

impl Editor {
  pub fn new(content: String, width: usize, height: usize) -> Self {
    Self {
      plane: Plane::new(content),
      viewport: Viewport::new(0, 0, width, height),
    }
  }

  pub fn resize(&mut self, width: usize, height: usize) {
    self.viewport.width = width;
    self.viewport.height = height;
  }

  /// Returns the position of the cursor in viewport's coordinates.
  pub fn cursor(&self) -> (usize, usize) {
    let (col, row) = self.plane.cursor();
    (col.saturating_sub(self.viewport.left), row.saturating_sub(self.viewport.top))
  }

  pub fn content(&self) -> &Vec<Vec<char>> {
    self.plane.content()
  }

  pub fn cursor_move_right(&mut self) -> Option<(usize, usize)> {
    if self.plane.cursor_move_right() {
      return Some(self.cursor());
    }
    None
  }

  pub fn cursor_move_left(&mut self) -> Option<(usize, usize)> {
    if self.plane.cursor_move_left() {
      return Some(self.cursor());
    }
    None
  }

  pub fn cursor_move_up(&mut self) -> Option<(usize, usize)> {
    if self.plane.cursor_move_up() {
      return Some(self.cursor());
    }
    None
  }

  pub fn cursor_move_down(&mut self) -> Option<(usize, usize)> {
    if self.plane.cursor_move_down() {
      return Some(self.cursor());
    }
    None
  }

  pub fn cursor_move_cell_start(&mut self) -> Option<(usize, usize)> {
    if self.plane.cursor_move_cell_start() {
      return Some(self.cursor());
    }
    None
  }

  pub fn cursor_move_cell_end(&mut self) -> Option<(usize, usize)> {
    if self.plane.cursor_move_cell_end() {
      return Some(self.cursor());
    }
    None
  }

  pub fn cursor_move_cell_next(&mut self) -> Option<(usize, usize)> {
    if self.plane.cursor_move_cell_next() {
      return Some(self.cursor());
    }
    None
  }

  pub fn cursor_move_cell_prev(&mut self) -> Option<(usize, usize)> {
    if self.plane.cursor_move_cell_prev() {
      return Some(self.cursor());
    }
    None
  }

  pub fn cursor_move_row_start(&mut self) -> Option<(usize, usize)> {
    if self.plane.cursor_move_row_start() {
      return Some(self.cursor());
    }
    None
  }

  pub fn cursor_move_row_end(&mut self) -> Option<(usize, usize)> {
    if self.plane.cursor_move_row_end() {
      return Some(self.cursor());
    }
    None
  }

  pub fn toggle_cursor(&mut self) -> CursorShape {
    self.plane.toggle_cursor()
  }

  /// Returns the character pointed by cursor.
  pub fn char_under_cursor(&self) -> Option<char> {
    self.plane.char_under_cursor()
  }
}
