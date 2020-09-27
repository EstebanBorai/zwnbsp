<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/EstebanBorai/zwnbsp/main/assets/icon.png" height="120" width="120" />
  </div>
  <h1 align="center">zwnbsp</h1>
  <h4 align="center">Zero width no-breaking space character utilities for Unicode and HTML</h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/zwnbsp.svg)](https://crates.io/crates/zwnbsp)
  [![Documentation](https://docs.rs/zwnbsp/badge.svg)](https://docs.rs/zwnbsp)
  ![Build](https://github.com/EstebanBorai/zwnbsp/workflows/build/badge.svg)
  ![Lint](https://github.com/EstebanBorai/zwnbsp/workflows/clippy/fmt/badge.svg)
  ![Tests](https://github.com/EstebanBorai/zwnbsp/workflows/tests/badge.svg)

</div>

## Usage

### Creating a HTML representation on ZWNBSP characters for "Hi!"

```rust
use zwnbsp::ZeroWidth;

fn main() {
    let zero_width = ZeroWidth::new("Hi!").unwrap().to_html();

    println!("{}", zero_width);
    // &#8204;&#8203;&#8204;&#8203;&#8203;&#8204;&#8203;&#8203;&#8203;&#8204;&#8205;&#8204;&#8203;&#8204;&#8204;&#8203;&#8204;&#8203;&#8203;&#8204;&#8204;&#8205;&#8204;&#8203;&#8204;&#8203;&#8203;&#8203;&#8203;&#8204;&#8204;&#8205
}
```

### Creating an Unicode representation on ZWNBSP characters for "Hi!"

```rust
use zwnbsp::ZeroWidth;

fn main() {
    let zero_width = ZeroWidth::new("Hi!").unwrap().to_unicode();

    println!("{}", zero_width);
    // ​​​​​​‌‍​​​​‌‍
}
```

## How it works

For every conversion a binary representation of the ASCII text provided is done.
This reduces the characters required to represent this data to 3 characters.

One character will represent the `0` value from the binary representations, the second
will represent the `1` value from the binary representation and finally the third
represents spaces to mark off the starting and the end of each binary set.

### From ASCII to ZWNBSP

When performing a conversion from ASCII text to ZWNBSP, the ASCII text is first converted into
its binary representation.

Given the text `Hi!` the result from encoding into binary would be
`01001000 01101001 00100001`.

Then, each value of the binary representation is replaced with its corresponding
zero width character.

### From ZWNBSP to ASCII

The process to convert ZWNBSP representations back to human readable ASCII is the "reverse" of
converting from ASCII to ZWNBSP. The binary representation with replaced characters is then
converted back to its binary representation in `1`, `0` and ` `, and then from binary
representation to text.

### Caveats

When converting some text back to ASCII you must have in mind that the corresponding values may not be the
same as used by this crate.

### Conversion Flowchart

The following flowchart explains the conversion from ASCII to ZWNBSP and from ZWNBSP back to ASCII.

<div align="center">
  <img src="https://raw.githubusercontent.com/EstebanBorai/zwnbsp/main/assets/how-it-works.png" width="400" />
</div>

### Checking the values in Unicode

The example [Creating an Unicode representation on ZWNBSP characters for "Hi!"](#creating-an-unicode-representation-on-zwnbsp-characters-for-hi!)
contains the value in the comment that appears to be empty.

You can copy the complete snippet, go to [diffchecker.com](https://www.diffchecker.com/) and paste
it to find the hidden characters.

<div align="center">
  <img src="https://raw.githubusercontent.com/EstebanBorai/zwnbsp/main/assets/diffchecker.png" />
</div>

## Release

To release a new version you must tag with git and push to the `main` branch.

```bash
git tag -a v0.1.0 -m "First Release"
git push origin main --follow-tags
```

## Contributing

Every contribution to this project is welcome! Feel free to open a pull request or an issue.

## License

Licensed under MIT License.
