#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
  Number,
  String,
  Boolean,
  Table(TableType),
  Function(FunctionType),
  Generic(GenericType),
  Union(UnionType),
  Optional(OptionalType),
  Unknown,
  Identifier(String),
  Void,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnionType {
  pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalType {
  pub type_: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableType {
  pub key_type: Box<Type>,
  pub value_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionType {
  pub params: Vec<Type>,
  pub return_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericType {
  pub name: String,
  pub types: Vec<Type>,
}

impl Type {
  pub fn check_match(&self, other: &Type) -> bool {
    match (self, other) {
      (Type::Number, Type::Number) => true,
      (Type::String, Type::String) => true,
      (Type::Boolean, Type::Boolean) => true,
      (Type::Unknown, Type::Unknown) => true,
      (Type::Void, Type::Void) => true,
      (Type::Optional(left), Type::Optional(right)) => left.type_.check_match(&right.type_),
      (Type::Union(left), Type::Union(right)) => check_match_union(&left.types, &right.types),
      (Type::Table(left), Type::Table(right)) => check_match_table(&left, &right),
      (Type::Function(left), Type::Function(right)) => check_match_function(&left, &right),
      (Type::Generic(left), Type::Generic(right)) => check_match_generic(&left, &right),
      (_, _) => false,
    }
  }
  pub fn check_is_can_replace(&self, replaced: &Type) -> bool {
    match (self, replaced) {
      (Type::Unknown, _) => true,
      _ => false,
    }
  }

  // to string
  pub fn to_string(&self) -> String {
    match self {
      Type::Void => "void".to_string(),
      Type::Number => "number".to_string(),
      Type::String => "string".to_string(),
      Type::Boolean => "boolean".to_string(),
      Type::Table(table) => format_table_type(table),
      Type::Function(function) => format_function_type(function),
      Type::Generic(generic) => format_generic_type(generic),
      Type::Union(union) => format_union_type(union),
      Type::Optional(optional) => format_optional_type(optional),
      Type::Unknown => "unknown".to_string(),
      Type::Identifier(name) => name.to_string(),
    }
  }

  // helper functions to create types
  pub fn new_type(name: String) -> Self {
    match name.as_str() {
      "number" => Type::Number,
      "string" => Type::String,
      "boolean" => Type::Boolean,
      "unknown" => Type::Unknown,
      _ => Type::Identifier(name),
    }
  }
  pub fn new_number() -> Self {
    Type::Number
  }

  pub fn new_string() -> Self {
    Type::String
  }

  pub fn new_boolean() -> Self {
    Type::Boolean
  }

  pub fn new_table(key_t: Type, value_t: Type) -> Self {
    Type::Table(TableType { key_type: Box::new(key_t), value_type: Box::new(value_t) })
  }

  pub fn new_function(params: Vec<Type>, t_return: Type) -> Self {
    Type::Function(FunctionType { params, return_type: Box::new(t_return) })
  }

  pub fn new_generic(name: String, tt: Vec<Type>) -> Self {
    return Type::Generic(GenericType { name, types: tt });
  }

  pub fn new_union(tt: Vec<Type>) -> Self {
    Type::Union(UnionType { types: tt })
  }

  pub fn new_optional(t: Type) -> Self {
    Type::Optional(OptionalType { type_: Box::new(t) })
  }
}

fn check_match_union(left: &Vec<Type>, right: &Vec<Type>) -> bool {
  if left.len() != right.len() {
    return false;
  }
  for (left_type, right_type) in left.iter().zip(right.iter()) {
    if !left_type.check_match(right_type) {
      return false;
    }
  }
  return true;
}

fn check_match_optional(left: &Type, right: &Type) -> bool {
  return left.check_match(right);
}

fn check_match_table(left: &TableType, right: &TableType) -> bool {
  if !left.key_type.check_match(&right.key_type) {
    return false;
  }
  return left.value_type.check_match(&right.value_type);
}

fn check_match_function(left: &FunctionType, right: &FunctionType) -> bool {
  if left.params.len() != right.params.len() {
    return false;
  }
  for (left_type, right_type) in left.params.iter().zip(right.params.iter()) {
    if !left_type.check_match(right_type) {
      return false;
    }
  }
  left.return_type.check_match(&right.return_type)
}

fn check_match_generic(left: &GenericType, right: &GenericType) -> bool {
  if left.name != right.name {
    return false;
  }
  if left.types.len() != right.types.len() {
    return false;
  }

  for (left_type, right_type) in left.types.iter().zip(right.types.iter()) {
    if !left_type.check_match(right_type) {
      return false;
    }
  }
  return true;
}

fn format_generic_type(generic: &GenericType) -> String {
  let tt = generic.types.iter().map(|t| t.to_string()).collect::<Vec<String>>();
  format!("{}<{}>", generic.name, tt.join(", "))
}

fn format_function_type(function: &FunctionType) -> String {
  let tt = function.params.iter().map(|t| t.to_string()).collect::<Vec<String>>();
  format!("function<{}>", format!("{}", tt.join(", ")))
}

fn format_table_type(table: &TableType) -> String {
  let key_str = table.key_type.to_string();
  let value_str = table.value_type.to_string();
  format!("table<{}, {}>", key_str, value_str)
}

fn format_union_type(union: &UnionType) -> String {
  let tt = union.types.iter().map(|t| t.to_string()).collect::<Vec<String>>();
  format!("union<{}>", tt.join(" | "))
}

fn format_optional_type(optional: &OptionalType) -> String {
  format!("optional<{}>", optional.type_.to_string())
}
