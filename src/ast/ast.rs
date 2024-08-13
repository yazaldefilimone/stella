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

impl Statement {
  pub fn get_range(&self) -> Range {
    match self {
      Statement::Assign(assign) => assign.get_range(),
      Statement::Function(function) => function.get_range(),
      Statement::Return(return_) => return_.get_range(),
      Statement::If(if_) => if_.get_range(),
      Statement::While(while_) => while_.get_range(),
      Statement::Repeat(repeat) => repeat.get_range(),
      Statement::For(for_) => for_.get_range(),
      Statement::Break(break_) => break_.get_range(),
      Statement::Goto(goto) => goto.get_range(),
      Statement::Block(block) => block.get_range(),
      Statement::Empty(empty) => empty.get_range(),
      Statement::VariableDeclaration(variable) => variable.get_range(),
      Statement::TypeDeclaration(declaration) => declaration.get_range(),
      Statement::Expression(expression) => expression.get_range(),
    }
  }
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
  pub range_return_type: Option<Range>,
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
    range_return_type: Option<Range>,
  ) -> Self {
    FunctionStatement { name, local, generics, arguments, return_type, body: Box::new(body), range, range_return_type }
  }

  pub fn get_range(&self) -> Range {
    return self.range.clone();
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
  pub fn get_range(&self) -> Range {
    self.range.clone()
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

  pub fn get_range(&self) -> Range {
    self.range.clone()
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
  pub fn get_range(&self) -> Range {
    self.range.clone()
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

  pub fn get_range(&self) -> Range {
    self.range.clone()
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

  pub fn get_range(&self) -> Range {
    self.range.clone()
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
  pub fn get_range(&self) -> Range {
    self.range.clone()
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

  pub fn get_range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockStatement {
  pub statements: Vec<Statement>,
}

impl BlockStatement {
  pub fn new(statements: Vec<Statement>) -> Self {
    BlockStatement { statements }
  }
  pub fn get_range(&self) -> Range {
    if self.statements.len() == 0 {
      return Range::new();
    }
    let first_statement = self.statements.first().unwrap();
    let last_statement = self.statements.last().unwrap();
    create_middle_range(&first_statement.get_range(), &last_statement.get_range())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyStatement {}

impl EmptyStatement {
  pub fn get_range(&self) -> Range {
    Range::new()
  }
}

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
  pub fn get_range(&self) -> Range {
    // todo: check if it's correct
    create_middle_range(&self.range, &self.name.range)
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
    if let Some(initializer) = &self.initializer {
      let right_range = initializer.get_range();
      create_middle_range(&left_range, &right_range)
    } else {
      left_range
    }
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
  Table(TableExpression),
  Member(MemberExpression),
  Index(IndexExpression),
}

impl Expression {
  pub fn new_literal(value: LiteralExpression) -> Self {
    Expression::Literal(value)
  }

  pub fn new_identifier(name: String, range: Range) -> Self {
    Expression::Identifier(Identifier::new(name, range))
  }

  pub fn new_call(name: Token, args: Expression) -> Self {
    Expression::Call(CallExpression::new(name, Box::new(args)))
  }

  pub fn new_require(module_name: Token, range: Range) -> Self {
    Expression::Require(RequireExpression::new(module_name, range))
  }

  pub fn new_grouped(expressions: Vec<Expression>, range: Range) -> Self {
    Expression::Grouped(GroupedExpression::new(expressions, range))
  }

  pub fn new_table(values: Vec<(Expression, Option<Expression>)>, range: Range) -> Self {
    Expression::Table(TableExpression::new(values, range))
  }

  pub fn new_member(base: Expression, member: Expression) -> Self {
    Expression::Member(MemberExpression::new(Box::new(base), Box::new(member)))
  }

  pub fn new_index(base: Expression, index: Expression, bracket_range: Range) -> Self {
    Expression::Index(IndexExpression::new(Box::new(base), Box::new(index), bracket_range))
  }

  pub fn new_function(
    arguments: Vec<(Token, Option<Type>)>,
    return_type: Option<Type>,
    body: Statement,
    range: Range,
    return_range: Option<Range>,
  ) -> Self {
    Expression::Function(FunctionExpression::new(arguments, return_type, body, range, return_range))
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
      Expression::Table(table) => table.get_range(),
      Expression::Member(member) => member.get_range(),
      Expression::Index(index) => index.get_range(),
    }
  }

  pub fn is_grouped(&self) -> bool {
    matches!(self, Expression::Grouped(_))
  }

  pub fn is_function(&self) -> bool {
    matches!(self, Expression::Function(_))
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
}

impl CallExpression {
  pub fn new(name: Token, args: Box<Expression>) -> Self {
    CallExpression { name, args }
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
pub struct MemberExpression {
  pub base: Box<Expression>,
  pub member: Box<Expression>,
}
impl MemberExpression {
  pub fn new(base: Box<Expression>, member: Box<Expression>) -> Self {
    MemberExpression { base, member }
  }

  pub fn get_range(&self) -> Range {
    let left_range = self.base.get_range();
    let right_range = self.member.get_range();
    create_middle_range(&left_range, &right_range)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexExpression {
  pub base: Box<Expression>,
  pub index: Box<Expression>,
  pub bracket_range: Range,
}
impl IndexExpression {
  pub fn new(base: Box<Expression>, index: Box<Expression>, bracket_range: Range) -> Self {
    IndexExpression { base, index, bracket_range }
  }

  pub fn get_range(&self) -> Range {
    let left_range = self.base.get_range();
    let right_range = self.index.get_range();
    create_middle_range(&left_range, &right_range)
  }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TableExpression {
  pub values: Vec<(Expression, Option<Expression>)>,
  pub range: Range,
}

impl TableExpression {
  pub fn new(values: Vec<(Expression, Option<Expression>)>, range: Range) -> Self {
    TableExpression { values, range }
  }

  pub fn get_range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionExpression {
  pub arguments: Vec<(Token, Option<Type>)>,
  pub return_type: Option<Type>,
  pub body: Box<Statement>,
  pub range: Range,
  pub range_return_type: Option<Range>,
}

impl FunctionExpression {
  pub fn new(
    arguments: Vec<(Token, Option<Type>)>,
    return_type: Option<Type>,
    body: Statement,
    range: Range,
    range_return_type: Option<Range>,
  ) -> Self {
    FunctionExpression { arguments, return_type, body: Box::new(body), range, range_return_type }
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
  Nil(NilLiteral),
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

  pub fn new_nil(range: Range) -> Self {
    LiteralExpression::Nil(NilLiteral::new(range))
  }

  pub fn get_range(&self) -> Range {
    match self {
      LiteralExpression::Number(number) => number.range.clone(),
      LiteralExpression::String(string) => string.range.clone(),
      LiteralExpression::Boolean(boolean) => boolean.range.clone(),
      LiteralExpression::Nil(nil) => nil.range.clone(),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct NilLiteral {
  pub range: Range,
}

impl NilLiteral {
  pub fn new(range: Range) -> Self {
    NilLiteral { range }
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
