use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

const PRESS: KeyEventKind = KeyEventKind::Press;
const S_NONE: KeyEventState = KeyEventState::NONE;
const M_NONE: KeyModifiers = KeyModifiers::NONE;
const M_CTRL: KeyModifiers = KeyModifiers::CONTROL;
const M_SHIFT: KeyModifiers = KeyModifiers::SHIFT;

#[derive(Debug)]
pub enum Key {
  Enter,
  Backspace,
  Delete,
  Down,
  End,
  Home,
  Left,
  Right,
  Tab,
  Up,
  ShiftEnd,
  ShiftHome,
  ShiftTab,
  CtrlQ,
  Char(char),
  Resize(usize, usize),
}

pub fn read_key() -> Key {
  loop {
    if let Ok(event) = event::read() {
      match event {
        Event::Key(KeyEvent { code, modifiers, kind, state }) => match (code, modifiers, kind, state) {
          (KeyCode::Enter, M_NONE, PRESS, S_NONE) => return Key::Enter,
          (KeyCode::Left, M_NONE, PRESS, S_NONE) => return Key::Left,
          (KeyCode::Right, M_NONE, PRESS, S_NONE) => return Key::Right,
          (KeyCode::Up, M_NONE, PRESS, S_NONE) => return Key::Up,
          (KeyCode::Down, M_NONE, PRESS, S_NONE) => return Key::Down,
          (KeyCode::Backspace, M_NONE, PRESS, S_NONE) => return Key::Backspace,
          (KeyCode::Delete, M_NONE, PRESS, S_NONE) => return Key::Delete,
          (KeyCode::End, M_NONE, PRESS, S_NONE) => return Key::End,
          (KeyCode::End, M_SHIFT, PRESS, S_NONE) => return Key::ShiftEnd,
          (KeyCode::Home, M_NONE, PRESS, S_NONE) => return Key::Home,
          (KeyCode::Home, M_SHIFT, PRESS, S_NONE) => return Key::ShiftHome,
          (KeyCode::Tab, M_NONE, PRESS, S_NONE) => return Key::Tab,
          (KeyCode::BackTab, M_SHIFT, PRESS, S_NONE) => return Key::ShiftTab,
          (KeyCode::Char('q'), M_CTRL, PRESS, S_NONE) => return Key::CtrlQ,
          (KeyCode::Char(ch), _, PRESS, S_NONE) => return Key::Char(ch),
          _ => {}
        },
        Event::Resize(width, height) => return Key::Resize(width as usize, height as usize),
        _ => {}
      }
    }
  }
}
