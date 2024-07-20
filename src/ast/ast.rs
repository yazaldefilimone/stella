#![allow(dead_code)]
use crate::utils::location::Location;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ast {
  pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
  AssignStatement(AssignStatement),
  FunctionStatement(FunctionStatement),
  ReturnStatement(ReturnStatement),
  IfStatement(IfStatement),
  WhileStatement(WhileStatement),
  RepeatStatement(RepeatStatement),
  ForStatement(ForStatement),
  BreakStatement(BreakStatement),
  ContinueStatement(ContinueStatement),
  BlockStatement(BlockStatement),
  EmptyStatement(EmptyStatement),
}

// Definição de Declarações e Expressões
#[derive(Debug)]
pub struct AssignStatement {
  pub name: String,
  pub value: ExpressionStatement,
  pub location: Location,
}

#[derive(Debug)]
pub struct FunctionStatement {
  pub name: String,
  pub arguments: Vec<(String, Type)>,
  pub return_type: Type,
  pub body: Vec<Statement>,
  pub location: Location,
}

#[derive(Debug)]
pub struct ReturnStatement {
  pub value: ExpressionStatement,
  pub location: Location,
}

#[derive(Debug)]
pub struct IfStatement {
  pub condition: ExpressionStatement,
  pub body: Vec<Statement>,
  pub else_body: Option<Vec<Statement>>,
  pub location: Location,
}

#[derive(Debug)]
pub struct WhileStatement {
  pub condition: ExpressionStatement,
  pub body: Vec<Statement>,
  pub location: Location,
}

#[derive(Debug)]
pub struct RepeatStatement {
  pub body: Vec<Statement>,
  pub condition: ExpressionStatement,
  pub location: Location,
}

#[derive(Debug)]
pub struct ForStatement {
  pub initializer: Option<ExpressionStatement>,
  pub condition: Option<ExpressionStatement>,
  pub increment: Option<ExpressionStatement>,
  pub body: Vec<Statement>,
  pub location: Location,
}

#[derive(Debug)]
pub struct BreakStatement {
  pub label: Option<String>,
  pub location: Location,
}

#[derive(Debug)]
pub struct ContinueStatement {
  pub label: Option<String>,
  pub location: Location,
}

#[derive(Debug)]
pub struct BlockStatement {
  pub body: Vec<Statement>,
  pub location: Location,
}

#[derive(Debug)]
pub struct EmptyStatement {
  pub location: Location,
}

#[derive(Debug)]
pub enum ExpressionStatement {
  LiteralExpression(LiteralExpression),
  CallExpression(CallExpression),
  UnaryExpression(UnaryExpression),
  BinaryExpression(BinaryExpression),
  DeclarationExpression(DeclarationExpression),
}

#[derive(Debug)]
pub struct DeclarationExpression {
  pub name: String,
  pub value: Box<ExpressionStatement>,
  pub local: bool,
  pub location: Location,
}

#[derive(Debug)]
pub struct CallExpression {
  pub name: String,
  pub args: Vec<ExpressionStatement>,
  pub location: Location,
}

#[derive(Debug)]
pub struct UnaryExpression {
  pub operator: UnaryOperator,
  pub operand: Box<ExpressionStatement>,
  pub location: Location,
}

#[derive(Debug)]
pub struct BinaryExpression {
  pub operator: BinaryOperator,
  pub left: Box<ExpressionStatement>,
  pub right: Box<ExpressionStatement>,
  pub location: Location,
}

#[derive(Debug)]
pub enum LiteralExpression {
  NumberLiteral(NumberLiteral),
  StringLiteral(StringLiteral),
  BoolLiteral(BoolLiteral),
}

#[derive(Debug)]
pub struct NumberLiteral {
  pub value: f64,
  pub location: Location,
}

#[derive(Debug)]
pub struct StringLiteral {
  pub value: String,
  pub location: Location,
}

#[derive(Debug)]
pub struct BoolLiteral {
  pub value: bool,
  pub location: Location,
}

// Definição de Tipos
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  Void,
  Number,
  String,
  Bool,
  FunctionType(FunctionType),
  UnionType(UnionType),
  TupleType(TupleType),
  ArrayType(ArrayType),
  RecordType(RecordType),
  EnumType(EnumType),
  GenericType(GenericType),
  LiteralType(LiteralType),
  OptionalType(Box<Type>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
  pub arguments_types: Vec<Type>,
  pub return_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionType {
  pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupleType {
  pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
  pub element_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordType {
  pub fields: HashMap<String, Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumType {
  pub variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericType {
  pub name: String,
  pub bounds: Vec<Type>,
  pub variance: Variance,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variance {
  Covariant,
  Contravariant,
  Invariant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
  Number,
  String,
  Bool,
}

// Definição de Operadores Unários e Binários
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
  Negate,
  Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulus,
  And,
  Or,
  Equal,
  NotEqual,
  LessThan,
  GreaterThan,
  LessThanOrEqual,
  GreaterThanOrEqual,
}
