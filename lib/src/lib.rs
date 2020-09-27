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
mod zero_width_char;
mod error;
mod zero_width;

pub use binary::*;
pub use zero_width_char::*;
pub use error::*;
pub use zero_width::*;
