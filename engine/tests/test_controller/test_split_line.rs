use super::*;
use dtee::Controller;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

#[test]
fn _0001() {
  let expected = r#"
    ┌─────────────────┐
    │  Order options  │
    ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
    │ U │           │       ║    Or               ║             │           │
    │   │ Customer  │ Order ║der options          ║ Description │ Reference │
    │   │   type    │ size  ╟──────────┬──────────╢             │           │
    │   │           │       ║ Discount │ Priority ║             │           │
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
  actions(&mut controller, &[MoveDown(1), MoveRight(30), SplitLine(1)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0002() {
  let expected = r#"
    ┌─────────────────┐
    │  Order options  │
    ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
    │   │           │       ║    Order options    ║             │           │
    │ U │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
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
  actions(&mut controller, &[MoveDown(1), SplitLine(1)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0003() {
  let expected = r#"
    ┌─────────────────┐
    │                 │
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
  actions(&mut controller, &[SplitLine(1)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0004() {
  let expected = r#"
    ┌─────────────────┐
    │  Order          │
    │ options         │
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
  actions(&mut controller, &[CellEnd(1), SplitLine(1), MoveUp(1), MoveRight(7), SplitLine(1)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0005() {
  let expected = r#"
    ┌─────────────────┐
    │  Order options  │
    ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
    │ U │           │       ║    Order options    ║             │           │
    │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
    │   │   type    │ size  ║ Discount │ Priority ║             │           │
    │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │   │"Busi      │       ║          │"Normal", ║             │           │
    │   │ness",     │       ║          │ "High",  ║             │           │
    │   │"Private"  │       ║          │ "Low"    ║             │           │
    ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
    │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
    └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  actions(&mut controller, &[MoveDown(5), MoveRight(9), SplitLine(1)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0006() {
  let expected = r#"
    ┌─────────────────┐
    │  Ord            │
    │er               │
    │ options         │
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
  actions(&mut controller, &[MoveRight(7), SplitLine(1), MoveUp(1), MoveRight(5), SplitLine(1)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0007() {
  let expected = r#"
    ┌──────────────┐
    │              │
    │Or            │
    │der           │
    │op            │
    │tions         │
    ├───┬──────────┴┬───────╥─────────────────────╥─────────────┬───────────┐
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
  actions(
    &mut controller,
    &[
      DeleteUnder(2),
      MoveRight(5),
      SplitLine(1),
      DeleteUnder(1),
      MoveUp(1),
      MoveRight(2),
      SplitLine(1),
      MoveDown(1),
      MoveRight(2),
      SplitLine(1),
      MoveUp(3),
      SplitLine(1),
    ],
  );
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0008() {
  let expected = r#"
    ┌─────────────────┐
    │  Order opti     │
    │o                │
    │n                │
    │s                │
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
  actions(
    &mut controller,
    &[
      CellEnd(1),
      MoveLeft(3),
      SplitLine(1),
      MoveUp(1),
      CellEnd(1),
      MoveLeft(4),
      SplitLine(1),
      MoveUp(1),
      CellEnd(1),
      MoveLeft(5),
      SplitLine(1),
    ],
  );
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0009() {
  let expected = r#"
    ┌─────────────────┐
    │  Order options  │
    │                 │
    │                 │
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
  actions(&mut controller, &[CellEnd(1), SplitLine(1), SplitLine(1)]);
  assert_eq!(expected, text(&controller));
}
