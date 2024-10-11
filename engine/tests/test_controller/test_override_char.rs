use super::*;
use dtee::Controller;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

#[test]
fn _0001() {
  let expected = r#"
    ┌─────────────────┐
    │Information item9│
    ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
    │ U │AbcDefGhiJw│       ║    Order options    ║             │           │
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
      ToggleCaretBlock,
      InsertStr("Information item 123456789".to_string(), 1),
      CellStart(1),
      MoveDown(1),
      CellNext(1),
      InsertStr("AbcDefGhiJklMnoPqrStuVw".to_string(), 1),
    ],
  );
  assert_eq!(expected, text(&controller));
}
