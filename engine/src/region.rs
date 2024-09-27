//! # Region
//!
//! Region represents a rectangular space for handling editing operations.

use std::cmp::{max, min};
use std::fmt;
use std::fmt::Display;

const MOVE_MARGIN: usize = 1;

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
  /// Creates a new region with specified coordinates.
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

  pub fn left(&self) -> usize {
    self.left
  }

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

  pub fn move_left(&mut self, anchor: usize, minimum: usize) -> bool {
    let target = max(anchor.saturating_sub(MOVE_MARGIN), minimum);
    if target < self.left() {
      self.left = self.left.saturating_sub(self.left().saturating_sub(target));
      return true;
    }
    false
  }
  pub fn move_right(&mut self, anchor: usize, maximum: usize) -> bool {
    let target = min(anchor.saturating_add(MOVE_MARGIN), maximum);
    if target > self.right() {
      self.left = self.left.saturating_add(target.saturating_sub(self.right()));
      return true;
    }
    false
  }

  pub fn move_up(&mut self, anchor: usize, minimum: usize) -> bool {
    let target = max(anchor.saturating_sub(MOVE_MARGIN), minimum);
    if target < self.top() {
      self.top = self.top.saturating_sub(self.top().saturating_sub(target));
      return true;
    }
    false
  }

  pub fn move_down(&mut self, anchor: usize, maximum: usize) -> bool {
    let target = min(anchor.saturating_add(MOVE_MARGIN), maximum);
    if target > self.bottom() {
      self.top = self.top.saturating_add(target.saturating_sub(self.bottom()));
      return true;
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_should_work() {
    let region = Region::default();
    assert_eq!("(0, 0, 0, 0)", region.to_string());
  }

  #[test]
  fn debug_should_work() {
    let region = Region::new(10, 20, 100, 200);
    assert_eq!("Region { left: 10, top: 20, width: 100, height: 200 }", format!("{:?}", region));
  }

  #[test]
  fn display_should_work() {
    let region = Region::new(10, 20, 100, 200);
    assert_eq!("(10, 20, 100, 200)", region.to_string());
  }

  #[test]
  fn clipping_should_work() {
    let viewport = Region::new(0, 0, 300, 200);
    let changed = Region::new(90, 10, 10, 100);
    let clipped = Region::new(90, 10, 10, 100);
    assert_eq!(clipped, changed.clip(&viewport));
    let viewport = Region::new(10, 10, 200, 200);
    let changed = Region::new(180, 0, 120, 300);
    let clipped = Region::new(180, 10, 30, 200);
    assert_eq!(clipped, changed.clip(&viewport));
  }
}
