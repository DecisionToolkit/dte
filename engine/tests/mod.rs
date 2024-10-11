mod test_controller;
mod test_files;

use dtee::{Char, Controller, Row};
use std::fmt::Write;

enum Action {
  AssertPos(usize, usize),
  CellEnd(usize),
  CellNext(usize),
  CellPrev(usize),
  CellStart(usize),
  DeleteBefore(usize),
  DeleteUnder(usize),
  Insert(char, usize),
  InsertStr(String, usize),
  MoveDown(usize),
  MoveLeft(usize),
  MoveRight(usize),
  MoveUp(usize),
  RowEnd(usize),
  RowStart(usize),
  SplitLine(usize),
  ToggleCaretBlock,
  ToggleCaretUnderScore,
}

/// A utility function for printing the decision table like on paper.
fn paper(controller: &Controller) {
  println!("{}", text(controller));
}

/// A utility function for printing the decision table like on screen.
fn screen(controller: &Controller) {
  println!("{}", view(controller));
}

/// A utility function that converts a two-dimensional array of characters
/// into a single string containing the decision table.
fn text(controller: &Controller) -> String {
  let mut output = String::new();
  let _ = writeln!(output);
  for row in controller.content() {
    let _ = writeln!(output, "    {}", row.text());
  }
  let _ = write!(output, "  ");
  output
}

/// A utility function that converts a visible part
/// of the decision table into a single string.
fn view(controller: &Controller) -> String {
  let mut output = String::new();
  let mut last_row = None;
  let f = |_, row_index, chr: &Char| {
    if let Some(ix_row) = last_row {
      if row_index > ix_row {
        let _ = writeln!(output);
      }
    }
    let _ = write!(output, "{chr}");
    last_row = Some(row_index);
  };
  controller.visit_visible_content(f, Some('░'.into()), None, None);
  // wrap the decision table view in frame simulating the terminal
  let width = output.lines().map(|line| line.chars().count()).max().unwrap();
  let mut framed = String::new();
  let _ = write!(framed, "\n    ╭{}╮", "─".repeat(width));
  output.lines().for_each(|line| {
    let char_count = line.chars().count();
    let _ = write!(
      framed,
      "\n    │{}{}│",
      line,
      if char_count < width { " ".repeat(width - char_count) } else { "".to_string() }
    );
  });
  let _ = write!(framed, "\n    ╰{}╯\n  ", "─".repeat(width));
  framed
}

/// A utility function that converts a two-dimensional array of characters
/// into a single string containing character's attributes.
fn attr(input: &[Row]) -> String {
  let mut output = String::new();
  let _ = writeln!(output);
  for line in input {
    let _ = writeln!(
      output,
      "    {}",
      line
        .iter()
        .map(|chr| {
          if chr.is_join() {
            '◇'
          } else if chr.is_full_join() {
            '◆'
          } else {
            '•'
          }
        })
        .collect::<String>()
    );
  }
  let _ = write!(output, "  ");
  output
}

/// A utility function for executing editor actions.
fn actions(controller: &mut Controller, actions: &[Action]) {
  actions.iter().for_each(|action| match action {
    Action::AssertPos(col_index, row_index) => {
      assert_eq!((*col_index, *row_index), controller.cursor().pos());
    }
    Action::MoveUp(n) => {
      (0..*n).for_each(|_| {
        controller.cursor_move_up();
      });
    }
    Action::MoveRight(n) => {
      (0..*n).for_each(|_| {
        controller.cursor_move_right();
      });
    }
    Action::MoveDown(n) => {
      (0..*n).for_each(|_| {
        controller.cursor_move_down();
      });
    }
    Action::MoveLeft(n) => {
      (0..*n).for_each(|_| {
        controller.cursor_move_left();
      });
    }
    Action::CellEnd(n) => {
      (0..*n).for_each(|_| {
        controller.cursor_move_cell_end();
      });
    }
    Action::CellNext(n) => {
      (0..*n).for_each(|_| {
        controller.cursor_move_cell_next();
      });
    }
    Action::CellPrev(n) => {
      (0..*n).for_each(|_| {
        controller.cursor_move_cell_prev();
      });
    }
    Action::CellStart(n) => {
      (0..*n).for_each(|_| {
        controller.cursor_move_cell_start();
      });
    }
    Action::Insert(ch, n) => {
      (0..*n).for_each(|_| {
        controller.insert_char(*ch);
      });
    }
    Action::InsertStr(s, n) => {
      (0..*n).for_each(|_| {
        for ch in s.chars() {
          controller.insert_char(ch);
        }
      });
    }
    Action::DeleteBefore(n) => {
      (0..*n).for_each(|_| {
        controller.delete_char_before_cursor();
      });
    }
    Action::DeleteUnder(n) => {
      (0..*n).for_each(|_| {
        controller.delete_char_under_cursor();
      });
    }
    Action::RowEnd(n) => {
      (0..*n).for_each(|_| {
        controller.cursor_move_row_end();
      });
    }
    Action::RowStart(n) => {
      (0..*n).for_each(|_| {
        controller.cursor_move_row_start();
      });
    }
    Action::SplitLine(n) => {
      (0..*n).for_each(|_| {
        controller.split_line();
      });
    }
    Action::ToggleCaretBlock => {
      controller.cursor_toggle_caret_block();
    }
    Action::ToggleCaretUnderScore => {
      controller.cursor_toggle_caret_under_score();
    }
  })
}
