//! # Character classes and attributes

use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;

/// ` ` `U+0020` Space.
/// ```
/// # use dtee::*;
/// assert_eq!(' ', SPACE);
/// assert_eq!('\u{0020}', SPACE);
/// ```
pub const SPACE: char = '\u{0020}';

/// `─` `U+2500` Box drawings light horizontal.+
///
/// ```
/// # use dtee::*;
/// assert_eq!('─', LIGHT_HORIZONTAL);
/// assert_eq!('\u{2500}', LIGHT_HORIZONTAL);
/// ```
pub const LIGHT_HORIZONTAL: char = '\u{2500}';

/// `│` `U+2502` Box drawings light vertical.
/// ```
/// # use dtee::*;
/// assert_eq!('│', LIGHT_VERTICAL);
/// assert_eq!('\u{2502}', LIGHT_VERTICAL);
/// ```
pub const LIGHT_VERTICAL: char = '\u{2502}';

/// `┌` `U+250C` Box drawings light down and right.
/// ```
/// # use dtee::*;
/// assert_eq!('┌', LIGHT_DOWN_AND_RIGHT);
/// assert_eq!('\u{250C}', LIGHT_DOWN_AND_RIGHT);
/// ```
pub const LIGHT_DOWN_AND_RIGHT: char = '\u{250C}';

/// `┐` `U+2510` Box drawings light down and left.
/// ```
/// # use dtee::*;
/// assert_eq!('┐', LIGHT_DOWN_AND_LEFT);
/// assert_eq!('\u{2510}', LIGHT_DOWN_AND_LEFT);
/// ```
pub const LIGHT_DOWN_AND_LEFT: char = '\u{2510}';

/// `└` `U+2514` Box drawings light up and right.
/// ```
/// # use dtee::*;
/// assert_eq!('└', LIGHT_UP_AND_RIGHT);
/// assert_eq!('\u{2514}', LIGHT_UP_AND_RIGHT);
/// ```
pub const LIGHT_UP_AND_RIGHT: char = '\u{2514}';

/// `┘` `U+2518` Box drawings light up and left.
/// ```
/// # use dtee::*;
/// assert_eq!('┘', LIGHT_UP_AND_LEFT);
/// assert_eq!('\u{2518}', LIGHT_UP_AND_LEFT);
/// ```
pub const LIGHT_UP_AND_LEFT: char = '\u{2518}';

/// `├` `U+251C` Box drawings light vertical and right.
/// ```
/// # use dtee::*;
/// assert_eq!('├', LIGHT_VERTICAL_AND_RIGHT);
/// assert_eq!('\u{251C}', LIGHT_VERTICAL_AND_RIGHT);
/// ```
pub const LIGHT_VERTICAL_AND_RIGHT: char = '\u{251C}';

/// `┤` `U+2524` Box drawings light vertical and left.
/// ```
/// # use dtee::*;
/// assert_eq!('┤', LIGHT_VERTICAL_AND_LEFT);
/// assert_eq!('\u{2524}', LIGHT_VERTICAL_AND_LEFT);
/// ```
pub const LIGHT_VERTICAL_AND_LEFT: char = '\u{2524}';

/// `┬` `U+252C` Box drawings light down and horizontal.
/// ```
/// # use dtee::*;
/// assert_eq!('┬', LIGHT_DOWN_AND_HORIZONTAL);
/// assert_eq!('\u{252C}', LIGHT_DOWN_AND_HORIZONTAL);
/// ```
pub const LIGHT_DOWN_AND_HORIZONTAL: char = '\u{252C}';

/// `┴` `U+2534` Box drawings light up and horizontal.
/// ```
/// # use dtee::*;
/// assert_eq!('┴', LIGHT_UP_AND_HORIZONTAL);
/// assert_eq!('\u{2534}', LIGHT_UP_AND_HORIZONTAL);
/// ```
pub const LIGHT_UP_AND_HORIZONTAL: char = '\u{2534}';

/// `┼` `U+253C` Box drawings light vertical and horizontal.
/// ```
/// # use dtee::*;
/// assert_eq!('┼', LIGHT_VERTICAL_AND_HORIZONTAL);
/// assert_eq!('\u{253C}', LIGHT_VERTICAL_AND_HORIZONTAL);
/// ```
pub const LIGHT_VERTICAL_AND_HORIZONTAL: char = '\u{253C}';

/// `═` `U+2550` Box drawings double horizontal.
/// ```
/// # use dtee::*;
/// assert_eq!('═', DOUBLE_HORIZONTAL);
/// assert_eq!('\u{2550}', DOUBLE_HORIZONTAL);
/// ```
pub const DOUBLE_HORIZONTAL: char = '\u{2550}';

/// `║` `U+2551` Box drawings double vertical.
/// ```
/// # use dtee::*;
/// assert_eq!('║', DOUBLE_VERTICAL);
/// assert_eq!('\u{2551}', DOUBLE_VERTICAL);
/// ```
pub const DOUBLE_VERTICAL: char = '\u{2551}';

/// `╞` `U+255E` Box drawings vertical single and right double.
/// ```
/// # use dtee::*;
/// assert_eq!('╞', VERTICAL_SINGLE_AND_RIGHT_DOUBLE);
/// assert_eq!('\u{255E}', VERTICAL_SINGLE_AND_RIGHT_DOUBLE);
/// ```
pub const VERTICAL_SINGLE_AND_RIGHT_DOUBLE: char = '\u{255E}';

/// `╟` `U+255F` Box drawings vertical double and right single.
/// ```
/// # use dtee::*;
/// assert_eq!('╟', VERTICAL_DOUBLE_AND_RIGHT_SINGLE);
/// assert_eq!('\u{255F}', VERTICAL_DOUBLE_AND_RIGHT_SINGLE);
/// ```
pub const VERTICAL_DOUBLE_AND_RIGHT_SINGLE: char = '\u{255F}';

/// `╡` `U+2561` Box drawings vertical single and left double.
/// ```
/// # use dtee::*;
/// assert_eq!('╡', VERTICAL_SINGLE_AND_LEFT_DOUBLE);
/// assert_eq!('\u{2561}', VERTICAL_SINGLE_AND_LEFT_DOUBLE);
/// ```
pub const VERTICAL_SINGLE_AND_LEFT_DOUBLE: char = '\u{2561}';

/// `╢` `U+2562` Box drawings vertical double and left single.
/// ```
/// # use dtee::*;
/// assert_eq!('╢', VERTICAL_DOUBLE_AND_LEFT_SINGLE);
/// assert_eq!('\u{2562}', VERTICAL_DOUBLE_AND_LEFT_SINGLE);
/// ```
pub const VERTICAL_DOUBLE_AND_LEFT_SINGLE: char = '\u{2562}';

/// `╤` `U+2564` Box drawings down single and horizontal double.
/// ```
/// # use dtee::*;
/// assert_eq!('╤', DOWN_SINGLE_AND_HORIZONTAL_DOUBLE);
/// assert_eq!('\u{2564}', DOWN_SINGLE_AND_HORIZONTAL_DOUBLE);
/// ```
pub const DOWN_SINGLE_AND_HORIZONTAL_DOUBLE: char = '\u{2564}';

/// `╥` `U+2565` Box drawings down double and horizontal single.
/// ```
/// # use dtee::*;
/// assert_eq!('╥', DOWN_DOUBLE_AND_HORIZONTAL_SINGLE);
/// assert_eq!('\u{2565}', DOWN_DOUBLE_AND_HORIZONTAL_SINGLE);
/// ```
pub const DOWN_DOUBLE_AND_HORIZONTAL_SINGLE: char = '\u{2565}';

/// `╧` `U+2567` Box drawings up single and horizontal double.
/// ```
/// # use dtee::*;
/// assert_eq!('╧', UP_SINGLE_AND_HORIZONTAL_DOUBLE);
/// assert_eq!('\u{2567}', UP_SINGLE_AND_HORIZONTAL_DOUBLE);
/// ```
pub const UP_SINGLE_AND_HORIZONTAL_DOUBLE: char = '\u{2567}';

/// `╨` `U+2568` Box drawings up double and horizontal single.
/// ```
/// # use dtee::*;
/// assert_eq!('╨', UP_DOUBLE_AND_HORIZONTAL_SINGLE);
/// assert_eq!('\u{2568}', UP_DOUBLE_AND_HORIZONTAL_SINGLE);
/// ```
pub const UP_DOUBLE_AND_HORIZONTAL_SINGLE: char = '\u{2568}';

/// `╪` `U+256A` Box drawings vertical single and horizontal double.
/// ```
/// # use dtee::*;
/// assert_eq!('╪', VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE);
/// assert_eq!('\u{256A}', VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE);
/// ```
pub const VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE: char = '\u{256A}';

/// `╫` `U+256B` Box drawings vertical double and horizontal single.
/// ```
/// # use dtee::*;
/// assert_eq!('╫', VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE);
/// assert_eq!('\u{256B}', VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE);
/// ```
pub const VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE: char = '\u{256B}';

/// `╬` `U+256C` Box drawings double vertical and horizontal.
/// ```
/// # use dtee::*;
/// assert_eq!('╬', DOUBLE_VERTICAL_AND_HORIZONTAL);
/// assert_eq!('\u{256C}', DOUBLE_VERTICAL_AND_HORIZONTAL);
/// ```
pub const DOUBLE_VERTICAL_AND_HORIZONTAL: char = '\u{256C}';

/// Flag for clearing all attributes of the character.
pub const ATTRIBUTE_CLEAR: u8 = 0x0;

/// Flag indicating that a character is on the joining line between
/// the information item name and the decision table's body.
/// All characters constituting this line have this attribute set.
///
/// # Examples
///
/// ```text
/// ┌───────┐
/// │  SLA  │                                          THIS IS THE
/// ├───┬───┴─────────────┬───────────────╥─────┐<---- JOINING LINE
/// │ U │ YearsAsCustomer │ NumberOfUnits ║ SLA │
/// │   ├─────────────────┼───────────────╫─────┤
/// ```
pub const ATTRIBUTE_JOIN: u8 = 0x01;

/// Flag indicating that the information item name cell
/// is already filled to the extent of the decision table's body.
/// All characters constituting such a line have this attribute set.
///
/// # Examples
///
/// ```text
/// ┌───────────────────────────────────────────┐
/// │  SLA                                      │      THIS IS THE FULL
/// ├───┬─────────────────┬───────────────╥─────┤<---- JOINING LINE
/// │ U │ YearsAsCustomer │ NumberOfUnits ║ SLA │
/// │   ├─────────────────┼───────────────╫─────┤
/// ```
pub const ATTRIBUTE_FULL_JOIN: u8 = 0x02;

/// A character with associated attributes.
///
/// The [Char] structure encapsulates a Unicode character and a set of attributes
/// that define its visual and behavioral properties.
/// Both the character and its attributes are stored in [RefCell] wrappers,
/// allowing for interior mutability.
#[derive(Debug, Clone)]
pub struct Char {
  /// Unicode character.
  ch: RefCell<char>,
  /// Attributes associated with the character.
  attributes: RefCell<u8>,
}

impl Display for Char {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.ch.borrow())
  }
}

impl PartialEq for Char {
  fn eq(&self, other: &Self) -> bool {
    self.ch == other.ch
  }
}

impl From<char> for Char {
  fn from(ch: char) -> Self {
    Self {
      ch: RefCell::new(ch),
      attributes: RefCell::new(ATTRIBUTE_CLEAR),
    }
  }
}

impl Char {
  pub fn new(ch: char, attributes: u8) -> Self {
    Self {
      ch: RefCell::new(ch),
      attributes: RefCell::new(attributes),
    }
  }

  pub fn char(&self) -> char {
    *self.ch.borrow()
  }

  pub fn attributes(&self) -> u8 {
    *self.attributes.borrow()
  }

  pub fn set_char(&self, ch: char) {
    *self.ch.borrow_mut() = ch;
  }

  pub fn horz_fill(&self) -> Self {
    if self.is_single_vert_line_crossing_left() {
      Char::new(LIGHT_HORIZONTAL, self.attributes())
    } else if self.is_double_vert_line_crossing_left() {
      Char::new(DOUBLE_HORIZONTAL, self.attributes())
    } else {
      Char::new(SPACE, self.attributes())
    }
  }

  /// Returns `true` when the underlying character is a join between
  /// the information item name and the top border of the decision table's body.
  pub fn is_join(&self) -> bool {
    *self.attributes.borrow() & ATTRIBUTE_JOIN == ATTRIBUTE_JOIN
  }

  /// Marks the underlying character as being a join between
  /// the information item name and the top border of the decision table's body.
  pub fn set_join(&self) {
    *self.attributes.borrow_mut() |= ATTRIBUTE_JOIN;
    // setting ATTRIBUTE_JOIN flag resets ATTRIBUTE_FULL_JOIN flag
    *self.attributes.borrow_mut() &= !ATTRIBUTE_FULL_JOIN;
  }

  pub fn clear_join(&self) {
    *self.attributes.borrow_mut() &= !ATTRIBUTE_JOIN;
  }

  pub fn is_full_join(&self) -> bool {
    *self.attributes.borrow() & ATTRIBUTE_FULL_JOIN == ATTRIBUTE_FULL_JOIN
  }

  pub fn set_full_join(&self) {
    *self.attributes.borrow_mut() |= ATTRIBUTE_FULL_JOIN;
    // setting the ATTRIBUTE_FULL_JOIN flag resets the ATTRIBUTE_JOIN flag
    *self.attributes.borrow_mut() &= !ATTRIBUTE_JOIN;
  }

  pub fn clear_full_join(&self) {
    *self.attributes.borrow_mut() &= !ATTRIBUTE_FULL_JOIN;
  }

  /// Checks whether the character is a box-drawing character.
  ///
  /// # Examples
  ///
  /// ```
  /// # use dtee::*;
  /// assert_eq!(true, Char::from(LIGHT_HORIZONTAL).is_frame());
  /// assert_eq!(true, Char::from(LIGHT_VERTICAL).is_frame());
  /// assert_eq!(true, Char::from(LIGHT_DOWN_AND_RIGHT).is_frame());
  /// assert_eq!(true, Char::from(LIGHT_DOWN_AND_LEFT).is_frame());
  /// assert_eq!(true, Char::from(LIGHT_UP_AND_RIGHT).is_frame());
  /// assert_eq!(true, Char::from(LIGHT_UP_AND_LEFT).is_frame());
  /// assert_eq!(true, Char::from(LIGHT_VERTICAL_AND_RIGHT).is_frame());
  /// assert_eq!(true, Char::from(LIGHT_VERTICAL_AND_LEFT).is_frame());
  /// assert_eq!(true, Char::from(LIGHT_DOWN_AND_HORIZONTAL).is_frame());
  /// assert_eq!(true, Char::from(LIGHT_UP_AND_HORIZONTAL).is_frame());
  /// assert_eq!(true, Char::from(LIGHT_VERTICAL_AND_HORIZONTAL).is_frame());
  /// assert_eq!(true, Char::from(DOUBLE_HORIZONTAL).is_frame());
  /// assert_eq!(true, Char::from(DOUBLE_VERTICAL).is_frame());
  /// assert_eq!(true, Char::from(VERTICAL_SINGLE_AND_RIGHT_DOUBLE).is_frame());
  /// assert_eq!(true, Char::from(VERTICAL_DOUBLE_AND_RIGHT_SINGLE).is_frame());
  /// assert_eq!(true, Char::from(VERTICAL_SINGLE_AND_LEFT_DOUBLE).is_frame());
  /// assert_eq!(true, Char::from(VERTICAL_DOUBLE_AND_LEFT_SINGLE).is_frame());
  /// assert_eq!(true, Char::from(DOWN_SINGLE_AND_HORIZONTAL_DOUBLE).is_frame());
  /// assert_eq!(true, Char::from(DOWN_DOUBLE_AND_HORIZONTAL_SINGLE).is_frame());
  /// assert_eq!(true, Char::from(UP_SINGLE_AND_HORIZONTAL_DOUBLE).is_frame());
  /// assert_eq!(true, Char::from(UP_DOUBLE_AND_HORIZONTAL_SINGLE).is_frame());
  /// assert_eq!(true, Char::from(VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE).is_frame());
  /// assert_eq!(true, Char::from(VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE).is_frame());
  /// assert_eq!(true, Char::from(DOUBLE_VERTICAL_AND_HORIZONTAL).is_frame());
  /// ```
  pub fn is_frame(&self) -> bool {
    matches!(
      *self.ch.borrow(),
      LIGHT_HORIZONTAL
        | LIGHT_VERTICAL
        | LIGHT_DOWN_AND_RIGHT
        | LIGHT_DOWN_AND_LEFT
        | LIGHT_UP_AND_RIGHT
        | LIGHT_UP_AND_LEFT
        | LIGHT_VERTICAL_AND_RIGHT
        | LIGHT_VERTICAL_AND_LEFT
        | LIGHT_DOWN_AND_HORIZONTAL
        | LIGHT_UP_AND_HORIZONTAL
        | LIGHT_VERTICAL_AND_HORIZONTAL
        | DOUBLE_HORIZONTAL
        | DOUBLE_VERTICAL
        | VERTICAL_SINGLE_AND_RIGHT_DOUBLE
        | VERTICAL_DOUBLE_AND_RIGHT_SINGLE
        | VERTICAL_SINGLE_AND_LEFT_DOUBLE
        | VERTICAL_DOUBLE_AND_LEFT_SINGLE
        | DOWN_SINGLE_AND_HORIZONTAL_DOUBLE
        | DOWN_DOUBLE_AND_HORIZONTAL_SINGLE
        | UP_SINGLE_AND_HORIZONTAL_DOUBLE
        | UP_DOUBLE_AND_HORIZONTAL_SINGLE
        | VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE
        | VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE
        | DOUBLE_VERTICAL_AND_HORIZONTAL
    )
  }

  /// Checks whether the character is a crossing.
  ///
  /// # Examples
  ///
  /// ```
  /// # use dtee::*;
  /// assert_eq!(true, Char::from(LIGHT_DOWN_AND_RIGHT).is_crossing());
  /// assert_eq!(true, Char::from(LIGHT_DOWN_AND_LEFT).is_crossing());
  /// assert_eq!(true, Char::from(LIGHT_UP_AND_RIGHT).is_crossing());
  /// assert_eq!(true, Char::from(LIGHT_UP_AND_LEFT).is_crossing());
  /// assert_eq!(true, Char::from(LIGHT_VERTICAL_AND_RIGHT).is_crossing());
  /// assert_eq!(true, Char::from(LIGHT_VERTICAL_AND_LEFT).is_crossing());
  /// assert_eq!(true, Char::from(LIGHT_DOWN_AND_HORIZONTAL).is_crossing());
  /// assert_eq!(true, Char::from(LIGHT_UP_AND_HORIZONTAL).is_crossing());
  /// assert_eq!(true, Char::from(LIGHT_VERTICAL_AND_HORIZONTAL).is_crossing());
  /// assert_eq!(true, Char::from(VERTICAL_SINGLE_AND_RIGHT_DOUBLE).is_crossing());
  /// assert_eq!(true, Char::from(VERTICAL_DOUBLE_AND_RIGHT_SINGLE).is_crossing());
  /// assert_eq!(true, Char::from(VERTICAL_SINGLE_AND_LEFT_DOUBLE).is_crossing());
  /// assert_eq!(true, Char::from(VERTICAL_DOUBLE_AND_LEFT_SINGLE).is_crossing());
  /// assert_eq!(true, Char::from(DOWN_SINGLE_AND_HORIZONTAL_DOUBLE).is_crossing());
  /// assert_eq!(true, Char::from(DOWN_DOUBLE_AND_HORIZONTAL_SINGLE).is_crossing());
  /// assert_eq!(true, Char::from(UP_SINGLE_AND_HORIZONTAL_DOUBLE).is_crossing());
  /// assert_eq!(true, Char::from(UP_DOUBLE_AND_HORIZONTAL_SINGLE).is_crossing());
  /// assert_eq!(true, Char::from(VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE).is_crossing());
  /// assert_eq!(true, Char::from(VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE).is_crossing());
  /// assert_eq!(true, Char::from(DOUBLE_VERTICAL_AND_HORIZONTAL).is_crossing());
  ///
  /// assert_eq!(false, Char::from(LIGHT_HORIZONTAL).is_crossing());
  /// assert_eq!(false, Char::from(LIGHT_VERTICAL).is_crossing());
  /// assert_eq!(false, Char::from(DOUBLE_HORIZONTAL).is_crossing());
  /// assert_eq!(false, Char::from(DOUBLE_VERTICAL).is_crossing());
  /// ```
  pub fn is_crossing(&self) -> bool {
    matches!(
      *self.ch.borrow(),
      LIGHT_DOWN_AND_RIGHT
        | LIGHT_DOWN_AND_LEFT
        | LIGHT_UP_AND_RIGHT
        | LIGHT_UP_AND_LEFT
        | LIGHT_VERTICAL_AND_RIGHT
        | LIGHT_VERTICAL_AND_LEFT
        | LIGHT_DOWN_AND_HORIZONTAL
        | LIGHT_UP_AND_HORIZONTAL
        | LIGHT_VERTICAL_AND_HORIZONTAL
        | VERTICAL_SINGLE_AND_RIGHT_DOUBLE
        | VERTICAL_DOUBLE_AND_RIGHT_SINGLE
        | VERTICAL_SINGLE_AND_LEFT_DOUBLE
        | VERTICAL_DOUBLE_AND_LEFT_SINGLE
        | DOWN_SINGLE_AND_HORIZONTAL_DOUBLE
        | DOWN_DOUBLE_AND_HORIZONTAL_SINGLE
        | UP_SINGLE_AND_HORIZONTAL_DOUBLE
        | UP_DOUBLE_AND_HORIZONTAL_SINGLE
        | VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE
        | VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE
        | DOUBLE_VERTICAL_AND_HORIZONTAL
    )
  }

  /// Checks whether the character is a vertical line (single or double).
  ///
  /// # Examples
  ///
  /// ```
  /// # use dtee::Char;
  /// assert_eq!(true, Char::from('│').is_vert_line());
  /// assert_eq!(true, Char::from('║').is_vert_line());
  ///
  /// assert_eq!(false, Char::from('─').is_vert_line());
  /// assert_eq!(false, Char::from('═').is_vert_line());
  /// ```
  pub fn is_vert_line(&self) -> bool {
    matches!(*self.ch.borrow(), LIGHT_VERTICAL | DOUBLE_VERTICAL)
  }

  /// Checks whether the character is a vertical line or any crossing.
  pub fn is_vert_line_or_crossing(&self) -> bool {
    self.is_vert_line() || self.is_crossing()
  }

  /// Checks whether the character is a single vertical line.
  ///
  /// # Examples
  ///
  /// ```
  /// # use dtee::Char;
  /// assert_eq!(true, Char::from('│').is_single_vert_line());
  ///
  /// assert_eq!(false, Char::from('║').is_single_vert_line());
  /// ```
  pub fn is_single_vert_line(&self) -> bool {
    matches!(*self.ch.borrow(), LIGHT_VERTICAL)
  }

  /// Checks whether the character is a double vertical line.
  ///
  /// # Examples
  ///
  /// ```
  /// # use dtee::Char;
  /// assert_eq!(true, Char::from('║').is_double_vert_line());
  ///
  /// assert_eq!(false, Char::from('│').is_double_vert_line());
  /// ```
  pub fn is_double_vert_line(&self) -> bool {
    matches!(*self.ch.borrow(), DOUBLE_VERTICAL)
  }

  pub fn is_single_vert_line_crossing_left(&self) -> bool {
    matches!(
      *self.ch.borrow(),
      LIGHT_VERTICAL_AND_HORIZONTAL
        | LIGHT_DOWN_AND_HORIZONTAL
        | LIGHT_UP_AND_HORIZONTAL
        | LIGHT_DOWN_AND_LEFT
        | LIGHT_UP_AND_LEFT
        | LIGHT_VERTICAL_AND_LEFT
        | DOWN_DOUBLE_AND_HORIZONTAL_SINGLE
        | UP_DOUBLE_AND_HORIZONTAL_SINGLE
        | VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE
        | VERTICAL_DOUBLE_AND_LEFT_SINGLE
    )
  }

  pub fn is_double_vert_line_crossing_left(&self) -> bool {
    matches!(
      *self.ch.borrow(),
      VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE
        | DOUBLE_VERTICAL_AND_HORIZONTAL
        | VERTICAL_SINGLE_AND_LEFT_DOUBLE
        | DOWN_SINGLE_AND_HORIZONTAL_DOUBLE
        | UP_SINGLE_AND_HORIZONTAL_DOUBLE
    )
  }

  pub fn is_vert_line_crossing_left(&self) -> bool {
    self.is_single_vert_line_crossing_left() || self.is_double_vert_line_crossing_left()
  }

  /// Checks whether the character is a `left vertical line`,
  /// i.e. vertical line seen from the left side of the box-drawing character.
  ///
  /// # Examples
  ///
  /// ```
  /// # use dtee::Char;
  /// assert_eq!(true, Char::from('│').is_vert_line_left());
  /// assert_eq!(true, Char::from('├').is_vert_line_left());
  /// assert_eq!(true, Char::from('║').is_vert_line_left());
  /// assert_eq!(true, Char::from('╟').is_vert_line_left());
  ///
  /// assert_eq!(false, Char::from('┼').is_vert_line_left());
  /// assert_eq!(false, Char::from('╢').is_vert_line_left());
  /// ```
  pub fn is_vert_line_left(&self) -> bool {
    matches!(
      *self.ch.borrow(),
      LIGHT_VERTICAL | LIGHT_VERTICAL_AND_RIGHT | DOUBLE_VERTICAL | VERTICAL_DOUBLE_AND_RIGHT_SINGLE
    )
  }

  /// Checks whether the character is a `right vertical line`,
  /// i.e. vertical line seen from the right side of the box-drawing character.
  ///
  /// # Examples
  ///
  /// ```
  /// # use dtee::Char;
  /// assert_eq!(true, Char::from('│').is_vert_line_right());
  /// assert_eq!(true, Char::from('┤').is_vert_line_right());
  /// assert_eq!(true, Char::from('║').is_vert_line_right());
  /// assert_eq!(true, Char::from('╢').is_vert_line_right());
  ///
  /// assert_eq!(false, Char::from('┼').is_vert_line_right());
  /// assert_eq!(false, Char::from('╟').is_vert_line_right());
  /// ```
  pub fn is_vert_line_right(&self) -> bool {
    matches!(
      *self.ch.borrow(),
      LIGHT_VERTICAL | LIGHT_VERTICAL_AND_LEFT | DOUBLE_VERTICAL | VERTICAL_DOUBLE_AND_LEFT_SINGLE
    )
  }

  /// Checks whether the character is a horizontal line (single or double).
  ///
  /// # Examples
  ///
  /// ```
  /// # use dtee::Char;
  /// assert_eq!(true, Char::from('─').is_horz_line());
  /// assert_eq!(true, Char::from('═').is_horz_line());
  ///
  /// assert_eq!(false, Char::from('│').is_horz_line());
  /// assert_eq!(false, Char::from('║').is_horz_line());
  /// ```
  pub fn is_horz_line(&self) -> bool {
    matches!(*self.ch.borrow(), LIGHT_HORIZONTAL | DOUBLE_HORIZONTAL)
  }

  /// Checks whether the character is a horizontal line or any crossing.
  pub fn is_horz_line_or_crossing(&self) -> bool {
    self.is_horz_line() || self.is_crossing()
  }

  /// Checks whether the character is a space character.
  ///
  /// # Examples
  ///
  /// ```
  /// # use dtee::Char;
  /// assert_eq!(true, Char::from(' ').is_space());
  ///
  /// assert_eq!(false, Char::from('║').is_space());
  /// ```
  pub fn is_space(&self) -> bool {
    matches!(*self.ch.borrow(), SPACE)
  }
}
