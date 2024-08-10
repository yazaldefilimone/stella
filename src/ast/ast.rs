#![allow(dead_code)]
use super::tokens::Token;
use crate::{
  types::Type,
  utils::range::{create_middle_range, Range},
};
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
  TypeDeclaration(TypeDeclaration),
  Expression(Expression),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignStatement {
  pub values: Vec<(Token, Option<Type>)>,
  pub value: Expression,
}

impl AssignStatement {
  pub fn new(values: Vec<(Token, Option<Type>)>, value: Expression) -> Self {
    AssignStatement { values, value }
  }

  pub fn get_range(&self) -> Range {
    let left_range = self.values.first().unwrap().0.range.clone();
    let right_range = self.value.get_range();
    create_middle_range(&left_range, &right_range)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionStatement {
  pub name: Token,
  pub local: bool,
  pub arguments: Vec<(Token, Option<Type>)>,
  pub return_type: Option<Type>,
  pub generics: Vec<Type>,
  pub body: Box<Statement>,
  pub range: Range,
}

impl FunctionStatement {
  pub fn new(
    name: Token,
    local: bool,
    generics: Vec<Type>,
    arguments: Vec<(Token, Option<Type>)>,
    return_type: Option<Type>,
    body: Statement,
    range: Range,
  ) -> Self {
    FunctionStatement { name, local, generics, arguments, return_type, body: Box::new(body), range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnStatement {
  pub values: Vec<Expression>,
  pub range: Range,
}

impl ReturnStatement {
  pub fn new(values: Vec<Expression>, range: Range) -> Self {
    ReturnStatement { values, range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfStatement {
  pub condition: Expression,
  pub then_body: Box<Statement>,
  pub else_body: Option<Box<Statement>>,
  pub range: Range,
}

impl IfStatement {
  pub fn new(condition: Expression, then_body: Statement, else_body: Option<Statement>, range: Range) -> Self {
    IfStatement { condition, then_body: Box::new(then_body), else_body: else_body.map(Box::new), range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhileStatement {
  pub condition: Expression,
  pub body: Box<Statement>,
  pub range: Range,
}

impl WhileStatement {
  pub fn new(condition: Expression, body: Statement, range: Range) -> Self {
    WhileStatement { condition, body: Box::new(body), range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepeatStatement {
  pub body: Box<Statement>,
  pub condition: Expression,
  pub range: Range,
}

impl RepeatStatement {
  pub fn new(body: Statement, condition: Expression, range: Range) -> Self {
    RepeatStatement { body: Box::new(body), condition, range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForStatement {
  pub variable: Expression,
  pub init: Expression,
  pub limit: Expression,
  pub step: Option<Expression>,
  pub body: Box<Statement>,
  pub range: Range,
}

impl ForStatement {
  pub fn new(
    variable: Expression,
    init: Expression,
    limit: Expression,
    step: Option<Expression>,
    body: Statement,
    range: Range,
  ) -> Self {
    ForStatement { variable, init, limit, step, body: Box::new(body), range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BreakStatement {
  pub range: Range,
}

impl BreakStatement {
  pub fn new(range: Range) -> Self {
    BreakStatement { range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GotoStatement {
  pub label: Option<String>,
  pub range: Range,
}

impl GotoStatement {
  pub fn new(label: Option<String>, range: Range) -> Self {
    GotoStatement { label, range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockStatement {
  pub statements: Vec<Statement>,
  pub range: Range,
}

impl BlockStatement {
  pub fn new(statements: Vec<Statement>, range: Range) -> Self {
    BlockStatement { statements, range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyStatement {}

impl EmptyStatement {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TypeInitializer {
  Type(Type),
  Function(TypeFunction),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TypeDeclaration {
  pub name: Token,
  pub initiizer: Type,
  pub range: Range,
  pub generis: Vec<String>,
}

impl TypeDeclaration {
  pub fn new(name: Token, generis: Vec<String>, initiizer: Type, range: Range) -> Self {
    TypeDeclaration { name, generis, initiizer, range }
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TypeFunction {
  pub params: Vec<Type>,
  pub return_type: Type,
  pub range: Range,
}

impl TypeFunction {
  pub fn new(params: Vec<Type>, return_type: Type, range: Range) -> Self {
    TypeFunction { params, return_type, range }
  }

  pub fn get_range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableDeclaration {
  pub values: Vec<(Token, Option<Type>)>,
  pub local: bool,
  pub initializer: Option<Expression>,
}

impl VariableDeclaration {
  pub fn new(values: Vec<(Token, Option<Type>)>, local: bool, initializer: Option<Expression>) -> Self {
    VariableDeclaration { values, local, initializer }
  }

  pub fn get_range(&self) -> Range {
    let left_range = self.values.first().unwrap().0.range.clone();
    let right_range = self.initializer.as_ref().unwrap().get_range();
    create_middle_range(&left_range, &right_range)
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
  Function(FunctionExpression),
}

impl Expression {
  pub fn new_literal(value: LiteralExpression) -> Self {
    Expression::Literal(value)
  }

  pub fn new_identifier(name: String, range: Range) -> Self {
    Expression::Identifier(Identifier::new(name, range))
  }

  pub fn new_call(name: Token, args: Expression, range: Range) -> Self {
    Expression::Call(CallExpression::new(name, Box::new(args), range))
  }

  pub fn new_require(module_name: Token, range: Range) -> Self {
    Expression::Require(RequireExpression::new(module_name, range))
  }

  pub fn new_grouped(expressions: Vec<Expression>, range: Range) -> Self {
    Expression::Grouped(GroupedExpression::new(expressions, range))
  }

  pub fn new_function(
    arguments: Vec<(Token, Option<Type>)>,
    return_type: Option<Type>,
    body: Statement,
    loc: Range,
  ) -> Self {
    Expression::Function(FunctionExpression::new(arguments, return_type, body, loc))
  }

  pub fn get_range(&self) -> Range {
    match self {
      Expression::Literal(literal) => literal.get_range(),
      Expression::Identifier(identifier) => identifier.range.clone(),
      Expression::Call(call) => call.get_range(),
      Expression::Binary(binary) => binary.get_range(),
      Expression::Require(require) => require.get_range(),
      Expression::Grouped(grouped) => grouped.get_range(),
      Expression::Unary(unary) => unary.get_range(),
      Expression::Function(function) => function.get_range(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequireExpression {
  pub module_name: Token,
  pub range: Range,
}

impl RequireExpression {
  pub fn new(module_name: Token, range: Range) -> Self {
    RequireExpression { module_name, range }
  }

  pub fn get_range(&self) -> Range {
    let right_range = self.module_name.range.clone();
    let left_range = self.range.clone();
    create_middle_range(&left_range, &right_range)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupedExpression {
  pub expressions: Vec<Expression>,
  pub range: Range,
}

impl GroupedExpression {
  pub fn new(expressions: Vec<Expression>, range: Range) -> Self {
    GroupedExpression { expressions, range }
  }

  pub fn get_range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
  pub name: String,
  pub range: Range,
}

impl Identifier {
  pub fn new(name: String, range: Range) -> Self {
    Identifier { name, range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallExpression {
  pub name: Token,
  pub args: Box<Expression>,
  pub range: Range,
}

impl CallExpression {
  pub fn new(name: Token, args: Box<Expression>, range: Range) -> Self {
    CallExpression { name, args, range }
  }

  pub fn get_range(&self) -> Range {
    let left_range = self.name.range.clone();
    let right_range = self.args.get_range();
    create_middle_range(&left_range, &right_range)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnaryExpression {
  pub range: Range,
  pub operator: UnaryOperator,
  pub operand: Box<Expression>,
}

impl UnaryExpression {
  pub fn new(operator: UnaryOperator, operand: Box<Expression>, range: Range) -> Self {
    UnaryExpression { operator, operand, range }
  }

  pub fn get_range(&self) -> Range {
    let left_range = self.operand.get_range();
    let right_range = self.get_operator_range();
    create_middle_range(&left_range, &right_range)
  }

  pub fn get_operator_range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionExpression {
  pub arguments: Vec<(Token, Option<Type>)>,
  pub return_type: Option<Type>,
  pub body: Box<Statement>,
  pub range: Range,
}

impl FunctionExpression {
  pub fn new(arguments: Vec<(Token, Option<Type>)>, return_type: Option<Type>, body: Statement, range: Range) -> Self {
    FunctionExpression { arguments, return_type, body: Box::new(body), range }
  }

  pub fn get_range(&self) -> Range {
    let left_range = self.range.clone();
    return left_range;
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryExpression {
  pub operator: BinaryOperator,
  pub left: Box<Expression>,
  pub right: Box<Expression>,
  pub range: Range,
}

impl BinaryExpression {
  pub fn new(operator: BinaryOperator, left: Box<Expression>, right: Box<Expression>, range: Range) -> Self {
    BinaryExpression { operator, left, right, range }
  }
  pub fn get_range(&self) -> Range {
    let left_range = self.left.get_range();
    let right_range = self.right.get_range();
    create_middle_range(&left_range, &right_range)
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
  pub fn new_number(value: String, range: Range) -> Self {
    LiteralExpression::Number(NumberLiteral::new(value, range))
  }

  pub fn new_string(value: String, range: Range) -> Self {
    LiteralExpression::String(StringLiteral::new(value, range))
  }

  pub fn new_bool(value: bool, range: Range) -> Self {
    LiteralExpression::Boolean(BooleanLiteral::new(value, range))
  }

  pub fn get_range(&self) -> Range {
    match self {
      LiteralExpression::Number(number) => number.range.clone(),
      LiteralExpression::String(string) => string.range.clone(),
      LiteralExpression::Boolean(boolean) => boolean.range.clone(),
      LiteralExpression::Nil => Range::new(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NumberLiteral {
  pub value: String,
  pub range: Range,
}

impl NumberLiteral {
  pub fn new(value: String, range: Range) -> Self {
    NumberLiteral { value, range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringLiteral {
  pub value: String,
  pub range: Range,
}

impl StringLiteral {
  pub fn new(value: String, range: Range) -> Self {
    StringLiteral { value, range }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanLiteral {
  pub value: bool,
  pub range: Range,
}

impl BooleanLiteral {
  pub fn new(value: bool, range: Range) -> Self {
    BooleanLiteral { value, range }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
  Negate,
  Not,
  Hash,
}

impl UnaryOperator {
  pub fn to_str(&self) -> &str {
    match self {
      UnaryOperator::Negate => "-",
      UnaryOperator::Not => "not",
      UnaryOperator::Hash => "#",
    }
  }
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
  DoubleSlash,        // //
}

pub struct TypeExpression {
  pub name: Type,
  pub range: Range,
}

impl BinaryOperator {
  pub fn to_str(&self) -> &str {
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
      BinaryOperator::DoubleSlash => "//",
    }
  }
}
