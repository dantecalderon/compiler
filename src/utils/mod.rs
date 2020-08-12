/// Convert a byte to string.
pub fn as_string(byte: u8) -> String {
  let mut chars = vec![];
  chars.push(byte);
  String::from_utf8(chars).unwrap()
}

pub fn is_quote(byte: u8) -> bool {
  let value = as_string(byte);
  let value = value.as_str();

  value == "\"" || value == "'"
}

/// Comprobate if the byte is a letter.
pub fn is_letter(byte: u8) -> bool {
  byte.is_ascii_alphabetic()
}

/// Comprobate if the byte is a digit.
pub fn is_digit(byte: u8) -> bool {
  byte.is_ascii_digit()
}