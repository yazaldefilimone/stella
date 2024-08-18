#![allow(dead_code)]
use std::collections::BTreeMap;

use crate::types::Type;
mod io_type;
mod math;
mod number;
mod print;
mod string;
mod table;

pub fn create_stdlib() -> BTreeMap<String, Type> {
  let mut stdlib_variables = BTreeMap::new();
  stdlib_variables.insert("nil".to_string(), Type::Nil);
  stdlib_variables.insert("print".to_string(), print::create_print_type());
  stdlib_variables.insert("io".to_string(), io_type::create_io_type());
  stdlib_variables.insert("math".to_string(), math::create_math_type());
  return stdlib_variables;
}
