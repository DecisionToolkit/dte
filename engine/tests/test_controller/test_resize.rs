//! Test resizing the viewing area.

use super::*;
use dtee::Controller;

#[test]
fn shrink_view() {
  // width and height of the display area are greater than the width and height of the loaded content
  let mut controller = Controller::new(INPUT_0001.to_string()).with_viewport(600, 600);
  // now the viewing area is shrunk
  controller.resize(12, 12);
  assert_eq!((0, 0, 12, 12), controller.viewport().rect());
}

#[test]
fn extend_view() {
  // width and height of the display area are greater than the width and height of the loaded content
  let mut controller = Controller::new(INPUT_0001.to_string()).with_viewport(600, 600);
  // now the viewing area is extended
  controller.resize(1000, 1000);
  // because the display area is extended, there are some regions in the result,
  // these regions require repainting in the display
  assert_eq!((0, 0, 1000, 1000), controller.viewport().rect());
}
