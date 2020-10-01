//
//  Copyright (c) Sflynlang
//
//  This source code is licensed under the MIT license found in the
//  LICENSE file in the root directory of this source tree.
//

use crate::File;
use crate::Point;
use crate::Tok;
use crate::Token;

use std::collections::HashMap;

/// Get all available keywords with their token
/// in a hashmap.
fn get_keywords() -> HashMap<String, Token> {
  let mut keywords: HashMap<String, Token> = HashMap::new();

  // Variables
  keywords.insert(String::from("let"), Token::Let);
  keywords.insert(String::from("const"), Token::Const);

  // Asynchronous
  keywords.insert(String::from("async"), Token::Async);
  keywords.insert(String::from("await"), Token::Await);

  // Functions
  keywords.insert(String::from("func"), Token::Func);
  keywords.insert(String::from("return"), Token::Return);

  // Is data type
  keywords.insert(String::from("is"), Token::Is);

  // If-else and switch
  keywords.insert(String::from("if"), Token::If);
  keywords.insert(String::from("else"), Token::Else);
  keywords.insert(String::from("switch"), Token::Switch);
  keywords.insert(String::from("case"), Token::Case);
  keywords.insert(String::from("break"), Token::Break);

  // Do-while and while
  keywords.insert(String::from("do"), Token::Do);
  keywords.insert(String::from("while"), Token::While);
  keywords.insert(String::from("continue"), Token::Continue);

  // For in and for of
  keywords.insert(String::from("for"), Token::For);
  keywords.insert(String::from("in"), Token::In);
  keywords.insert(String::from("of"), Token::Of);

  // Import and export
  keywords.insert(String::from("import"), Token::Import);
  keywords.insert(String::from("from"), Token::From);
  keywords.insert(String::from("export"), Token::Export);
  keywords.insert(String::from("as"), Token::As);

  // Boolean values
  keywords.insert(String::from("true"), Token::True);
  keywords.insert(String::from("false"), Token::False);

  // Try and catch
  keywords.insert(String::from("try"), Token::Try);
  keywords.insert(String::from("catch"), Token::Catch);
  keywords.insert(String::from("finally"), Token::Finally);

  // Classes
  keywords.insert(String::from("class"), Token::Class);
  keywords.insert(String::from("constructor"), Token::Constructor);
  keywords.insert(String::from("internal"), Token::Internal);
  keywords.insert(String::from("private"), Token::Private);
  keywords.insert(String::from("protected"), Token::Protected);
  keywords.insert(String::from("public"), Token::Public);
  keywords.insert(String::from("readonly"), Token::Readonly);

  // Interface and enum
  keywords.insert(String::from("interface"), Token::Interface);
  keywords.insert(String::from("enum"), Token::Enum);

  // Data types
  keywords.insert(String::from("string"), Token::String);
  keywords.insert(String::from("number"), Token::Number);
  keywords.insert(String::from("boolean"), Token::Boolean);
  keywords.insert(String::from("null"), Token::Null);
  keywords.insert(String::from("void"), Token::Void);

  keywords
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
  file: File,

  current_line: usize,
  current_line_position: usize,
  current_position: usize,
  next_position: usize,

  current_character: Option<char>,
  next_character: Option<char>,

  keywords: HashMap<String, Token>,
}

impl Lexer {
  /// Create a new lexer object
  /// for a file object.
  pub fn new(file: File) -> Self {
    let mut lexer = Self {
      file,

      current_line: 1,
      current_line_position: 1,
      current_position: 0,
      next_position: 0,

      current_character: None,
      next_character: None,

      keywords: get_keywords(),
    };

    // Skip empty content at the begin.
    lexer.next_char();
    lexer.next_char();

    // Set the column at 1.
    lexer.current_line_position = 1;

    // Return the lexer object.
    lexer
  }

  /// Set the next character to the lexer
  /// object and return the current character
  /// before change it for the next character.
  fn next_char(&mut self) -> Option<char> {
    // Set the current position with the next position.
    self.current_position = self.next_position;

    // Get the next position.
    self.next_position += 1;

    // Add one to the column (current line position).
    self.current_line_position += 1;

    // Get the current character before change it.
    let current_character = self.current_character;

    // Set the current character with the next character.
    self.current_character = self.next_character;

    // Check if the next position is greater than the file content length.
    if self.next_position > self.file.get_content().len() {
      // Set the next character as None.
      self.next_character = None;
    }
    // Otherwise, get the next character from the file content.
    else {
      self.next_character = self.file.get_content()
        .chars()
        .skip(self.current_position)
        .next();
    }

    // Return the current character.
    current_character
  }

  /// Ignore all whitespaces of the code.
  fn skip_whitespaces(&mut self) {
    loop {
      match self.current_character {
        Some(' ') | Some('\t') => {
          self.next_char();
          continue
        },

        _ => break,
      }
    }
  }

  /// Get the code comments.
  /// 
  /// ## Example
  /// ```sf
  /// // This is a comment
  /// 
  /// /* This is other comment */
  /// ```
  fn get_comment(&mut self, init_token: Token) -> Token {
    let mut value = String::new();

    // Check if the initial token is `/*` (Multiple lines comment).
    if init_token == Token::SlashStar {
      value.push_str("/*");

      // Get the next character.
      self.next_char();

      loop {
        // Check if the next character is `*/` and finish the loop.
        if self.get_sign_token() == Ok(Token::StarSlash) {
          self.next_char();
          value.push_str("*/");
          break;
        }

        // Check if the current character is a end of line.
        if self.current_character == Some('\n') {
          self.next_char();
          self.current_line += 1;
          self.current_line_position = 1;
        }

        // Check if the current character is not a end of file.
        if let Some(character) = self.next_char() {
          // Append the character to the value string.
          value.push(character);
          continue;
        }

        break;
      }

      // Check if the value starts with `/*` and not ends with `*/`.
      if value.starts_with("/*") && !value.ends_with("*/") {
        // TODO!: Lexer error
      }
    } else if init_token == Token::DoubleSlash {
      value.push_str("//");

      // Get the next character.
      self.next_char();

      loop {
        // Check if the current token is a end of line or an end of file and finish the loop.
        if self.current_character == Some('\n') || self.current_character == None {
          break;
        }

        // Check if the current character is not a end of file.
        if let Some(character) = self.next_char() {
          // Append the character to the value string.
          value.push(character);
          continue;
        }

        break;
      }
    }

    if value.is_empty() {
      init_token
    } else {
      // Create a new comment token with the comment value.
      Token::Comment { value }
    }
  }

  /// Check if the current character is a sign and
  /// get the token for it.
  /// 
  /// ## Usage
  /// ```rust
  /// use sflynlang_parser::File;
  /// use sflynlang_parser::Lexer;
  /// 
  /// let file_name = "test.sf";
  /// let file_content = "let hello: string = 'world';";
  /// 
  /// // Create a new file object.
  /// let file = File::new(file_name.to_string(), file_content.to_string());
  /// 
  /// // Create a new lexer object.
  /// let mut lexer = Lexer::new(file);
  /// 
  /// // Get the next token.
  /// let token = lexer.next_token();
  /// 
  /// println!("{:?}", token);
  /// // Tok { point: Point { line: 1, column: 1 }, token: Token { Let } }
  /// ```
  /// 
  /// ## Example (Internal process)
  /// ```rust
  /// use sflynlang_parser::Token;
  /// 
  /// fn main() {
  ///   let token = match ";" {
  ///     ";"   => Token::SemiColon,
  ///     "..." => Token::Ellipsis,
  ///     _ => Token::Illegal
  ///   };
  /// 
  ///   println!("{:?}", token);
  /// }
  /// ```
  fn get_sign_token(&mut self) -> Result<Token, ()> {
    match self.current_character {
      // Semicolon (`;`).
      Some(';') => Ok(Token::SemiColon),

      // Colon (`:`).
      Some(':') => Ok(Token::Colon),

      // Comma (`,`).
      Some(',') => Ok(Token::Comma),

      // Parse dot and ellipsis (`.` and `...`).
      Some('.') => Ok(match self.next_character {
        // Double dot (`..`).
        Some('.') => {
          // Get the next character.
          self.next_char();

          match self.next_character {
            // Ellipsis (`...`).
            Some('.') => {
              // Get the next character.
              self.next_char();
              Token::Ellipsis
            },

            // Is not a valid character (`..` or whatever).
            _ => {
              // TODO!: Lexer error
              Token::Illegal
            },
          }
        },

        // Dot (`.`).
        _ => Token::Dot,
      }),

      // Parse equal and double equal (`=` and `==`).
      Some('=') => Ok(match self.next_character {
        // Double equal (`==`).
        Some('=') => {
          // Get the next character.
          self.next_char();
          Token::DoubleEqual
        },

        // Equal (`=`).
        _ => Token::Equal,
      }),

      // Parse plus, plus equal and double plus (`+`, `+=` and `++`).
      Some('+') => Ok(match self.next_character {
        // Plus equal (`+=`).
        Some('=') => {
          // Get the next character.
          self.next_char();
          Token::PlusEqual
        },

        // Double plus (`++`).
        Some('+') => {
          // Get the next character.
          self.next_char();
          Token::DoublePlus
        },

        // Plus (`+`).
        _ => Token::Plus,
      }),

      // Parse minus, minus equal and double minus (`-`, `-=` and `--`).
      Some('-') => Ok(match self.next_character {
        // Minus equal (`-=`).
        Some('=') => {
          // Get the next character.
          self.next_char();
          Token::MinusEqual
        },

        // Double minus (`--`).
        Some('-') => {
          // Get the next character.
          self.next_char();
          Token::DoubleMinus
        },

        // Minus (`-`).
        _ => Token::Minus,
      }),

      // Parse star, double star, double star equal,
      // star equal and star slash (`*`, `**`, `**=`, `*=` and `*/`).
      Some('*') => Ok(match self.next_character {
        // Double star and double star equal (`**` and `**=`).
        Some('*') => {
          // Get the next character.
          self.next_char();

          match self.next_character {
            // Double star equal (`**=`).
            Some('=') => {
              // Get the next character.
              self.next_char();
              Token::DoubleStarEqual
            },

            // Double star (`**`).
            _ => Token::DoubleStar,
          }
        },

        // Star equal (`*=`).
        Some('=') => {
          // Get the next character.
          self.next_char();
          Token::StarEqual
        },

        // Star slash (`*/`).
        Some('/') => {
          // Get the next character.
          self.next_char();
          Token::StarSlash
        },

        // Star (`*`).
        _ => Token::Star,
      }),

      // Parse slash, double slash, slash star and
      // slash equal (`/`, `//`, `/*` and `/=`).
      Some('/') => Ok(match self.next_character {
        // Double slash (`//`).
        Some('/') => {
          // Get the next character.
          self.next_char();

          // Get the single line comment token.
          self.get_comment(Token::DoubleSlash)
        },

        // Slash star (`/*`).
        Some('*') => {
          // Get the next character.
          self.next_char();

          // Get the multiple line comment token.
          self.get_comment(Token::SlashStar)
        },

        // Slash equal (`/=`).
        Some('=') => {
          // Get the next character.
          self.next_char();
          Token::SlashEqual
        },

        // Slash (`/`).
        _ => Token::Slash,
      }),

      // Parse percent and percent equal (`%` and `%=`).
      Some('%') => Ok(match self.next_character {
        // Percent equal (`%=`).
        Some('=') => {
          // Get the next character.
          self.next_char();
          Token::PercentEqual
        },

        // Percent (`%`).
        _ => Token::Percent,
      }),

      // Parse less and less equal (`<` and `<=`).
      Some('<') => Ok(match self.next_character {
        // Less equal (`<=`).
        Some('=') => {
          // Get the next character.
          self.next_char();
          Token::LessEqual
        },

        // Less (`<`).
        _ => Token::Less,
      }),

      // Parse greater and greater equal (`>` and `>=`).
      Some('>') => Ok(match self.next_character {
        // Greater equal (`>=`).
        Some('=') => {
          // Get the next character.
          self.next_char();
          Token::GreaterEqual
        },

        // Greater (`>`).
        _ => Token::Greater,
      }),

      // Left parentheses (`(`).
      Some('(') => Ok(Token::LeftParentheses),

      // Right parentheses (`)`).
      Some(')') => Ok(Token::RightParentheses),

      // Left brace (`{`).
      Some('{') => Ok(Token::LeftBrace),

      // Right brace (`}`).
      Some('}') => Ok(Token::RightBrace),

      // Left bracket (`[`).
      Some('[') => Ok(Token::LeftBracket),

      // Right bracket (`]`).
      Some(']') => Ok(Token::RightBracket),

      // At (`@`).
      Some('@') => Ok(Token::At),

      // Tilde (`~`).
      Some('~') => Ok(Token::Tilde),

      // Parse vertical bar and doube vertical bar (`|` and `||`).
      Some('|') => Ok(match self.next_character {
        // Double vertical bar (`||`).
        Some('|') => {
          // Get the next character.
          self.next_char();
          Token::DoubleVBar
        },

        // Vertical bar (`|`).
        _ => Token::VBar,
      }),

      // Parse circum flex and circum flex equal (`^` and `^=`).
      Some('^') => Ok(match self.next_character {
        // Circum flex equal (`^=`).
        Some('=') => {
          // Get the next character.
          self.next_char();
          Token::CircumFlexEqual
        },

        // Circum flex (`^`).
        _ => Token::CircumFlex,
      }),

      // Parse amper and double amper (`&` and `&&`).
      Some('&') => Ok(match self.next_character {
        // Double amper (`&&`).
        Some('&') => {
          // Get the next character.
          self.next_char();
          Token::DoubleAmper
        },

        // Amper (`&`).
        _ => Token::Amper,
      }),

      // Parse not and not equal (`!` and `!=`).
      Some('!') => Ok(match self.next_character {
        // Not equal (`!=`).
        Some('=') => {
          // Get the next character.
          self.next_char();
          Token::NotEqual
        },

        // Not (`!`).
        _ => Token::Not,
      }),

      // Is not a sign character.
      _ => Err(()),
    }
  }

  /// Check if the current character is an
  /// identifier begin (_, $ or an alphabetic letter).
  fn is_letter_begin(&mut self) -> bool {
    match self.current_character {
      // When character is `_` or `$`.
      Some('_') | Some('$') => true,

      // Check if the character is alphabetic.
      Some(character) => character.is_ascii_alphabetic(),

      // Is End Of File.
      None => false,
    }
  }

  /// Check if the current character probably is an
  /// identifier o a keyword.
  fn get_identifier_or_keyword_token(&mut self) -> Result<Token, ()> {
    if self.is_letter_begin() {
      let mut value: String = String::from(self.current_character.unwrap());

      // Get the next character.
      self.next_char();

      loop {
        match self.current_character {
          Some(character) => {
            // Check if the current character is a number, alphabetic or a underscore.
            if character.is_ascii_digit() || character.is_ascii_alphabetic() || character == '_' {
              // Append the character to the value.
              value.push(self.next_char().unwrap());
              continue;
            }

            break;
          },

          _ => break,
        }
      }

      // Check if the value is a keyword and return it.
      if let Some(keyword) = self.keywords.get(&value) {
        return Ok(keyword.clone());
      }

      // Return the value as an identifier.
      return Ok(Token::Identifier { value });
    }

    Err(())
  }

  /// Check if the current character is a valid
  /// number for the radix number.
  /// 
  /// ```text
  /// 2   = Binary numbers.
  /// 8   = Numbers between 0 and 7.
  /// 10  = Numbers between 0 and 9.
  /// 16  = Hexadecimal numbers (0-9, A-F and a-f).
  /// ```
  fn is_digit_of_radix(&self, radix: u32) -> bool {
    match radix {
      2 => match self.current_character {
        Some('0'..='1') => true,
        _ => false,
      },

      8 => match self.current_character {
        Some('0'..='7') => true,
        _ => false,
      },

      10 => match self.current_character {
        Some('0'..='9') => true,
        _ => false,
      },

      16 => match self.current_character {
        Some('0'..='9') | Some('a'..='f') | Some('A'..='F') => true,
        _ => false,
      },

      _ => false,
    }
  }

  /// Parse the characters as binary or hexadecimal.
  /// 
  /// The binaries numbers starts with `0b` and the
  /// hexadecimals numbers starts with `0x`.
  fn get_binary_or_hex_value(&mut self) -> Result<String, ()> {
    // Check if the current character is `0` and the next character is `b` or `x`.
    // `0b` for binaries and `0x` for hexadecimals.
    if self.current_character == Some('0') && (
      self.next_character == Some('b') ||
      self.next_character == Some('x')
    ) {
      let mut value = String::new();

      // Get the next character and append the current character to the value.
      value.push(self.next_char().unwrap());

      // Get the next character and set the current character in the variable.
      let ntype: char = self.next_char().unwrap();

      // Append the ntype to the value.
      value.push(ntype);

      // Get the radix number from the ntype.
      let num: u32 = if ntype == 'b' { 2 } else { 16 };

      loop {
        // Check if the current character is a valid number.
        if self.is_digit_of_radix(num) {
          // Append the number to the value.
          value.push(self.next_char().unwrap());
          continue;
        }

        break;
      }

      return Ok(value);
    }

    Err(())
  }

  fn get_number_token(&mut self) -> Result<Token, ()> {
    if self.is_digit_of_radix(10) {
      let mut value = String::new();
      let mut has_dot = false;

      if let Ok(binary_or_hex) = self.get_binary_or_hex_value() {
        value = binary_or_hex;
      }

      loop {
        match self.current_character {
          Some('.') => {
            if !has_dot {
              value.push(self.next_char().unwrap());
              has_dot = true;
              continue;
            }

            break;
          },

          Some(character) => {
            if self.is_digit_of_radix(10) || character == '_' {
              value.push(self.next_char().unwrap());
              continue;
            }

            break;
          },

          _ => break,
        }
      }

      return Ok(Token::Num { value });
    }

    Err(())
  }

  fn get_string_token(&mut self) -> Result<Token, ()> {
    let mut quote: char = ' ';
    let mut value = String::new();

    match self.current_character {
      Some('\'') |
      Some('"') => {
        quote = self.next_char().unwrap();
        value.push(quote);
      },

      _ => {},
    }

    if quote != ' ' {
      loop {
        if self.current_character == Some('\n') || self.current_character == None {
          // TODO: Lexer error
          break;
        }

        if self.current_character == Some(quote) {
          value.push(self.next_char().unwrap());
          break;
        }

        if let Some(character) = self.next_char() {
          value.push(character);
          continue;
        }

        break;
      }
    }

    if value.is_empty() {
      Err(())
    } else {
      Ok(Token::Str { value })
    }
  }

  /// Get the token object for the current character
  /// or group of characters.
  /// 
  /// ## Example (Internal process)
  /// ```rust
  /// use sflynlang_parser::Point;
  /// use sflynlang_parser::Tok;
  /// use sflynlang_parser::Token;
  /// 
  /// fn main() {
  ///   let tok = match "..." {
  ///     "..."       => Tok::new(Point::new(1, 1), Token::Ellipsis, Point::new(1, 4)),
  ///     "'Strings'" => Tok::new(Point::new(1, 1), Token::Str { value: String::from("'Strings'") }, Point::new(1, 10)),
  ///     _ => Tok::new(Point::new(1, 1), Token::Illegal, Point::new(1, 1)),
  ///   };
  /// 
  ///   println!("{:?}", tok);
  /// }
  /// ```
  pub fn next_token(&mut self) -> Tok {
    self.skip_whitespaces();

    let start_point = Point::new(self.current_line, self.current_line_position);

    let token = if self.current_character.is_none() {
      Token::EndOfFile
    } else if self.current_character == Some('\n') {
      self.current_line += 1;
      self.current_line_position = 1;
      Token::EndOfLine
    } else if let Ok(sign) = self.get_sign_token() {
      sign
    } else if let Ok(string) = self.get_string_token() {
      string
    } else if let Ok(identifier_or_keyword) = self.get_identifier_or_keyword_token() {
      identifier_or_keyword
    } else if let Ok(number) = self.get_number_token() {
      number
    } else {
      Token::Illegal
    };

    let end_point = Point::new(self.current_line, self.current_line_position);

    Tok::new(start_point, token, end_point)
  }
}

#[cfg(test)]
mod lexer_tests {
  use super::*;

  fn new_lexer(content: &str) -> Lexer {
    Lexer::new(File::new(
      String::from("test.sf"),
      content.to_string(),
    ))
  }

  fn new_tok(token: Token, line: usize, column: usize, column2: usize) -> Tok {
    Tok::new(Point::new(line, column), token, Point::new(line, column2))
  }

  fn test_token(content: &str, token: Tok) {
    let mut lexer = new_lexer(content);

    assert_eq!(lexer.next_token(), token);
  }

  #[test]
  fn tokens() {
    // General
    test_token("something", new_tok(Token::Identifier { value: String::from("something") }, 1, 1, 10));
    test_token("$something", new_tok(Token::Identifier { value: String::from("$something") }, 1, 1, 11));
    test_token("_something", new_tok(Token::Identifier { value: String::from("_something") }, 1, 1, 11));

    test_token("// Comment", new_tok(Token::Comment { value: String::from("// Comment") }, 1, 1, 11));
    test_token("/* Comment */", new_tok(Token::Comment { value: String::from("/* Comment */") }, 1, 1, 14));

    test_token("10", new_tok(Token::Num { value: String::from("10") }, 1, 1, 3));
    test_token("0x10", new_tok(Token::Num { value: String::from("0x10") }, 1, 1, 5));
    test_token("10.10", new_tok(Token::Num { value: String::from("10.10") }, 1, 1, 6));
    test_token("100_000", new_tok(Token::Num { value: String::from("100_000") }, 1, 1, 8));

    test_token("'something'", new_tok(Token::Str { value: String::from("'something'") }, 1, 1, 12));
    test_token("\"something\"", new_tok(Token::Str { value: String::from("\"something\"") }, 1, 1, 12));

    // Keywords
    test_token("let", new_tok(Token::Let, 1, 1, 4));
    test_token("const", new_tok(Token::Const, 1, 1, 6));

    test_token("async", new_tok(Token::Async, 1, 1, 6));
    test_token("await", new_tok(Token::Await, 1, 1, 6));

    test_token("func", new_tok(Token::Func, 1, 1, 5));
    test_token("return", new_tok(Token::Return, 1, 1, 7));

    test_token("is", new_tok(Token::Is, 1, 1, 3));

    test_token("if", new_tok(Token::If, 1, 1, 3));
    test_token("else", new_tok(Token::Else, 1, 1, 5));
    test_token("switch", new_tok(Token::Switch, 1, 1, 7));
    test_token("case", new_tok(Token::Case, 1, 1, 5));
    test_token("break", new_tok(Token::Break, 1, 1, 6));

    test_token("do", new_tok(Token::Do, 1, 1, 3));
    test_token("while", new_tok(Token::While, 1, 1, 6));
    test_token("continue", new_tok(Token::Continue, 1, 1, 9));

    test_token("for", new_tok(Token::For, 1, 1, 4));
    test_token("in", new_tok(Token::In, 1, 1, 3));
    test_token("of", new_tok(Token::Of, 1, 1, 3));

    test_token("import", new_tok(Token::Import, 1, 1, 7));
    test_token("from", new_tok(Token::From, 1, 1, 5));
    test_token("export", new_tok(Token::Export, 1, 1, 7));
    test_token("as", new_tok(Token::As, 1, 1, 3));

    test_token("true", new_tok(Token::True, 1, 1, 5));
    test_token("false", new_tok(Token::False, 1, 1, 6));

    test_token("try", new_tok(Token::Try, 1, 1, 4));
    test_token("catch", new_tok(Token::Catch, 1, 1, 6));
    test_token("finally", new_tok(Token::Finally, 1, 1, 8));

    test_token("class", new_tok(Token::Class, 1, 1, 6));
    test_token("constructor", new_tok(Token::Constructor, 1, 1, 12));
    test_token("internal", new_tok(Token::Internal, 1, 1, 9));
    test_token("private", new_tok(Token::Private, 1, 1, 8));
    test_token("protected", new_tok(Token::Protected, 1, 1, 10));
    test_token("public", new_tok(Token::Public, 1, 1, 7));
    test_token("readonly", new_tok(Token::Readonly, 1, 1, 9));

    test_token("interface", new_tok(Token::Interface, 1, 1, 10));
    test_token("enum", new_tok(Token::Enum, 1, 1, 5));

    test_token("string", new_tok(Token::String, 1, 1, 7));
    test_token("number", new_tok(Token::Number, 1, 1, 7));
    test_token("boolean", new_tok(Token::Boolean, 1, 1, 8));
    test_token("null", new_tok(Token::Null, 1, 1, 5));
    test_token("void", new_tok(Token::Void, 1, 1, 5));

    // Signs
    test_token(";", new_tok(Token::SemiColon, 1, 1, 2));
    test_token(":", new_tok(Token::Colon, 1, 1, 2));
    test_token(",", new_tok(Token::Comma, 1, 1, 2));
    test_token(".", new_tok(Token::Dot, 1, 1, 2));
    test_token("...", new_tok(Token::Ellipsis, 1, 1, 4));

    test_token("=", new_tok(Token::Equal, 1, 1, 2));
    test_token("==", new_tok(Token::DoubleEqual, 1, 1, 3));

    test_token("+", new_tok(Token::Plus, 1, 1, 2));
    test_token("+=", new_tok(Token::PlusEqual, 1, 1, 3));
    test_token("++", new_tok(Token::DoublePlus, 1, 1, 3));

    test_token("-", new_tok(Token::Minus, 1, 1, 2));
    test_token("-=", new_tok(Token::MinusEqual, 1, 1, 3));
    test_token("--", new_tok(Token::DoubleMinus, 1, 1, 3));

    test_token("*", new_tok(Token::Star, 1, 1, 2));
    test_token("*=", new_tok(Token::StarEqual, 1, 1, 3));
    test_token("*/", new_tok(Token::StarSlash, 1, 1, 3));

    test_token("**", new_tok(Token::DoubleStar, 1, 1, 3));
    test_token("**=", new_tok(Token::DoubleStarEqual, 1, 1, 4));

    test_token("/", new_tok(Token::Slash, 1, 1, 2));
    test_token("/=", new_tok(Token::SlashEqual, 1, 1, 3));

    test_token("%", new_tok(Token::Percent, 1, 1, 2));
    test_token("%=", new_tok(Token::PercentEqual, 1, 1, 3));

    test_token("<", new_tok(Token::Less, 1, 1, 2));
    test_token("<=", new_tok(Token::LessEqual, 1, 1, 3));

    test_token(">", new_tok(Token::Greater, 1, 1, 2));
    test_token(">=", new_tok(Token::GreaterEqual, 1, 1, 3));

    test_token("(", new_tok(Token::LeftParentheses, 1, 1, 2));
    test_token(")", new_tok(Token::RightParentheses, 1, 1, 2));

    test_token("{", new_tok(Token::LeftBrace, 1, 1, 2));
    test_token("}", new_tok(Token::RightBrace, 1, 1, 2));

    test_token("[", new_tok(Token::LeftBracket, 1, 1, 2));
    test_token("]", new_tok(Token::RightBracket, 1, 1, 2));

    test_token("@", new_tok(Token::At, 1, 1, 2));
    test_token("~", new_tok(Token::Tilde, 1, 1, 2));

    test_token("|", new_tok(Token::VBar, 1, 1, 2));
    test_token("||", new_tok(Token::DoubleVBar, 1, 1, 3));

    test_token("^", new_tok(Token::CircumFlex, 1, 1, 2));
    test_token("^=", new_tok(Token::CircumFlexEqual, 1, 1, 3));

    test_token("&", new_tok(Token::Amper, 1, 1, 2));
    test_token("&&", new_tok(Token::DoubleAmper, 1, 1, 3));

    test_token("!", new_tok(Token::Not, 1, 1, 2));
    test_token("!=", new_tok(Token::NotEqual, 1, 1, 3));

    test_token("\n", new_tok(Token::EndOfLine, 1, 1, 2));
    test_token("", new_tok(Token::EndOfFile, 1, 1, 1));
  }
}
