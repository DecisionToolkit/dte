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

use super::*;
use dtee::{Controller, Updates};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// After creating the controller, the cursor position should
/// be in the top left corner of the decision table, value (1, 1).
/// Because on the left side of the cursor is the border
/// of the decision table, so moving cursor to the left should
/// have no effect, cursor stays where it was.
#[test]
fn _0001() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // moving the cursor to the left shouldn't generate any change
  assert_eq!(Updates::default(), controller.cursor_move_left());
  // cursor position is still (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
}

/// Moving to the right should change the cursor position.
/// Moving then to the left should also update the position.
#[test]
fn _0002() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // moving the cursor to the right should generate a change without repaint
  assert_eq!(Some(false), controller.cursor_move_right().temporary());
  // cursor position is now (1, 2)
  assert_eq!((2, 1), controller.cursor().pos());
  // moving the cursor again to the left should generate a change without repaint
  assert_eq!((true, false, false, false), controller.cursor_move_left().get());
  // cursor position should be now (1, 1) again
  assert_eq!((1, 1), controller.cursor().pos());
}

/// Moving to the right should stop before the right border of the decision table.
/// After creating the controller, cursor is on the left-most side of the top cell
/// containing the information item name of the decision table.
/// This cell is 17 characters wide. So moving cursor to the right more
/// than 17 times should have no more effect.
#[test]
fn _0003() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // moving the cursor to the right 17 times should generate a change
  (1..=17).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_right().temporary());
  });
  assert_eq!((18, 1), controller.cursor().pos());
  // now moving several time to the right should have no effect
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_right().temporary());
    assert_eq!((18, 1), controller.cursor().pos());
  });
}

/// Moving to the right and back to the left but now the decision table
/// is wider than the viewing area. So cursor moves change the cursor position,
/// but also signal that the view should be updated (repainted), because
/// the character under the moved cursor should be brought into view.
#[test]
fn _0004() {
  // The width of the viewing area is now less than the width of the decision table
  let mut controller = Controller::new(INPUT_0002).with_viewport(10, 21);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // moving the cursor to the right 7 times should generate change without view update
  (1..=6).for_each(|i| {
    assert_eq!(Some(false), controller.cursor_move_right().temporary());
    assert_eq!((1 + i, 1), controller.cursor().pos());
  });
  assert_eq!((7, 1), controller.cursor().pos());
  // moving the cursor to the right 10 times should generate change with view update
  (1..=11).for_each(|i| {
    assert_eq!(Some(true), controller.cursor_move_right().temporary());
    assert_eq!((7 + i, 1), controller.cursor().pos());
  });
  assert_eq!((18, 1), controller.cursor().pos());
  // now moving again several times to the right should have no effect
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_right().temporary());
    assert_eq!((18, 1), controller.cursor().pos());
  });
  // now we move back to the left several times
  // firstly moving to the left should generate change without update
  (1..=6).for_each(|i| {
    assert_eq!((true, false, false, false), controller.cursor_move_left().get());
    assert_eq!((18 - i, 1), controller.cursor().pos());
  });
  assert_eq!((12, 1), controller.cursor().pos());
  // secondly moving the cursor to the left 10 times should generate change with view update
  (1..=11).for_each(|i| {
    assert_eq!((true, false, true, false), controller.cursor_move_left().get());
    assert_eq!((12 - i, 1), controller.cursor().pos());
  });
  assert_eq!((1, 1), controller.cursor().pos());
  // finally moving to the left several times should have no effect
  (1..=100).for_each(|_| {
    assert_eq!((false, false, false, false), controller.cursor_move_left().get());
    assert_eq!((1, 1), controller.cursor().pos());
  });
}

/// Moving cursor up should have no effect, because at the beginning it is
/// in the top left corner of the decision table.
#[test]
fn _0005() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // moving the cursor up shouldn't generate any change
  assert_eq!(None, controller.cursor_move_up().temporary());
  // cursor position is still (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
}

/// Moving cursor down should generate change.
#[test]
fn _0006() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // moving the cursor down should generate change without update
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  // cursor position is moved below the bottom horizontal line of the first cell,
  // so it is (1, 3) now
  assert_eq!((1, 3), controller.cursor().pos());
}

/// Moving cursor down to the end of the decision table and up
/// to the beginning of the decision table.
#[test]
fn _0007() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // moving the cursor down should generate change without update
  (1..=10).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_down().temporary());
  });
  assert_eq!((1, 15), controller.cursor().pos());
  // when the bottom is reached, no more cursor movements are expected
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_down().temporary());
    assert_eq!((1, 15), controller.cursor().pos());
  });
  // moving the cursor up should generate change without update
  (1..=10).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_up().temporary());
  });
  assert_eq!((1, 1), controller.cursor().pos());
  // when the top is reached, no more cursor movements are expected
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_up().temporary());
    assert_eq!((1, 1), controller.cursor().pos());
  });
}

/// Moving cursor again down to the end of the decision table and up
/// to the beginning of the decision table, but now the height of the
/// display area is less than the height of the decision table.
#[test]
fn _0008() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, 10);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // firstly, moving the cursor down should generate change without update
  (1..=5).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_down().temporary());
  });
  assert_eq!((1, 7), controller.cursor().pos());
  // secondly, moving the cursor down should generate change with update
  (1..=5).for_each(|_| {
    assert_eq!(Some(true), controller.cursor_move_down().temporary());
  });
  assert_eq!((1, 15), controller.cursor().pos());
  // when the bottom is reached, no more cursor movements are expected
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_down().temporary());
    assert_eq!((1, 15), controller.cursor().pos());
  });
  // now moving up, firstly, moving the cursor up should generate change without update
  (1..=3).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_up().temporary());
  });
  assert_eq!((1, 9), controller.cursor().pos());
  // secondly, moving the cursor up should generate change with update
  (1..=7).for_each(|_| {
    assert_eq!(Some(true), controller.cursor_move_up().temporary());
  });
  assert_eq!((1, 1), controller.cursor().pos());
  // when the top is reached, no more cursor movements are expected
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_up().temporary());
    assert_eq!((1, 1), controller.cursor().pos());
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
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move down 4 times
  (1..=4).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_down().temporary());
  });
  assert_eq!((1, 6), controller.cursor().pos());
  // move 3 times to the right
  (1..=3).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_right().temporary());
  });
  assert_eq!((4, 6), controller.cursor().pos());
  // move several times to the right
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_right().temporary());
  });
  assert_eq!((4, 6), controller.cursor().pos());
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
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move down one row
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  // move right to the end of the decision table (row)
  assert_eq!(Some(false), controller.cursor_move_row_end().temporary());
  assert_eq!((72, 3), controller.cursor().pos());
  // move down one row
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((72, 4), controller.cursor().pos());
  // now moving to the left should generate changes until the '╢' is encountered
  (1..=25).for_each(|_| {
    assert_eq!((true, false, false, false), controller.cursor_move_left().get());
  });
  assert_eq!((47, 4), controller.cursor().pos());
  // moving several times to the left should have no effect
  (1..=100).for_each(|_| {
    assert_eq!((false, false, false, false), controller.cursor_move_left().get());
  });
  assert_eq!((47, 4), controller.cursor().pos());
}

/// Moving to the cell start, firstly when the cursor is already there,
/// then moving to the right and again to the cell start.
///
/// ```text
/// ┌─────────────────┐
/// │█ Order options  │
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │ U │           │       ║    Order options    ║             │           │
/// ```
#[test]
fn _0011() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // moving the cursor to cell start shouldn't generate any change
  assert_eq!(None, controller.cursor_move_cell_start().temporary());
  // cursor position is still (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move several times to the right
  (1..=5).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_right().temporary());
  });
  assert_eq!((6, 1), controller.cursor().pos());
  // now moving the cursor to cell start should generate a change without update
  assert_eq!(Some(false), controller.cursor_move_cell_start().temporary());
  assert_eq!((1, 1), controller.cursor().pos());
}

/// Moving to the cell end, firstly when the cursor is already there,
/// then moving to the left and again to the cell end.
///
/// ```text
/// ┌─────────────────┐
/// │  Order options █│
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │ U │           │       ║    Order options    ║             │           │
/// ```
#[test]
fn _0012() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // moving the cursor to cell end should generate a change without update
  assert_eq!(Some(false), controller.cursor_move_cell_end().temporary());
  // moving the cursor again to the cell end should not generate any change
  assert_eq!(None, controller.cursor_move_cell_end().temporary());
  assert_eq!((18, 1), controller.cursor().pos());
  // move several times to the left
  (1..=5).for_each(|_| {
    assert_eq!((true, false, false, false), controller.cursor_move_left().get());
  });
  assert_eq!((13, 1), controller.cursor().pos());
  // now moving the cursor again to cell end should generate a change without update
  assert_eq!(Some(false), controller.cursor_move_cell_end().temporary());
  assert_eq!((18, 1), controller.cursor().pos());
}

/// Moving to the cell end and then to the cell start when the viewing
/// area is narrower than the cell itself
///
/// ```text
/// ┌─────────────────┐
/// │█ Order options █│
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │ U │           │       ║    Order options    ║             │           │
/// ```
#[test]
fn _0013() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(10, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // moving the cursor to cell end should generate a change with update
  assert_eq!(Some(true), controller.cursor_move_cell_end().temporary());
  assert_eq!((18, 1), controller.cursor().pos());
  // moving the cursor to the cell start should generate a change with update
  assert_eq!(Some(true), controller.cursor_move_cell_start().temporary());
  assert_eq!((1, 1), controller.cursor().pos());
}

/// Moving to the row end.
///
/// ```text
/// ┌─────────────────┐
/// │  Order options  │
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │ U │           │       ║    Order options    ║             │          █│
/// │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
/// ```
#[test]
fn _0014() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move ones down
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  // move to the end of the row, change without update
  assert_eq!(Some(false), controller.cursor_move_row_end().temporary());
  assert_eq!((72, 3), controller.cursor().pos());
  // move again to row end should not generate any change
  assert_eq!(None, controller.cursor_move_row_end().temporary());
  assert_eq!((72, 3), controller.cursor().pos());
}

/// Moving to the row start.
///
/// ```text
/// ┌─────────────────┐
/// │  Order options  │
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │█U │           │       ║    Order options    ║             │           │
/// │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
/// ```
#[test]
fn _0015() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move ones down
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  // move to the end of the row, change without update
  assert_eq!(Some(false), controller.cursor_move_row_end().temporary());
  assert_eq!((72, 3), controller.cursor().pos());
  // move to row start should generate a change without update
  assert_eq!(Some(false), controller.cursor_move_row_start().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  // move to row start again should not generate any change
  assert_eq!(None, controller.cursor_move_row_start().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
}

/// Moving to the row end and row start when the viewing area is narrower
/// than the decision table width. This test uses all cursor shapes.
///
/// ```text
/// ┌─────────────────┐
/// │  Order options  │
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │█U │           │       ║    Order options    ║             │          █│
/// │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
/// ```
#[test]
fn _0016() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(30, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move one down
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 3), controller.cursor().pos());

  // CARET
  assert!(controller.cursor().is_caret());
  // move to the end of the row, change with update
  assert_eq!(Some(true), controller.cursor_move_row_end().temporary());
  assert_eq!((72, 3), controller.cursor().pos());
  // move to row start should generate a change with update
  assert_eq!(Some(true), controller.cursor_move_row_start().temporary());
  assert_eq!((1, 3), controller.cursor().pos());

  // BLOCK
  controller.cursor_toggle_caret_block();
  assert!(controller.cursor().is_block());
  // move to the end of the row, change with update
  assert_eq!(Some(true), controller.cursor_move_row_end().temporary());
  assert_eq!((71, 3), controller.cursor().pos());
  // move to row start should generate a change with update
  assert_eq!(Some(true), controller.cursor_move_row_start().temporary());
  assert_eq!((1, 3), controller.cursor().pos());

  // UNDERSCORE
  controller.cursor_toggle_caret_under_score();
  assert!(controller.cursor().is_under_score());
  // move to the end of the row, change with update
  assert_eq!(Some(true), controller.cursor_move_row_end().temporary());
  assert_eq!((71, 3), controller.cursor().pos());
  // move to row start should generate a change with update
  assert_eq!(Some(true), controller.cursor_move_row_start().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
}

/// Moving to the next cell.
///
/// ```text
/// ┌─────────────────┐
/// │  Order options  │
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │ U │█          │█      ║█   Order options    ║█            │█         █│
/// │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
/// ```
#[test]
fn _0017() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move one down
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  assert_eq!((5, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  assert_eq!((17, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  assert_eq!((25, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  assert_eq!((47, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  assert_eq!((61, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  assert_eq!((72, 3), controller.cursor().pos());
  // stay at the end
  assert_eq!(None, controller.cursor_move_cell_next().temporary());
  assert_eq!((72, 3), controller.cursor().pos());
}

/// Moving to the previous cell.
///
/// ```text
/// ┌─────────────────┐
/// │  Order options  │
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │█U█│          █│      █║    Order options   █║            █│           │
/// │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
/// ```
#[test]
fn _0018() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move one down
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  // move to the row end
  assert_eq!(Some(false), controller.cursor_move_row_end().temporary());
  assert_eq!((72, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(false), controller.cursor_move_cell_prev().temporary());
  assert_eq!((60, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(false), controller.cursor_move_cell_prev().temporary());
  assert_eq!((46, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(false), controller.cursor_move_cell_prev().temporary());
  assert_eq!((24, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(false), controller.cursor_move_cell_prev().temporary());
  assert_eq!((16, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(false), controller.cursor_move_cell_prev().temporary());
  assert_eq!((4, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(false), controller.cursor_move_cell_prev().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  // stay at the beginning
  assert_eq!(None, controller.cursor_move_cell_prev().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
}

/// Moving to the next cell and back to the previous cell when the display
/// area is narrower than the decision table width.
#[test]
fn _0019() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(40, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move one down
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  assert_eq!((5, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  assert_eq!((17, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  assert_eq!((25, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(true), controller.cursor_move_cell_next().temporary());
  assert_eq!((47, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(true), controller.cursor_move_cell_next().temporary());
  assert_eq!((61, 3), controller.cursor().pos());
  // next cell
  assert_eq!(Some(true), controller.cursor_move_cell_next().temporary());
  assert_eq!((72, 3), controller.cursor().pos());
  // stay at the end
  assert_eq!(None, controller.cursor_move_cell_next().temporary());
  assert_eq!((72, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(false), controller.cursor_move_cell_prev().temporary());
  assert_eq!((60, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(false), controller.cursor_move_cell_prev().temporary());
  assert_eq!((46, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(true), controller.cursor_move_cell_prev().temporary());
  assert_eq!((24, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(true), controller.cursor_move_cell_prev().temporary());
  assert_eq!((16, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(true), controller.cursor_move_cell_prev().temporary());
  assert_eq!((4, 3), controller.cursor().pos());
  // previous cell
  assert_eq!(Some(true), controller.cursor_move_cell_prev().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  // stay at the beginning
  assert_eq!(None, controller.cursor_move_cell_prev().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
}

/// SPECIAL CASE: jumping to the end of the row and back to the beginning
/// of the row should work, even if there is a horizontal line on the way.
///
/// ```text
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │ U │           │       ║    Order options    ║             │           │
/// │█  │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference█│
/// │   │   type    │ size  ║ Discount │ Priority ║             │           │
/// │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
/// ```
#[test]
fn _0020() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move two down
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 4), controller.cursor().pos());
  // move to the row end
  assert_eq!(Some(false), controller.cursor_move_row_end().temporary());
  assert_eq!((72, 4), controller.cursor().pos());
  // stay at the end
  assert_eq!(None, controller.cursor_move_row_end().temporary());
  assert_eq!((72, 4), controller.cursor().pos());
  // move to the row start
  assert_eq!(Some(false), controller.cursor_move_row_start().temporary());
  assert_eq!((1, 4), controller.cursor().pos());
  // stay at the beginning
  assert_eq!(None, controller.cursor_move_row_start().temporary());
  assert_eq!((1, 4), controller.cursor().pos());
}

/// SPECIAL CASE: jumping to the end of the row and back to the beginning
/// of the row should work, but inside the column when horizontal line is to the end.
///
/// ```text
/// │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
/// │   │   type    │ size  ║ Discount │ Priority ║             │           │
/// │█ █├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
/// │   │"Business",│       ║          │"Normal", ║             │           │
/// │   │"Private"  │       ║          │ "High",  ║             │           │
/// ```
#[test]
fn _0021() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // move two down
  (1..=4).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_down().temporary());
  });
  assert_eq!((1, 6), controller.cursor().pos());
  // move to the row end
  assert_eq!(Some(false), controller.cursor_move_row_end().temporary());
  assert_eq!((4, 6), controller.cursor().pos());
  // stay at the end of cell
  assert_eq!(None, controller.cursor_move_row_end().temporary());
  assert_eq!((4, 6), controller.cursor().pos());
  // move to the row start
  assert_eq!(Some(false), controller.cursor_move_row_start().temporary());
  assert_eq!((1, 6), controller.cursor().pos());
  // stay at the beginning of the cell
  assert_eq!(None, controller.cursor_move_row_start().temporary());
  assert_eq!((1, 6), controller.cursor().pos());
}

/// Moving around `┬` should be possible, when the cursor is a caret.
///
/// ```text
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │ U │           │       ║    Order█o█ptions   ║             │           │
/// │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
/// │   │   type    │ size  ║ Discount█│█Priority ║             │           │
/// │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
/// ```
#[test]
fn _0022() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // make sure the cursor is a caret
  assert!(controller.cursor().is_caret());
  // move to the starting position
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  (1..=3).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  });
  assert_eq!((25, 3), controller.cursor().pos());
  (1..=10).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_right().temporary());
  });
  assert_eq!((35, 3), controller.cursor().pos());
  // walk around the '┬' character
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((35, 5), controller.cursor().pos());
  assert_eq!(Some(false), controller.cursor_move_right().temporary());
  assert_eq!((36, 5), controller.cursor().pos());
  assert_eq!(Some(false), controller.cursor_move_up().temporary());
  assert_eq!((36, 3), controller.cursor().pos());
}

/// Moving around `┬` should bot be possible, when the cursor is not a caret.
///
/// ```text
/// ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
/// │ U │           │       ║    Order █ptions    ║             │           │
/// │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
/// │   │   type    │ size  ║ Discount │ Priority ║             │           │
/// │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
/// ```
#[test]
fn _0023() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  // make sure the cursor is a block
  controller.cursor_toggle_caret_block();
  assert!(controller.cursor().is_block());
  // move to the starting position
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  (1..=3).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_cell_next().temporary());
  });
  assert_eq!((25, 3), controller.cursor().pos());
  (1..=10).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_right().temporary());
  });
  // now the block cursor should stay over the 'o' letter
  assert_eq!((35, 3), controller.cursor().pos());
  assert_eq!(Some('o'), controller.cursor_char());
  // moving down over the '┬' character should not be possible
  assert_eq!(None, controller.cursor_move_down().temporary());
  assert_eq!((35, 3), controller.cursor().pos());
  // change to underscore cursor
  controller.cursor_toggle_caret_under_score();
  assert!(controller.cursor().is_under_score());
  // moving down over the '┬' character should not be possible
  assert_eq!(None, controller.cursor_move_down().temporary());
  assert_eq!((35, 3), controller.cursor().pos());
}

/// Cursor should jump over vertical lines when moved right and left,
/// this test is for `caret` cursor.
#[test]
fn _0024() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  assert!(controller.cursor().is_caret());
  // move one down
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  (1..=71).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_right().temporary());
  });
  assert_eq!((72, 3), controller.cursor().pos());
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_right().temporary());
    assert_eq!((72, 3), controller.cursor().pos());
  });
  (1..=71).for_each(|_| {
    assert_eq!((true, false, false, false), controller.cursor_move_left().get());
  });
  assert_eq!((1, 3), controller.cursor().pos());
  (1..=100).for_each(|_| {
    assert_eq!((false, false, false, false), controller.cursor_move_left().get());
    assert_eq!((1, 3), controller.cursor().pos());
  });
}

/// Cursor should jump over vertical lines when moved right and left,
/// this test is for `block` cursor.
#[test]
fn _0025() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  controller.cursor_toggle_caret_block();
  assert!(controller.cursor().is_block());
  // move one down
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  (1..=65).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_right().temporary());
  });
  assert_eq!((71, 3), controller.cursor().pos());
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_right().temporary());
    assert_eq!((71, 3), controller.cursor().pos());
  });
  (1..=65).for_each(|_| {
    assert_eq!((true, false, false, false), controller.cursor_move_left().get());
  });
  assert_eq!((1, 3), controller.cursor().pos());
  (1..=100).for_each(|_| {
    assert_eq!((false, false, false, false), controller.cursor_move_left().get());
    assert_eq!((1, 3), controller.cursor().pos());
  });
}

/// Cursor should jump over vertical lines when moved right and left,
/// this test is for `underscore` cursor.
#[test]
fn _0026() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor position is (1, 1)
  assert_eq!((1, 1), controller.cursor().pos());
  controller.cursor_toggle_caret_under_score();
  assert!(controller.cursor().is_under_score());
  // move one down
  assert_eq!(Some(false), controller.cursor_move_down().temporary());
  assert_eq!((1, 3), controller.cursor().pos());
  (1..=65).for_each(|_| {
    assert_eq!(Some(false), controller.cursor_move_right().temporary());
  });
  assert_eq!((71, 3), controller.cursor().pos());
  (1..=100).for_each(|_| {
    assert_eq!(None, controller.cursor_move_right().temporary());
    assert_eq!((71, 3), controller.cursor().pos());
  });
  (1..=65).for_each(|_| {
    assert_eq!((true, false, false, false), controller.cursor_move_left().get());
  });
  assert_eq!((1, 3), controller.cursor().pos());
  (1..=100).for_each(|_| {
    assert_eq!((false, false, false, false), controller.cursor_move_left().get());
    assert_eq!((1, 3), controller.cursor().pos());
  });
}

/// Cursor is placed right before the vertical line.
/// Moving to the next cell should work.
#[test]

fn _0027() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  assert_eq!((1, 1), controller.cursor().pos());
  actions(&mut controller, &[MoveDown(1), MoveRight(3)]);
  assert_eq!((4, 3), controller.cursor().pos());
  actions(&mut controller, &[CellNext(1)]);
  assert_eq!((5, 3), controller.cursor().pos());
}

/// Cursor is placed right after the vertical line.
/// Moving to the previous cell should work.
#[test]
fn _0028() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(
    &mut controller,
    &[AssertPos(1, 1), MoveDown(1), MoveRight(4), AssertPos(5, 3), CellPrev(1), AssertPos(4, 3)],
  );
}

/// Moving to the end of the cell and returning to the cell start should work.
#[test]
fn _0029() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(&mut controller, &[AssertPos(1, 1), CellEnd(1), AssertPos(18, 1), CellStart(1), AssertPos(1, 1)]);
}

/// Moving to the end of the row and returning to the row start should work.
#[test]
fn _0030() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(&mut controller, &[MoveDown(1), AssertPos(1, 3), RowEnd(1), AssertPos(72, 3), RowStart(1), AssertPos(1, 3)]);
}
