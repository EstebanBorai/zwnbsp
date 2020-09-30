use crate::zero_width_char::{
  WORD_JOINER,
  ZERO_WIDTH_JOINER,
  ZERO_WIDTH_NO_BREAK_SPACE,
  ZERO_WIDTH_NON_JOINER,
  ZERO_WIDTH_SPACE
};

use std::str::FromStr;

/// Output encoding types
#[derive(Debug, PartialEq)]
pub enum Input {
  HTML,
  Unicode,
}

#[derive(Debug, PartialEq)]
pub struct NotSupportedText;

impl FromStr for Input {
  type Err = NotSupportedText;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let zwnbsp_chars: [(char, &str); 5] = [
      WORD_JOINER,
      ZERO_WIDTH_JOINER,
      ZERO_WIDTH_NO_BREAK_SPACE,
      ZERO_WIDTH_NON_JOINER,
      ZERO_WIDTH_SPACE
    ];

    for zwel in zwnbsp_chars.iter() {
      if s.find(zwel.0).is_some() {
        return Ok(Input::Unicode);
      }

      if s.find(zwel.1).is_some() {
        return Ok(Input::HTML);
      }
    }

    Err(NotSupportedText)
  }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_checks_raw_for_html() {
      let raw = "&#8204;&#8203;&#8204;";
      let found = Input::from_str(raw);

      assert_eq!(Input::HTML, found.unwrap());
    }

    #[test]
    fn it_checks_raw_for_unicode() {
      let raw = " ​​​​​​‌‍​​​​‌‍";
      let found = Input::from_str(raw);

      assert_eq!(Input::Unicode, found.unwrap());
    }

    #[test]
    fn it_breaks_if_no_supported_text() {
      let raw = "Lorem Ipsum";
      let found = Input::from_str(raw);

      assert_eq!(Err(NotSupportedText), found);
    }
}
