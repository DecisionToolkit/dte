//! Tests for cursor moves.
//!
//! The input decision table used in the following test cases looks like this:
//!
//! ```text
//! ┌─────────────────┐
//! │  Order options  │
//! ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
//! │ U │           │       ║    Order options    ║             │           │
//! │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
//! │   │   type    │ size  ║ Discount │ Priority ║             │           │
//! │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
//! │   │"Business",│       ║          │"Normal", ║             │           │
//! │   │"Private"  │       ║          │ "High",  ║             │           │
//! │   │           │       ║          │ "Low"    ║             │           │
//! ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
//! │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
//! ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
//! │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
//! ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
//! │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
//! └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
//! ```
//!
//! To try all these test cases in "real-life" just run `task run2`
//! and follow the movements from test cases.

use crate::test_files::INPUT_0002;
use dtee::Controller;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// After creating the controller, the cursor position should
/// be in the top left corner of the decision table, value (1, 1).
/// Because on the left side of the cursor is the border
/// of the decision table, so moving cursor to the left should
/// have no effect, cursor stays where it was.
#[test]
fn _0001() {
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
  // moving the cursor to the left shouldn't generate any change
  assert_eq!(None, controller.cursor_move_left());
  // cursor position is still (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
}

/// Moving to the right should change the cursor position.
/// Moving then to the left should also update the position.
#[test]
fn _0002() {
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
  // moving the cursor to the right should generate a change without repaint
  assert_eq!(Some(false), controller.cursor_move_right());
  // cursor position is now (1, 2)
  assert_eq!((2, 1), controller.cursor_position());
  // moving the cursor again to the left should generate a change without repaint
  assert_eq!(Some(false), controller.cursor_move_left());
  // cursor position should be now (1, 1) again
  assert_eq!((1, 1), controller.cursor_position());
}

/// Moving to the right should stop before the right border of the decision table.
/// After creating the controller, cursor is on the left-most side of the top cell
/// containing the information item name of the decision table.
/// This cell is 17 characters wide. So moving cursor to the right more
/// than 17 times should have no more effect.
#[test]
fn _0003() {
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
  // moving the cursor to the right 17 times should generate a change
  (1..=17).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_right());
  });
  assert_eq!((18, 1), controller.cursor_position());
  // now moving several time to the right should have no effect
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_right());
    assert_eq!((18, 1), controller.cursor_position());
  });
}

/// Moving to the right and back to the left but now the decision table
/// is wider than the viewing area. So cursor moves change the cursor position,
/// but also signal that the view should be updated (repainted), because
/// the character under the moved cursor should be brought into view.
#[test]
fn _0004() {
  // The width of the viewing area is now less than the width of the decision table
  let mut controller = Controller::new(INPUT_0002, 10, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
  // moving the cursor to the right 7 times should generate change without view update
  (1..=7).for_each(|i| {
    assert_eq!(Some(false), controller.cursor_move_right());
    assert_eq!((1 + i, 1), controller.cursor_position());
  });
  assert_eq!((8, 1), controller.cursor_position());
  // moving the cursor to the right 10 times should generate change with view update
  (1..=10).for_each(|i| {
    assert_eq!(Some(true), controller.cursor_move_right());
    assert_eq!((8 + i, 1), controller.cursor_position());
  });
  assert_eq!((18, 1), controller.cursor_position());
  // now moving again several times to the right should have no effect
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_right());
    assert_eq!((18, 1), controller.cursor_position());
  });
  // now we move back to the left several times
  // firstly moving to the left should generate change without update
  (1..=7).for_each(|i| {
    assert_eq!(Some(false), controller.cursor_move_left());
    assert_eq!((18 - i, 1), controller.cursor_position());
  });
  assert_eq!((11, 1), controller.cursor_position());
  // secondly moving the cursor to the left 10 times should generate change with view update
  (1..=10).for_each(|i| {
    assert_eq!(Some(true), controller.cursor_move_left());
    assert_eq!((11 - i, 1), controller.cursor_position());
  });
  assert_eq!((1, 1), controller.cursor_position());
  // finally moving to the left several times should have no effect
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_left());
    assert_eq!((1, 1), controller.cursor_position());
  });
}

/// Moving cursor up should have no effect, because at the beginning it is
/// in the top left corner of the decision table.
#[test]
fn _0005() {
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
  // moving the cursor up shouldn't generate any change
  assert_eq!(None, controller.cursor_move_up());
  // cursor position is still (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
}

/// Moving cursor down should generate change.
#[test]
fn _0006() {
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
  // moving the cursor down should generate change without update
  assert_eq!(Some(false), controller.cursor_move_down());
  // cursor position is moved below the bottom horizontal line of the first cell,
  // so it is (1, 3) now
  assert_eq!((1, 3), controller.cursor_position());
}

/// Moving cursor down to the end of the decision table and up
/// to the beginning of the decision table.
#[test]
fn _0007() {
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
  // moving the cursor down should generate change without update
  (1..=10).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_down());
  });
  assert_eq!((1, 15), controller.cursor_position());
  // when the bottom is reached, no more cursor movements are expected
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_down());
    assert_eq!((1, 15), controller.cursor_position());
  });
  // moving the cursor up should generate change without update
  (1..=10).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_up());
  });
  assert_eq!((1, 1), controller.cursor_position());
  // when the top is reached, no more cursor movements are expected
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_up());
    assert_eq!((1, 1), controller.cursor_position());
  });
}

/// Moving cursor again down to the end of the decision table and up
/// to the beginning of the decision table, but now the height of the
/// display area is less than the height of the decision table.
#[test]
fn _0008() {
  let mut controller = Controller::new(INPUT_0002, WIDTH, 10);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
  // firstly, moving the cursor down should generate change without update
  (1..=6).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_down());
  });
  assert_eq!((1, 8), controller.cursor_position());
  // secondly, moving the cursor down should generate change with update
  (1..=4).for_each(|_| {
    assert_eq!(Some(true), controller.cursor_move_down());
  });
  assert_eq!((1, 15), controller.cursor_position());
  // when the bottom is reached, no more cursor movements are expected
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_down());
    assert_eq!((1, 15), controller.cursor_position());
  });
  // now moving up, firstly, moving the cursor up should generate change without update
  (1..=4).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_up());
  });
  assert_eq!((1, 8), controller.cursor_position());
  // secondly, moving the cursor up should generate change with update
  (1..=6).for_each(|_| {
    assert_eq!(Some(true), controller.cursor_move_up());
  });
  assert_eq!((1, 1), controller.cursor_position());
  // when the top is reached, no more cursor movements are expected
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_up());
    assert_eq!((1, 1), controller.cursor_position());
  });
}

/// SPECIAL CASE: when moving down 4 times, the cursor stays in the first cell
/// that has `├` on the right side. So moving cursor to the right should stop
/// at the 4th column.
///
/// ```text
/// │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
/// │   │   type    │ size  ║ Discount │ Priority ║             │           │
/// │  █├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
/// │   │"Business",│       ║          │"Normal", ║             │           │
/// │   │"Private"  │       ║          │ "High",  ║             │           │
/// ```
#[test]
fn _0009() {
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
  // move down 4 times
  (1..=4).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_down());
  });
  assert_eq!((1, 6), controller.cursor_position());
  // move 3 times to the right
  (1..=3).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_right());
  });
  assert_eq!((4, 6), controller.cursor_position());
  // move several times to the right
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_right());
  });
  assert_eq!((4, 6), controller.cursor_position());
}

/// SPECIAL CASE: when moving down 1 row, then moving to the end of the decision table row,
/// then moving down one row, then moving to the left should stop before '╢' character.
///
/// ```text
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │ U │           │       ║    Order options    ║             │           │
/// │   │ Customer  │ Order ╟──────────┬──────────╢█Description │ Reference │
/// │   │   type    │ size  ║ Discount │ Priority ║             │           │
/// │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
/// ```
#[test]
fn _0010() {
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor_position());
  // move down one row
  assert_eq!(Some(false), controller.cursor_move_down());
  assert_eq!((1, 3), controller.cursor_position());
  // move right to the end of the decision table (row)
  assert_eq!(Some(false), controller.cursor_move_row_end());
  assert_eq!((72, 3), controller.cursor_position());
  // move down one row
  assert_eq!(Some(false), controller.cursor_move_down());
  assert_eq!((72, 4), controller.cursor_position());
  // now moving to the left should generate changes until the '╢' is encountered
  (1..=25).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_left());
  });
  assert_eq!((47, 4), controller.cursor_position());
  // moving several times to the left should have no effect
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_left());
  });
  assert_eq!((47, 4), controller.cursor_position());
}
