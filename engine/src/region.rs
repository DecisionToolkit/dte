//! # Region
//!
//! [Region] represents a rectangular region for handling editing operations.

use std::fmt;
use std::fmt::Display;

/// Rectangular region.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Region {
  /// Left coordinate of the region.
  left: usize,
  /// Top coordinate of the region.
  top: usize,
  /// Width of the region.
  width: usize,
  /// Height of the region.
  height: usize,
}

impl Display for Region {
  /// Implements [Display] trait for [Region].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {}, {}, {})", self.left, self.top, self.width, self.height)
  }
}

impl Region {
  /// Creates a new region with specified coordinates and size.
  ///
  /// # Examples
  ///
  /// ```
  /// use dtee::Region;
  ///
  /// let region = Region::new(10, 11, 80, 60);
  /// assert_eq!("(10, 11, 80, 60)", region.to_string());
  /// ```
  pub fn new(left: usize, top: usize, width: usize, height: usize) -> Self {
    Self { left, top, width, height }
  }

  /// Returns the left coordinate of the region.
  pub fn left(&self) -> usize {
    self.left
  }

  /// Returns the right coordinate of the region.
  pub fn right(&self) -> usize {
    self.left.saturating_add(self.width.saturating_sub(1))
  }

  pub fn top(&self) -> usize {
    self.top
  }

  pub fn bottom(&self) -> usize {
    self.top.saturating_add(self.height.saturating_sub(1))
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn offset(&self) -> (usize, usize) {
    (self.left, self.top)
  }

  pub fn size(&self) -> (usize, usize) {
    (self.width, self.height)
  }

  pub fn rect(&self) -> (usize, usize, usize, usize) {
    (self.left, self.top, self.width, self.height)
  }

  pub fn resize(&mut self, width: usize, height: usize) {
    self.width = width;
    self.height = height;
  }

  pub fn clip(&self, other: &Region) -> Region {
    let left = if other.left() > self.left() { other.left() } else { self.left() };
    let top = if other.top() > self.top() { other.top() } else { self.top() };
    let right = if other.right() < self.right() { other.right() } else { self.right() };
    let bottom = if other.bottom() < self.bottom() { other.bottom() } else { self.bottom() };
    let width = right.saturating_sub(left).saturating_add(1);
    let height = bottom.saturating_sub(top).saturating_add(1);
    Region { left, top, width, height }
  }

  pub fn shift_left_when_needed(&mut self, column: usize, margin: usize) -> bool {
    let column_needed = column.saturating_sub(margin);
    if column_needed < self.left() {
      self.left = self.left.saturating_sub(self.left().saturating_sub(column_needed));
      return true;
    }
    false
  }
  pub fn shift_right_when_needed(&mut self, column: usize, margin: usize) -> bool {
    let column_needed = column.saturating_add(margin);
    if column_needed > self.right() {
      self.left = self.left.saturating_add(column_needed.saturating_sub(self.right()));
      return true;
    }
    false
  }

  pub fn shift_up_when_needed(&mut self, row: usize, margin: usize) -> bool {
    let row_needed = row.saturating_sub(margin);
    if row_needed < self.top() {
      self.top = self.top.saturating_sub(self.top().saturating_sub(row_needed));
      return true;
    }
    false
  }

  pub fn shift_down_when_needed(&mut self, row: usize, margin: usize) -> bool {
    let row_needed = row.saturating_add(margin);
    if row_needed > self.bottom() {
      self.top = self.top.saturating_add(row_needed.saturating_sub(self.bottom()));
      return true;
    }
    false
  }
}
