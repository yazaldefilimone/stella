#![allow(dead_code)]
use std::collections::BTreeMap;

use crate::types::Type;
use print::create_print_type;
pub mod io_type;
pub mod number;
mod print;
pub mod string;
pub mod table;

pub fn create_stdlib() -> BTreeMap<String, Type> {
  let mut stdlib = BTreeMap::new();
  // nill
  stdlib.insert("nil".to_string(), Type::Nil);
  stdlib.insert("print".to_string(), create_print_type());
  stdlib
}
