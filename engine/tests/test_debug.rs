use dtee::{debug_to_file, DEBUG_FILE_NAME};
use std::fs;

/// Utility function that counts the number of instances of the specified pattern
/// in lines of text in debug file.
fn count(pattern: &str) -> usize {
  fs::read_to_string(DEBUG_FILE_NAME)
    .unwrap_or("".to_string())
    .lines()
    .filter(|line| line.contains(pattern))
    .count()
}

#[test]
fn debug_messaging_should_work() {
  let msg = "message#123456";
  let before = count(msg);
  debug_to_file(msg);
  let after = count(msg);
  assert_eq!(after, before + 1);
}
