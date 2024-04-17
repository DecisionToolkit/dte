//! # Text editing plane

use crate::cursor::{Cursor, CursorShape};
use crate::Region;
use std::fmt;
use std::fmt::Display;

const CH_WS: char = ' ';

/// Checks whether the specified character is a box-drawing character.
macro_rules! is_frame {
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

/// Model for edited text.
pub(crate) struct Model {
  /// Edited text stored as rows of characters.
  content: Vec<Vec<char>>,
  /// Current cursor position and attributes.
  cursor: Cursor,
  /// Information item height (=0 when not present).
  ii_height: usize,
  /// Calculated size of the textual content.
  size: Option<Region>,
}

impl Display for Model {
  /// Implements [Display] trait for the [Model].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.content.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n"))
  }
}

impl Model {
  /// Creates a new plane with specified content.
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
    let ii_height = Self::information_item_height(&content);
    Self {
      content,
      cursor,
      ii_height,
      size: None,
    }
  }

  /// Returns the textual content.
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

  pub fn invalidate_size(&mut self) {
    self.size = None;
  }

  /// Returns the position of the cursor in plane's coordinates.
  pub fn cursor_position(&self) -> (usize, usize) {
    self.cursor.get()
  }

  /// Returns the character pointed by cursor.
  pub fn cursor_char(&self) -> Option<char> {
    let (col, row) = self.cursor.get();
    if let Some(row) = self.content.get(row) {
      if let Some(ch) = row.get(col) {
        return Some(*ch);
      }
    }
    None
  }

  /// Returns `true` if the current cursor position is valid.
  fn is_valid_cursor_pos(&self) -> bool {
    (1..self.content.len() - 1).contains(&self.cursor.row()) && (1..self.content[self.cursor.col()].len() - 1).contains(&self.cursor.col())
  }

  /// Moves cursor up in the same column.
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

  /// Moves cursor down in the same column.
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

  /// Moves cursor to the left in the same row.
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

  /// Moves cursor to the right in the same row.
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

  /// Places cursor at the last character of the cell in the same row.
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

  /// Places cursor at the first character of the first cell in the same row.
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

  /// Places cursor at the last character of the last cell in the same row.
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

  /// Places ths cursor at the first character of the next cell in the same row.
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

  /// Inserts a character at the current position.
  pub fn insert_char(&mut self, ch: char) {
    if self.is_valid_cursor_pos() {
      let pos = self.last_col_before_vert_line_right();
      let (found, offset) = self.is_whitespace_before_vert_line();
      let columns = &mut self.content[self.cursor.row()];
      columns.insert(self.cursor.col(), ch);
      if found {
        columns.remove(self.cursor.col() + offset + 1);
      } else {
        self.insert_column_before_vert_line(pos);
      }
      self.cursor_move(1, 0);
      self.update_joins();
    }
  }

  /// Deletes a character placed *before* the cursor.
  pub fn delete_char_before(&mut self) {
    if self.is_allowed_position(0, -1) {
      let pos = self.last_col_before_vert_line_right();
      self.content[self.cursor.row()].insert(pos + 1, CH_WS);
      self.content[self.cursor.row()].remove(self.cursor.col() - 1);
      if self.is_whitespace_column_before_vert_line(pos, Op::Delete) {
        self.delete_column_before_vert_line(pos);
      }
      self.cursor_move(-1, 0);
      self.update_joins();
    }
  }

  /// Deletes a character placed *under* the cursor.
  pub fn delete_char(&mut self) {
    let pos = self.last_col_before_vert_line_right();
    self.content[self.cursor.row()].insert(pos + 1, CH_WS);
    self.content[self.cursor.row()].remove(self.cursor.col());
    if self.is_whitespace_column_before_vert_line(pos, Op::Delete) {
      self.delete_column_before_vert_line(pos);
    }
    if is_frame!(self.content[self.cursor.row()][self.cursor.col()]) {
      self.cursor_move(-1, 0);
    }
    self.update_joins();
  }

  /// Splits the current line and moves the right side of the split to the line below.
  pub fn split_line(&mut self) {
    let col_first = self.first_col_after_vert_line_left();
    let col_last = self.last_col_before_vert_line_right();
    let row_last = self.last_row_before_horz_line_below();
    // check if the last row before the horizontal line is empty (contains only characters)
    let empty = self.content[row_last][col_first..=col_last].iter().all(|ch| *ch == CH_WS);
    if !empty {
      // add new empty line before the horizontal line
    }
    // move all lines one line down

    // move characters from the right side of the split to the beginning of the next line
    for (offset, col_index) in (self.cursor.col()..=col_last).enumerate() {
      self.content[self.cursor.row() + 1][col_first + offset] = self.content[self.cursor.row()][col_index];
      self.content[self.cursor.row()][col_index] = CH_WS;
    }
    self.cursor.inc_row(1);
    self.cursor.set_col(col_first);
  }

  /// Moves the cursor to new position.
  fn cursor_move(&mut self, col_offset: isize, row_offset: isize) {
    if self.is_allowed_position(row_offset, col_offset) {
      let (col, row) = self.cursor.offset(col_offset, row_offset);
      if (1..self.content.len() - 1).contains(&row) && (1..self.content[row].len() - 1).contains(&col) {
        self.cursor.set(col, row);
      }
    }
  }

  /// Updated join character between information item name cell and the body of the decision table.
  fn update_joins(&mut self) {
    if self.ii_height > 0 {
      let row_index = self.ii_height;
      // remove old joining character...
      for ch in &mut self.content[row_index] {
        match ch {
          '┴' => *ch = '─',
          '┼' => *ch = '┬',
          '┤' => *ch = '┐',
          _ => {}
        }
      }
      // ...and replace with new joining character
      let col_index = self.content[0].len() - 1;
      if col_index < self.content[row_index].len() {
        let ch = &mut self.content[row_index][col_index];
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
    let offset = self.content[self.cursor.row()].len();
    for (col_index, ch) in self.content[self.cursor.row()].iter().rev().enumerate().skip(offset - self.cursor.col() + 1) {
      if is_vert_line_right!(ch) {
        return offset - col_index;
      }
    }
    self.cursor.col()
  }

  /// Returns the index of the last column before the vertical line
  /// to the right from the character pointed by current cursor position.
  fn last_col_before_vert_line_right(&self) -> usize {
    for (col_index, ch) in self.content[self.cursor.row()].iter().enumerate().skip(self.cursor.col()) {
      if is_vert_line_left!(ch) {
        return col_index - 1;
      }
    }
    self.cursor.col()
  }

  /// Returns the index of the last row before the horizontal line
  /// below the character pointed by current cursor position.
  fn last_row_before_horz_line_below(&self) -> usize {
    for (row_index, row) in self.content.iter().enumerate().skip(self.cursor.row()) {
      if (1..row.len() - 1).contains(&self.cursor.col()) && is_horz_line_top!(row[self.cursor.col()]) {
        return row_index - 1;
      }
    }
    self.cursor.row()
  }

  fn is_whitespace_before_vert_line(&self) -> (bool, usize) {
    let mut count = 0;
    let mut offset = 0;
    for ch in &self.content[self.cursor.row()][self.cursor.col() + 1..] {
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
    for (row_index, row) in self.content.iter_mut().enumerate().skip(skip).take(take) {
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
    for (row_index, row) in self.content.iter().enumerate().skip(skip).take(take) {
      // check if the current column is not after the end of each row
      if (1..row.len() - 1).contains(&pos) {
        // check the character at column position, if box-drawing then skip
        let ch = self.content[row_index][pos];
        if !is_frame!(ch) {
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
              if is_frame!(chars[0]) {
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
    for row in self.content.iter_mut().skip(skip).take(take) {
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
    let (col, row) = self.cursor.offset(col_offset, row_offset);
    if row < self.content.len() && col < self.content[row].len() {
      is_horz_line_top!(self.content[row][col])
    } else {
      false
    }
  }

  /// Returns `true` when the character at the specified position is a vertical line.
  fn is_vert_line(&self, row_offset: isize, col_offset: isize) -> bool {
    let (col, row) = self.cursor.offset(col_offset, row_offset);
    if row < self.content.len() && col < self.content[row].len() {
      matches!(self.content[row][col], '│' | '║')
    } else {
      false
    }
  }

  /// Returns `true` when the cursor position is allowed according to horizontal and vertical offset.
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

  /// Returns the offset of the vertical line to the right from current cursor position.
  fn get_vert_line_offset_right(&self) -> Option<isize> {
    if let Some(row) = self.content.get(self.cursor.row()) {
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
    if let Some(row) = self.content.get(self.cursor.row()) {
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
    if self.cursor.row() < self.ii_height {
      // operation takes place in information item cell
      match op {
        Op::Insert => {
          //
          let pos = self.last_col_before_vert_line_right();
          if pos + 1 >= self.content[self.ii_height].len() {
            (0, self.content.len())
          } else {
            (0, self.ii_height)
          }
        }
        Op::Delete => {
          //
          (0, self.ii_height)
        }
      }
    } else {
      // operation takes place in decision table body
      match op {
        Op::Insert => {
          //
          (self.ii_height, self.content.len() - self.ii_height)
        }
        Op::Delete => {
          //
          let l_first = self.content[0].len();
          let l_current = self.content[self.cursor.row()].len();
          if l_current > l_first {
            (self.ii_height, self.content.len() - self.ii_height)
          } else {
            (0, self.content.len())
          }
        }
      }
    }
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

  /// Calculates the height of the information item cell at the beginning of the decision table.
  fn information_item_height(rows: &[Vec<char>]) -> usize {
    for (row_index, row) in rows.iter().enumerate() {
      for (col_index, ch) in row.iter().enumerate() {
        if col_index == 0 && *ch != '┌' && *ch != '├' {
          // skip rows that do not begin with '┌' or '├'
          break;
        }
        if *ch == '╥' {
          // index of the row that contains '╥' character
          // is the information item height
          return row_index;
        }
      }
    }
    0
  }
}
