//! # Data model

use crate::cursor::{Cursor, CursorShape};
use crate::Region;

/// Checks whether the specified character is a box-drawing character.
macro_rules! is_frame {
  ($ch:expr) => {
    matches!(
      $ch,
      '┌' | '┐' | '└' | '┘' | '─' | '│' | '├' | '┤' | '┴' | '┬' | '┼' | '╪' | '╫' | '╬' | '╞' | '╡' | '╥' | '╨' | '═' | '║' | '╟' | '╢'
    )
  };
}

/// Checks whether the specified character is a vertical line
/// seen from the left side of the character.
macro_rules! is_vert_line_left {
  ($ch:expr) => {
    matches!($ch, '│' | '├' | '║' | '╟')
  };
}

/// Data model for editing text.
pub(crate) struct Model {
  /// Edited text stored as rows of characters.
  content: Vec<Vec<char>>,
  /// Current cursor position and attributes.
  cursor: Cursor,
  /// Calculated size of the textual content.
  size: Option<Region>,
}

impl Model {
  /// Creates a new text plane populated with specified content.
  pub fn new(content: String) -> Self {
    let content = content
      .lines()
      .filter_map(|line| {
        let line = line.trim();
        if line.is_empty() {
          None
        } else {
          Some(line.to_string())
        }
      })
      .collect::<Vec<String>>()
      .iter()
      .map(|line| line.chars().collect::<Vec<char>>())
      .collect::<Vec<Vec<char>>>();
    let cursor = Cursor::new(CursorShape::Bar, 1, 1);
    Self { content, cursor, size: None }
  }

  /// Returns a reference to the content of the text plane.
  pub fn content(&self) -> &[Vec<char>] {
    &self.content
  }

  /// Returns content region.
  pub fn content_region(&mut self) -> Region {
    if self.size.is_none() {
      let width = self.content.iter().map(|row| row.len()).max().unwrap_or_default();
      let height = self.content.len();
      self.size = Some(Region::new(0, 0, width, height));
    }
    self.size.unwrap()
  }

  /// Returns the position of the cursor in text plane's coordinates.
  pub fn cursor_position(&self) -> (usize, usize) {
    self.cursor.get()
  }

  /// Returns the character pointed by the cursor.
  pub fn cursor_char(&self) -> Option<char> {
    let (col, row) = self.cursor.get();
    if let Some(row) = self.content.get(row) {
      if let Some(ch) = row.get(col) {
        return Some(*ch);
      }
    }
    None
  }

  /// Moves the cursor up in the same column.
  pub fn cursor_move_up(&mut self) -> bool {
    if self.is_allowed_position(-1, 0) {
      self.cursor.dec_row(1);
      return true;
    }
    if self.is_allowed_position(-2, 0) {
      self.cursor.dec_row(2);
      return true;
    }
    false
  }

  /// Moves the cursor down in the same column.
  pub fn cursor_move_down(&mut self) -> bool {
    if self.is_allowed_position(1, 0) {
      self.cursor.inc_row(1);
      return true;
    }
    if self.is_allowed_position(2, 0) {
      self.cursor.inc_row(2);
      return true;
    }
    false
  }

  /// Moves the cursor to the left in the same row.
  pub fn cursor_move_left(&mut self) -> bool {
    if self.is_allowed_position(0, -1) {
      self.cursor.dec_col(1);
      return true;
    }
    if self.is_allowed_position(0, -2) {
      self.cursor.dec_col(2);
      return true;
    }
    false
  }

  /// Moves the cursor to the right in the same row.
  pub fn cursor_move_right(&mut self) -> bool {
    if self.is_allowed_position(0, 1) {
      self.cursor.inc_col(1);
      return true;
    }
    if self.is_allowed_position(0, 2) {
      self.cursor.inc_col(2);
      return true;
    }
    false
  }

  /// Places the cursor at the first character of the current cell in the same row.
  pub fn cursor_move_cell_start(&mut self) -> bool {
    if let Some(row) = self.row() {
      if let Some(chars) = self.before(row) {
        for (offset, ch) in chars.iter().rev().enumerate() {
          if is_frame!(ch) {
            self.cursor.dec_col(offset);
            return true;
          }
        }
      }
    }
    false
  }

  /// Places the cursor at the last character of the cell in the same row.
  pub fn cursor_move_cell_end(&mut self) -> bool {
    if let Some(row) = self.row() {
      if let Some(chars) = self.after(row) {
        for (offset, ch) in chars.iter().enumerate() {
          if is_frame!(ch) {
            self.cursor.inc_col(if self.cursor.is_bar() { offset } else { offset.saturating_sub(1) });
            return true;
          }
        }
      }
    }
    false
  }

  /// Places the cursor at the first character of the first cell in the same row.
  pub fn cursor_move_row_start(&mut self) -> bool {
    if let Some(row) = self.row() {
      for (pos, ch) in row.iter().enumerate() {
        if !is_frame!(ch) {
          self.cursor.set_col(pos);
          return true;
        }
      }
    }
    false
  }

  /// Places the cursor at the last character of the last cell in the same row.
  pub fn cursor_move_row_end(&mut self) -> bool {
    if let Some(row) = self.row() {
      let len = row.len();
      for (mut pos, ch) in row.iter().rev().enumerate() {
        if !is_frame!(ch) {
          if !self.cursor.is_bar() {
            pos = pos.saturating_add(1);
          }
          self.cursor.set_col(len.saturating_sub(pos));
          return true;
        }
      }
    }
    false
  }

  /// Places the cursor at the first character of the next cell in the same row.
  pub fn cursor_move_cell_next(&mut self) -> bool {
    if self.cursor_move_cell_end() {
      if let Some(row) = self.row() {
        if let Some(chars) = self.after(row) {
          for (offset, ch) in chars[1..].iter().enumerate() {
            if !is_frame!(ch) {
              self.cursor.inc_col(offset.saturating_add(1));
              break;
            }
          }
        }
      }
      return true;
    }
    false
  }

  /// Places the cursor at the last character of the previous cell in the same row.
  pub fn cursor_move_cell_prev(&mut self) -> bool {
    if self.cursor_move_cell_start() {
      if let Some(row) = self.row() {
        if let Some(chars) = self.before(row) {
          for (offset, ch) in chars.iter().rev().enumerate() {
            if !is_frame!(ch) {
              self.cursor.dec_col(if self.cursor.is_bar() { offset } else { offset.saturating_add(1) });
              break;
            }
          }
        }
      }
      return true;
    }
    false
  }

  fn row(&self) -> Option<&[char]> {
    let row = self.cursor.row();
    if row > 0 && row < self.content.len() - 1 {
      return Some(&self.content[row]);
    }
    None
  }

  fn before<'a>(&self, row: &'a [char]) -> Option<&'a [char]> {
    let col = self.cursor.col();
    if col > 0 && col < row.len() {
      return Some(&row[0..col]);
    }
    None
  }

  fn after<'a>(&self, row: &'a [char]) -> Option<&'a [char]> {
    let col = self.cursor.col();
    if col > 0 && col < row.len() {
      return Some(&row[col..]);
    }
    None
  }

  /// Returns `true` when the cursor position is allowed, according to horizontal and vertical offset.
  fn is_allowed_position(&self, row_offset: isize, col_offset: isize) -> bool {
    let (col, row) = self.cursor.offset(col_offset, row_offset);
    if row > 0 && row < self.content.len() - 1 && col > 0 && col < self.content[row].len() {
      if self.cursor.is_bar() {
        return !is_frame!(self.content[row][col]) || is_vert_line_left!(self.content[row][col]);
      } else if col < self.content[row].len() - 1 {
        return !is_frame!(self.content[row][col]);
      }
    }
    false
  }

  pub fn cursor_toggle(&mut self) {
    self.cursor.toggle();
  }

  pub fn cursor_toggle_bar_block(&mut self) {
    self.cursor.toggle_bar_block();
  }

  pub fn cursor_is_bar(&self) -> bool {
    self.cursor.is_bar()
  }

  pub fn cursor_is_block(&self) -> bool {
    self.cursor.is_block()
  }

  pub fn cursor_is_underscore(&self) -> bool {
    self.cursor.is_underscore()
  }
}
