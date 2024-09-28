//! Test resizing the display area.

use crate::test_files::INPUT_0001;
use dtee::Controller;

#[test]
fn test_shrink() {
  // width and height of the display area are greater than the width and height of the loaded content
  let mut controller = Controller::new(INPUT_0001.to_string(), 600, 600);
  // now the display area is shrunk
  let dirty_regions = controller.resize(12, 12);
  // because the display area is shrunk, there are no dirty regions in the result,
  // no regions require repainting in the display
  assert_eq!(0, dirty_regions.len());
}

#[test]
fn test_extend() {
  // width and height of the display area are greater than the width and height of the loaded content
  let mut controller = Controller::new(INPUT_0001.to_string(), 600, 600);
  // now the display area is extended
  let dirty_regions = controller.resize(1000, 1000);
  // because the display area is extended, there are some regions in the result,
  // these regions require repainting in the display
  assert_eq!(2, dirty_regions.len());
  assert_eq!("(600, 0, 400, 1000)", dirty_regions[0].to_string());
  assert_eq!("(0, 600, 1000, 400)", dirty_regions[1].to_string());
}
