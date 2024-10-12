//! # Updates

/// Updates done on the underlying data model containing the edited decision table.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Updates {
  cursor_pos_changed: bool,
  cursor_shape_changed: bool,
  viewport_pos_changed: bool,
  viewport_size_changed: bool,
  content_changed: bool,
}

impl From<Updates> for (bool, bool, bool, bool, bool) {
  fn from(value: Updates) -> Self {
    (
      value.cursor_pos_changed,
      value.cursor_shape_changed,
      value.viewport_pos_changed,
      value.viewport_size_changed,
      value.content_changed,
    )
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

  pub fn viewport_size_changed(&self) -> bool {
    self.viewport_size_changed
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
