//! # Utilities

use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;

/// Name of the output file for storing debug messages.
pub const DEBUG_FILE_NAME: &str = "../target/debug.txt";

/// Writes a debug message to debug file.
///
/// Use this function **only** for debugging purposes while testing.
///
/// # Panics
///
/// This function panics if any i/o error occurs.
///
/// # Examples
///
/// ```
/// use dtee::debug_to_file;
///
/// debug_to_file("Cursor reached the end of the decision table.");
/// ```
pub fn debug_to_file<T: Display>(message: T) {
  let mut file = OpenOptions::new().create(true).append(true).open(DEBUG_FILE_NAME).unwrap();
  file.write_all(format!("{}\n", message).as_bytes()).unwrap();
  file.flush().unwrap()
}
