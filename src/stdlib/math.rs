use std::collections::BTreeMap;

use crate::types::Type;

pub fn create_math_type() -> Type {
  let mut math_table = BTreeMap::new();
  math_table.insert("abs".to_string(), Type::new_function(vec![Type::Number], Type::Number));
  math_table.insert("acos".to_string(), Type::new_function(vec![Type::Number], Type::Number));
  math_table.insert("asin".to_string(), Type::new_function(vec![Type::Number], Type::Number));
  math_table.insert("atan".to_string(), Type::new_function(vec![Type::Number], Type::Number));
  math_table.insert("atan2".to_string(), Type::new_function(vec![Type::Number, Type::Number], Type::Number));
  math_table.insert("ceil".to_string(), Type::new_function(vec![Type::Number], Type::Number));
  math_table.insert("cos".to_string(), Type::new_function(vec![Type::Number], Type::Number));
  math_table.insert("cosh".to_string(), Type::new_function(vec![Type::Number], Type::Number));
  math_table.insert("deg".to_string(), Type::new_function(vec![Type::Number], Type::Number));
  math_table.insert("exp".to_string(), Type::new_function(vec![Type::Number], Type::Number));
  math_table.insert("floor".to_string(), Type::new_function(vec![Type::Number], Type::Number));
  Type::new_table(None, Some(math_table))
}
