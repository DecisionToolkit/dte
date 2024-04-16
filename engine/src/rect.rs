#[derive(Copy, Clone)]
pub struct Rect {
  left: usize,
  top: usize,
  width: usize,
  height: usize,
}

impl Rect {
  pub fn new(left: usize, top: usize, width: usize, height: usize) -> Self {
    Self { left, top, width, height }
  }

  pub fn resize(&mut self, width: usize, height: usize) {
    self.width = width;
    self.height = height;
  }

  pub fn left(&self) -> usize {
    self.left
  }

  pub fn top(&self) -> usize {
    self.top
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }
}
