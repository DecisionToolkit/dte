mod test_controller;
mod test_debug;
mod test_files;

/// Utility function that converts a two-dimensional
/// array of characters into a single string with newlines.
fn text(input: &[Vec<char>]) -> String {
  let mut output = String::new();
  output.push_str("\n");
  for line in input {
    output.push_str("    ");
    output.push_str(&line.iter().map(|ch| *ch).collect::<String>());
    output.push_str("\n");
  }
  output.push_str("  ");
  output
}
