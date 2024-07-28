#![allow(dead_code)]
use super::tokens::Token;
use crate::{types::Type, utils::location::Location};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
  pub statements: Vec<Statement>,
}

impl Program {
  pub fn new() -> Program {
    Program { statements: Vec::new() }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Statement {
  Assign(AssignStatement),
  Function(FunctionStatement),
  Return(ReturnStatement),
  If(IfStatement),
  While(WhileStatement),
  Repeat(RepeatStatement),
  For(ForStatement),
  Break(BreakStatement),
  Goto(GotoStatement),
  Block(BlockStatement),
  Empty(EmptyStatement),
  VariableDeclaration(VariableDeclaration),
  Expression(Expression),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignStatement {
  pub name: Token,
  pub value: Expression,
  pub location: Location,
}

impl AssignStatement {
  pub fn new(name: Token, value: Expression, location: Location) -> Self {
    AssignStatement { name, value, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionStatement {
  pub name: Token,
  pub local: bool,
  pub arguments: Vec<(Token, Option<Type>)>,
  pub return_type: Option<Type>,
  pub body: Box<Statement>,
  pub location: Location,
}

impl FunctionStatement {
  pub fn new(
    name: Token,
    local: bool,
    arguments: Vec<(Token, Option<Type>)>,
    return_type: Option<Type>,
    body: Statement,
    location: Location,
  ) -> Self {
    FunctionStatement { name, local, arguments, return_type, body: Box::new(body), location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnStatement {
  pub values: Vec<Expression>,
  pub location: Location,
}

impl ReturnStatement {
  pub fn new(values: Vec<Expression>, location: Location) -> Self {
    ReturnStatement { values, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfStatement {
  pub condition: Expression,
  pub then_body: Box<Statement>,
  pub else_body: Option<Box<Statement>>,
  pub location: Location,
}

impl IfStatement {
  pub fn new(condition: Expression, then_body: Statement, else_body: Option<Statement>, location: Location) -> Self {
    IfStatement { condition, then_body: Box::new(then_body), else_body: else_body.map(Box::new), location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhileStatement {
  pub condition: Expression,
  pub body: Box<Statement>,
  pub location: Location,
}

impl WhileStatement {
  pub fn new(condition: Expression, body: Statement, location: Location) -> Self {
    WhileStatement { condition, body: Box::new(body), location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepeatStatement {
  pub body: Box<Statement>,
  pub condition: Expression,
  pub location: Location,
}

impl RepeatStatement {
  pub fn new(body: Statement, condition: Expression, location: Location) -> Self {
    RepeatStatement { body: Box::new(body), condition, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForStatement {
  pub variable: Expression,
  pub init: Expression,
  pub limit: Expression,
  pub step: Option<Expression>,
  pub body: Box<Statement>,
  pub location: Location,
}

impl ForStatement {
  pub fn new(
    variable: Expression,
    init: Expression,
    limit: Expression,
    step: Option<Expression>,
    body: Statement,
    location: Location,
  ) -> Self {
    ForStatement { variable, init, limit, step, body: Box::new(body), location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BreakStatement {
  pub location: Location,
}

impl BreakStatement {
  pub fn new(location: Location) -> Self {
    BreakStatement { location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GotoStatement {
  pub label: Option<String>,
  pub location: Location,
}

impl GotoStatement {
  pub fn new(label: Option<String>, location: Location) -> Self {
    GotoStatement { label, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockStatement {
  pub statements: Vec<Statement>,
  pub location: Location,
}

impl BlockStatement {
  pub fn new(statements: Vec<Statement>, location: Location) -> Self {
    BlockStatement { statements, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyStatement {}

impl EmptyStatement {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableDeclaration {
  pub name: Token,
  pub local: bool,
  pub var_type: Option<Type>,
  pub initializer: Option<Expression>,
  pub location: Location,
}

impl VariableDeclaration {
  pub fn new(
    name: Token,
    local: bool,
    var_type: Option<Type>,
    initializer: Option<Expression>,
    location: Location,
  ) -> Self {
    VariableDeclaration { name, local, var_type, initializer, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expression {
  Literal(LiteralExpression),
  Identifier(Identifier),
  Call(CallExpression),
  Unary(UnaryExpression),
  Grouped(GroupedExpression),
  Binary(BinaryExpression),
  Require(RequireExpression),
}

impl Expression {
  pub fn new_literal(value: LiteralExpression) -> Self {
    Expression::Literal(value)
  }

  pub fn new_identifier(name: String, location: Location) -> Self {
    Expression::Identifier(Identifier::new(name, location))
  }

  pub fn new_call(name: Token, args: Expression, location: Location) -> Self {
    Expression::Call(CallExpression::new(name, Box::new(args), location))
  }

  pub fn new_require(module_name: Token, location: Location) -> Self {
    Expression::Require(RequireExpression::new(module_name, location))
  }

  pub fn get_location(&self) -> Location {
    match self {
      Expression::Literal(literal) => literal.get_location(),
      Expression::Identifier(identifier) => identifier.location.clone(),
      Expression::Call(call) => call.location.clone(),
      Expression::Binary(binary) => binary.location.clone(),
      Expression::Require(require) => require.location.clone(),
      Expression::Grouped(grouped) => grouped.location.clone(),
      Expression::Unary(unary) => unary.get_location(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequireExpression {
  pub module_name: Token,
  pub location: Location,
}

impl RequireExpression {
  pub fn new(module_name: Token, location: Location) -> Self {
    RequireExpression { module_name, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupedExpression {
  pub expressions: Vec<Expression>,
  pub location: Location,
}

impl GroupedExpression {
  pub fn new(expressions: Vec<Expression>, location: Location) -> Self {
    GroupedExpression { expressions, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
  pub name: String,
  pub location: Location,
}

impl Identifier {
  pub fn new(name: String, location: Location) -> Self {
    Identifier { name, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallExpression {
  pub name: Token,
  pub args: Box<Expression>,
  pub location: Location,
}

impl CallExpression {
  pub fn new(name: Token, args: Box<Expression>, location: Location) -> Self {
    CallExpression { name, args, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnaryExpression {
  pub operator: UnaryOperator,
  pub operand: Box<Expression>,
}

impl UnaryExpression {
  pub fn new(operator: UnaryOperator, operand: Box<Expression>) -> Self {
    UnaryExpression { operator, operand }
  }

  pub fn get_location(&self) -> Location {
    self.operand.get_location()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryExpression {
  pub operator: BinaryOperator,
  pub left: Box<Expression>,
  pub right: Box<Expression>,
  pub location: Location,
}

impl BinaryExpression {
  pub fn new(operator: BinaryOperator, left: Box<Expression>, right: Box<Expression>, location: Location) -> Self {
    BinaryExpression { operator, left, right, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LiteralExpression {
  Number(NumberLiteral),
  String(StringLiteral),
  Boolean(BooleanLiteral),
  Nil,
}

impl LiteralExpression {
  pub fn new_number(value: String, location: Location) -> Self {
    LiteralExpression::Number(NumberLiteral::new(value, location))
  }

  pub fn new_string(value: String, location: Location) -> Self {
    LiteralExpression::String(StringLiteral::new(value, location))
  }

  pub fn new_bool(value: bool, location: Location) -> Self {
    LiteralExpression::Boolean(BooleanLiteral::new(value, location))
  }

  pub fn get_location(&self) -> Location {
    match self {
      LiteralExpression::Number(number) => number.location.clone(),
      LiteralExpression::String(string) => string.location.clone(),
      LiteralExpression::Boolean(boolean) => boolean.location.clone(),
      LiteralExpression::Nil => Location::new(),
    }
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
pub struct BooleanLiteral {
  pub value: bool,
  pub location: Location,
}

impl BooleanLiteral {
  pub fn new(value: bool, location: Location) -> Self {
    BooleanLiteral { value, location }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
  Negate,
  Not,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
  Add,                // +
  Subtract,           // -
  Multiply,           // *
  Divide,             // /
  Modulus,            // %
  And,                // and
  Or,                 // or
  Equal,              // ==
  NotEqual,           // ~=
  LessThan,           // <
  GreaterThan,        // >
  LessThanOrEqual,    // <=
  GreaterThanOrEqual, // >=
  DoubleDot,          // ..
}

impl BinaryOperator {
  pub fn to_string(&self) -> &str {
    match self {
      BinaryOperator::Add => "+",
      BinaryOperator::Subtract => "-",
      BinaryOperator::Multiply => "*",
      BinaryOperator::Divide => "/",
      BinaryOperator::Modulus => "%",
      BinaryOperator::And => "and",
      BinaryOperator::Or => "or",
      BinaryOperator::Equal => "==",
      BinaryOperator::NotEqual => "~=",
      BinaryOperator::LessThan => "<",
      BinaryOperator::GreaterThan => ">",
      BinaryOperator::LessThanOrEqual => "<=",
      BinaryOperator::GreaterThanOrEqual => ">=",
      BinaryOperator::DoubleDot => "..",
    }
  }
}
