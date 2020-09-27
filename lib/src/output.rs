/// Output encoding types
#[derive(Debug, PartialEq)]
pub enum Output {
  HTML,
  Unicode,
}

impl Output {
  pub fn from_raw(raw: &str) -> Option<Self> {
    let ref mut chars = raw.chars();

    // Evaluate `raw` to be HTML text
    if chars.count() > 2 {
      // if the current `raw` value have more than 2 characters and
      // the first two characters are `&#` then this is escaped HTML char
      let (first, second) = (chars.next().unwrap(), chars.next().unwrap());

      if first == '&' && second == '#' {
        return Some(Self::HTML);
      }
    }

    None
  }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_checks_raw_for_html() {
      let raw = "&#8204;&#8203;&#8204;";
      let found = Output::from_raw(raw);

      assert_eq!(Output::HTML, found.unwrap());
    }
}
