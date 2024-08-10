#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};

use crate::{ast::ast::BinaryOperator, utils::range::Range};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
  Number,                       // number e.g. 10
  String,                       // string e.g. "hello"
  Boolean,                      // boolean e.g. true
  Table(TableType),             // table<number, string>  e.g. {1, "one"}
  Function(FunctionType),       // (left: number, right: number): number  e.g. function(a, b) return a + b end
  Generic(GenericType),         // type Either<T, U> = {left: T, right: U}
  GenericCall(GenericCallType), // fn<number, string> = function(a, b) return a + b end
  Union(UnionType),             // number | string or union<number, string>  e.g. 1 | "one"
  Optional(OptionalType),       // optional<number>  e.g. number | nil
  Unknown,                      // unknown
  Identifier(IdentifierType),   // type
  Nil,                          // nil
  Grup(GrupType),               // return type
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifierType {
  pub name: String,
  pub range: Range,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericCallType {
  pub name: String,
  pub types: Vec<Type>,
  pub range: Range,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrupType {
  pub types: Vec<Type>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SingleType {
  pub return_type: Box<Type>,
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
  pub variables: Vec<String>,
  pub value: Box<Type>,
  pub range: Range,
}
impl GenericType {
  pub fn new(name: String, variables: Vec<String>, value: Type, range: Range) -> Self {
    GenericType { name, variables, value: Box::new(value), range }
  }
}

impl Type {
  pub fn supports_operator(&self, operator: &BinaryOperator) -> bool {
    match (self, operator) {
      (Type::Number, BinaryOperator::Add)
      | (Type::Number, BinaryOperator::Subtract)
      | (Type::Number, BinaryOperator::Multiply)
      | (Type::Number, BinaryOperator::DoubleSlash)
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
      // support any operator for unknown type
      | (Type::Unknown, _)
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
      (Type::Number, Type::Number, BinaryOperator::DoubleSlash) => Type::Number,
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
      (Type::Grup(left), Type::Grup(right)) => check_match_grup_return_type(&left, &right),
      (Type::Grup(left), right) => check_match_grup_return_with_single_type(&left, right),
      (left, Type::Grup(right)) => check_match_grup_return_with_single_type(&right, left),
      _ => false,
    }
  }

  pub fn can_replace(&self, replaced: &Type) -> bool {
    match (self, replaced) {
      (Type::Unknown, _) => true,
      (Type::Nil, _) => true,
      _ => false,
    }
  }

  pub fn replace_generic(&self, replaced: &Type) -> Type {
    match (self, replaced) {
      (Type::Identifier(identifier), _) => {
        return replaced.clone();
      }
      _ => self.clone(),
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Type::Number => "number".to_string(),
      Type::String => "string".to_string(),
      Type::Boolean => "boolean".to_string(),
      Type::Nil => "nil".to_string(),
      Type::Unknown => "unknown".to_string(),
      Type::Table(table) => format_table_type(table),
      Type::Function(function) => format_function_type(function),
      Type::Generic(generic) => format_generic_type(generic),
      Type::Union(union) => format_union_type(union),
      Type::Optional(optional) => format_optional_type(optional),
      Type::Identifier(identifier) => format_identifier_type(identifier),
      Type::Grup(grup_return) => format_grup_return_type(grup_return),
      Type::GenericCall(generic_call) => format_generic_call_type(generic_call),
    }
  }
  pub fn is_grup(&self) -> bool {
    match self {
      Type::Grup(_) => true,
      _ => false,
    }
  }

  pub fn same_grup_length(&self, other: &Type) -> bool {
    match (self, other) {
      (Type::Grup(left), Type::Grup(right)) => left.types.len() == right.types.len(),
      _ => false,
    }
  }

  // Helper functions to create instances of various types.
  pub fn new_type(name: &str, range: Range) -> Self {
    match name {
      "number" => Type::Number,
      "string" => Type::String,
      "boolean" => Type::Boolean,
      "nil" => Type::Nil,
      "unknown" => Type::Unknown,
      _ => Type::new_identifier(name, range),
    }
  }

  pub fn new_identifier(name: &str, range: Range) -> Self {
    Type::Identifier(IdentifierType { name: name.to_owned(), range })
  }
  pub fn new_generic(name: &str, variables: Vec<String>, value: Type, range: Range) -> Self {
    Type::Generic(GenericType { name: name.to_owned(), variables, value: Box::new(value), range })
  }

  pub fn eccept_generic(name: &str) -> bool {
    match name {
      "number" | "string" | "boolean" | "nil" | "unknown" => false,
      _ => true,
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

  pub fn new_function(params: Vec<Type>, return_type: Type) -> Type {
    Type::Function(FunctionType { params, return_type: Box::new(return_type) })
  }

  pub fn new_union(types: Vec<Type>) -> Self {
    Type::Union(UnionType { types })
  }

  pub fn new_optional(inner_type: Type) -> Self {
    Type::Optional(OptionalType { inner_type: Box::new(inner_type) })
  }

  pub fn new_grup(types: Vec<Type>) -> Self {
    Type::Grup(GrupType { types })
  }

  pub fn new_generic_call(name: String, types: Vec<Type>, range: Range) -> Self {
    Type::GenericCall(GenericCallType { name, types, range })
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
  if !left.params.iter().zip(&right.params).all(|(l, r)| l.check_match(r)) {
    return false;
  }
  return left.return_type.check_match(&*right.return_type);
}

fn check_match_grup_return_type(left: &GrupType, right: &GrupType) -> bool {
  left.types.iter().zip(&right.types).all(|(l, r)| l.check_match(r))
}

fn check_match_grup_return_with_single_type(left: &GrupType, right: &Type) -> bool {
  if left.types.len() != 1 {
    return false;
  }
  left.types[0].check_match(right)
}

fn check_match_generic(left: &GenericType, right: &GenericType) -> bool {
  left.name == right.name
    && left.variables.len() == right.variables.len()
    && left.variables.iter().zip(&right.variables).all(|(l, r)| l == r)
    && left.value.check_match(&right.value)
}

fn format_generic_type(generic: &GenericType) -> String {
  let types_str = generic.variables.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ");
  if types_str.is_empty() {
    format!("{}", generic.name)
  } else {
    format!("{}<{}>", generic.name, types_str)
  }
}

fn format_function_type(function: &FunctionType) -> String {
  let params_str = function.params.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ");
  format!("function({}): {}", params_str, function.return_type.to_string())
}

fn format_table_type(table: &TableType) -> String {
  format!("table<{}, {}>", table.key_type.to_string(), table.value_type.to_string())
}

fn format_union_type(union: &UnionType) -> String {
  let types_str = union.types.iter().map(Type::to_string).collect::<Vec<String>>().join(", ");
  format!("union<{}>", types_str)
}

fn format_optional_type(optional: &OptionalType) -> String {
  format!("optional<{}>", optional.inner_type.to_string())
}
fn format_identifier_type(identifier: &IdentifierType) -> String {
  return format!("{}", identifier.name);
}

fn format_grup_return_type(grup_return: &GrupType) -> String {
  if grup_return.types.len() == 1 {
    return grup_return.types[0].to_string();
  }
  let types_str = grup_return.types.iter().map(Type::to_string).collect::<Vec<String>>().join(", ");
  format!("({})", types_str)
}

fn format_generic_call_type(generic_call: &GenericCallType) -> String {
  let types_str = generic_call.types.iter().map(Type::to_string).collect::<Vec<String>>().join(", ");
  format!("{}<{}>", generic_call.name, types_str)
}

pub fn can_replace_grup_return_type(replaced: &GrupType, replaced_type: &GrupType) -> bool {
  if replaced.types.len() != replaced_type.types.len() {
    return false;
  }
  replaced.types.iter().zip(&replaced_type.types).all(|(l, r)| l.can_replace(r))
}

pub fn replace_type(replaced: &Type, replaced_type: &Type) -> Type {
  match (replaced, replaced_type) {
    (Type::Grup(replaced), Type::Grup(replaced_type)) => {
      let replaced_type = replace_grup_return_type(replaced, replaced_type);
      Type::Grup(replaced_type)
    }
    _ => {
      if replaced.can_replace(replaced_type) {
        replaced_type.clone()
      } else {
        replaced.clone()
      }
    }
  }
}

pub fn replace_grup_return_type(replaced: &GrupType, replaced_type: &GrupType) -> GrupType {
  let mut types = replaced.types.clone();
  for (replaced_type, replaced_type_type) in replaced.types.iter().zip(replaced_type.types.iter()) {
    if replaced_type.can_replace(replaced_type_type) {
      types.push(replaced_type_type.clone());
    }
  }
  GrupType { types }
}
