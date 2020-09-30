//! # zwnbsp
//!
//! Create zero-width text representations from ASCII text
//! for Unicode and HTML.
//!
//! **zwnbsp** crate contains utilities to parse strings from ASCII to
//! a zero-width representation of this data. Currently the library supports
//! Unicode and HTML outputs.
//!
//! ## Create Unicode representation in zero width characters for "Rustaceans"
//!
//! ```
//! use zwnbsp::ZeroWidth;
//!
//! let zero_width = ZeroWidth::new("Rustaceans").unwrap().to_unicode();
//! let unicode_representation = "\u{200c}\u{200b}\u{200c}\u{200b}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200b}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}";
//!
//! assert_eq!(zero_width, unicode_representation);
//! ```
//!
//! ## Create HTML representation in zero width characters for "Rustaceans"
//!
//!```
//! use zwnbsp::ZeroWidth;
//!
//! let zero_width = ZeroWidth::new("Rustaceans").unwrap().to_html();
//! let unicode_representation = "&#8204;&#8203;&#8204;&#8203;&#8204;&#8203;&#8203;&#8204;&#8203;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8204;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8203;&#8204;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8204;&#8203;&#8203;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8203;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8203;&#8204;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8204;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8203;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8203;&#8204;&#8204;&#8204;&#8205";
//!
//! assert_eq!(zero_width, unicode_representation);
//!```
//!
mod binary;
mod error;
mod input;
mod zero_width;
mod zero_width_char;

pub use binary::*;
pub use error::*;
pub use input::*;
pub use zero_width::*;
pub use zero_width_char::*;

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    const RUSTACEANS_ZW_UNICODE: &str = "\u{200c}\u{200b}\u{200c}\u{200b}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200b}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200b}\u{200b}\u{200b}\u{200c}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200d}\u{200c}\u{200b}\u{200c}\u{200c}\u{200c}\u{200b}\u{200b}\u{200c}\u{200c}\u{200c}";
    const RUSTACEANS_ZW_HTML: &str = "&#8204;&#8203;&#8204;&#8203;&#8204;&#8203;&#8203;&#8204;&#8203;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8204;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8203;&#8204;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8204;&#8203;&#8203;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8203;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8203;&#8204;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8204;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8203;&#8203;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8204;&#8203;&#8203;&#8204;&#8204;&#8204;&#8205";

    #[test]
    fn it_uses_custom_config() {
      let mut zw = ZeroWidth::new("Rustaceans").unwrap();

      zw.set_config(&[
        ZeroWidthChar::NonJoiner,
        ZeroWidthChar::Joiner,
        ZeroWidthChar::NoBreakSpace,
      ]);

      let have = zw.to_html();
      let want = "&#8205;&#8204;&#8205;&#8204;&#8205;&#8204;&#8204;&#8205;&#8204;&#8205;&#65279;&#8205;&#8204;&#8205;&#8205;&#8205;&#8204;&#8205;&#8204;&#8205;&#8205;&#65279;&#8205;&#8204;&#8205;&#8205;&#8205;&#8204;&#8204;&#8205;&#8205;&#8205;&#65279;&#8205;&#8204;&#8205;&#8205;&#8205;&#8204;&#8205;&#8204;&#8204;&#8205;&#65279;&#8205;&#8204;&#8205;&#8205;&#8204;&#8204;&#8204;&#8204;&#8205;&#8205;&#65279;&#8205;&#8204;&#8205;&#8205;&#8204;&#8204;&#8204;&#8205;&#8205;&#8205;&#65279;&#8205;&#8204;&#8205;&#8205;&#8204;&#8204;&#8205;&#8204;&#8205;&#8205;&#65279;&#8205;&#8204;&#8205;&#8205;&#8204;&#8204;&#8204;&#8204;&#8205;&#8205;&#65279;&#8205;&#8204;&#8205;&#8205;&#8204;&#8205;&#8205;&#8205;&#8204;&#8205;&#65279;&#8205;&#8204;&#8205;&#8205;&#8205;&#8204;&#8204;&#8205;&#8205;&#8205;&#65279";

      assert_eq!(have, want);
    }

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

    #[test]
    fn get_the_string_representation() {
        let binary_raw = "01010010 01110101 01110011 01110100 01100001 01100011 01100101 01100001 01101110 01110011";
        let binary_string = Binary::from_str("Rustaceans").unwrap();

        assert_eq!(binary_raw, binary_string.to_string());
    }

    #[test]
    fn converts_to_ascii() {
        let original = "Rustaceans";
        let decoded = Binary::from_str(original).unwrap();
        let decoded = decoded.decode().unwrap();

        assert_eq!(original.to_string(), decoded);
    }
}
