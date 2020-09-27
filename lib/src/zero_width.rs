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
pub type Config = [ZeroWidthChar; 3];

/// Zero width characters builder
pub struct ZeroWidth {
    binary: Binary,
    config: Config,
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

    /// Creates the Unicode zero width character representation
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
    const RUSTACEANS_ZW_UNICODE: &str = "\u{200c}\u{200b}\u{200c}\u{200b}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200b}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}";
    const RUSTACEANS_ZW_HTML: &str = "&#8204;&#8203;&#8204;&#8203;&#8204;&#8203;&#8203;&#8204;&#8203;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8204;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8203;&#8204;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8204;&#8203;&#8203;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8203;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8203;&#8204;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8204;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8203;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8203;&#8204;&#8204;&#8204;&#8205";

    #[test]
    fn it_zw_into_unicode() {
        let have = ZeroWidth::new("Rustaceans").unwrap().to_unicode();
        let want = RUSTACEANS_ZW_UNICODE.to_string();

        assert_eq!(have, want);
    }

    #[test]
    fn it_zw_into_html() {
        let have = ZeroWidth::new("Rustaceans").unwrap().to_html();
        let want = RUSTACEANS_ZW_HTML.to_string();

        assert_eq!(have, want);
    }
}
