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
  actions(&mut controller, &[SplitLine(1), DeleteBefore(1)]);
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0003() {
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
  actions(&mut controller, &[MoveDown(1), CellNext(3), SplitLine(1), DeleteBefore(1)]);
  assert_eq!(expected, text(&controller));
}
