//! Tests for cursor shapes.

use super::*;
use dtee::Controller;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

#[test]
fn cursor_mode_should_work() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  // initial cursor should be caret shape and indent mode
  assert!(controller.cursor().is_caret());
  assert!(controller.cursor().insert_mode());
  assert!(!controller.cursor().override_mode());
  // change the cursor to block, should switch to override mode
  controller.cursor_toggle_caret_block();
  assert!(controller.cursor().is_block());
  assert!(controller.cursor().override_mode());
  assert!(!controller.cursor().insert_mode());
  // change back to caret
  controller.cursor_toggle_caret_block();
  assert!(controller.cursor().is_caret());
  // change to underscore
  controller.cursor_toggle_caret_under_score();
  assert!(controller.cursor().is_under_score());
  assert!(controller.cursor().override_mode());
  assert!(!controller.cursor().insert_mode());
}

#[test]
fn cursor_toggle_should_work() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  assert!(controller.cursor().is_caret());
  controller.cursor_toggle_caret_block();
  assert!(controller.cursor().is_block());
  controller.cursor_toggle_caret_block();
  assert!(controller.cursor().is_caret());
  controller.cursor_toggle_caret_under_score();
  assert!(controller.cursor().is_under_score());
  controller.cursor_toggle_caret_under_score();
  assert!(controller.cursor().is_caret());
  controller.cursor_toggle_caret_block();
  assert!(controller.cursor().is_block());
  controller.cursor_toggle_caret_under_score();
  assert!(controller.cursor().is_under_score());
  controller.cursor_toggle_caret_under_score();
  assert!(controller.cursor().is_caret());
  controller.cursor_toggle_caret_under_score();
  assert!(controller.cursor().is_under_score());
  controller.cursor_toggle_caret_block();
  assert!(controller.cursor().is_block());
  controller.cursor_toggle_caret_block();
  assert!(controller.cursor().is_caret());
}

#[test]
fn cursor_toggle_caret_block_should_move_left_when_needed() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  assert!(controller.cursor().is_caret());
  actions(&mut controller, &[CellEnd(1), ToggleCaretBlock, AssertPos(17, 1)]);
}

#[test]
fn cursor_toggle_caret_under_score_should_move_left_when_needed() {
  let mut controller = Controller::new(INPUT_0002).with_viewport(WIDTH, HEIGHT);
  assert!(controller.cursor().is_caret());
  actions(&mut controller, &[CellEnd(1), ToggleCaretUnderScore, AssertPos(17, 1)]);
}
