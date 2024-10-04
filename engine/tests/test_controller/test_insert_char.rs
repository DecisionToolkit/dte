use super::*;
use dtee::Controller;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

#[test]
fn _0001() {
  let expected = r#"
    ┌─────────────────┐
    │AB  Order options│
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
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  assert!(controller.cursor_is_caret());
  ('A'..='B').for_each(|ch| {
    controller.insert_char(ch);
  });
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0002() {
  let expected = r#"
    ┌─────────────────┐
    │  Order options  │
    ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
    │A U│           │       ║    Order options    ║             │           │
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
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  assert!(controller.cursor_is_caret());
  controller.cursor_move_down().unwrap();
  controller.insert_char('A');
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0003() {
  let expected = r#"
    ┌─────────────────┐
    │  Order options  │
    ├────┬───────────┬┴──────╥─────────────────────╥─────────────┬───────────┐
    │ U X│           │       ║    Order options    ║             │           │
    │    │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
    │    │   type    │ size  ║ Discount │ Priority ║             │           │
    │    ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │    │"Business",│       ║          │"Normal", ║             │           │
    │    │"Private"  │       ║          │ "High",  ║             │           │
    │    │           │       ║          │ "Low"    ║             │           │
    ╞════╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
    │ 1  │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
    ├────┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 2  │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
    ├────┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
    │ 3  │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
    └────┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  assert!(controller.cursor_is_caret());
  controller.cursor_move_down().unwrap();
  (1..=3).for_each(|_| {
    controller.cursor_move_right();
  });
  controller.insert_char('X');
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0004() {
  let expected = r#"
    ┌───────────────────────────────────────────────────────────────────────────┐
    │XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX  Order options│
    ├───┬───────────┬───────╥─────────────────────╥─────────────┬───────────────┤
    │ U │           │       ║    Order options    ║             │               │
    │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference     │
    │   │   type    │ size  ║ Discount │ Priority ║             │               │
    │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────────┤
    │   │"Business",│       ║          │"Normal", ║             │               │
    │   │"Private"  │       ║          │ "High",  ║             │               │
    │   │           │       ║          │ "Low"    ║             │               │
    ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════════╡
    │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1       │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────────┤
    │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2       │
    ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────────┤
    │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3       │
    └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  assert!(controller.cursor_is_caret());
  (1..=60).for_each(|_| {
    controller.insert_char('X');
  });
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0005() {
  let expected = r#"
    ┌───────────────────────────────────────────────────────────────────────┐
    │XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX  Order options│
    ├───┬───────────┬───────╥─────────────────────╥──────────────┬──────────┴┐
    │ U │           │       ║    Order options    ║          XXXX│           │
    │   │ Customer  │ Order ╟──────────┬──────────╢ Description  │ Reference │
    │   │   type    │ size  ║ Discount │ Priority ║              │           │
    │   ├───────────┼───────╫──────────┼──────────╫──────────────┼───────────┤
    │   │"Business",│       ║          │"Normal", ║              │           │
    │   │"Private"  │       ║          │ "High",  ║              │           │
    │   │           │       ║          │ "Low"    ║              │           │
    ╞═══╪═══════════╪═══════╬══════════╪══════════╬══════════════╪═══════════╡
    │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order  │   Ref 1   │
    ├───┼───────────┼───────╫──────────┼──────────╫──────────────┼───────────┤
    │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order  │   Ref 2   │
    ├───┼───────────┼───────╫──────────┼──────────╫──────────────┼───────────┤
    │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders   │   Ref 3   │
    └───┴───────────┴───────╨──────────┴──────────╨──────────────┴───────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  assert!(controller.cursor_is_caret());
  (1..=56).for_each(|_| {
    controller.insert_char('X');
  });
  controller.cursor_move_down().unwrap();
  (1..=4).for_each(|_| {
    controller.insert_char('X');
  });
  assert_eq!(expected, text(&controller));
}

#[test]
fn _0006() {
  let expected = r#"
    ┌─────────────────┐
    │  Order options  │
    ├───┬───────────┬─┴─────╥──────────────────────╥─────────────┬───────────┐
    │ U │           │       ║    Order options     ║             │           │
    │   │ Customer  │ Order ╟───────────┬──────────╢ Description │ Reference │
    │   │   type    │ size  ║XX Discount│ Priority ║             │           │
    │   ├───────────┼───────╫───────────┼──────────╫─────────────┼───────────┤
    │   │"Business",│       ║           │"Normal", ║             │           │
    │   │"Private"  │       ║           │ "High",  ║             │           │
    │   │           │       ║           │ "Low"    ║             │           │
    ╞═══╪═══════════╪═══════╬═══════════╪══════════╬═════════════╪═══════════╡
    │ 1 │"Business" │  <10  ║   0.10    │ "Normal" ║ Small order │   Ref 1   │
    ├───┼───────────┼───────╫───────────┼──────────╫─────────────┼───────────┤
    │ 2 │"Business" │ >=10  ║   0.15    │  "High"  ║ Large order │   Ref 2   │
    ├───┼───────────┼───────╫───────────┼──────────╫─────────────┼───────────┤
    │ 3 │"Private"  │   -   ║   0.05    │  "Low"   ║ All orders  │   Ref 3   │
    └───┴───────────┴───────╨───────────┴──────────╨─────────────┴───────────┘
  "#;
  let mut controller = Controller::new(INPUT_0002, WIDTH, HEIGHT);
  assert!(controller.cursor_is_caret());
  (1..=3).for_each(|_| {
    controller.cursor_move_down().unwrap();
  });
  (1..=3).for_each(|_| {
    controller.cursor_move_cell_next().unwrap();
  });
  (1..=2).for_each(|_| {
    controller.insert_char('X');
  });
  assert_eq!(expected, text(&controller));
}
