//! # Utilities

use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{Result, Write};

/// Name of the output file for storing debug messages.
const DEBUG_FILE_NAME: &str = "../target/debug.txt";

/// Writes a debug message to debug file.
///
/// # Example
///
/// ```
/// use dtee::debug_to_file;
///
/// debug_to_file("Cursor reached the end of the decision table.").unwrap();
/// ```
pub fn debug_to_file<T: Display>(message: T) -> Result<()> {
  let mut file = OpenOptions::new().create(true).append(true).open(DEBUG_FILE_NAME)?;
  file.write_all(format!("{}\n", message).as_bytes())?;
  file.flush()
}
