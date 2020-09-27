use crate::error::Error;
use std::str::FromStr;
use std::string::ToString;

/// Binary Representation of a `String` value
pub struct Binary(String);

/// The representation of each binary `char` (`0`, `1` or `[Space]`)
pub enum BinaryUnit {
    Zero,
    One,
    Space,
}

impl Binary {
    /// Decodes a `Binary` back to the _ASCII_ representation
    pub fn decode(&self) -> Result<String, Error> {
        let as_u8: Vec<u8> = (0..self.0.len())
            .step_by(9)
            .map(|i| u8::from_str_radix(&self.0[i..i + 8], 2).expect("Unable to parse to u8"))
            .collect();

        Ok(String::from_utf8(as_u8)?)
    }
}

impl ToString for Binary {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FromStr for Binary {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut binary = String::default();
        let ascii: String = s.into();

        for character in ascii.into_bytes() {
            binary += &format!("0{:b} ", character);
        }

        // removes the trailing space at the end
        binary.pop();

        Ok(Self(binary))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn converts_to_binary() {
        let ascii = "Rustaceans";
        let binary = "01010010 01110101 01110011 01110100 01100001 01100011 01100101 01100001 01101110 01110011";

        assert_eq!(binary, Binary::from_str(ascii).unwrap().0);
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
