/// Minimal terminal width.
const MIN_TERMINAL_WIDTH: usize = 30;

/// Minimal terminal height.
const MIN_TERMINAL_HEIGHT: usize = 10;

#[derive(Copy, Clone, Debug)]
pub enum SizeStateChange {
  Normal,
  Small,
  IntoNormal,
  IntoSmall,
}

pub struct SizeState {
  change: SizeStateChange,
}

impl SizeState {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      change: if width < MIN_TERMINAL_WIDTH || height < MIN_TERMINAL_HEIGHT {
        SizeStateChange::IntoSmall
      } else {
        SizeStateChange::IntoNormal
      },
    }
  }

  pub fn resize(&mut self, width: usize, height: usize) {
    self.change = if width < MIN_TERMINAL_WIDTH || height < MIN_TERMINAL_HEIGHT {
      match self.change {
        SizeStateChange::Small => SizeStateChange::Small,
        SizeStateChange::IntoSmall => SizeStateChange::Small,
        _ => SizeStateChange::IntoSmall,
      }
    } else {
      match self.change {
        SizeStateChange::Normal => SizeStateChange::Normal,
        SizeStateChange::IntoNormal => SizeStateChange::Normal,
        _ => SizeStateChange::IntoNormal,
      }
    }
  }

  pub fn change(&self) -> SizeStateChange {
    self.change
  }
}
