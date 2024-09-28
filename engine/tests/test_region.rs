use dtee::Region;

#[test]
fn default_should_work() {
  let region = Region::default();
  assert_eq!("(0, 0, 0, 0)", region.to_string());
}

#[test]
fn debug_should_work() {
  let region = Region::new(10, 20, 100, 200);
  assert_eq!("Region { left: 10, top: 20, width: 100, height: 200 }", format!("{:?}", region));
}

#[test]
fn display_should_work() {
  let region = Region::new(10, 20, 100, 200);
  assert_eq!("(10, 20, 100, 200)", region.to_string());
}

#[test]
fn clipping_should_work() {
  let viewport = Region::new(0, 0, 300, 200);
  let changed = Region::new(90, 10, 10, 100);
  let clipped = Region::new(90, 10, 10, 100);
  assert_eq!(clipped, changed.clip(&viewport));
  let viewport = Region::new(10, 10, 200, 200);
  let changed = Region::new(180, 0, 120, 300);
  let clipped = Region::new(180, 10, 30, 200);
  assert_eq!(clipped, changed.clip(&viewport));
}

#[test]
fn offset_should_work() {
  let region = Region::new(10, 20, 100, 200);
  assert_eq!((10, 20), region.offset());
}

#[test]
fn size_should_work() {
  let region = Region::new(10, 20, 100, 200);
  assert_eq!((100, 200), region.size());
}
