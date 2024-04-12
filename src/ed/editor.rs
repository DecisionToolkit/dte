use crate::ed::plane::Plane;
use std::io;

pub struct Editor {
  plane: Plane,
}

impl Editor {
  pub fn new(content: String) -> Self {
    Self { plane: Plane::new(content) }
  }

  pub fn repaint<F>(&self, mut handler: F) -> io::Result<()>
  where
    F: FnMut(usize, &Vec<char>) -> io::Result<()>,
  {
    for (row_index, row) in self.plane.chars.iter().enumerate() {
      handler(row_index, row)?;
    }
    Ok(())
  }

  pub fn resize(&mut self, _width: usize, _height: usize) {
    //
  }
}
