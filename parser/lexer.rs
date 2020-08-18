use super::tokens::*;

/// Convert a character to a str.
fn character_to_str(character: u8) -> &'static str {
  let mut characters = Vec::new();
  characters.push(character);

  match String::from_utf8(characters) {
    Ok(string) => Box::leak(string.into_boxed_str()),
    Err(_) => "",
  }
}

/// Check if a character is a valid letter for an identifier.
fn is_letter_identifier(character: u8) -> bool {
  let character_as_str = character_to_str(character);

  character.is_ascii_alphabetic() || character_as_str == "_" || character_as_str == "$"
}

/// Check if a character is a valid number.
fn is_number(character: u8) -> bool {
  character.is_ascii_digit()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
  pub file_name: String,
  pub file_content: String,

  pub current_line: usize,
  pub current_line_position: usize,
  pub current_position: usize,
  pub current_character: u8,

  pub next_position: usize,
}

impl Lexer {
  /// Create a new lexer.
  pub fn new(file_name: String, file_content: String) -> Lexer {
    let mut lexer = Lexer {
      file_name,
      file_content,

      current_line: 1,
      current_line_position: 0,
      current_position: 0,
      current_character: 0,

      next_position: 0,
    };

    lexer.read_next_character();

    lexer
  }

  /// Read the next character and move the cursor to the next position.
  fn read_next_character(&mut self) {
    self.current_position = self.next_position;
    self.next_position += 1;
    self.current_line_position += 1;

    if self.current_position >= self.file_content.len() {
      self.current_character = 0;
    } else {
      self.current_character = self.file_content.as_bytes()[self.current_position];
    }
  }

  /// Get the next character.
  fn get_next_character(self) -> u8 {
    if self.next_position >= self.file_content.len() {
      0
    } else {
      self.file_content.as_bytes()[self.next_position]
    }
  }

  /// Get the two next character.
  fn get_two_next_character(self) -> u8 {
    if self.next_position + 1 >= self.file_content.len() {
      0
    } else {
      self.file_content.as_bytes()[self.next_position + 1]
    }
  }

  /// Ignore the whitespaces in the current line.
  fn skip_whitespace(&mut self) {
    loop {
      let current_character_str = character_to_str(self.current_character.clone());

      if current_character_str != " " && current_character_str != "\t" {
        break;
      }

      self.read_next_character();
    }
  }

  fn read_identifier_or_keyword(&mut self) -> String {
    let start_position = self.current_position;

    loop {
      if !is_letter_identifier(self.current_character) && !self.current_character.is_ascii_digit() {
        return self.file_content[start_position..self.current_position].to_string();
      }

      self.read_next_character();
    }
  }

  fn read_number(&mut self) -> String {
    let start_position = self.current_position;
    let mut has_dot = false;

    loop {
      let character_as_str = character_to_str(self.current_character);

      if !has_dot && character_as_str == "." {
        has_dot = true;
        self.read_next_character();
      }

      if !is_number(self.current_character) {
        return self.file_content[start_position..self.current_position].to_string();
      }

      self.read_next_character();
    }
  }

  fn read_string(&mut self, quote: u8) -> String {
    let start_position = self.current_position;
    let mut readed_first_quote = false;

    loop {
      if self.current_character == quote {
        if readed_first_quote {
          self.read_next_character();
          return self.file_content[start_position..self.current_position].to_string();
        } else {
          readed_first_quote = true;
        }
      }

      self.read_next_character();
    }
  }

  pub fn read_next_token(&mut self) -> Token {
    self.skip_whitespace();

    if self.current_character == 0 {
      Token::new(Box::new(Tokens::EOF), String::new(), self.current_line, self.current_line_position)
    } else {
      let start_position = self.current_line_position;
      let current_character_str = character_to_str(self.current_character);

      let mut current_token = Token::from_value(current_character_str.to_string(), self.current_line, start_position);

      // Check if the current token is a string, keyword, identifier or number.
      if current_token.token.clone().is_illegal() {
        if current_character_str == "\"" || current_character_str == "'" {
          // Read strings.
          current_token = Token::new(Box::new(Tokens::STRING), self.read_string(self.current_character), self.current_line, start_position);
        } else if is_letter_identifier(self.current_character) {
          // Read identifier or keyword.
          current_token = Token::from_value(self.read_identifier_or_keyword(), self.current_line, start_position);

          // If the current token is not a keyword, set it as identifier.
          if current_token.token.clone().is_illegal() {
            current_token.token = Box::new(Tokens::IDENTIFIER);
          }
        } else if is_number(self.current_character) {
          // Read numbers.
          current_token = Token::new(Box::new(Tokens::NUMBER), self.read_number(), self.current_line, start_position);
        } else {
          // Illegal token.
          self.read_next_character();
        }
      } else {
        if current_token.token.clone().is_end_of_line() {
          self.current_line += 1;
          self.current_line_position = 0;
        }

        self.read_next_character();
      }

      current_token
    }
  }
}
