/// Returns show cursor action.
pub fn c_show() -> crossterm::cursor::Show {
  crossterm::cursor::Show
}

/// Returns hide cursor action.
pub fn c_hide() -> crossterm::cursor::Hide {
  crossterm::cursor::Hide
}

/// Returns move cursor action.
pub fn c_move(x: usize, y: usize) -> crossterm::cursor::MoveTo {
  crossterm::cursor::MoveTo(x as u16, y as u16)
}

pub fn c_blinking_bar() -> crossterm::cursor::SetCursorStyle {
  crossterm::cursor::SetCursorStyle::BlinkingBar
}

pub fn c_blinking_block() -> crossterm::cursor::SetCursorStyle {
  crossterm::cursor::SetCursorStyle::BlinkingBlock
}

pub fn c_default_user_shape() -> crossterm::cursor::SetCursorStyle {
  crossterm::cursor::SetCursorStyle::DefaultUserShape
}

/// Returns clear whole terminal action.
pub fn t_clear_all() -> crossterm::terminal::Clear {
  crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
}

/// Returns enter alternate screen action.
pub fn t_enter_alternate_screen() -> crossterm::terminal::EnterAlternateScreen {
  crossterm::terminal::EnterAlternateScreen
}

/// Returns an action that triggers leaving alternate screen.
pub fn t_leave_alternate_screen() -> crossterm::terminal::LeaveAlternateScreen {
  crossterm::terminal::LeaveAlternateScreen
}

/// Returns the size (width, height) of the terminal.
pub fn t_size() -> std::io::Result<(usize, usize)> {
  crossterm::terminal::size().map(|(width, height)| (width as usize, height as usize))
}
