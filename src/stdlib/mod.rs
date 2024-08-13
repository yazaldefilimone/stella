#![allow(dead_code)]
use std::collections::BTreeMap;

use crate::types::Type;
use print::create_print_type;
mod io_type;
mod number;
mod print;
mod string;
mod table;

pub fn create_stdlib() -> BTreeMap<String, Type> {
  let mut stdlib_variables = BTreeMap::new();
  stdlib_variables.insert("nil".to_string(), Type::Nil);
  stdlib_variables.insert("print".to_string(), create_print_type());
  return stdlib_variables;
}
