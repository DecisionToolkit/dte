mod keys;

use crate::keys::{read_key, Key};
use crossterm::{terminal, ExecutableCommand};
use std::io;

fn run() -> io::Result<()> {
  let mut stdout = io::stdout();
  stdout.execute(terminal::Clear(terminal::ClearType::All))?;
  loop {
    match read_key() {
      Key::CtrlQ => break,
      Key::Char(ch) => {
        // TODO remove
        println!("{}", ch)
      }
      other => {
        // TODO remove
        println!("{:?}", other)
      }
    }
  }
  Ok(())
}

fn main() -> io::Result<()> {
  // enter raw mode, directly process each key press
  terminal::enable_raw_mode()?;
  let _ = run();
  // return to canonical mode, process input after pressing Enter
  terminal::disable_raw_mode()?;
  Ok(())
}
