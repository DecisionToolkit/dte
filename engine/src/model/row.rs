//! # Single row of text

use crate::{Char, SPACE};
use std::ops::{Deref, DerefMut};

/// A single row of text.
#[derive(Debug, Clone, PartialEq)]
pub struct Row(Vec<Char>);

impl Deref for Row {
  type Target = Vec<Char>;

  /// Implements the [Deref] trait for [Row].
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Row {
  /// Implements the [DerefMut] trait for [Row].
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T: ToString> From<T> for Row {
  /// Implements the [From] trait for [Row] for all types that implement [ToString].
  fn from(value: T) -> Self {
    Self(value.to_string().chars().map(Into::into).collect::<Vec<Char>>())
  }
}

impl Row {
  pub fn new(width: usize, ch: char) -> Self {
    let mut chars = Vec::with_capacity(width);
    for _ in 0..width {
      chars.push(Char::from(ch));
    }
    Self(chars)
  }

  /// Checks if the row is the joining line between information item name
  /// and the decision table's body.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Row;
  ///
  /// let row: Row = "".into();
  /// assert!(!row.is_join());
  /// ```
  pub fn is_join(&self) -> bool {
    self.0.first().map_or(false, |chr| chr.is_join())
  }

  pub fn set_join(&self) {
    self.0.iter().for_each(|chr| chr.set_join());
  }

  /// Checks if the row is fully joining line between information item name
  /// and the decision table's body.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Row;
  ///
  /// let row: Row = "".into();
  /// assert!(!row.is_full_join());
  /// ```
  pub fn is_full_join(&self) -> bool {
    self.0.first().map_or(false, |chr| chr.is_full_join())
  }

  /// Searches for a `left vertical line` starting from the specified
  /// column index and moving to the right from starting point.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Row;
  ///
  /// let row: Row = "Order options    ║   ┼  ".into();
  /// assert_eq!(Some(17), row.search_vert_line_right(4));
  /// assert_eq!(None, row.search_vert_line_right(19));
  /// ```
  pub fn search_vert_line_right(&self, mut col_index: usize) -> Option<usize> {
    while let Some(chr) = self.get(col_index) {
      if chr.is_vert_line_left() {
        return Some(col_index);
      }
      col_index += 1;
    }
    None
  }

  /// Searches for a `right vertical line` starting from the specified
  /// column index and moving to the left from starting point.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Row;
  ///
  /// let row: Row = "  ┼    ║  Order options".into();
  /// assert_eq!(Some(7), row.search_vert_line_left(12));
  /// assert_eq!(None, row.search_vert_line_left(5));
  /// ```
  pub fn search_vert_line_left(&self, mut col_index: usize) -> Option<usize> {
    col_index = col_index.saturating_sub(1);
    while let Some(chr) = self.get(col_index) {
      if chr.is_vert_line_right() {
        return Some(col_index);
      }
      if col_index == 0 {
        break;
      }
      col_index -= 1;
    }
    None
  }

  pub fn cell_range(&self, col_index: usize) -> Option<(usize, usize)> {
    if let Some((left_index, right_index)) = self.search_vert_line_left(col_index).zip(self.search_vert_line_right(col_index)) {
      Some((left_index.saturating_add(1), right_index.saturating_sub(1)))
    } else {
      None
    }
  }

  pub fn is_empty_range(&self, left_index: usize, right_index: usize) -> bool {
    self[left_index..=right_index].iter().all(|chr| chr.is_space())
  }

  pub fn insert_fill(&mut self, mut col_index: usize) {
    while let Some(chr) = self.0.get(col_index) {
      if chr.is_vert_line_or_crossing() {
        self.0.insert(col_index, chr.horz_fill());
        return;
      }
      col_index += 1;
    }
  }

  pub fn is_vert_whitespace(&self, mut col_index: usize) -> bool {
    while let Some(chr) = self.0.get(col_index) {
      if chr.is_vert_line_crossing_left() {
        return true;
      }
      if chr.is_vert_line_left() {
        return self.0.get(col_index.saturating_sub(1)).map_or(false, |chr| chr.is_space());
      }
      col_index += 1;
    }
    false
  }

  pub fn delete_whitespace(&mut self, mut col_index: usize) {
    while let Some(chr) = self.0.get(col_index) {
      if chr.is_vert_line_or_crossing() {
        self.0.remove(col_index.saturating_sub(1));
        break;
      }
      col_index += 1;
    }
  }

  /// Shifts the text to the right and inserts character at the beginning.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Row;
  ///
  /// let mut row: Row = "║order options    ║".into();
  /// row.shift_text_right(1, 17, 'b');
  /// assert_eq!("║border options   ║", row.text());
  /// ```
  pub fn shift_text_right(&mut self, start_index: usize, end_index: usize, ch: char) {
    self.0[start_index..=end_index].rotate_right(1);
    self.0[start_index].set_char(ch);
  }

  /// Shifts the text to the left and deletes the first character.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Row;
  ///
  /// let mut row: Row = "║border options   ║".into();
  /// row.shift_text_left(1, 17);
  /// assert_eq!("║order options    ║", row.text());
  /// ```
  pub fn shift_text_left(&mut self, start_index: usize, end_index: usize) {
    self.0[start_index].set_char(SPACE);
    self.0[start_index..=end_index].rotate_left(1);
  }

  /// Returns the content of the row as a [String].
  ///
  /// Usually this should be implemented as [Display](std::fmt::Display) trait,
  /// but then it is conflicting with the `impl<T: ToString> From<T>`.
  /// It is more important for a row to have a [From] conversion implementation
  /// than the [Display](std::fmt::Display) implementation. So this function substitutes
  /// the [Display](std::fmt::Display) trait.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Row;
  ///
  /// let mut row: Row = "║border options   ║".into();
  /// assert_eq!("║border options   ║", row.text());
  /// ```
  pub fn text(&self) -> String {
    self.0.iter().map(|chr| chr.char()).collect::<String>()
  }
}
