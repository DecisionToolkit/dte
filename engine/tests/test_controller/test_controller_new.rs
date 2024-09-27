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
  let controller = Controller::new(T0001.to_string(), 10, 6);
  assert_eq!(expected, text(controller.content()));
  assert_eq!("(0, 0, 10, 6)", controller.viewport().to_string());
}
