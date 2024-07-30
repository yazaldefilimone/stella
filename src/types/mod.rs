#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};

use crate::ast::ast::BinaryOperator;

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
  Nil,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnionType {
  pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalType {
  pub inner_type: Box<Type>,
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
  pub fn supports_operator(&self, operator: &BinaryOperator) -> bool {
    match (self, operator) {
      (Type::Number, BinaryOperator::Add)
      | (Type::Number, BinaryOperator::Subtract)
      | (Type::Number, BinaryOperator::Multiply)
      | (Type::Number, BinaryOperator::Divide)
      | (Type::Number, BinaryOperator::Modulus)
      | (Type::Number, BinaryOperator::And)
      | (Type::Number, BinaryOperator::Or)
      | (Type::Number, BinaryOperator::Equal)
      | (Type::Number, BinaryOperator::NotEqual)
      | (Type::Number, BinaryOperator::LessThan)
      | (Type::Number, BinaryOperator::GreaterThan)
      | (Type::Number, BinaryOperator::LessThanOrEqual)
      | (Type::Number, BinaryOperator::GreaterThanOrEqual)
      | (Type::Number, BinaryOperator::DoubleDot)
      | (Type::String, BinaryOperator::Add)
      | (Type::String, BinaryOperator::Equal)
      | (Type::String, BinaryOperator::NotEqual)
      | (Type::String, BinaryOperator::LessThan)
      | (Type::String, BinaryOperator::GreaterThan)
      | (Type::String, BinaryOperator::LessThanOrEqual)
      | (Type::String, BinaryOperator::GreaterThanOrEqual)
      | (Type::String, BinaryOperator::DoubleDot)
      | (Type::Boolean, BinaryOperator::And)
      | (Type::Boolean, BinaryOperator::Or)
      | (Type::Boolean, BinaryOperator::Equal)
      | (Type::Boolean, BinaryOperator::NotEqual)
      | (Type::Nil, BinaryOperator::Equal)
      | (Type::Nil, BinaryOperator::NotEqual) => true,
      _ => false,
    }
  }

  pub fn get_operator_result_type(&self, other: &Type, operator: &BinaryOperator) -> Type {
    match (self, other, operator) {
      (Type::Number, Type::Number, BinaryOperator::Equal)
      | (Type::Number, Type::Number, BinaryOperator::NotEqual)
      | (Type::Number, Type::Number, BinaryOperator::LessThan)
      | (Type::Number, Type::Number, BinaryOperator::GreaterThan)
      | (Type::Number, Type::Number, BinaryOperator::LessThanOrEqual)
      | (Type::Number, Type::Number, BinaryOperator::GreaterThanOrEqual)
      | (Type::String, Type::String, BinaryOperator::Equal)
      | (Type::String, Type::String, BinaryOperator::NotEqual)
      | (Type::String, Type::String, BinaryOperator::LessThan)
      | (Type::String, Type::String, BinaryOperator::GreaterThan)
      | (Type::Boolean, Type::Boolean, BinaryOperator::Equal)
      | (Type::Boolean, Type::Boolean, BinaryOperator::NotEqual)
      | (Type::Nil, Type::Nil, _) => Type::Boolean,

      (Type::Number, Type::Number, _) => Type::Number,
      (Type::String, Type::String, _) => Type::String,
      (Type::Number, Type::String, BinaryOperator::DoubleDot)
      | (Type::String, Type::Number, BinaryOperator::DoubleDot) => Type::String,

      _ => Type::Unknown,
    }
  }

  pub fn check_match(&self, other: &Type) -> bool {
    match (self, other) {
      (Type::Number, Type::Number)
      | (Type::String, Type::String)
      | (Type::Boolean, Type::Boolean)
      | (Type::Nil, Type::Nil)
      | (Type::Unknown, Type::Unknown) => true,

      (Type::Optional(left), Type::Optional(right)) => left.inner_type.check_match(&right.inner_type),
      (Type::Union(left), Type::Union(right)) => check_match_union(&left.types, &right.types),
      (Type::Table(left), Type::Table(right)) => check_match_table(left, right),
      (Type::Function(left), Type::Function(right)) => check_match_function(left, right),
      (Type::Generic(left), Type::Generic(right)) => check_match_generic(left, right),
      (Type::Unknown, _) | (_, Type::Unknown) => true,
      _ => false,
    }
  }

  pub fn can_replace(&self, replaced: &Type) -> bool {
    match (self, replaced) {
      (Type::Unknown, _) => true,
      _ => false,
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Type::Number => "number".to_string(),
      Type::String => "string".to_string(),
      Type::Boolean => "boolean".to_string(),
      Type::Table(table) => format_table_type(table),
      Type::Function(function) => format_function_type(function),
      Type::Generic(generic) => format_generic_type(generic),
      Type::Union(union) => format_union_type(union),
      Type::Optional(optional) => format_optional_type(optional),
      Type::Unknown => "unknown".to_string(),
      Type::Identifier(name) => name.clone(),
      Type::Nil => "nil".to_string(),
    }
  }

  // Helper functions to create instances of various types.
  pub fn new_type(name: &str) -> Self {
    match name {
      "number" => Type::Number,
      "string" => Type::String,
      "boolean" => Type::Boolean,
      "nil" => Type::Nil,
      "unknown" => Type::Unknown,
      _ => Type::Identifier(name.to_owned()),
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

  pub fn new_table(key_type: Type, value_type: Type) -> Self {
    Type::Table(TableType { key_type: Box::new(key_type), value_type: Box::new(value_type) })
  }

  pub fn new_function(params: Vec<Type>, return_type: Type) -> Self {
    Type::Function(FunctionType { params, return_type: Box::new(return_type) })
  }

  pub fn new_generic(name: String, types: Vec<Type>) -> Self {
    Type::Generic(GenericType { name, types })
  }

  pub fn new_union(types: Vec<Type>) -> Self {
    Type::Union(UnionType { types })
  }

  pub fn new_optional(inner_type: Type) -> Self {
    Type::Optional(OptionalType { inner_type: Box::new(inner_type) })
  }
}

fn check_match_union(left: &Vec<Type>, right: &Vec<Type>) -> bool {
  left.len() == right.len() && left.iter().zip(right).all(|(left_type, right_type)| left_type.check_match(right_type))
}

fn check_match_table(left: &TableType, right: &TableType) -> bool {
  left.key_type.check_match(&right.key_type) && left.value_type.check_match(&right.value_type)
}

fn check_match_function(left: &FunctionType, right: &FunctionType) -> bool {
  if left.params.len() != right.params.len() {
    return false;
  }
  left.params.iter().zip(&right.params).all(|(l, r)| l.check_match(r))
    && left.return_type.check_match(&right.return_type)
}

fn check_match_generic(left: &GenericType, right: &GenericType) -> bool {
  left.name == right.name
    && left.types.len() == right.types.len()
    && left.types.iter().zip(&right.types).all(|(l, r)| l.check_match(r))
}

fn format_generic_type(generic: &GenericType) -> String {
  let types_str = generic.types.iter().map(Type::to_string).collect::<Vec<String>>().join(", ");
  format!("{}<{}>", generic.name, types_str)
}

fn format_function_type(function: &FunctionType) -> String {
  let params_str = function.params.iter().map(Type::to_string).collect::<Vec<String>>().join(", ");
  format!("function<({}) -> {}>", params_str, function.return_type.to_string())
}

fn format_table_type(table: &TableType) -> String {
  format!(
    "table<{}, {}>",
    table.key_type.to_string(),
    table.value_type.to_string()
  )
}

fn format_union_type(union: &UnionType) -> String {
  let types_str = union.types.iter().map(Type::to_string).collect::<Vec<String>>().join(" | ");
  format!("union<{}>", types_str)
}

fn format_optional_type(optional: &OptionalType) -> String {
  format!("optional<{}>", optional.inner_type.to_string())
}
