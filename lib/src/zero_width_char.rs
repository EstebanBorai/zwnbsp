pub const ZERO_WIDTH_SPACE: (char, &str) = ('\u{200B}', "&#8203;");
pub const ZERO_WIDTH_NON_JOINER: (char, &str) = ('\u{200C}', "&#8204;");
pub const ZERO_WIDTH_JOINER: (char, &str) = ('\u{200D}', "&#8205;");
pub const WORD_JOINER: (char, &str) = ('\u{2060}', "&#8288;");
pub const ZERO_WIDTH_NO_BREAK_SPACE: (char, &str) = ('\u{FEFF}', "&#65279;");

#[derive(Debug, Copy, Clone)]
pub enum ZeroWidthChar {
    /// # Zero-width Space (ZWSP)
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
    Space,

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
    NonJoiner,

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
    Joiner,

    /// # Word Joiner (WJ)
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
    WordJoiner,

    /// # Zero-width No-break Space (ZWNBSP)
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
    NoBreakSpace,
}

impl ZeroWidthChar {
    /// Retireve the **HTML** representation of `self` as `&str`
    pub fn as_html(&self) -> &str {
        match self {
            ZeroWidthChar::Space => ZERO_WIDTH_SPACE.1,
            ZeroWidthChar::NonJoiner => ZERO_WIDTH_NON_JOINER.1,
            ZeroWidthChar::Joiner => ZERO_WIDTH_JOINER.1,
            ZeroWidthChar::WordJoiner => WORD_JOINER.1,
            ZeroWidthChar::NoBreakSpace => ZERO_WIDTH_NO_BREAK_SPACE.1,
        }
    }

    /// Retireve the **Unicode** representation of `self` as `char`
    pub fn as_unicode(&self) -> char {
        match self {
            ZeroWidthChar::Space => ZERO_WIDTH_SPACE.0,
            ZeroWidthChar::NonJoiner => ZERO_WIDTH_NON_JOINER.0,
            ZeroWidthChar::Joiner => ZERO_WIDTH_JOINER.0,
            ZeroWidthChar::WordJoiner => WORD_JOINER.0,
            ZeroWidthChar::NoBreakSpace => ZERO_WIDTH_NO_BREAK_SPACE.0,
        }
    }
}
