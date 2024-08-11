#![allow(dead_code, unused_variables)]

use std::collections::BTreeMap;

use crate::{ast::ast::BinaryOperator, utils::range::Range};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
  Number,                       // e.g. 10
  String,                       // e.g. "hello"
  Boolean,                      // e.g. true
  Table(TableType),             // e.g. {1, "one"}
  Function(FunctionType),       // e.g. function(a, b) return a + b end
  Generic(GenericType),         // e.g. type Either<T, U> = {left: T, right: U}
  GenericCall(GenericCallType), // e.g. fn<number, string> = function(a, b) return a + b end
  Union(UnionType),             // e.g. number | string
  Optional(OptionalType),       // e.g. number | nil
  Unknown,                      // unknown
  Identifier(IdentifierType),   // Identificador de tipos
  Nil,                          // nil
  Grup(GrupType),               // Grupo de tipos
}

impl Type {
  pub fn is_unknown(&self) -> bool {
    matches!(self, Type::Unknown)
  }
  pub fn is_nil(&self) -> bool {
    matches!(self, Type::Nil)
  }

  pub fn is_string(&self) -> bool {
    matches!(self, Type::String)
  }

  pub fn is_number(&self) -> bool {
    matches!(self, Type::Number)
  }

  pub fn is_boolean(&self) -> bool {
    matches!(self, Type::Boolean)
  }
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
pub struct UnionType {
  pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalType {
  pub inner_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableType {
  pub array: Option<Vec<Type>>,
  pub map: Option<BTreeMap<String, Type>>,
}

impl TableType {
  pub fn new(array: Option<Vec<Type>>, map: Option<BTreeMap<String, Type>>) -> Self {
    TableType { array, map }
  }

  pub fn new_array(array: Vec<Type>) -> Self {
    TableType { array: Some(array), map: None }
  }

  pub fn new_map(map: BTreeMap<String, Type>) -> Self {
    TableType { array: None, map: Some(map) }
  }

  pub fn is_array(&self) -> bool {
    self.array.is_some()
  }

  pub fn is_map(&self) -> bool {
    self.map.is_some()
  }

  pub fn get_type(&self, key: &str) -> Option<&Type> {
    if let Some(map) = &self.map {
      map.get(key)
    } else {
      None
    }
  }
  pub fn get_array_len(&self) -> Option<usize> {
    self.array.as_ref().map(|array| array.len())
  }

  pub fn get_array_type(&self, index: usize) -> Option<&Type> {
    self.array.as_ref().and_then(|array| array.get(index))
  }

  pub fn get_array(&self) -> Option<&Vec<Type>> {
    self.array.as_ref()
  }

  pub fn get_map(&self) -> Option<&BTreeMap<String, Type>> {
    self.map.as_ref()
  }

  pub fn to_string(&self) -> String {
    let mut map_str = String::new();
    if let Some(map) = &self.map {
      map_str = format!(
        "<{}>",
        map.iter().map(|(k, v)| format!("{}: {}", k, v.to_string())).collect::<Vec<String>>().join(", ")
      );
    }

    if let Some(array) = &self.array {
      map_str = format!("<{}>", array.iter().map(Type::to_string).collect::<Vec<String>>().join(", "));
    }

    format!(
      "table{}",
      format!(
        "{}{}",
        map_str,
        self
          .array
          .as_ref()
          .map(|array| format!("<{}>", array.iter().map(Type::to_string).collect::<Vec<String>>().join(", ")))
          .unwrap_or(String::new())
      )
    )
  }
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
    use BinaryOperator::*;
    matches!(
      (self, operator),
      (Type::Number, Add)
        | (Type::Number, Subtract)
        | (Type::Number, Multiply)
        | (Type::Number, DoubleSlash)
        | (Type::Number, Divide)
        | (Type::Number, Modulus)
        | (Type::Number, And)
        | (Type::Number, Or)
        | (Type::Number, Equal)
        | (Type::Number, NotEqual)
        | (Type::Number, LessThan)
        | (Type::Number, GreaterThan)
        | (Type::Number, LessThanOrEqual)
        | (Type::Number, GreaterThanOrEqual)
        | (Type::Number, DoubleDot)
        | (Type::String, Add)
        | (Type::String, Equal)
        | (Type::String, NotEqual)
        | (Type::String, LessThan)
        | (Type::String, GreaterThan)
        | (Type::String, LessThanOrEqual)
        | (Type::String, GreaterThanOrEqual)
        | (Type::String, DoubleDot)
        | (Type::Boolean, And)
        | (Type::Boolean, Or)
        | (Type::Boolean, Equal)
        | (Type::Boolean, NotEqual)
        | (Type::Nil, Equal)
        | (Type::Nil, NotEqual)
        | (Type::Unknown, _)
    )
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
      | (Type::Unknown, Type::Unknown)
      | (Type::Unknown, _)
      | (_, Type::Unknown) => true,
      (Type::Optional(left), Type::Optional(right)) => left.inner_type.check_match(&right.inner_type),
      (Type::Union(left), Type::Union(right)) => check_match_union(&left.types, &right.types),
      (Type::Table(left), Type::Table(right)) => check_match_table(left, right),
      (Type::Function(left), Type::Function(right)) => check_match_function(left, right),
      (Type::Generic(left), Type::Generic(right)) => check_match_generic(left, right),
      (Type::Grup(left), Type::Grup(right)) => check_match_grup_return_type(left, right),
      (Type::Grup(left), right) => check_match_grup_return_with_single_type(left, right),
      (left, Type::Grup(right)) => check_match_grup_return_with_single_type(right, left),
      _ => false,
    }
  }

  pub fn can_replace(&self, replaced: &Type) -> bool {
    matches!(self, Type::Unknown)
  }

  pub fn replace_generic(&self, replaced: &Type) -> Type {
    if let Type::Identifier(_) = self {
      replaced.clone()
    } else {
      self.clone()
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
    matches!(self, Type::Grup(_))
  }

  pub fn same_grup_length(&self, other: &Type) -> bool {
    if let (Type::Grup(left), Type::Grup(right)) = (self, other) {
      left.types.len() == right.types.len()
    } else {
      false
    }
  }

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
    Type::Generic(GenericType::new(name.to_owned(), variables, value, range))
  }

  pub fn accept_generic(name: &str) -> bool {
    !matches!(name, "number" | "string" | "boolean" | "nil" | "unknown")
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

  pub fn new_table(array: Option<Vec<Type>>, map: Option<BTreeMap<String, Type>>) -> Self {
    Type::Table(TableType { array, map })
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
  if let (Some(left_array), Some(right_array)) = (&left.array, &right.array) {
    left_array.len() == right_array.len() && left_array.iter().zip(right_array).all(|(l, r)| l.check_match(r))
  } else if let (Some(left_map), Some(right_map)) = (&left.map, &right.map) {
    left_map.len() == right_map.len() && left_map.iter().zip(right_map).all(|(l, r)| l.1.check_match(&r.1))
  } else {
    false
  }
}

fn check_match_function(left: &FunctionType, right: &FunctionType) -> bool {
  left.params.len() == right.params.len()
    && left.params.iter().zip(&right.params).all(|(l, r)| l.check_match(r))
    && left.return_type.check_match(&*right.return_type)
}

fn check_match_grup_return_type(left: &GrupType, right: &GrupType) -> bool {
  left.types.iter().zip(&right.types).all(|(l, r)| l.check_match(r))
}

fn check_match_grup_return_with_single_type(left: &GrupType, right: &Type) -> bool {
  left.types.len() == 1 && left.types[0].check_match(right)
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
  let mut array_str = String::new();
  let mut map_str = String::new();

  if let Some(array) = &table.array {
    array_str = format!("<{}>", array.iter().map(Type::to_string).collect::<Vec<String>>().join(", "));
  }

  if let Some(map) = &table.map {
    map_str =
      format!("<{}>", map.iter().map(|(k, v)| format!("{}: {}", k, v.to_string())).collect::<Vec<String>>().join(", "));
  }

  format!("table{}", format!("{}{}", array_str, map_str))
}

fn format_union_type(union: &UnionType) -> String {
  let types_str = union.types.iter().map(Type::to_string).collect::<Vec<String>>().join(", ");
  format!("union<{}>", types_str)
}

fn format_optional_type(optional: &OptionalType) -> String {
  format!("optional<{}>", optional.inner_type.to_string())
}

fn format_identifier_type(identifier: &IdentifierType) -> String {
  identifier.name.clone()
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
  replaced.types.len() == replaced_type.types.len()
    && replaced.types.iter().zip(&replaced_type.types).all(|(l, r)| l.can_replace(r))
}

pub fn replace_type(replaced: &Type, replaced_type: &Type) -> Type {
  match (replaced, replaced_type) {
    (Type::Grup(replaced), Type::Grup(replaced_type)) => Type::Grup(replace_grup_return_type(replaced, replaced_type)),
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
  let mut types = Vec::with_capacity(replaced.types.len());
  for (replaced_type, replaced_type_type) in replaced.types.iter().zip(replaced_type.types.iter()) {
    if replaced_type.can_replace(replaced_type_type) {
      types.push(replaced_type_type.clone());
    }
  }
  GrupType { types }
}
