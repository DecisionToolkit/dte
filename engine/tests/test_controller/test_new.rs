//! Test controller constructors.

use crate::test_files::*;
use crate::text;
use dtee::Controller;

#[test]
fn larger_view() {
  let expected = r#"
    ┌───────┐
    │  SLA  │
    ├───┬───┴─────────────┬───────────────╥─────┐
    │ U │ YearsAsCustomer │ NumberOfUnits ║ SLA │
    │   ├─────────────────┼───────────────╫─────┤
    │   │    [0..100]     │ [0..1000000]  ║ 1,2 │
    ╞═══╪═════════════════╪═══════════════╬═════╡
    │ 1 │       <2        │    <1000      ║  1  │
    ├───┼─────────────────┼───────────────╫─────┤
    │ 2 │       <2        │   >=1000      ║  2  │
    ├───┼─────────────────┼───────────────╫─────┤
    │ 3 │      >=2        │     <500      ║  1  │
    ├───┼─────────────────┼───────────────╫─────┤
    │ 4 │      >=2        │    >=500      ║  2  │
    └───┴─────────────────┴───────────────╨─────┘
  "#;
  const WIDTH: usize = 300; // this is a width of the display area
  const HEIGHT: usize = 200; // this is a height of the display area

  // both width and height are greater than the width and height of the loaded content
  let mut controller = Controller::new(INPUT_0001, WIDTH, HEIGHT);
  assert_eq!(expected, text(controller.content()));

  // viewport size is set to the display size
  assert_eq!(format!("(0, 0, {WIDTH}, {HEIGHT})"), controller.viewport().to_string());

  // cursor position should be in the left top corner, but not at the '┌' character
  // that's why it is (1, 1) and not (0, 0)
  assert_eq!((1, 1), controller.cursor_position());

  // this is the region of the edited text
  assert_eq!("(0, 0, 45, 15)", controller.content_region().to_string());
}

#[test]
fn smaller_view() {
  let expected = r#"
    ┌───────┐
    │  SLA  │
    ├───┬───┴─────────────┬───────────────╥─────┐
    │ U │ YearsAsCustomer │ NumberOfUnits ║ SLA │
    │   ├─────────────────┼───────────────╫─────┤
    │   │    [0..100]     │ [0..1000000]  ║ 1,2 │
    ╞═══╪═════════════════╪═══════════════╬═════╡
    │ 1 │       <2        │    <1000      ║  1  │
    ├───┼─────────────────┼───────────────╫─────┤
    │ 2 │       <2        │   >=1000      ║  2  │
    ├───┼─────────────────┼───────────────╫─────┤
    │ 3 │      >=2        │     <500      ║  1  │
    ├───┼─────────────────┼───────────────╫─────┤
    │ 4 │      >=2        │    >=500      ║  2  │
    └───┴─────────────────┴───────────────╨─────┘
  "#;
  const WIDTH: usize = 10; // this is a width of the display area
  const HEIGHT: usize = 6; // this is a height of the display area

  // both width and height are less than the width and height of the loaded content
  let mut controller = Controller::new(INPUT_0001, 10, 6);
  assert_eq!(expected, text(controller.content()));

  // viewport size is set to the display size
  assert_eq!(format!("(0, 0, {WIDTH}, {HEIGHT})"), controller.viewport().to_string());

  // cursor position should be in the left top corner, but not at the '┌' character
  // that's why it is (1, 1) and not (0, 0)
  assert_eq!((1, 1), controller.cursor_position());

  // this is the region of the edited text
  assert_eq!("(0, 0, 45, 15)", controller.content_region().to_string());
}

#[test]
fn empty_lines_are_skipped() {
  let mut controller2 = Controller::new(INPUT_0001, 600, 600);
  let mut controller3 = Controller::new(INPUT_0003, 600, 600);
  assert_eq!(controller2.content(), controller3.content());
  assert_eq!(controller2.content_region(), controller3.content_region());
  assert_eq!(text(controller2.content()), text(controller3.content()));
}
