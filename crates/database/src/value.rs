use std::fmt::{
  Display,
  Formatter
};

pub enum Value {
  String(String),
  Number(i64),
  Float(f64),
}

impl Value {
  pub fn to_query(&self) -> String {
    format!("{}", self)
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}