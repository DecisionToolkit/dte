//! Test moving the cursor to the left.
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
fn test_0001() {
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
  (1..=10).for_each(|_| {
    assert_eq!(None, controller.cursor_move_right());
  });
  assert_eq!((18, 1), controller.cursor_position());
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
  });
  assert_eq!((18, 1), controller.cursor_position());
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
  });
  assert_eq!((1, 1), controller.cursor_position());
}
