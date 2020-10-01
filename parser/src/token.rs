
use crate::Point;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  // General
  Identifier { value: String },
  Comment { value: String },
  Num { value: String },
  Str { value: String },

  // Keywords
  Let,
  Const,

  Async,
  Await,

  Func,
  Return,

  Is,

  If,
  Else,
  Switch,
  Case,
  Break,

  Do,
  While,
  Continue,

  For,
  In,
  Of,

  Import,
  From,
  Export,
  As,

  True,
  False,

  Try,
  Catch,
  Finally,

  Class,
  Constructor,
  Internal,
  Private,
  Protected,
  Public,
  Readonly,

  Interface,
  Enum,

  String,
  Number,
  Boolean,
  Null,
  Void,

  // Signs
  SemiColon, // ;
  Colon, // :
  Comma, // ,
  Dot, // .
  Ellipsis, // ...

  Equal, // =
  DoubleEqual, // ==

  Plus, // +
  PlusEqual, // +=
  DoublePlus, // ++

  Minus, // -
  MinusEqual, // -=
  DoubleMinus, // --

  Star, // *
  StarEqual, // *=
  StarSlash, // */

  DoubleStar, // **
  DoubleStarEqual, // **=

  Slash, // /
  SlashStar, // /*
  SlashEqual, // /=
  DoubleSlash, // //

  Percent, // %
  PercentEqual, // %=

  Less, // <
  LessEqual, // <=

  Greater, // >
  GreaterEqual, // >=

  LeftParentheses, // (
  RightParentheses, // )

  LeftBrace, // {
  RightBrace, // }

  LeftBracket, // [
  RightBracket, // ]

  At, // @
  Tilde, // ~

  VBar, // |
  DoubleVBar, // ||

  CircumFlex, // ^
  CircumFlexEqual, // ^=

  Amper, // &
  DoubleAmper, // &&

  Not, // !
  NotEqual, // !=

  // Other
  Illegal,
  EndOfLine,
  EndOfFile,
}

#[derive(Debug, PartialEq)]
pub struct Tok {
  start_point: Point,
  token: Token,
  end_point: Point,
}

impl Tok {
  /// Create a new tok object with the point
  /// and the token.
  /// 
  /// ## Example
  /// ```rust
  /// use sflynlang_parser::Point;
  /// use sflynlang_parser::Tok;
  /// use sflynlang_parser::Token;
  /// 
  /// fn main() {
  ///   let start_point = Point::new(1, 1);
  ///   let token = Token::Let;
  ///   let end_point = Point::new(1, 3);
  /// 
  ///   let tok = Tok::new(start_point, token, end_point);
  /// 
  ///   println!("{:?}", tok);
  /// }
  /// ```
  pub fn new(start_point: Point, token: Token, end_point: Point) -> Self {
    Self { start_point, token, end_point }
  }

  /// Get the start point object.
  pub fn get_start_point(&self) -> Point {
    self.start_point
  }

  /// Get the token object.
  pub fn get_token(&self) -> Token {
    self.token.clone()
  }

  /// Get the end point object.
  pub fn get_end_point(&self) -> Point {
    self.end_point
  }
}
