use super::*;
use dtee::Controller;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

/// Deleting two characters under cursor, cursor position stays unchanged.
#[test]
fn _0001() {
  let expected = r#"
    ┌───────────────┐
    │Order options  │
    ├───┬───────────┼───────╥─────────────────────╥─────────────┬───────────┐
    │ U │           │       ║    Order options    ║             │           │
    │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
    │   │   type    │ size  ║ Discount │ Priority ║             │           │
    │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │   │"Business",│       ║          │"Normal", ║             │           │
    │   │"Private"  │       ║          │ "High",  ║             │           │
    │   │           │       ║          │ "Low"    ║             │           │
    ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
    │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
    └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(&mut controller, &[DeleteUnder(2), AssertPos(1, 1)]);
  assert_eq!(expected, text(&controller));
}

/// Deleting all characters under cursor in the information item name cell.
/// Cursor position stays unchanged, but a single character should be left in this cell.
#[test]
fn _0002() {
  let expected = r#"
    ┌─┐
    │ │
    ├─┴─┬───────────┬───────╥─────────────────────╥─────────────┬───────────┐
    │ U │           │       ║    Order options    ║             │           │
    │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
    │   │   type    │ size  ║ Discount │ Priority ║             │           │
    │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │   │"Business",│       ║          │"Normal", ║             │           │
    │   │"Private"  │       ║          │ "High",  ║             │           │
    │   │           │       ║          │ "Low"    ║             │           │
    ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
    │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
    └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(&mut controller, &[DeleteUnder(100), AssertPos(1, 1)]);
  assert_eq!(expected, text(&controller));
}

/// Deleting all characters under cursor in the first column.
/// A single character should be left in every cell og the first column.
#[test]
fn _0003() {
  let expected = r#"
    ┌─────────────────┐
    │  Order options  │
    ├─┬───────────┬───┴───╥─────────────────────╥─────────────┬───────────┐
    │ │           │       ║    Order options    ║             │           │
    │ │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
    │ │   type    │ size  ║ Discount │ Priority ║             │           │
    │ ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ │"Business",│       ║          │"Normal", ║             │           │
    │ │"Private"  │       ║          │ "High",  ║             │           │
    │ │           │       ║          │ "Low"    ║             │           │
    ╞═╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
    │ │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
    ├─┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
    ├─┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
    └─┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(
    &mut controller,
    &[
      MoveDown(1),
      DeleteUnder(5),
      MoveDown(7),
      DeleteUnder(5),
      MoveDown(1),
      DeleteUnder(5),
      MoveDown(1),
      DeleteUnder(5),
    ],
  );
  assert_eq!(expected, text(&controller));
}

/// Deleting the last character in the cell should not have any effect for caret cursor.
#[test]
fn _0004() {
  let expected = r#"
    ┌─────────────────┐
    │  Order options  │
    ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
    │ U │           │       ║    Order options    ║             │           │
    │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
    │   │   type    │ size  ║ Discount │ Priority ║             │           │
    │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │   │"Business",│       ║          │"Normal", ║             │           │
    │   │"Private"  │       ║          │ "High",  ║             │           │
    │   │           │       ║          │ "Low"    ║             │           │
    ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
    │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
    └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(&mut controller, &[CellEnd(1), DeleteUnder(1), AssertPos(18, 1)]);
  assert_eq!(expected, text(&controller));
}

/// Deleting the last character in the cell, when the cursor is a block,
/// should delete this last character and move the block cursor one position left.
#[test]
fn _0005() {
  let expected = r#"
    ┌───────────────┐
    │  Order options│
    ├───┬───────────┼───────╥─────────────────────╥─────────────┬───────────┐
    │ U │           │       ║    Order options    ║             │           │
    │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
    │   │   type    │ size  ║ Discount │ Priority ║             │           │
    │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │   │"Business",│       ║          │"Normal", ║             │           │
    │   │"Private"  │       ║          │ "High",  ║             │           │
    │   │           │       ║          │ "Low"    ║             │           │
    ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
    │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
    └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(&mut controller, &[ToggleCaretBlock, CellEnd(1), DeleteUnder(2), AssertPos(15, 1)]);
  paper(&controller);
  assert_eq!(expected, text(&controller));
}

/// Deleting the last character in the cell, when the cursor is an underscore,
/// should delete this last character and move the underscore cursor one position left.
#[test]
fn _0006() {
  let expected = r#"
    ┌─────────────────┐
    │  Order options  │
    ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬────────┐
    │ U │           │       ║    Order options    ║             │        │
    │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Refere │
    │   │   type    │ size  ║ Discount │ Priority ║             │        │
    │   ├───────────┼───────╫──────────┼──────────╫─────────────┼────────┤
    │   │"Business",│       ║          │"Normal", ║             │        │
    │   │"Private"  │       ║          │ "High",  ║             │        │
    │   │           │       ║          │ "Low"    ║             │        │
    ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪════════╡
    │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1│
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼────────┤
    │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2│
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼────────┤
    │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3│
    └───┴───────────┴───────╨──────────┴──────────╨─────────────┴────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(&mut controller, &[ToggleCaretUnderScore, MoveDown(2), RowEnd(1), DeleteUnder(10), AssertPos(68, 4)]);
  assert_eq!(expected, text(&controller));
}
