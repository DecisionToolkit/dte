use std::fs::OpenOptions;
use std::io::Write;

const FILE_NAME: &str = "./target/output.txt";

pub fn debug_to_file(message: impl ToString) {
  let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(FILE_NAME)
    .unwrap_or_else(|_| panic!("failed to open debug file: {}", FILE_NAME));
  let _ = file
    .write(format!("{}\n", message.to_string()).as_bytes())
    .unwrap_or_else(|_| panic!("failed to write debug message: {}", FILE_NAME));
  file.flush().unwrap_or_else(|_| panic!("failed to flush debug file: {}", FILE_NAME));
}
