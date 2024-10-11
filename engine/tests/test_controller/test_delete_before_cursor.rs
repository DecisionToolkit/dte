use super::*;
use dtee::Controller;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

/// At the beginning, when the cursor is not moved, it is placed in the top-left corner
/// just after vertical line. Deleting a character before cursor should have no effect
/// in this case, because the lines are not removable.
#[test]
fn _0001() {
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
  actions(&mut controller, &[DeleteBefore(1)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0002() {
  let expected = r#"
    ┌────────────────┐
    │ Order options  │
    ├───┬───────────┬┴──────╥─────────────────────╥─────────────┬───────────┐
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
  actions(&mut controller, &[MoveRight(1), DeleteBefore(1)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0003() {
  let expected = r#"
    ┌───┬───────────┬──────╥─────────────────────╥─────────────┬───────────┐
    │ U │           │      ║    Order options    ║             │           │
    │   │ Customer  │ Order╟──────────┬──────────╢ Description │ Reference │
    │   │   type    │ size ║ Discount │ Priority ║             │           │
    │   ├───────────┼──────╫──────────┼──────────╫─────────────┼───────────┤
    │   │"Business",│      ║          │"Normal", ║             │           │
    │   │"Private"  │      ║          │ "High",  ║             │           │
    │   │           │      ║          │ "Low"    ║             │           │
    ╞═══╪═══════════╪══════╬══════════╪══════════╬═════════════╪═══════════╡
    │ 1 │"Business" │  <10 ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
    ├───┼───────────┼──────╫──────────┼──────────╫─────────────┼───────────┤
    │ 2 │"Business" │ >=10 ║   0.15   │  "High"  ║ Large order │   Ref 2   │
    ├───┼───────────┼──────╫──────────┼──────────╫─────────────┼───────────┤
    │ 3 │"Private"  │   -  ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
    └───┴───────────┴──────╨──────────┴──────────╨─────────────┴───────────┘
  "#;
  let mut controller = Controller::new(INPUT_0004).with_viewport(WIDTH, HEIGHT);
  actions(&mut controller, &[MoveRight(20), DeleteBefore(2)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0004() {
  let expected = r#"
    ┌───────────────────────────────────────────────────────────────────────┐
    │XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX  Order options│
    ├───┬───────────┬───────╥─────────────────────╥─────────────┬───────────┤
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
  actions(&mut controller, &[Insert('X', 57), DeleteBefore(1)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0005() {
  let expected = r#"
    ┌─┐
    │Y│
    ├─┴─┬───────────┬───────╥─────────────────────╥─────────────┬──────────┐
    │ U │           │       ║    Order options    ║             │          │
    │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference│
    │   │   type    │ size  ║ Discount │ Priority ║             │          │
    │   ├───────────┼───────╫──────────┼──────────╫─────────────┼──────────┤
    │   │"Business",│       ║          │"Normal", ║             │          │
    │   │"Private"  │       ║          │ "High",  ║             │          │
    │   │           │       ║          │ "Low"    ║             │          │
    ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪══════════╡
    │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1  │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼──────────┤
    │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2  │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼──────────┤
    │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3  │
    └───┴───────────┴───────╨──────────┴──────────╨─────────────┴──────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(&mut controller, &[Insert('X', 100), RowEnd(1), DeleteBefore(115), Insert('Y', 1)]);
  assert_eq!(expected, text(&controller));
}
