use crate::{Binary, Error, ZeroWidthChar};
use std::str::FromStr;

pub struct ZeroWidth {
    binary: Binary,
}

impl From<Binary> for ZeroWidth {
    fn from(binary: Binary) -> Self {
        Self {
            binary
        }
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
                    zero_width.push(ZeroWidthChar::Space.as_unicode());
                } else {
                    zero_width.push(ZeroWidthChar::NonJoiner.as_unicode());
                }
            });

            zero_width.push(ZeroWidthChar::Joiner.as_unicode());
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
                    zero_width.push_str(ZeroWidthChar::Space.as_html());
                } else {
                    zero_width.push_str(ZeroWidthChar::NonJoiner.as_html());
                }
            });

            zero_width.push_str(ZeroWidthChar::Joiner.as_html());
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
