use crate::{Binary, Error};
use std::str::FromStr;

/// # Zero-width space (ZWSP)
///
/// `char` for the Unicode representation on the ZWSP character
///
/// ## Representations
///
/// Encoding | Code
/// --- | ---
/// Unicode | `U+200B`
/// HTML | `&#8203;` `&ZeroWidthSpace;`
/// TeX | `\hskip0pt`
/// LaTex | `\hspace{0pt}`
/// groff | `\:`
///
/// ## Prohibited in URLs
///
/// ICANN rules prohibit domain names from including non-displayed characters
/// such as zero-width space, and most browsers prohibit their use within domain
/// names, because they can be used to create a homograph attack, where a malicious
/// URL is visually indistinguishable from a legitimate one.
///
/// ## Reference
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Zero-width_space)
///
pub const ZERO_WIDTH_SPACE: (char, &str) = ('\u{200B}', "&#8203;");

/// # Zero-width non-joiner (ZWNJ)
///
/// `char` for the Unicode representation on the ZWNJ character
///
/// ## Representations
///
/// Encoding | Code
/// --- | ---
/// Unicode | `U+200C`
/// HTML | `&#8204;` `&zwnj;`
///
/// ## Reference
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Zero-width_non-joiner)
///
pub const ZERO_WIDTH_NON_JOINER: (char, &str) = ('\u{200C}', "&#8204;");

/// # Zero-width joiner (ZWJ)
///
/// `char` for the Unicode representation on the ZWJ character
///
/// ## Representations
///
/// Encoding | Code
/// --- | ---
/// Unicode | `U+200D`
/// HTML | `&#8205;` `&zwj;`
///
/// ## Reference
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Zero-width_joiner)
///
pub const ZERO_WIDTH_JOINER: (char, &str) = ('\u{200D}', "&#8205;");

/// # Word joiner (WJ)
///
/// ## Representations
///
/// Encoding | Code
/// --- | ---
/// Unicode | `U+2060`
/// HTML | `&#8288;` `&NoBreak;`
///
/// ## Reference
///
/// [Wikipedia](https://en.wikipedia.org/wiki/Word_joiner)
///
pub const WORD_JOINER: (char, &str) = ('\u{2060}', "&#8288;");

/// # Zero-width No-Break Space (ZWNBSP)
///
/// `char` for the Unicode representation on the ZWNBSP character
///
/// ## Representations
///
/// Encoding | Code
/// --- | ---
/// Unicode | `U+FEFF`
/// HTML | `&#65279;`
///
/// ## Reference
///
/// [Wikipedia](https://www.fileformat.info/info/unicode/char/feff/index.htm)
///
pub const ZERO_WIDTH_NO_BREAK_SPACE: (char, &str) = ('\u{FEFF}', "&#65279;");

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
                    zero_width.push(ZERO_WIDTH_SPACE.0);
                } else {
                    zero_width.push(ZERO_WIDTH_NON_JOINER.0);
                }
            });

            zero_width.push(ZERO_WIDTH_JOINER.0);
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
                    zero_width.push_str(ZERO_WIDTH_SPACE.1);
                } else {
                    zero_width.push_str(ZERO_WIDTH_NON_JOINER.1);
                }
            });

            zero_width.push_str(ZERO_WIDTH_JOINER.1);
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
