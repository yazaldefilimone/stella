use crate::{
  ast::ast::{BinaryOperator, UnaryOperator},
  utils::range::Range,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
mod match_type;
use match_type::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
  Number,
  String,
  Boolean,
  Alias(AliasType),
  Table(TableType),
  Function(FunctionType),
  Generic(GenericType),
  GenericCall(GenericCallType),
  Union(UnionType),
  Option(OptionType),
  Unknown,
  Nil,
  Group(GroupType),
  Variadic(VariadicType),
}

impl Hash for Type {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Type::Number => state.write_u8(0),
      Type::String => state.write_u8(1),
      Type::Boolean => state.write_u8(2),
      Type::Unknown => state.write_u8(3),
      Type::Nil => state.write_u8(4),
      Type::Table(table) => table.hash(state),
      Type::Function(function) => function.hash(state),
      Type::Generic(generic) => generic.hash(state),
      Type::GenericCall(generic_call) => generic_call.hash(state),
      Type::Union(union) => union.hash(state),
      Type::Option(option) => option.hash(state),
      Type::Alias(identifier) => identifier.hash(state),
      Type::Group(group) => group.hash(state),
      Type::Variadic(variadic) => variadic.hash(state),
    }
  }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableType {
  pub array: Option<HashSet<Type>>,
  // todo: hash map or btree map?
  pub map: Option<BTreeMap<String, Type>>,
}

impl Hash for TableType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    if let Some(array) = &self.array {
      state.write_u8(10);
      for element in array {
        element.hash(state);
      }
    }
    if let Some(map) = &self.map {
      state.write_u8(20);
      for (key, value) in map {
        key.hash(state);
        value.hash(state);
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasType {
  pub name: String,
  pub range: Range,
}

impl Hash for AliasType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state);
  }
}

impl Type {
  pub fn new(name: &str, range: Range) -> Self {
    match name {
      "boolean" => Type::Boolean,
      "number" => Type::Number,
      "string" => Type::String,
      "nil" => Type::Nil,
      "unknown" => Type::Unknown,
      _ => Type::Alias(AliasType { name: name.to_string(), range }),
    }
  }
  pub fn is_nil(&self) -> bool {
    match self {
      Type::Nil => true,
      Type::Group(group) => group.types.len() == 1 && group.types[0].is_nil(),
      _ => false,
    }
  }

  pub fn is_group(&self) -> bool {
    matches!(self, Type::Group(_))
  }
  pub fn is_variadic(&self) -> bool {
    matches!(self, Type::Variadic(_))
  }

  pub fn new_variadic(inner_type: Type) -> Self {
    Type::Variadic(VariadicType { inner_type: Box::new(inner_type) })
  }
  pub fn new_group(types: Vec<Type>) -> Self {
    Type::Group(GroupType { types })
  }
  pub fn new_table(array: Option<HashSet<Type>>, map: Option<BTreeMap<String, Type>>) -> Self {
    Type::Table(TableType { array, map })
  }
  pub fn new_function(params: Vec<Type>, return_type: Type) -> Self {
    Type::Function(FunctionType { params, return_type: Box::new(return_type) })
  }
  pub fn new_union(types: Vec<Type>) -> Self {
    Type::Union(UnionType { types })
  }
  pub fn new_option(inner_type: Type) -> Self {
    Type::Option(OptionType { inner_type: Box::new(inner_type) })
  }
  pub fn new_generic(name: &str, variables: Vec<String>, value: Type, range: Range) -> Self {
    Type::Generic(GenericType::new(name.to_string(), variables, value, range))
  }
  pub fn new_generic_call(name: String, types: Vec<Type>, range: Range) -> Self {
    Type::GenericCall(GenericCallType { name, types, range })
  }

  pub fn new_boolean() -> Self {
    Type::Boolean
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
      // table
      (Type::Table(left), Type::Table(right)) => check_match_table(left, right),

      // function
      (Type::Function(left), Type::Function(right)) => check_match_function(left, right),

      // generic
      (Type::Generic(left), Type::Generic(right)) => check_match_generic(left, right),

      // group
      (Type::Group(left), Type::Group(right)) => check_match_group(left, right),
      (Type::Group(left), right) => check_match_group_with_single_type(left, right),
      (left, Type::Group(right)) => check_match_group_with_single_type(right, left),

      // Option
      (Type::Option(left), Type::Option(right)) => check_match_option(left, right),
      (Type::Option(left), right) => check_match_option_right(left, right),
      (_, Type::Option(_)) => false,

      // union
      (Type::Union(left), Type::Union(right)) => check_match_union(&left.types, &right.types),
      (Type::Union(left), right) => check_match_union_with_single_type(left, right),
      (left, Type::Union(right)) => check_match_union_with_single_type(right, left),

      // variadic
      (Type::Variadic(left), Type::Variadic(right)) => check_match_variadic(left, right),
      (Type::Variadic(left), right) => check_match_variadic_with_single_type(left, right),
      (left, Type::Variadic(right)) => check_match_variadic_with_single_type(right, left),
      _ => false,
    }
  }

  pub fn supports_operator(&self, operator: &BinaryOperator) -> bool {
    if matches!(self, Type::Number) {
      return operator.support_number();
    }
    if matches!(self, Type::String) {
      return operator.support_string();
    }
    if matches!(self, Type::Boolean) {
      return operator.support_boolean();
    }
    if matches!(self, Type::Nil) {
      return operator.support_nil();
    }
    return matches!(self, Type::Unknown) || supports_stdlib_operator(self, operator);
  }

  pub fn suport_unary_operator(&self, operator: &UnaryOperator) -> bool {
    if matches!(self, Type::Number) {
      return operator.support_number();
    }
    if matches!(self, Type::String) {
      return operator.support_string();
    }
    if matches!(self, Type::Boolean) {
      return operator.support_boolean();
    }
    if matches!(self, Type::Nil) {
      return operator.support_nil();
    }
    return matches!(self, Type::Unknown) || supports_stdlib_unary_operator(self, operator);
  }

  pub fn get_unary_operator_result_type(&self, operator: &UnaryOperator) -> Type {
    use UnaryOperator::*;
    match (self, operator) {
      (Type::Number, Negate) => Type::Number,
      (Type::Table(_) | Type::String, Hash) => Type::Number,
      (_, Not) => Type::Boolean,
      (Type::Unknown, _) => Type::Unknown,
      _ => unreachable!("{} right: {:#?}", operator, self),
    }
  }

  pub fn get_operator_result_type(&self, other: &Type, operator: &BinaryOperator) -> Type {
    use BinaryOperator::*;
    let result = get_operator_stdlib_result_type(self, operator);
    if result.is_some() {
      return result.unwrap();
    }
    // format!("operator: {:#?}", operator).as_str();

    match operator {
      // string concat
      DoubleDot => match (self, other) {
        (Type::String, Type::String)
        | (Type::Number, Type::String)
        | (Type::String, Type::Number)
        | (Type::Unknown, Type::String)
        | (Type::String, Type::Unknown)
        | (Type::Unknown, Type::Unknown) => Type::String,
        _ => unreachable!("left: {:#?} {} right: {:#?}", self, operator, other),
      },
      // math operators
      Add | Subtract | Multiply | Divide | Modulus | DoubleSlash => match (self, other) {
        (Type::Number, Type::Number)
        | (Type::Unknown, Type::Number)
        | (Type::Number, Type::Unknown)
        | (Type::Unknown, Type::Unknown) => Type::Number,
        _ => unreachable!("left: {:#?} {} right: {:#?}", self, operator, other),
      },
      // comparison operators
      // println!("comparison operators");
      Equal | NotEqual | LessThan | GreaterThan | LessThanOrEqual | GreaterThanOrEqual => match (self, other) {
        (Type::Number, Type::Number)
        | (Type::Unknown, Type::Number)
        | (Type::Number, Type::Unknown)
        | (Type::String, Type::String)
        | (Type::Unknown, Type::String)
        | (Type::String, Type::Unknown)
        | (Type::Boolean, Type::Boolean)
        | (Type::Unknown, Type::Boolean)
        | (Type::Boolean, Type::Unknown)
        | (Type::Unknown, Type::Unknown) => Type::Boolean,
        _ => unreachable!("left: {:#?} {} right: {:#?}", self, operator, other),
      },
      //  logical operators
      And | Or => match (self, other) {
        (Type::Boolean, Type::Boolean)
        | (Type::Unknown, Type::Boolean)
        | (Type::Boolean, Type::Unknown)
        | (Type::Unknown, Type::Unknown) => Type::Boolean,
        _ => unreachable!("left: {:#?} {} right: {:#?}", self, operator, other),
      },
    }
  }

  pub fn can_replace(&self, replaced: &Type) -> bool {
    matches!(self, Type::Unknown) || !matches!(replaced, Type::Unknown)
  }

  pub fn same_group_length(&self, other: &Type) -> bool {
    if let (Type::Group(left), Type::Group(right)) = (self, other) {
      left.types.len() == right.types.len()
    } else {
      false
    }
  }
}

fn supports_stdlib_operator(left: &Type, operator: &BinaryOperator) -> bool {
  matches!(left, Type::Option(_) | Type::Union(_))
    && matches!(operator, BinaryOperator::Equal | BinaryOperator::NotEqual | BinaryOperator::DoubleDot)
}

fn supports_stdlib_unary_operator(left: &Type, operator: &UnaryOperator) -> bool {
  // tables
  if matches!(left, Type::Table(_)) {
    return matches!(operator, UnaryOperator::Not | UnaryOperator::Hash);
  }

  if matches!(left, Type::Option(_)) {
    return matches!(operator, UnaryOperator::Not);
  }

  return false;
}

fn get_operator_stdlib_result_type(left: &Type, operator: &BinaryOperator) -> Option<Type> {
  if matches!(left, Type::Union(_)) {
    if matches!(operator, BinaryOperator::Equal | BinaryOperator::NotEqual) {
      return Some(Type::Boolean);
    }
    if matches!(operator, BinaryOperator::DoubleDot) {
      return Some(Type::String);
    }
  }

  if matches!(left, Type::Union(_)) {
    if matches!(operator, BinaryOperator::Equal | BinaryOperator::NotEqual) {
      return Some(Type::Boolean);
    }
    if matches!(operator, BinaryOperator::DoubleDot) {
      return Some(Type::String);
    }
  }

  if matches!(left, Type::Table(_)) {
    if matches!(operator, BinaryOperator::Equal | BinaryOperator::NotEqual) {
      return Some(Type::Boolean);
    }

    if matches!(operator, BinaryOperator::DoubleDot) {
      return Some(Type::String);
    }
  }

  return None;
}

impl TableType {
  pub fn get_type(&self, key: &str) -> Option<&Type> {
    self.map.as_ref()?.get(key)
  }
}

impl GenericType {
  pub fn new(name: String, variables: Vec<String>, value: Type, range: Range) -> Self {
    GenericType { name, variables, value: Box::new(value), range }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifierType {
  pub name: String,
  pub range: Range,
}

impl Hash for IdentifierType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state);
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericCallType {
  pub name: String,
  pub types: Vec<Type>,
  pub range: Range,
}

impl Hash for GenericCallType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state);
    for type_ in &self.types {
      type_.hash(state);
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupType {
  pub types: Vec<Type>,
}

impl Hash for GroupType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_u8(30);
    for type_ in &self.types {
      type_.hash(state);
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariadicType {
  pub inner_type: Box<Type>,
}
impl Hash for VariadicType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_u8(40);
    self.inner_type.hash(state);
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnionType {
  pub types: Vec<Type>,
}

impl Hash for UnionType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_u8(50);
    for type_ in &self.types {
      type_.hash(state);
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionType {
  pub inner_type: Box<Type>,
}
impl Hash for OptionType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_u8(60);
    self.inner_type.hash(state);
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionType {
  pub params: Vec<Type>,
  pub return_type: Box<Type>,
}

impl Hash for FunctionType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_u8(70);
    for type_ in &self.params {
      type_.hash(state);
    }
    self.return_type.hash(state);
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericType {
  pub name: String,
  pub variables: Vec<String>,
  pub value: Box<Type>,
  pub range: Range,
}

impl Hash for GenericType {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_u8(80);
    self.name.hash(state);
    for variable in &self.variables {
      variable.hash(state);
    }
    self.value.hash(state);
  }
}
