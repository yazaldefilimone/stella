#![allow(dead_code)]
use super::tokens::Token;
use crate::{types::Type, utils::location::Location};
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
  GotoStatement(GotoStatement),
  BlockStatement(BlockStatement),
  EmptyStatement(EmptyStatement),
  VariableDeclaration(VariableDeclaration),
  CallStatement(CallStatement),
  ExpressionStatement(Expression),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallStatement {
  pub expression: Expression,
}

impl CallStatement {
  pub fn new(expression: Expression) -> Self {
    CallStatement { expression }
  }
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
  pub arguments: Vec<(Token, Option<Type>)>,
  pub return_t: Option<Type>,
  pub body: Box<Statement>,
  pub location: Location,
}

impl FunctionStatement {
  pub fn new(
    name: Token,
    arguments: Vec<(Token, Option<Type>)>,
    return_t: Option<Type>,
    body: Statement,
    location: Location,
  ) -> Self {
    FunctionStatement { name, arguments, return_t, body: Box::new(body), location }
  }
}
#[derive(Debug, Serialize, Deserialize)]
// return 10 // return "Hello", 20
// return function(a, b) return a + b end -- return a function
pub struct ReturnStatement {
  pub value: Vec<Expression>,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfStatement {
  pub condition: Expression,
  pub body: Box<Statement>,
  pub else_body: Option<Box<Statement>>,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhileStatement {
  pub condition: Expression,
  pub body: Box<Statement>,
  pub location: Location,
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
// for i = 1, 10, 2 do ... end
pub struct ForStatement {
  pub variable: Expression,
  pub init: Expression,
  pub limit: Expression,
  pub step: Option<Expression>,
  pub body: Box<Statement>,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BreakStatement {
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GotoStatement {
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
pub struct VariableDeclaration {
  pub name: Token,
  pub local: bool,
  pub t: Option<Type>,
  pub init: Option<Expression>,
  pub location: Location,
}

impl VariableDeclaration {
  pub fn new(name: Token, local: bool, t: Option<Type>, init: Option<Expression>, location: Location) -> Self {
    VariableDeclaration { name, local, t, init, location }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expression {
  LiteralExpression(LiteralExpression),
  Identifier(Identifier),
  CallExpression(CallExpression),
  UnaryExpression(UnaryExpression),
  GroupedExpression(GroupedExpression),
  BinaryExpression(BinaryExpression),
  RequireExpression(RequireExpression),
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
  pub expression: Vec<Expression>,
  pub location: Location,
}

impl GroupedExpression {
  pub fn new(expression: Vec<Expression>, location: Location) -> Self {
    GroupedExpression { expression, location }
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

impl Expression {
  pub fn new_number_literal(value: String, location: Location) -> Self {
    let literal = LiteralExpression::new_number_literal(value, location);
    return Expression::LiteralExpression(literal);
  }

  pub fn new_identifier(name: String, location: Location) -> Self {
    let literal = Identifier::new(name, location);
    return Expression::Identifier(literal);
  }

  pub fn new_string_literal(value: String, location: Location) -> Self {
    let literal = LiteralExpression::new_string_literal(value, location);
    return Expression::LiteralExpression(literal);
  }

  pub fn new_bool_literal(value: bool, location: Location) -> Self {
    let literal = LiteralExpression::new_bool_literal(value, location);
    return Expression::LiteralExpression(literal);
  }

  pub fn new_call_expression(name: Token, args: Expression, location: Location) -> Self {
    let call_expression = CallExpression::new(name, Box::new(args), location);
    return Expression::CallExpression(call_expression);
  }

  pub fn new_require_expression(module_name: Token, location: Location) -> Self {
    let require_expression = RequireExpression::new(module_name, location);
    return Expression::RequireExpression(require_expression);
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
  NumberLiteral(NumberLiteral),
  StringLiteral(StringLiteral),
  BoolLiteral(BoolLiteral),
  NilLiteral,
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
  pub fn to_string(&self) -> String {
    let text = match self {
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
    };
    return text.to_string();
  }
}
