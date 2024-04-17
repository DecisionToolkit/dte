use std::fs::OpenOptions;
use std::io::Write;

const FILE_NAME: &str = "./target/output.txt";

pub fn debug_to_file(message: impl ToString) {
  let mut file = OpenOptions::new().create(true).append(true).open(FILE_NAME).expect("failed to open debug output file");
  let _ = file.write(format!("{}\n", message.to_string()).as_bytes()).expect("failed to write debug message");
  file.flush().expect("failed to flush debug output file");
}
