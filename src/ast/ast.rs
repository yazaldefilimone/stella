#![allow(dead_code)]
use super::tokens::Token;
use crate::utils::location::Location;
#[allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
  pub statements: Vec<Statement>,
}

impl Program {
  pub fn new() -> Program {
    Program { statements: vec![] }
  }
}

#[derive(Debug, Serialize, Deserialize)]
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
  LocalStatement(LocalStatement),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignStatement {
  pub name: String,
  pub value: ExpressionStatement,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionStatement {
  pub name: Token,
  pub arguments: Vec<(Token, Type)>,
  pub return_type: Type,
  pub body: Box<Statement>,
  pub location: Location,
}

impl FunctionStatement {
  pub fn new(
    name: Token,
    arguments: Vec<(Token, Type)>,
    return_type: Type,
    body: Statement,
    location: Location,
  ) -> Self {
    FunctionStatement { name, arguments, return_type, body: Box::new(body), location }
  }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnStatement {
  pub value: ExpressionStatement,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfStatement {
  pub condition: ExpressionStatement,
  pub body: Vec<Statement>,
  pub else_body: Option<Vec<Statement>>,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhileStatement {
  pub condition: ExpressionStatement,
  pub body: Vec<Statement>,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepeatStatement {
  pub body: Vec<Statement>,
  pub condition: ExpressionStatement,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForStatement {
  pub initializer: Option<ExpressionStatement>,
  pub condition: Option<ExpressionStatement>,
  pub increment: Option<ExpressionStatement>,
  pub body: Vec<Statement>,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BreakStatement {
  pub label: Option<String>,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueStatement {
  pub label: Option<String>,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockStatement {
  pub body: Vec<Statement>,
  pub location: Location,
}
impl BlockStatement {
  pub fn new(body: Vec<Statement>, location: Location) -> Self {
    BlockStatement { body, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyStatement {
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalStatement {
  pub name: Token,
  pub type_: Option<Type>,
  pub init: ExpressionStatement,
  pub location: Location,
}

impl LocalStatement {
  pub fn new(name: Token, type_: Option<Type>, init: ExpressionStatement, location: Location) -> Self {
    LocalStatement { name, type_, init, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExpressionStatement {
  LiteralExpression(LiteralExpression),
  CallExpression(CallExpression),
  UnaryExpression(UnaryExpression),
  BinaryExpression(BinaryExpression),
  DeclarationExpression(DeclarationExpression),
}

impl ExpressionStatement {
  pub fn new_number_literal(value: String, location: Location) -> Self {
    let literal = LiteralExpression::new_number_literal(value, location);
    return ExpressionStatement::LiteralExpression(literal);
  }

  pub fn new_string_literal(value: String, location: Location) -> Self {
    let literal = LiteralExpression::new_string_literal(value, location);
    return ExpressionStatement::LiteralExpression(literal);
  }

  pub fn new_bool_literal(value: bool, location: Location) -> Self {
    let literal = LiteralExpression::new_bool_literal(value, location);
    return ExpressionStatement::LiteralExpression(literal);
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationExpression {
  pub name: Token,
  pub value: Box<ExpressionStatement>,
  pub local: bool,
  pub location: Location,
}

impl DeclarationExpression {
  pub fn new(name: Token, value: Box<ExpressionStatement>, local: bool, location: Location) -> Self {
    DeclarationExpression { name, value, local, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallExpression {
  pub name: Token,
  pub args: Vec<ExpressionStatement>,
  pub location: Location,
}

impl CallExpression {
  pub fn new(name: Token, args: Vec<ExpressionStatement>, location: Location) -> Self {
    CallExpression { name, args, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnaryExpression {
  pub operator: UnaryOperator,
  pub operand: Box<ExpressionStatement>,
  pub location: Location,
}

impl UnaryExpression {
  pub fn new(operator: UnaryOperator, operand: Box<ExpressionStatement>, location: Location) -> Self {
    UnaryExpression { operator, operand, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryExpression {
  pub operator: BinaryOperator,
  pub left: Box<ExpressionStatement>,
  pub right: Box<ExpressionStatement>,
  pub location: Location,
}

impl BinaryExpression {
  pub fn new(
    operator: BinaryOperator,
    left: Box<ExpressionStatement>,
    right: Box<ExpressionStatement>,
    location: Location,
  ) -> Self {
    BinaryExpression { operator, left, right, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LiteralExpression {
  NumberLiteral(NumberLiteral),
  StringLiteral(StringLiteral),
  BoolLiteral(BoolLiteral),
}

impl LiteralExpression {
  pub fn new_number_literal(value: String, location: Location) -> Self {
    LiteralExpression::NumberLiteral(NumberLiteral::new(value, location))
  }

  pub fn new_string_literal(value: String, location: Location) -> Self {
    LiteralExpression::StringLiteral(StringLiteral::new(value, location))
  }

  pub fn new_bool_literal(value: bool, location: Location) -> Self {
    LiteralExpression::BoolLiteral(BoolLiteral::new(value, location))
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NumberLiteral {
  pub value: String,
  pub location: Location,
}

impl NumberLiteral {
  pub fn new(value: String, location: Location) -> Self {
    NumberLiteral { value, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringLiteral {
  pub value: String,
  pub location: Location,
}

impl StringLiteral {
  pub fn new(value: String, location: Location) -> Self {
    StringLiteral { value, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoolLiteral {
  pub value: bool,
  pub location: Location,
}

impl BoolLiteral {
  pub fn new(value: bool, location: Location) -> Self {
    BoolLiteral { value, location }
  }
}

// Definição de Tipos

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
  Void,
  Number,
  String,
  Bool,
  Identifier(String),
}

impl Type {
  pub fn identifier(name: String) -> Self {
    Type::Identifier(name)
  }

  pub fn new_type(text: String) -> Self {
    match text.as_str() {
      "void" => Type::Void,
      "number" => Type::Number,
      "string" => Type::String,
      "bool" => Type::Bool,
      _ => Type::Identifier(text),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
  Negate,
  Not,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
