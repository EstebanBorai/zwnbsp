use crate::{Binary, BinaryUnit, Error, Input, NotSupportedText, ZeroWidthChar};
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
    /// The `Binary` representation of the raw text
    binary: Binary,
    /// The elements to replace `BinaryUnits` with.
    ///
    /// The first element (`0`) represents the `BinaryUnit::Zero`,
    /// the second element represents the `BinaryUnit::One` and finally
    /// the third element represent the `BinaryUnit::Space` used as a "padding"
    /// for binary sets
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

    pub fn decode(&self, raw: &str) -> Result<String, NotSupportedText> {
        let raw_type = Input::from_str(raw)?;
        
        match raw_type {
            Input::HTML => Ok(self.from_html(raw)),
            Input::Unicode => Ok(self.from_unicode(raw)),
        }
    }

    pub fn from_html(&self, raw: &str) -> String {
        let encoded_binaries = self.get_binary_sets(
            raw,
            self.get_separator(Input::HTML).as_str()
        );

        encoded_binaries.into_iter().for_each(|binary_set| {
            binary_set
        });
    }

    pub fn from_unicode(&self, raw: &str) -> String {
        todo!()
    }

    fn get_binary_sets<'a>(&self, raw: &'a str, separator: &str) -> Vec<String> {
        raw.split(separator)
            .map(|set| set.to_string())
            .collect::<Vec<String>>()
    }

    fn get_separator(&self, raw_type: Input) -> String {
        let separator = self.config[2];
        return match raw_type {
            Input::HTML => separator.as_html().to_string(),
            Input::Unicode => separator.as_unicode().to_string(),
        };
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
