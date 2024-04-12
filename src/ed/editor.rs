use crate::ed::plane::Plane;

pub struct Editor {
  plane: Plane,
}

impl Editor {
  pub fn new(content: String) -> Self {
    Self { plane: Plane::new(content) }
  }

  pub fn repaint<F>(&self, mut handler: F)
  where
    F: FnMut(usize, &[char]),
  {
    for (row_index, row) in self.plane.chars.iter().enumerate() {
      handler(row_index, row);
    }
  }

  pub fn resize(&mut self, _width: usize, _height: usize) {
    //
  }
}
