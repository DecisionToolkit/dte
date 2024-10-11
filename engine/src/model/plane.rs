//! # Edited text with cursor

use crate::model::characters::*;
use crate::model::cursor::{Cursor, CursorShape};
use crate::model::Row;
use crate::{Region, SPACE};

type JoinRowInfo = Option<(usize, bool)>;

/// Edited text with cursor.
pub struct Plane {
  /// Edited text stored as rows of characters.
  rows: Vec<Row>,
  /// Current cursor position and attributes.
  cursor: Cursor,
  /// Calculated size of the textual content.
  size: Option<Region>,
}

fn join_index(content: &[Row]) -> Option<usize> {
  let left_top = content.first().and_then(|row| row.first().map(|chr| chr.char()))?;
  if left_top == LIGHT_DOWN_AND_RIGHT {
    let right_top = content.first().and_then(|row| row.last().map(|chr| chr.char()))?;
    if right_top == LIGHT_DOWN_AND_LEFT {
      let col_index = content.first().unwrap().len() - 1;
      for (index, row) in content.iter().enumerate() {
        if let Some(chr) = row.get(col_index) {
          if chr.char() == LIGHT_UP_AND_HORIZONTAL || chr.char() == LIGHT_VERTICAL_AND_HORIZONTAL || chr.char() == VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE {
            return Some(index);
          }
        }
      }
    }
  }
  None
}

impl Plane {
  /// Creates a new text plane populated with the specified content.
  pub fn new<T: ToString>(content: T) -> Self {
    let content = content
      .to_string()
      .lines()
      .filter_map(|line| {
        let line = line.trim();
        if line.is_empty() {
          None
        } else {
          Some(line.into())
        }
      })
      .collect::<Vec<Row>>();

    //FIXME start: setting join this way is temporary
    if let Some(index) = join_index(&content) {
      content[index].set_join();
    }
    //FIXME end ^^^^^^^^^^^^^^^^^^

    let cursor = Cursor::new(CursorShape::Caret, 1, 1);
    Self {
      rows: content,
      cursor,
      size: None,
    }
  }

  /// Returns a reference to the content of the text plane.
  pub fn content(&self) -> &[Row] {
    &self.rows
  }

  /// Returns the region of the content.
  pub fn region(&mut self) -> &Region {
    if self.size.is_none() {
      let width = self.rows.iter().map(|row| row.len()).max().unwrap_or_default();
      let height = self.rows.len();
      self.size = Some(Region::new(0, 0, width, height));
    }
    self.size.as_ref().unwrap()
  }

  pub fn cursor(&self) -> &Cursor {
    &self.cursor
  }

  /// Returns the position of the cursor in text coordinates.
  ///
  /// Cursor position is a tuple `(column, row)`.
  pub fn cursor_pos(&self) -> (usize, usize) {
    self.cursor.pos()
  }

  pub fn cursor_col(&self) -> usize {
    self.cursor.col()
  }

  /// Returns the character pointed by the cursor.
  pub fn cursor_char(&self) -> Option<&Char> {
    let (col_index, row_index) = self.cursor.pos();
    self.rows.get(row_index).and_then(|row| row.get(col_index))
  }

  /// Returns the character above the cursor.
  pub fn cursor_char_above(&self) -> Option<&Char> {
    let (col_index, row_index) = self.cursor.pos();
    if row_index > 0 {
      self.rows.get(row_index - 1).and_then(|row| row.get(col_index))
    } else {
      None
    }
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
          if ch.is_frame() {
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
          if ch.is_frame() {
            self.cursor.inc_col(if self.cursor.is_caret() { offset } else { offset.saturating_sub(1) });
            return true;
          }
        }
      }
    }
    false
  }

  pub fn cursor_move_cell_top(&mut self) -> bool {
    let (ix_col, mut ix_row) = self.cursor.pos();
    ix_row = ix_row.saturating_sub(1);
    let mut moved = false;
    while let Some(chr) = self.rows.get(ix_row).and_then(|row| row.get(ix_col)) {
      if !chr.is_horz_line_or_crossing() || chr.is_vert_line_left() {
        self.cursor.dec_row(1);
        moved = true;
      } else {
        break;
      }
      if ix_row == 0 {
        break;
      }
      ix_row = ix_row.saturating_sub(1);
    }
    moved
  }

  pub fn cursor_move_cell_bottom(&mut self) -> bool {
    let (ix_col, mut ix_row) = self.cursor.pos();
    ix_row = ix_row.saturating_add(1);
    let mut moved = false;
    while let Some(chr) = self.rows.get(ix_row).and_then(|row| row.get(ix_col)) {
      if !chr.is_horz_line_or_crossing() || chr.is_vert_line_left() {
        self.cursor.inc_row(1);
        moved = true;
      } else {
        break;
      }
      ix_row = ix_row.saturating_add(1);
    }
    moved
  }

  /// Places the cursor at the first character of the first cell in the same row.
  pub fn cursor_move_row_start(&mut self) -> bool {
    if let Some(row) = self.row() {
      let old_column = self.cursor.col();
      for (pos, ch) in row.iter().enumerate() {
        if !ch.is_frame() {
          self.cursor.set_col(pos);
          return self.cursor.col() != old_column;
        }
      }
    }
    false
  }

  /// Places the cursor at the last character of the last cell in the same row.
  pub fn cursor_move_row_end(&mut self) -> bool {
    if let Some(row) = self.row() {
      let old_column = self.cursor.col();
      let len = row.len();
      for (mut pos, ch) in row.iter().rev().enumerate() {
        if !ch.is_frame() {
          if !self.cursor.is_caret() {
            pos = pos.saturating_add(1);
          }
          self.cursor.set_col(len.saturating_sub(pos));
          return self.cursor.col() != old_column;
        }
      }
    }
    false
  }

  /// Places the cursor at the character of the first row in the same column.
  pub fn cursor_move_col_start(&mut self) {
    let ix_col = self.cursor.col();
    for (ix_row, row) in self.rows.iter().enumerate().rev() {
      if let Some(chr) = row.get(ix_col) {
        if self.cursor.is_caret() {
          if !chr.is_horz_line_or_crossing() || chr.is_vert_line_left() {
            self.cursor.set_row(ix_row);
          }
        } else if !chr.is_frame() {
          self.cursor.set_row(ix_row);
        }
      }
    }
  }

  /// Places the cursor at the character of the last row in the same column.
  pub fn cursor_move_col_end(&mut self) {
    let ix_col = self.cursor.col();
    for (ix_row, row) in self.rows.iter().enumerate() {
      if let Some(chr) = row.get(ix_col) {
        if self.cursor.is_caret() {
          if !chr.is_horz_line_or_crossing() || chr.is_vert_line_left() {
            self.cursor.set_row(ix_row);
          }
        } else if !chr.is_frame() {
          self.cursor.set_row(ix_row);
        }
      }
    }
  }

  /// Places the cursor at the first character of the next cell in the same row.
  pub fn cursor_move_cell_next(&mut self) -> bool {
    if self.cursor_move_cell_end() {
      if let Some(row) = self.row() {
        if let Some(chars) = self.after(row) {
          for (offset, ch) in chars[1..].iter().enumerate() {
            if !ch.is_frame() {
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
            if !ch.is_frame() {
              self.cursor.dec_col(if self.cursor.is_caret() { offset } else { offset.saturating_add(1) });
              break;
            }
          }
        }
      }
      return true;
    }
    false
  }

  fn row(&self) -> Option<&[Char]> {
    let row = self.cursor.row();
    if row > 0 && row < self.rows.len().saturating_sub(1) {
      return Some(&self.rows[row]);
    }
    None
  }

  fn before<'a>(&self, row: &'a [Char]) -> Option<&'a [Char]> {
    let col = self.cursor.col();
    if col > 0 && col < row.len() {
      return Some(&row[0..col]);
    }
    None
  }

  fn after<'a>(&self, row: &'a [Char]) -> Option<&'a [Char]> {
    let col = self.cursor.col();
    if col > 0 && col < row.len() {
      return Some(&row[col..]);
    }
    None
  }

  /// Returns `true` when the cursor position is allowed, according to horizontal and vertical offset.
  fn is_allowed_position(&self, row_offset: isize, col_offset: isize) -> bool {
    let (col, row) = self.cursor.offset(col_offset, row_offset);
    if row > 0 && row < self.rows.len() - 1 && col > 0 && col < self.rows[row].len() {
      if self.cursor.is_caret() {
        return !self.rows[row][col].is_frame() || self.rows[row][col].is_vert_line_left();
      } else if col < self.rows[row].len() - 1 {
        return !self.rows[row][col].is_frame();
      }
    }
    false
  }

  pub fn cursor_toggle_caret_block(&mut self) {
    let shape = self.cursor.toggle_caret_block();
    if matches!(shape, CursorShape::Block | CursorShape::UnderScore) {
      if let Some(chr) = self.rows.get(self.cursor.row()).and_then(|row| row.get(self.cursor.col())) {
        if chr.is_frame() {
          self.cursor.dec_col(1);
        }
      }
    }
  }

  pub fn cursor_toggle_caret_under_score(&mut self) {
    let shape = self.cursor.toggle_caret_under_score();
    if matches!(shape, CursorShape::Block | CursorShape::UnderScore) {
      if let Some(chr) = self.rows.get(self.cursor.row()).and_then(|row| row.get(self.cursor.col())) {
        if chr.is_frame() {
          self.cursor.dec_col(1);
        }
      }
    }
  }

  pub fn insert_char(&mut self, ch: char) -> bool {
    // get the current cursor position
    let (col_index, row_index) = self.cursor.pos();
    // find the index of the first `left vertical line` starting from the current cursor position
    if let Some(vert_line_index) = self.rows.get(row_index).and_then(|row| row.search_vert_line_right(col_index)) {
      // if there is a minimum one space before the vertical line then simply shift
      // the text to the right and insert a new character at the cursor position
      let space_index = vert_line_index.saturating_sub(1);
      if let Some(row) = self.rows.get_mut(row_index) {
        if let Some(chr) = row.get_mut(space_index) {
          if col_index <= space_index && chr.is_space() {
            row.shift_text_right(col_index, space_index, ch);
            self.cursor.inc_col(1);
            // Return `true` to signal that a character wad inserted.
            // Returning from here when shifting text right is enough.
            return true;
          }
        }
      }

      // there was no space before the `left vertical line`, so the filling character
      // must be inserted in appropriate rows before lhe vertical line
      let join_row_info = self.join_row_info();
      let row_iterator = if let Some((join_row_index, is_full)) = join_row_info {
        match (is_full, row_index < join_row_index) {
          (true, true) => self.rows.iter_mut(),
          (false, true) => self.rows[..join_row_index].iter_mut(),
          (_, false) => self.rows[join_row_index..].iter_mut(),
        }
      } else {
        self.rows.iter_mut()
      };
      for row in row_iterator {
        row.insert_fill(vert_line_index);
      }

      // shift text to the right and insert a new character at the cursor position
      if let Some(row) = self.rows.get_mut(row_index) {
        row.shift_text_right(col_index, vert_line_index, ch);
        self.cursor.inc_col(1);
      }

      // update the joining row
      self.update_joining_row(join_row_info);

      // invalidate the content region because the width has changed
      self.invalidate_content_region();

      // Return `true` to signal that a character was inserted.
      return true;
    }
    // Return `false` to signal that no changes were made.
    false
  }

  pub fn override_char(&mut self, ch: char) -> bool {
    // Get the current cursor position.
    let (col_index, row_index) = self.cursor.pos();
    // Find the index of the first `left vertical line` starting from the current cursor position.
    if let Some(vert_line_index) = self.rows.get(row_index).and_then(|row| row.search_vert_line_right(col_index)) {
      // Cursor must be placed before the vertical line.
      if col_index < vert_line_index {
        // Overwrite the character under the cursor.
        if let Some(chr) = self.rows.get(row_index).and_then(|row| row.get(col_index)) {
          chr.set_char(ch);
          // Move cursor one position right if there is no vertical line directly to the right.
          if col_index.saturating_add(1) < vert_line_index {
            self.cursor.inc_col(1);
          }
          // Return `true` to signal that the character was overridden.
          return true;
        }
      }
    }
    false
  }

  pub fn delete_char_before_cursor(&mut self) -> bool {
    // get the current cursor position
    let (col_index, row_index) = self.cursor.pos();
    // get the character before the cursor, otherwise there is nothing to do
    let delete_char_index = col_index.saturating_sub(1);
    if let Some(chr) = self.rows.get(row_index).and_then(|row| row.get(delete_char_index)) {
      // the character before the cursor must be different from any frame character
      if !chr.is_frame() {
        if let Some(row) = self.rows.get_mut(row_index) {
          // get the index of the `left vertical line` on the right side of the cell
          if let Some(vert_line_index) = row.search_vert_line_right(col_index) {
            row.shift_text_left(delete_char_index, vert_line_index.saturating_sub(1));
            self.cursor.dec_col(1);
            // remove whitespaces before the vertical line
            self.remove_vertical_spaces(col_index, row_index);
            // invalidate the content region because the width has changed
            self.invalidate_content_region();
            // Return `true` to signal that a character was deleted.
            return true;
          }
        }
      } else {
        // When the cursor is placed just before the frame character, then try to
        // move the content from current line to the previous line and shrink the cell height.
        // If the height of the decision table has changes, then the content region
        // is invalidated inside this function (!).
        return self.unsplit_line();
      }
    }
    false
  }

  pub fn delete_char_under_cursor(&mut self) -> bool {
    // Get the current cursor position.
    let (col_index, row_index) = self.cursor.pos();
    // Get the character under the cursor, otherwise there is nothing to do.
    if let Some(chr) = self.rows.get(row_index).and_then(|row| row.get(col_index)) {
      // The character under the cursor must be different from the frame character.
      // This is important when the cursor shape is caret, because it looks like being on the left side
      // of the character, but in fact it is positioned on the frame character.
      if !chr.is_frame() {
        // The current row must be valid.
        if let Some(row) = self.rows.get_mut(row_index) {
          // Get the index of the `left vertical line` on the right side of the edited cell.
          if let Some(vert_line_index) = row.search_vert_line_right(col_index) {
            // Shift the text contained in the cell one character to the left, deleting the first one.
            row.shift_text_left(col_index, vert_line_index.saturating_sub(1));
            // Remove single whitespace before each vertical line.
            self.remove_vertical_spaces(col_index, row_index);
            // Invalidate the content region because the width has changed.
            self.invalidate_content_region();
            // When the cursor is block or underscore, then check if after removing spaces,
            // the cursor is not positioned on the frame. If this is the case,
            // then move the cursor one position left.
            if (self.cursor.is_block() || self.cursor().is_under_score()) && self.cursor_char().map_or(false, |chr| chr.is_frame()) {
              self.cursor.dec_col(1);
            }
            // Return `true` to signal that a character was deleted.
            return true;
          }
        }
      }
    }
    false
  }

  /// Splits line at cursor position.
  pub fn split_line(&mut self) -> bool {
    // get the current cursor position
    let (col_index, row_index) = self.cursor.pos();
    // get the index of the nearest horizontal line moving down in the current column
    if let Some(horz_line_row_index) = self.search_horizontal_line_or_crossing_down(col_index, row_index) {
      // get current row for calculating cell range
      if let Some(row) = self.rows.get(row_index) {
        // get the cell range (lef and right index)
        if let Some((left_index, right_index)) = row.cell_range(col_index) {
          // get the index of the last row before the nearest horizontal line down
          let mut last_row_index = horz_line_row_index.saturating_sub(1);
          // check if the last row before the horizontal line is empty
          let is_empty = self.rows[last_row_index].is_empty_range(left_index, right_index);
          // if the last row is not empty, or current row is the last row
          // then place whitespaces before appropriate horizontal lines down
          if !is_empty || row_index == last_row_index {
            // prepare vector for top row indexes, highest horizontal line starting from horz_line_row_index
            let row_width = row.len();
            let mut top_row_indexes = Vec::with_capacity(row_width);
            // find the horizontal lines and populate the vector
            for ix_col in 0..row_width {
              for (ix_row, row) in self.rows[horz_line_row_index..].iter().enumerate() {
                if let Some(chr) = row.get(ix_col) {
                  if chr.is_horz_line_or_crossing() {
                    top_row_indexes.push(horz_line_row_index + ix_row);
                    break;
                  }
                }
              }
            }
            // find the lowest horizontal line index an insert a new row just before this line
            let bottom_row_index = *top_row_indexes.iter().max().unwrap();
            self.rows.insert(bottom_row_index, Row::new(row_width, SPACE));
            // shift down the content and insert whitespace at the top of the shifted area
            for (ix_col, top_row_index) in top_row_indexes.iter().enumerate() {
              for ix_row in (*top_row_index + 1..=bottom_row_index).rev() {
                self.rows[ix_row][ix_col].set_char(self.rows[ix_row - 1][ix_col].char());
              }
              let chr = &self.rows[*top_row_index - 1][ix_col];
              let ch = if chr.is_frame() { chr.char() } else { SPACE };
              self.rows[*top_row_index][ix_col].set_char(ch);
            }
            // whitespaces are inserted also in the edited cell, so adjust the last row index
            last_row_index += 1;
          }
          // shift the content of the edited cell one row down starting below the current row
          for ix_col in left_index..=right_index {
            for ix_row in (row_index + 1..=last_row_index).rev() {
              self.rows[ix_row][ix_col].set_char(self.rows[ix_row - 1][ix_col].char());
            }
          }
          // move text after current cursor position to line below
          let mut ix_col = col_index;
          let mut ix_col_below = left_index;
          while ix_col <= right_index {
            self.rows[row_index + 1][ix_col_below].set_char(self.rows[row_index][ix_col].char());
            self.rows[row_index][ix_col].set_char(SPACE);
            ix_col += 1;
            ix_col_below += 1;
          }
          while ix_col_below <= right_index {
            self.rows[row_index + 1][ix_col_below].set_char(SPACE);
            ix_col_below += 1;
          }
          // update the cursor position to the below at the beginning of the cell
          self.cursor.set(left_index, row_index + 1);
          // The height of the decision table has changed, so invalidate the content region.
          self.invalidate_content_region();
          // Return `true` to signal that the content has changed.
          return true;
        }
      }
    }
    false
  }

  /// Unsplits (joins) the current line with the line in a row above.
  pub fn unsplit_line(&mut self) -> bool {
    // Retrieve the current cursor position.
    let (col_index, row_index) = self.cursor.pos();
    // Make sure, there is no `frame` character above the current cursor position.
    if let Some(chr_above) = self.cursor_char_above() {
      if !chr_above.is_frame() {
        // Retrieve the row above and the current row.
        if let Some((row_above, row)) = self.rows.get(row_index - 1).zip(self.rows.get(row_index)) {
          // Retrieve the indexes of the first and last character in the current cell.
          if let Some((left_index, right_index)) = row.cell_range(col_index) {
            // Calculate the length of the content in the row above current. This length is calculated from the cell begin to the last non-space character.
            let text_len_above = row_above[left_index..=right_index].iter().rposition(|chr| !chr.is_space()).map_or(0, |pos| pos + 1);
            // Calculate the length of the content in the current row. This length is calculated from the cell begin to the last non-space character.
            let text_len_below = row[left_index..=right_index].iter().rposition(|chr| !chr.is_space()).map_or(0, |pos| pos + 1);
            // Check if the cell is wide enough to hold both pieces of text.
            if right_index - left_index + 1 >= text_len_above + text_len_below {
              // If there is enough space, append the content from the current row to the end of the row above.
              for i in 0..text_len_below {
                row_above[left_index + text_len_above + i].set_char(row[left_index + i].char());
                row[left_index + i].set_char(SPACE);
              }
              // Shift the rest of the cell up by one row, leaving only whitespace before the horizontal line.
              if let Some(horz_line_row_index) = self.search_horizontal_line_or_crossing_down(col_index, row_index) {
                let last_row_index = horz_line_row_index.saturating_sub(1);
                for ix_col in left_index..=right_index {
                  for ix_row in row_index..last_row_index {
                    self.rows[ix_row][ix_col].set_char(self.rows[ix_row + 1][ix_col].char());
                  }
                  self.rows[last_row_index][ix_col].set_char(SPACE);
                }
              }
              // Reduce the height by removing whitespace before the horizontal line below.
              let changed = self.remove_horizontal_whitespaces();
              // Update the cursor position.
              self.cursor.set(left_index + text_len_above, row_index - 1);
              // Return the changed flag to signal if the content has changed or not.
              return changed;
            }
          }
        }
      }
    }
    false
  }

  /// aaa
  fn remove_horizontal_whitespaces(&mut self) -> bool {
    // Retrieve the current cursor position.
    let (col_index, row_index) = self.cursor.pos();
    // Find the index of the nearest row containing horizontal line or crossing in the column pointed by the cursor.
    if let Some(horz_line_row_index) = self.search_horizontal_line_or_crossing_down(col_index, row_index) {
      // Retrieve the current row.
      if let Some(row) = self.rows.get(row_index) {
        // Prepare the vector for nearest row indexes, i.e. nearest horizontal line starting from `horz_line_row_index`.
        let row_width = row.len();
        let mut nearest_row_indexes: Vec<usize> = Vec::with_capacity(row_width);
        // Find the nearest horizontal lines and populate the vector of nearest indexes.
        for ix_col in 0..row_width {
          for (ix_row, row) in self.rows[horz_line_row_index..].iter().enumerate() {
            if let Some(chr) = row.get(ix_col) {
              if chr.is_horz_line_or_crossing() {
                nearest_row_indexes.push(horz_line_row_index + ix_row);
                break;
              }
            }
          }
        }
        // Check whether before each horizontal line there is only a space character.
        let all_whitespaces = nearest_row_indexes.iter().enumerate().all(|(ix_col, ix_row)| {
          *ix_row > 0
            && self
              .rows
              .get(ix_row - 1)
              .and_then(|row| row.get(ix_col))
              .map_or(true, |chr| chr.is_space() || chr.is_vert_line())
        });
        // If only spaces are before th horizontal line, then shrink the decision table's height.
        if all_whitespaces {
          //-------------------------------------------------------------------------
          // Here handle a special case, where the deleted spaces are in the
          // information item name cell (above the decision table's body).
          //-------------------------------------------------------------------------
          if let Some(((ix_join, _), (ix_max, ix_min))) = self.join_row_info().zip(nearest_row_indexes.iter().max().zip(nearest_row_indexes.iter().min())) {
            if ix_join == *ix_max && ix_join == *ix_min {
              // Delete the row above the joining row.
              self.rows.remove(ix_join - 1);
              // The height of the decision table has changed, so invalidate the content region.
              self.invalidate_content_region();
              // Return `true` to signal that the content has changed.
              // WATCH OUT THIS EARLY RETURN!
              return true;
            }
          }
          //-------------------------------------------------------------------------
          // Here handle a standard situation, when spaces to be deleted
          // are inside the decision table's body.
          //-------------------------------------------------------------------------
          // Shift the content of each column up to remove spaces.
          for (ix_col, nearest_row_index) in nearest_row_indexes.iter().enumerate() {
            for (ix_row, row) in self.rows[nearest_row_index - 1..].iter().enumerate() {
              let ix_row_below = nearest_row_index + ix_row;
              if let Some(row_below) = self.rows.get(ix_row_below) {
                if let Some((chr, chr_below)) = row.get(ix_col).zip(row_below.get(ix_col)) {
                  chr.set_char(chr_below.char());
                }
              }
            }
          }
          // Delete the last row of the decision table.
          self.rows.pop();
          // The height of the decision table has changed, so invalidate the content region.
          self.invalidate_content_region();
          // Return `true` to signal that the content has changed.
          return true;
        }
      }
    }
    false
  }

  /// Searches for the index of the first row containing a horizontal line.
  fn search_horizontal_line_or_crossing_down(&self, col_index: usize, mut row_index: usize) -> Option<usize> {
    while let Some(row) = self.rows.get(row_index) {
      if let Some(chr) = row.get(col_index) {
        if chr.is_horz_line_or_crossing() {
          return Some(row_index);
        }
      }
      row_index += 1;
    }
    None
  }

  /// This is complicated. Document this ;-)
  fn remove_vertical_spaces(&mut self, col_index: usize, row_index: usize) {
    let join_row_info = self.join_row_info();
    if let Some((join_row_index, is_full)) = join_row_info {
      if row_index < join_row_index {
        if let Some(vert_line_index) = self.rows.get(row_index).and_then(|row| row.search_vert_line_right(col_index)) {
          let spaces_present_above_join_row = self.rows[..join_row_index].iter().all(|row| row.is_deletable_space(vert_line_index));
          let spaces_present_below_join_row = self.rows[join_row_index..].iter().all(|row| row.is_deletable_space(vert_line_index));
          if spaces_present_above_join_row {
            for row in self.rows[..join_row_index].iter_mut() {
              row.delete_space(vert_line_index);
            }
          }
          if is_full && spaces_present_below_join_row {
            for row in self.rows[join_row_index..].iter_mut() {
              row.delete_space(vert_line_index);
            }
          }
        }
      } else {
        let spaces_present_above_join_row = self.rows[..join_row_index].iter().all(|row| row.is_deletable_space(col_index));
        let space_present_below_join_row = self.rows[join_row_index..].iter().all(|row| row.is_deletable_space(col_index));
        if is_full {
          if spaces_present_above_join_row && space_present_below_join_row {
            for row in self.rows.iter_mut() {
              row.delete_space(col_index);
            }
          }
        } else if space_present_below_join_row {
          for row in self.rows[join_row_index..].iter_mut() {
            row.delete_space(col_index);
          }
        }
      }
      self.update_joining_row(join_row_info);
    } else {
      let spaces_present = self.rows.iter().all(|row| row.is_deletable_space(col_index));
      if spaces_present {
        for row in self.rows.iter_mut() {
          row.delete_space(col_index);
        }
      }
    }
  }

  fn update_joining_row(&mut self, join_row_info: JoinRowInfo) {
    if let Some((join_row_index, _)) = join_row_info {
      if let Some((row, upper_row)) = self.rows.get(join_row_index).zip(self.rows.get(join_row_index.saturating_sub(1))) {
        let full_join = row.len() == upper_row.len();
        for (i, chr) in row.iter().enumerate() {
          if full_join {
            chr.set_full_join();
          } else {
            chr.set_join();
          }
          if let Some(upper_chr) = upper_row.get(i) {
            if upper_chr.is_single_vert_line() {
              match chr.char() {
                LIGHT_HORIZONTAL => chr.set_char(LIGHT_UP_AND_HORIZONTAL),
                LIGHT_DOWN_AND_HORIZONTAL => chr.set_char(LIGHT_VERTICAL_AND_HORIZONTAL),
                LIGHT_DOWN_AND_LEFT => chr.set_char(LIGHT_VERTICAL_AND_LEFT),
                DOWN_DOUBLE_AND_HORIZONTAL_SINGLE => chr.set_char(VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE),
                _ => {}
              }
              continue; // <--- DO NOT OVERSEE THIS WHEN ANALYSING THE CODE
            }
          }
          match chr.char() {
            LIGHT_UP_AND_HORIZONTAL => chr.set_char(LIGHT_HORIZONTAL),
            LIGHT_VERTICAL_AND_HORIZONTAL => chr.set_char(LIGHT_DOWN_AND_HORIZONTAL),
            LIGHT_VERTICAL_AND_LEFT => chr.set_char(LIGHT_DOWN_AND_LEFT),
            VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE => chr.set_char(DOWN_DOUBLE_AND_HORIZONTAL_SINGLE),
            _ => {}
          }
          // <-- do not place any code here, because of `continue` several lines above
        }
      }
    }
  }

  /// Searches for the join row and returns its index and flags.
  ///
  /// This is usually a very short loop because the joining row (if present)
  /// is always somewhere at the top of the decision table.
  fn join_row_info(&self) -> JoinRowInfo {
    for (index, row) in self.rows.iter().enumerate() {
      if row.is_join() || row.is_full_join() {
        return Some((index, row.is_full_join()));
      }
    }
    None
  }

  /// Invalidates the size (width and height) of the text content.
  ///
  /// Next time the width or height is needed, then will be calculated
  /// based on the current number of columns and rows.
  fn invalidate_content_region(&mut self) {
    self.size = None;
  }

  pub fn is_invalidated_content_region(&self) -> bool {
    self.size.is_none()
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn empty_model() {
    let mut model = Plane::new("");
    assert_eq!(None, model.row());
    assert_eq!(None, model.after(&['a'.into()]));
    assert_eq!(None, model.before(&['a'.into()]));
    assert!(!model.cursor_move_cell_next());
    assert!(!model.cursor_move_cell_prev());
    assert!(!model.cursor_move_cell_start());
    assert!(!model.cursor_move_cell_end());
    assert!(!model.cursor_move_row_start());
    assert!(!model.cursor_move_row_end());
    assert_eq!(None, model.cursor_char());
  }
}
