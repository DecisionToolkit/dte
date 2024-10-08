//! # Cursor

/// Cursor shapes.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum CursorShape {
  /// Cursor is represented as a vertical caret, similar to `│`.
  Caret,
  /// Cursor is represented as a block, similar to `█`.
  Block,
  /// Cursor is represented as an underscore, similar to `_`.
  UnderScore,
}

/// Cursor position and shape.
pub struct Cursor {
  /// Cursor shape.
  shape: CursorShape,
  /// Horizontal position of the cursor (column).
  column: usize,
  /// Vertical position of the cursor (row).
  row: usize,
}

impl Cursor {
  pub fn new(shape: CursorShape, column: usize, row: usize) -> Self {
    Self { shape, column, row }
  }

  /// Returns the cursor coordinates as a tuple `(column, row)`.
  pub fn get(&self) -> (usize, usize) {
    (self.column, self.row)
  }

  pub fn set(&mut self, col: usize, row: usize) {
    self.column = col;
    self.row = row;
  }

  pub fn set_col(&mut self, col: usize) {
    self.column = col;
  }

  pub fn col(&self) -> usize {
    self.column
  }

  pub fn inc_col(&mut self, value: usize) {
    self.column = self.column.saturating_add(value);
  }

  pub fn dec_col(&mut self, value: usize) {
    self.column = self.column.saturating_sub(value);
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

  /// Calculates the cursor position after applying specified offsets.
  pub fn offset(&self, column_offset: isize, row_offset: isize) -> (usize, usize) {
    (
      if column_offset < 0 {
        self.column.saturating_sub(column_offset.unsigned_abs())
      } else {
        self.column.saturating_add(column_offset as usize)
      },
      if row_offset < 0 {
        self.row.saturating_sub(row_offset.unsigned_abs())
      } else {
        self.row.saturating_add(row_offset as usize)
      },
    )
  }

  /// Returns `true` when the current cursor shape is a bar (`│`).
  pub fn is_caret(&self) -> bool {
    matches!(self.shape, CursorShape::Caret)
  }

  /// Returns `true` when the current cursor shape is a block (`█`).
  pub fn is_block(&self) -> bool {
    matches!(self.shape, CursorShape::Block)
  }

  /// Returns `true` when the current cursor shape is an underscore (`_`).
  pub fn is_underscore(&self) -> bool {
    matches!(self.shape, CursorShape::UnderScore)
  }

  pub fn toggle(&mut self) {
    match self.shape {
      CursorShape::Caret => self.shape = CursorShape::Block,
      CursorShape::Block => self.shape = CursorShape::UnderScore,
      CursorShape::UnderScore => self.shape = CursorShape::Caret,
    }
  }

  pub fn toggle_bar_block(&mut self) {
    match self.shape {
      CursorShape::Caret => self.shape = CursorShape::Block,
      _ => self.shape = CursorShape::Caret,
    }
  }
}
