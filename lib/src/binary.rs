pub fn to_binary(ascii: &str) -> String {
  let mut binary = String::default();
  let ascii: String = ascii.into();

  for character in ascii.clone().into_bytes() {
    binary += &format!("0{:b} ", character);
  }

  binary.pop();

  binary
}

#[cfg(test)]
mod test {
  use super::*;

    #[test]
    fn converts_to_binary() {
      let ascii = "Rustaceans";
      let binary = "01010010 01110101 01110011 01110100 01100001 01100011 01100101 01100001 01101110 01110011";

      assert_eq!(binary, to_binary(ascii));
    }
}
