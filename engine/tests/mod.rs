mod test_controller;
mod test_files;

use dtee::{Controller, Row};
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
  MoveDown(usize),
  MoveLeft(usize),
  MoveRight(usize),
  MoveUp(usize),
  RowEnd(usize),
  RowStart(usize),
  SplitLine(usize),
}

/// A utility function for printing the whole decision table.
fn show(controller: &Controller) {
  println!("{}", text(controller));
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
      assert_eq!((*col_index, *row_index), controller.cursor_position());
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
  })
}
