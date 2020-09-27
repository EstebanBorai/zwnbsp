use crate::{Binary, BinaryUnit, Error, ZeroWidthChar};
use std::str::FromStr;

/// Character replacement configuration represented by binary
/// values `0`, `1` and ` ` (space).
///
/// Default values for each binary representation are:
///
/// `0`: `ZeroWidthChar::Space`
/// `1`: `ZeroWidthChar::NonJoiner`
/// `[Space]`: `ZeroWidthChar::Joiner`
///
pub type ReplacementConfig = [ZeroWidthChar; 3];

/// Zero width characters builder
pub struct ZeroWidth {
    binary: Binary,
    config: ReplacementConfig,
}

impl From<Binary> for ZeroWidth {
    fn from(binary: Binary) -> Self {
        let config: [ZeroWidthChar; 3] = [
            ZeroWidthChar::Space,
            ZeroWidthChar::NonJoiner,
            ZeroWidthChar::Joiner,
        ];

        Self { binary, config }
    }
}

impl ZeroWidth {
    /// Creates a new `ZeroWidth` instance given a string slice
    pub fn new(string: &str) -> Result<Self, Error> {
        let binary = Binary::from_str(string)?;

        Ok(ZeroWidth::from(binary))
    }

    /// Set the replacement characters configuration.
    ///
    /// Its important to note that theres no validation on the provided
    /// characters due to provide maximum flexibility.
    ///
    /// The values provided to this method are settled _as is_ for the
    /// `ZeroWidth` instance in question.
    pub fn set_config(&mut self, config: &ReplacementConfig) {
        self.config = config.clone();
    }

    /// Retrieve the binary representation of the ASCII
    /// value provided
    pub fn get_binary_string(&self) -> String {
        self.binary.to_string()
    }

    /// Get the `ZeroWidthChar` equivalent to the provided `BinaryUnit`
    pub fn get_from_binary(&self, unit: BinaryUnit) -> ZeroWidthChar {
        match unit {
            BinaryUnit::Zero => *self.config.get(0).unwrap(),
            BinaryUnit::One => *self.config.get(1).unwrap(),
            BinaryUnit::Space => *self.config.get(2).unwrap(),
        }
    }

    /// Get the `ZeroWidthChar` **Unicode** equivalent to the provided `BinaryUnit`
    pub fn get_unicode_from_binary(&self, unit: BinaryUnit) -> char {
        self.get_from_binary(unit).as_unicode()
    }

    /// Get the `ZeroWidthChar` **HTML** equivalent to the provided `BinaryUnit`
    pub fn get_html_from_binary(&self, unit: BinaryUnit) -> String {
        let html = &self.get_from_binary(unit);
        let html = html.as_html();

        html.to_string()
    }

    /// Creates the Unicode zero width character representation
    /// from the binary representation of the ASCII value
    pub fn to_unicode(&self) -> String {
        let string_value = self.get_binary_string();
        let mut zero_width = String::default();

        // split each ascii letter representation
        let chars: Vec<&str> = string_value.split(' ').collect();

        chars.into_iter().for_each(|character| {
            character.split("").for_each(|ch| {
                if ch.eq("0") {
                    zero_width.push(self.get_unicode_from_binary(BinaryUnit::Zero));
                } else {
                    zero_width.push(self.get_unicode_from_binary(BinaryUnit::One));
                }
            });

            zero_width.push(self.get_unicode_from_binary(BinaryUnit::Space));
        });

        // remove trailing zero width character
        zero_width.pop();

        zero_width
    }

    /// Creates the HTML zero width character representation
    /// from the binary representation of the ASCII value
    pub fn to_html(&self) -> String {
        let string_value = self.get_binary_string();
        let mut zero_width = String::default();

        // split each ascii letter representation
        let chars: Vec<&str> = string_value.split(' ').collect();

        chars.into_iter().for_each(|character| {
            character.split("").for_each(|ch| {
                if ch.eq("0") {
                    zero_width.push_str(&self.get_html_from_binary(BinaryUnit::Zero));
                } else {
                    zero_width.push_str(&self.get_html_from_binary(BinaryUnit::One));
                }
            });

            zero_width.push_str(&self.get_html_from_binary(BinaryUnit::Space));
        });

        // remove trailing zero width character
        zero_width.pop();

        zero_width
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_sets_custom_config() {
      let conf: [ZeroWidthChar; 3]= [
        ZeroWidthChar::NonJoiner,
        ZeroWidthChar::Joiner,
        ZeroWidthChar::NoBreakSpace,
      ];

      let mut zw = ZeroWidth::new("Rustaceans").unwrap();

      zw.set_config(&conf);

      assert_eq!(zw.config.get(0).unwrap(), conf.get(0).unwrap());
      assert_eq!(zw.config.get(1).unwrap(), conf.get(1).unwrap());
      assert_eq!(zw.config.get(2).unwrap(), conf.get(2).unwrap());
    }
}
