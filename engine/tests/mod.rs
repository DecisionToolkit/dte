mod test_controller;
mod test_files;

use std::fmt::Write;

/// Utility function that converts a two-dimensional
/// array of characters into a single string with newlines.
fn text(input: &[Vec<char>]) -> String {
  let mut output = String::new();
  let _ = writeln!(output);
  for line in input {
    let _ = writeln!(output, "    {}", line.iter().collect::<String>());
  }
  let _ = write!(output, "  ");
  output
}
