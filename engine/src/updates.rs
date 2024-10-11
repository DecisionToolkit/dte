//! # Updates

pub const NO_UPDATES: Updates = Updates {
  cursor_pos_changed: false,
  cursor_shape_changed: false,
  viewport_pos_changed: false,
  viewport_size_changed: false,
  content_changed: false,
};

/// Updates done on the underlying data model containing the edited decision table.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Updates {
  cursor_pos_changed: bool,
  cursor_shape_changed: bool,
  viewport_pos_changed: bool,
  viewport_size_changed: bool,
  content_changed: bool,
}

impl Default for Updates {
  fn default() -> Self {
    NO_UPDATES
  }
}

impl Updates {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_cursor_pos(mut self, changed: bool) -> Self {
    self.cursor_pos_changed = changed;
    self
  }

  pub fn with_cursor_shape(mut self, changed: bool) -> Self {
    self.cursor_shape_changed = changed;
    self
  }

  pub fn with_viewport_pos(mut self, changed: bool) -> Self {
    self.viewport_pos_changed = changed;
    self
  }

  pub fn with_viewport_size(mut self, changed: bool) -> Self {
    self.viewport_size_changed = changed;
    self
  }

  pub fn with_content_changed(mut self, changed: bool) -> Self {
    self.content_changed = changed;
    self
  }

  pub fn cursor_pos_changed(&self) -> bool {
    self.cursor_pos_changed
  }

  pub fn cursor_shape_changed(&self) -> bool {
    self.cursor_shape_changed
  }

  pub fn viewport_pos_changed(&self) -> bool {
    self.viewport_pos_changed
  }

  pub fn content_changed(&self) -> bool {
    self.content_changed
  }

  pub fn get(&self) -> (bool, bool, bool, bool) {
    (self.cursor_pos_changed, self.cursor_shape_changed, self.viewport_pos_changed, self.viewport_size_changed)
  }

  pub fn temporary(&self) -> Option<bool> {
    if self.cursor_pos_changed {
      Some(self.viewport_pos_changed || self.viewport_size_changed)
    } else {
      None
    }
  }
}
