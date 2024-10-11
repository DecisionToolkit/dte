//! # Cursor

/// Cursor shapes.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CursorShape {
  /// Cursor is represented as a vertical caret, similar to `│`.
  Caret,
  /// Cursor is represented as a block, similar to `█`.
  Block,
  /// Cursor is represented as an underscore, similar to `_`.
  UnderScore,
}

/// Cursor position and shape.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

  pub fn shape(&self) -> CursorShape {
    self.shape
  }

  /// Returns the cursor position as a tuple `(column, row)`.
  pub fn pos(&self) -> (usize, usize) {
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

  pub fn set_row(&mut self, row: usize) {
    self.row = row;
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
  pub fn is_under_score(&self) -> bool {
    matches!(self.shape, CursorShape::UnderScore)
  }

  /// Returns `true` when the cursor is signaling the inserting mode.
  pub fn insert_mode(&self) -> bool {
    self.shape == CursorShape::Caret
  }

  /// Returns `true` when the cursor is signaling the overriding mode.
  pub fn override_mode(&self) -> bool {
    self.shape != CursorShape::Caret
  }

  /// Toggles the cursor shape between caret and block.
  ///
  /// Returns the cursor shape after toggling.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::{Cursor, CursorShape};
  ///
  /// let mut cursor = Cursor::new(CursorShape::Caret, 1, 1);
  /// assert_eq!(true, cursor.is_caret());
  ///
  /// assert_eq!(CursorShape::Block, cursor.toggle_caret_block());
  /// assert_eq!(CursorShape::Caret, cursor.toggle_caret_block());
  ///
  /// let mut cursor = Cursor::new(CursorShape::UnderScore, 1, 1);
  /// assert_eq!(true, cursor.is_under_score());
  ///
  /// assert_eq!(CursorShape::Block, cursor.toggle_caret_block());
  /// assert_eq!(CursorShape::Caret, cursor.toggle_caret_block());
  /// ```
  pub fn toggle_caret_block(&mut self) -> CursorShape {
    match self.shape {
      CursorShape::Caret | CursorShape::UnderScore => self.shape = CursorShape::Block,
      _ => self.shape = CursorShape::Caret,
    }
    self.shape
  }

  /// Toggles the cursor shape between caret and underscore.
  ///
  /// Returns the cursor shape after toggling.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::{Cursor, CursorShape};
  ///
  /// let mut cursor = Cursor::new(CursorShape::Caret, 1, 1);
  /// assert_eq!(true, cursor.is_caret());
  ///
  /// assert_eq!(CursorShape::UnderScore, cursor.toggle_caret_under_score());
  /// assert_eq!(CursorShape::Caret, cursor.toggle_caret_under_score());
  ///
  /// let mut cursor = Cursor::new(CursorShape::Block, 1, 1);
  /// assert_eq!(true, cursor.is_block());
  ///
  /// assert_eq!(CursorShape::UnderScore, cursor.toggle_caret_under_score());
  /// assert_eq!(CursorShape::Caret, cursor.toggle_caret_under_score());
  /// ```
  pub fn toggle_caret_under_score(&mut self) -> CursorShape {
    match self.shape {
      CursorShape::Caret | CursorShape::Block => self.shape = CursorShape::UnderScore,
      _ => self.shape = CursorShape::Caret,
    }
    self.shape
  }
}
