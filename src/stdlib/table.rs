#![allow(dead_code)]
use std::collections::BTreeMap;

use crate::types::Type;

pub fn create_table_type() -> Type {
  let mut table_type = BTreeMap::new();
  // todo: impove and add more functions
  // let table_tt_unkown =Type::new_table(None, None) ;
  let insert_param_table = vec![Type::Unknown, Type::Unknown];
  table_type.insert("insert".to_string(), Type::new_function(insert_param_table.clone(), Type::Nil));
  table_type.insert("remove".to_string(), Type::new_function(insert_param_table, Type::Nil));
  Type::new_table(None, Some(table_type))
}
