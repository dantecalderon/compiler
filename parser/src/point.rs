use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
  line: usize,
  column: usize,
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Ln {}, Col {}", self.line, self.column)
  }
}

impl Point {
  /// Create a new point object with the line and
  /// column inside the file content.
  /// 
  /// ## Example
  /// ```rust
  /// use sflynlang_parser::Point;
  /// 
  /// fn main() {
  ///   // New point in the line 1 and column 1.
  ///   let point = Point::new(1, 1);
  /// 
  ///   println!("{}", point);
  /// }
  /// ```
  pub fn new(line: usize, column: usize) -> Self {
    Self { line, column }
  }

  /// Get the point line.
  pub fn get_line(&self) -> usize {
    self.line
  }

  /// Get the point column.
  pub fn get_column(&self) -> usize {
    self.column
  }
}
