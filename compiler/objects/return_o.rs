use super::{Objects, Object};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnO {
  pub value: Box<Objects>,
}

impl Object for ReturnO {
  fn string(self) -> String {
    self.value.string()
  }
}

impl ReturnO {
  pub fn new(value: Box<Objects>) -> Box<Objects> {
    Box::new(Objects::RETURN(ReturnO { value }))
  }
}
