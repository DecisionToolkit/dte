//! # Text editing plane

use crate::ed::cursor::{Cursor, CursorType};
use std::cmp::min;
use std::fmt;
use std::fmt::Display;

const CH_WS: char = ' ';

/// Checks whether the specified character is a box-drawing character.
macro_rules! is_box_drawing_character {
  ($ch:expr) => {
    matches!(
      $ch,
      '┌' | '┐' | '└' | '┘' | '─' | '│' | '├' | '┤' | '┴' | '┬' | '┼' | '╪' | '╫' | '╬' | '╞' | '╡' | '╥' | '╨' | '═' | '║' | '╟' | '╢'
    )
  };
}

/// Checks whether the specified character is a vertical line seen from the left side.
macro_rules! is_vert_line_left {
  ($ch:expr) => {
    matches!($ch, '│' | '├' | '║' | '╟')
  };
}

/// Checks whether the specified character is a vertical line seen from the right side.
macro_rules! is_vert_line_right {
  ($ch:expr) => {
    matches!($ch, '│' | '┤' | '║' | '╢')
  };
}

/// Checks whether the specified character is a crossing with vertical line.
macro_rules! is_vert_line_crossing {
  ($ch:expr) => {
    matches!($ch, '│' | '┼' | '┬' | '┴' | '╪' | '┐' | '┘' | '├' | '║' | '╟' | '╬' | '╥' | '╨' | '╫' | '╢' | '┤' | '╡')
  };
}

/// Checks whether the specified character is a horizontal line seen from top side.
macro_rules! is_horz_line_top {
  ($ch:expr) => {
    matches!($ch, '─' | '┬' | '═' | '╥')
  };
}

enum Op {
  Insert,
  Delete,
}

/// Plane containing rows of characters.
pub struct Plane {
  /// Rows in plane.
  pub chars: Vec<Vec<char>>,
  /// Cursor properties.
  cursor: Cursor,
  /// Information item height (0 when not present).
  iih: usize,
}

impl Display for Plane {
  /// Converts a [Plane] into its string representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      self
        .chars
        .iter()
        .fold("".to_string(), |plane, row| format!(
          "{}\n{}",
          plane,
          row.iter().fold("".to_string(), |line, ch| format!("{}{}", line, ch))
        ))
        .trim()
    )
  }
}

impl Plane {
  /// Creates a new plane with specified content.
  pub fn new(content: String) -> Self {
    let mut rows = vec![];
    for content_line in content.lines() {
      let line = content_line.trim();
      if !line.is_empty() {
        let mut columns = vec![];
        for ch in line.chars() {
          columns.push(ch);
        }
        rows.push(columns);
      }
    }
    let iih = information_item_height(&rows);
    Self {
      chars: rows,
      cursor: Cursor::new(CursorType::Bar, 1, 1),
      iih,
    }
  }

  /// Returns the position of the cursor in plane's coordinates.
  pub fn cursor(&self) -> (usize, usize) {
    self.cursor.pos()
  }

  /// Returns the character pointed by cursor.
  pub fn char_under_cursor(&self) -> Option<char> {
    let (col, row) = self.cursor.pos();
    if let Some(row) = self.chars.get(row) {
      if let Some(ch) = row.get(col) {
        return Some(*ch);
      }
    }
    None
  }

  pub fn content(&self) -> &Vec<Vec<char>> {
    &self.chars
  }

  /// Returns `true` if the current cursor position is valid.
  fn is_valid_cursor_pos(&self) -> bool {
    (1..self.chars.len() - 1).contains(&self.cursor.row()) && (1..self.chars[self.cursor.col()].len() - 1).contains(&self.cursor.col())
  }

  /// Moves cursor up.
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

  /// Moves cursor down.
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

  /// Moves cursor left.
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

  /// Moves cursor right.
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

  /// Places cursor at the first character in the cell.
  pub fn cursor_move_cell_start(&mut self) -> bool {
    if let Some(row) = self.chars.get(self.cursor.row()) {
      if self.cursor.col() < row.len() {
        for (offset, ch) in row[..self.cursor.col()].iter().rev().enumerate() {
          if is_box_drawing_character!(ch) {
            self.cursor.dec_col(offset);
            return true;
          }
        }
      }
    }
    false
  }

  /// Places cursor at the last character in the cell (same row).
  pub fn cursor_move_cell_end(&mut self) -> bool {
    if let Some(row) = self.chars.get(self.cursor.row()) {
      if self.cursor.col() < row.len() {
        for (offset, ch) in row[self.cursor.col()..].iter().enumerate() {
          if is_box_drawing_character!(ch) {
            self.cursor.inc_col(if self.cursor.is_bar() { offset } else { offset.saturating_sub(1) });
            return true;
          }
        }
      }
    }
    false
  }

  /// Places cursor at the first character of the first cell in row.
  pub fn cursor_move_row_start(&mut self) -> bool {
    if let Some(row) = self.chars.get(self.cursor.row()) {
      for (offset, ch) in row.iter().enumerate() {
        if is_vert_line_right!(ch) {
          self.cursor.set_col(min(offset + 1, row.len() - 1));
          return true;
        }
      }
    }
    false
  }

  /// Places cursor at the last character of the last cell in row.
  pub fn cursor_move_row_end(&mut self) -> bool {
    if let Some(row) = self.chars.get(self.cursor.row()) {
      for (offset, ch) in row.iter().rev().enumerate() {
        if is_vert_line_left!(ch) {
          self.cursor.set_col(if self.cursor.is_bar() {
            row.len().saturating_sub(offset + 1)
          } else {
            row.len().saturating_sub(offset + 2)
          });
          return true;
        }
      }
    }
    false
  }

  /// Places ths cursor at the first character of the next cell in a row.
  pub fn cursor_move_cell_next(&mut self) -> bool {
    if let Some(offset) = self.get_vert_line_offset_right() {
      return if self.is_allowed_position(0, offset + 1) {
        self.cursor.adj_col(offset + 1);
        true
      } else {
        self.cursor_move_cell_end()
      };
    }
    false
  }

  /// Places the cursor at the last character of the previous cell in a row.
  pub fn cursor_move_cell_prev(&mut self) -> bool {
    if let Some(offset) = self.get_vert_line_offset_left() {
      return if self.is_allowed_position(0, offset - 1) {
        self.cursor_move(0, offset - 1);
        true
      } else {
        self.cursor_move_cell_start()
      };
    }
    false
  }

  /// Inserts a character at the current position.
  pub fn insert_char(&mut self, ch: char) {
    if self.is_valid_cursor_pos() {
      let pos = self.last_col_before_vert_line_right();
      let (found, offset) = self.is_whitespace_before_vert_line();
      let columns = &mut self.chars[self.cursor.row()];
      columns.insert(self.cursor.col(), ch);
      if found {
        columns.remove(self.cursor.col() + offset + 1);
      } else {
        self.insert_column_before_vert_line(pos);
      }
      self.cursor_move(0, 1);
      self.update_joins();
    }
  }

  /// Deletes a character placed *before* the cursor.
  pub fn delete_char_before(&mut self) {
    if self.is_allowed_position(0, -1) {
      let pos = self.last_col_before_vert_line_right();
      self.chars[self.cursor.row()].insert(pos + 1, CH_WS);
      self.chars[self.cursor.row()].remove(self.cursor.col() - 1);
      if self.is_whitespace_column_before_vert_line(pos, Op::Delete) {
        self.delete_column_before_vert_line(pos);
      }
      self.cursor_move(0, -1);
      self.update_joins();
    }
  }

  /// Deletes a character placed *under* the cursor.
  pub fn delete_char(&mut self) {
    let pos = self.last_col_before_vert_line_right();
    self.chars[self.cursor.row()].insert(pos + 1, CH_WS);
    self.chars[self.cursor.row()].remove(self.cursor.col());
    if self.is_whitespace_column_before_vert_line(pos, Op::Delete) {
      self.delete_column_before_vert_line(pos);
    }
    if is_box_drawing_character!(self.chars[self.cursor.row()][self.cursor.col()]) {
      self.cursor_move(0, -1);
    }
    self.update_joins();
  }

  /// Splits the current line and moves the right side of the split to the line below.
  pub fn split_line(&mut self) {
    let col_first = self.first_col_after_vert_line_left();
    let col_last = self.last_col_before_vert_line_right();
    let row_last = self.last_row_before_horz_line_below();
    // check if the last row before the horizontal line is empty (contains only characters)
    let empty = self.chars[row_last][col_first..=col_last].iter().all(|ch| *ch == CH_WS);
    if !empty {
      // add new empty line before the horizontal line
    }
    // move all lines one line down

    // move characters from the right side of the split to the beginning of the next line
    for (offset, col_index) in (self.cursor.col()..=col_last).enumerate() {
      self.chars[self.cursor.row() + 1][col_first + offset] = self.chars[self.cursor.row()][col_index];
      self.chars[self.cursor.row()][col_index] = CH_WS;
    }
    self.cursor.inc_row(1);
    self.cursor.set_col(col_first);
  }

  /// Moves the cursor to new position.
  fn cursor_move(&mut self, row_offset: isize, col_offset: isize) {
    if self.is_allowed_position(row_offset, col_offset) {
      let (row, col) = self.cursor.adjusted(row_offset, col_offset);
      if (1..self.chars.len() - 1).contains(&row) && (1..self.chars[row].len() - 1).contains(&col) {
        self.cursor.set(col, row);
      }
    }
  }

  /// Updated join character between information item name cell and the body of the decision table.
  fn update_joins(&mut self) {
    if self.iih > 0 {
      let row_index = self.iih;
      // remove old joining character...
      for ch in &mut self.chars[row_index] {
        match ch {
          '┴' => *ch = '─',
          '┼' => *ch = '┬',
          '┤' => *ch = '┐',
          _ => {}
        }
      }
      // ...and replace with new joining character
      let col_index = self.chars[0].len() - 1;
      if col_index < self.chars[row_index].len() {
        let ch = &mut self.chars[row_index][col_index];
        match ch {
          '─' => *ch = '┴',
          '┬' => *ch = '┼',
          '┐' => *ch = '┤',
          _ => {}
        }
      }
    }
  }

  /// Returns the index of the first column after the vertical line
  /// to the left from the character pointed by current cursor position.
  fn first_col_after_vert_line_left(&self) -> usize {
    let offset = self.chars[self.cursor.row()].len();
    for (col_index, ch) in self.chars[self.cursor.row()].iter().rev().enumerate().skip(offset - self.cursor.col() + 1) {
      if is_vert_line_right!(ch) {
        return offset - col_index;
      }
    }
    self.cursor.col()
  }

  /// Returns the index of the last column before the vertical line
  /// to the right from the character pointed by current cursor position.
  fn last_col_before_vert_line_right(&self) -> usize {
    for (col_index, ch) in self.chars[self.cursor.row()].iter().enumerate().skip(self.cursor.col()) {
      if is_vert_line_left!(ch) {
        return col_index - 1;
      }
    }
    self.cursor.col()
  }

  /// Returns the index of the last row before the horizontal line
  /// below the character pointed by current cursor position.
  fn last_row_before_horz_line_below(&self) -> usize {
    for (row_index, row) in self.chars.iter().enumerate().skip(self.cursor.row()) {
      if (1..row.len() - 1).contains(&self.cursor.col()) && is_horz_line_top!(row[self.cursor.col()]) {
        return row_index - 1;
      }
    }
    self.cursor.row()
  }

  fn is_whitespace_before_vert_line(&self) -> (bool, usize) {
    let mut count = 0;
    let mut offset = 0;
    for ch in &self.chars[self.cursor.row()][self.cursor.col() + 1..] {
      if is_vert_line_left!(ch) {
        break;
      } else if *ch == CH_WS {
        count += 1;
      } else {
        count = 0;
      }
      offset += 1;
    }
    (count > 0, offset)
  }

  fn insert_column_before_vert_line(&mut self, col_pos: usize) {
    let (skip, take) = self.rows_skip_and_take(Op::Insert);
    for (row_index, row) in self.chars.iter_mut().enumerate().skip(skip).take(take) {
      if row_index != self.cursor.row() && col_pos < row.len() - 1 {
        let mut found_char = CH_WS;
        let mut found_index = 0;
        for (col_index, ch) in row[col_pos..].iter().enumerate() {
          if is_vert_line_crossing!(ch) {
            found_char = *ch;
            found_index = col_pos + col_index;
            break;
          }
        }
        match found_char {
          '│' | '├' | '║' | '╟' => row.insert(found_index, CH_WS),
          '┼' | '┬' | '┴' | '┐' | '┘' | '┤' | '╥' | '╨' | '╫' | '╢' => row.insert(found_index, '─'),
          '╪' | '╬' | '╡' => row.insert(found_index, '═'),
          _ => {}
        }
      }
    }
  }

  /// Returns `true` if there is a whitespace is before the next vertical line
  /// to the right from the specified position in each checked row.
  fn is_whitespace_column_before_vert_line(&self, pos: usize, op: Op) -> bool {
    let (skip, take) = self.rows_skip_and_take(op);
    for (row_index, row) in self.chars.iter().enumerate().skip(skip).take(take) {
      // check if the current column is not after the end of each row
      if (1..row.len() - 1).contains(&pos) {
        // check the character at column position, if box-drawing then skip
        let ch = self.chars[row_index][pos];
        if !is_box_drawing_character!(ch) {
          // move to the right until vertical line is found
          for chars in row[pos - 1..].windows(3) {
            if is_vert_line_left!(chars[2]) {
              // if there is no whitespace before vertical line,
              // no further checking is needed, just return `false`
              if chars[1] != CH_WS {
                return false;
              }
              // if there is a whitespace, but just between two box-drawing
              // characters, no further checking is needed, just return `false`
              if is_box_drawing_character!(chars[0]) {
                return false;
              }
              // whitespace found, check the next row
              break;
            }
          }
        }
      }
    }
    true
  }

  /// Deletes a single character before the next vertical line to the right
  /// from the specified position.
  fn delete_column_before_vert_line(&mut self, pos: usize) {
    let (skip, take) = self.rows_skip_and_take(Op::Delete);
    for row in self.chars.iter_mut().skip(skip).take(take) {
      if pos < row.len() - 1 {
        let mut found_index = 0;
        for (col_index, ch) in row[pos..].iter().enumerate() {
          if is_vert_line_crossing!(ch) {
            found_index = pos + col_index;
            break;
          }
        }
        if found_index > 0 {
          row.remove(found_index - 1);
        }
      }
    }
  }

  /// Returns `true` when the character at the specified position is a horizontal line.
  fn is_horz_line(&self, row_offset: isize, col_offset: isize) -> bool {
    let (r, c) = self.cursor.adjusted(row_offset, col_offset);
    if r < self.chars.len() && c < self.chars[r].len() {
      is_horz_line_top!(self.chars[r][c])
    } else {
      false
    }
  }

  /// Returns `true` when the character at the specified position is a vertical line.
  fn is_vert_line(&self, row_offset: isize, col_offset: isize) -> bool {
    let (col, row) = self.cursor.adjusted_1(col_offset, row_offset);
    if row < self.chars.len() && col < self.chars[row].len() {
      matches!(self.chars[row][col], '│' | '║')
    } else {
      false
    }
  }

  /// Returns `true` when the cursor position is allowed according to horizontal and vertical offset.
  fn is_allowed_position(&self, row_offset: isize, col_offset: isize) -> bool {
    let (col, row) = self.cursor.adjusted_1(col_offset, row_offset);
    if row > 0 && row < self.chars.len() - 1 && col > 0 && col < self.chars[row].len() {
      if self.cursor.is_bar() {
        return !is_box_drawing_character!(self.chars[row][col]) || is_vert_line_left!(self.chars[row][col]);
      } else if col < self.chars[row].len() - 1 {
        return !is_box_drawing_character!(self.chars[row][col]);
      }
    }
    false
  }

  /// Returns the offset of the vertical line to the right from current cursor position.
  fn get_vert_line_offset_right(&self) -> Option<isize> {
    if let Some(row) = self.chars.get(self.cursor.row()) {
      if self.cursor.col() < row.len() - 1 {
        for (offset, ch) in row[self.cursor.col()..].iter().enumerate() {
          if is_vert_line_left!(ch) {
            return Some(if self.cursor.is_bar() { offset as isize } else { (offset as isize) - 1 });
          }
        }
      }
    }
    None
  }

  /// Returns the offset of the vertical line to the left from current cursor position.
  fn get_vert_line_offset_left(&self) -> Option<isize> {
    if let Some(row) = self.chars.get(self.cursor.row()) {
      if self.cursor.col() < row.len() {
        for (offset, ch) in row[0..self.cursor.col()].iter().rev().enumerate() {
          if is_vert_line_right!(ch) {
            return Some(-(offset as isize));
          }
        }
      }
    }
    None
  }

  /// Returns the number of rows to skip and to take while iterating over rows.
  fn rows_skip_and_take(&self, op: Op) -> (usize, usize) {
    if self.cursor.row() < self.iih {
      // operation takes place in information item cell
      match op {
        Op::Insert => {
          //
          let pos = self.last_col_before_vert_line_right();
          if pos + 1 >= self.chars[self.iih].len() {
            (0, self.chars.len())
          } else {
            (0, self.iih)
          }
        }
        Op::Delete => {
          //
          (0, self.iih)
        }
      }
    } else {
      // operation takes place in decision table body
      match op {
        Op::Insert => {
          //
          (self.iih, self.chars.len() - self.iih)
        }
        Op::Delete => {
          //
          let l_first = self.chars[0].len();
          let l_current = self.chars[self.cursor.row()].len();
          if l_current > l_first {
            (self.iih, self.chars.len() - self.iih)
          } else {
            (0, self.chars.len())
          }
        }
      }
    }
  }

  pub fn toggle_cursor(&mut self) -> CursorType {
    self.cursor.toggle()
  }
}

/// Calculates the height of the information item cell at the beginning of the decision table.
fn information_item_height(rows: &[Vec<char>]) -> usize {
  for (row_index, row) in rows.iter().enumerate() {
    for (col_index, ch) in row.iter().enumerate() {
      if col_index == 0 && *ch != '┌' && *ch != '├' {
        // skip rows that do not begin with horizontal line crossing
        break;
      }
      if *ch == '╥' {
        // index of the row that contains '╥' character is the height
        return row_index;
      }
    }
  }
  0
}
