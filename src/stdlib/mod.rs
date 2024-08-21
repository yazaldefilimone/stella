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
  // stdlib_variables.insert("number".to_string(), number::create_number_type());
  // stdlib_variables.insert("string".to_string(), string::create_string_type());
  stdlib_variables.insert("table".to_string(), table::create_table_type());
  return stdlib_variables;
}
