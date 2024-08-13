use std::collections::BTreeMap;

use crate::types::Type;

pub fn create_io_type() -> Type {
  let mut io_table = BTreeMap::new();
  io_table.insert("read".to_string(), Type::new_function(vec![Type::String], Type::String));
  io_table.insert("write".to_string(), Type::new_function(vec![Type::String], Type::Nil));
  io_table.insert("flush".to_string(), Type::new_function(vec![], Type::Nil));
  io_table.insert("close".to_string(), Type::new_function(vec![], Type::Nil));
  Type::new_table(None, Some(io_table))
}
