use std::fs::OpenOptions;
use std::io::Write;

pub fn debug(message: impl ToString) {
  let mut file = OpenOptions::new().create(true).append(true).open("debug.txt").expect("failed to open debug file");
  let _ = file.write(format!("{}\n", message.to_string()).as_bytes()).expect("failed to write debug message");
  file.flush().unwrap();
}
