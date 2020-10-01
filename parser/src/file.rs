//
//  Copyright (c) Sflynlang
//
//  This source code is licensed under the MIT license found in the
//  LICENSE file in the root directory of this source tree.
//

#[derive(Debug, Clone, PartialEq)]
pub struct File {
  name: String,
  content: String,
}

impl File {
  /// Create a new file object.
  /// 
  /// ## Example
  /// ```rust
  /// use sflynlang_parser::File;
  /// 
  /// fn main() {
  ///   let file_name = "test.sf";
  ///   let file_content = "print('Hello world!');";
  /// 
  ///   let file = File::new(file_name.to_string(), file_content.to_string());
  /// 
  ///   println!("{:?}", file);
  /// }
  /// ```
  pub fn new(name: String, content: String) -> Self {
    Self { name, content }
  }

  /// Get the file name.
  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  /// Get the file content.
  pub fn get_content(&self) -> String {
    self.content.clone()
  }
}
