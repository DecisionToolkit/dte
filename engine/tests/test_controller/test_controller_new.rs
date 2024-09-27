use crate::test_files::T0001;
use crate::text;
use dtee::Controller;

#[test]
fn test_controller_new() {
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
  let mut controller = Controller::new(T0001.to_string(), WIDTH, HEIGHT);
  assert_eq!(expected, text(controller.content()));
  // viewport size is set to the display size
  assert_eq!(format!("(0, 0, {WIDTH}, {HEIGHT})"), controller.viewport().to_string());
  // cursor position should be in the left top corner, but not at the '┌' character
  // that's why it is (1, 1) and NOT (0, 0)
  assert_eq!((1, 1), controller.cursor_position());
  // this is the region of the edited text
  assert_eq!("(0, 0, 45, 15)", controller.content_region().to_string());
}

#[test]
fn test_controller_new_small() {
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
  let mut controller = Controller::new(T0001.to_string(), 10, 6);
  assert_eq!(expected, text(controller.content()));
  // viewport size is set to the display size
  assert_eq!(format!("(0, 0, {WIDTH}, {HEIGHT})"), controller.viewport().to_string());
  // cursor position should be in the left top corner, but not at the '┌' character
  // that's why it is (1, 1) and NOT (0, 0)
  assert_eq!((1, 1), controller.cursor_position());
  // this is the region of the edited text
  assert_eq!("(0, 0, 45, 15)", controller.content_region().to_string());
}
