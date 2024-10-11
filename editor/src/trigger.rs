use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

const KIND_PRESS: KeyEventKind = KeyEventKind::Press;
const STATUS_NONE: KeyEventState = KeyEventState::NONE;
const MODIFIER_NONE: KeyModifiers = KeyModifiers::NONE;
const MODIFIER_CTRL: KeyModifiers = KeyModifiers::CONTROL;
const MODIFIER_ALT: KeyModifiers = KeyModifiers::ALT;
const MODIFIER_SHIFT: KeyModifiers = KeyModifiers::SHIFT;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Trigger {
  AltLeft,
  AltInsert,
  Backspace,
  CtrlEnd,
  CtrlHome,
  CtrlInsert,
  CtrlPageDown,
  CtrlPageUp,
  Delete,
  Down,
  End,
  Exit,
  Enter,
  F1,
  Home,
  Insert,
  Left,
  PageDown,
  PageUp,
  Right,
  Up,
  ShiftEnd,
  ShiftHome,
  ShiftInsert,
  ShiftLeft,
  ShiftPageDown,
  ShiftPageUp,
  ShiftTab,
  Tab,
  Char(char),
  Resize(usize, usize),
}

pub fn read_trigger() -> Trigger {
  loop {
    if let Ok(event) = event::read() {
      match event {
        Event::Key(KeyEvent { code, modifiers, kind, state }) => match (code, modifiers, kind, state) {
          (KeyCode::Enter, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::Enter,
          (KeyCode::Left, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::Left,
          (KeyCode::Left, MODIFIER_SHIFT, KIND_PRESS, STATUS_NONE) => return Trigger::ShiftLeft,
          (KeyCode::Left, MODIFIER_ALT, KIND_PRESS, STATUS_NONE) => return Trigger::AltLeft,
          (KeyCode::Right, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::Right,
          (KeyCode::Up, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::Up,
          (KeyCode::Down, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::Down,
          (KeyCode::Backspace, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::Backspace,
          (KeyCode::Delete, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::Delete,
          (KeyCode::Insert, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::Insert,
          (KeyCode::Insert, MODIFIER_SHIFT, KIND_PRESS, STATUS_NONE) => return Trigger::ShiftInsert,
          (KeyCode::Insert, MODIFIER_CTRL, KIND_PRESS, STATUS_NONE) => return Trigger::CtrlInsert,
          (KeyCode::Insert, MODIFIER_ALT, KIND_PRESS, STATUS_NONE) => return Trigger::AltInsert,
          (KeyCode::End, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::End,
          (KeyCode::End, MODIFIER_SHIFT, KIND_PRESS, STATUS_NONE) => return Trigger::ShiftEnd,
          (KeyCode::End, MODIFIER_CTRL, KIND_PRESS, STATUS_NONE) => return Trigger::CtrlEnd,
          (KeyCode::F(1), MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::F1,
          (KeyCode::Home, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::Home,
          (KeyCode::Home, MODIFIER_SHIFT, KIND_PRESS, STATUS_NONE) => return Trigger::ShiftHome,
          (KeyCode::Home, MODIFIER_CTRL, KIND_PRESS, STATUS_NONE) => return Trigger::CtrlHome,
          (KeyCode::PageDown, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::PageDown,
          (KeyCode::PageDown, MODIFIER_SHIFT, KIND_PRESS, STATUS_NONE) => return Trigger::ShiftPageDown,
          (KeyCode::PageDown, MODIFIER_CTRL, KIND_PRESS, STATUS_NONE) => return Trigger::CtrlPageDown,
          (KeyCode::PageUp, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::PageUp,
          (KeyCode::PageUp, MODIFIER_SHIFT, KIND_PRESS, STATUS_NONE) => return Trigger::ShiftPageUp,
          (KeyCode::PageUp, MODIFIER_CTRL, KIND_PRESS, STATUS_NONE) => return Trigger::CtrlPageUp,
          (KeyCode::Tab, MODIFIER_NONE, KIND_PRESS, STATUS_NONE) => return Trigger::Tab,
          (KeyCode::BackTab, MODIFIER_SHIFT, KIND_PRESS, STATUS_NONE) => return Trigger::ShiftTab,
          (KeyCode::Char('q'), MODIFIER_CTRL, KIND_PRESS, STATUS_NONE) => return Trigger::Exit,
          (KeyCode::Char(ch), _, KIND_PRESS, STATUS_NONE) => return Trigger::Char(ch),
          _ => {}
        },
        Event::Resize(width, height) => return Trigger::Resize(width as usize, height as usize),
        _ => {}
      }
    }
  }
}
