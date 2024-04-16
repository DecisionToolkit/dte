//! # Cursor

/// Cursor types (shapes).
#[derive(Copy, Clone)]
pub enum CursorShape {
  /// Cursor is represented as vertical bar, similar to `│`.
  Bar,
  /// Cursor is represented as block, similar to `█`.
  Block,
  /// Cursor is represented as underscore, similar to `_`.
  UnderScore,
}

/// Cursor properties.
pub struct Cursor {
  /// Cursor shape.
  shape: CursorShape,
  /// Column position.
  col: usize,
  /// Row position.
  row: usize,
}

impl Cursor {
  pub fn new(shape: CursorShape, col: usize, row: usize) -> Self {
    Self { shape, col, row }
  }

  pub fn get(&self) -> (usize, usize) {
    (self.col, self.row)
  }

  pub fn set(&mut self, col: usize, row: usize) {
    self.col = col;
    self.row = row;
  }

  pub fn set_col(&mut self, col: usize) {
    self.col = col;
  }

  pub fn set_row(&mut self, row: usize) {
    self.row = row;
  }

  pub fn col(&self) -> usize {
    self.col
  }

  pub fn inc_col(&mut self, value: usize) {
    self.col = self.col.saturating_add(value);
  }

  pub fn dec_col(&mut self, value: usize) {
    self.col = self.col.saturating_sub(value);
  }

  pub fn adj_col(&mut self, value: isize) {
    if value < 0 {
      self.col = self.col.saturating_sub(value.unsigned_abs());
    } else {
      self.col = self.col.saturating_add(value as usize);
    }
  }

  pub fn row(&self) -> usize {
    self.row
  }

  pub fn inc_row(&mut self, value: usize) {
    self.row = self.row.saturating_add(value);
  }

  pub fn dec_row(&mut self, value: usize) {
    self.row = self.row.saturating_sub(value);
  }

  pub fn adj_row(&mut self, value: isize) {
    if value < 0 {
      self.row = self.row.saturating_sub(value.unsigned_abs());
    } else {
      self.row = self.row.saturating_add(value as usize);
    }
  }

  pub fn adjusted(&self, row_offset: isize, col_offset: isize) -> (usize, usize) {
    (
      if row_offset < 0 {
        self.row.saturating_sub(row_offset.unsigned_abs())
      } else {
        self.row.saturating_add(row_offset as usize)
      },
      if col_offset < 0 {
        self.col.saturating_sub(col_offset.unsigned_abs())
      } else {
        self.col.saturating_add(col_offset as usize)
      },
    )
  }

  /// This should be the target version.
  /// TODO fix
  pub fn adjusted_1(&self, col_offset: isize, row_offset: isize) -> (usize, usize) {
    (
      if col_offset < 0 {
        self.col.saturating_sub(col_offset.unsigned_abs())
      } else {
        self.col.saturating_add(col_offset as usize)
      },
      if row_offset < 0 {
        self.row.saturating_sub(row_offset.unsigned_abs())
      } else {
        self.row.saturating_add(row_offset as usize)
      },
    )
  }

  pub fn is_bar(&self) -> bool {
    matches!(self.shape, CursorShape::Bar)
  }

  pub fn toggle(&mut self) -> CursorShape {
    match self.shape {
      CursorShape::Bar => self.shape = CursorShape::Block,
      CursorShape::Block => self.shape = CursorShape::UnderScore,
      CursorShape::UnderScore => self.shape = CursorShape::Bar,
    }
    self.shape
  }
}
