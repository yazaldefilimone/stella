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
  TypeDeclaration(TypeDeclaration),
  Continue(ContinueStatement),
  Local(LocalStatement),
  Expression(Expression),
}

impl Statement {
  pub fn get_range(&self) -> Range {
    match self {
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
      // Statement::VariableDeclaration(variable) => variable.get_range(),
      Statement::TypeDeclaration(declaration) => declaration.get_range(),
      Statement::Expression(expression) => expression.get_range(),
      Statement::Continue(continue_) => continue_.get_range(),
      Statement::Local(local) => local.get_range(),
    }
  }

  pub fn new_function(function: FunctionStatement) -> Self {
    Statement::Function(function)
  }

  pub fn new_return(return_: ReturnStatement) -> Self {
    Statement::Return(return_)
  }

  pub fn new_if(if_: IfStatement) -> Self {
    Statement::If(if_)
  }

  pub fn new_while(while_: WhileStatement) -> Self {
    Statement::While(while_)
  }

  pub fn new_repeat(repeat: RepeatStatement) -> Self {
    Statement::Repeat(repeat)
  }

  pub fn new_for(for_: ForStatement) -> Self {
    Statement::For(for_)
  }

  pub fn new_break(break_: BreakStatement) -> Self {
    Statement::Break(break_)
  }

  pub fn new_goto(goto: GotoStatement) -> Self {
    Statement::Goto(goto)
  }

  pub fn new_block(block: BlockStatement) -> Self {
    Statement::Block(block)
  }

  pub fn new_empty(empty: EmptyStatement) -> Self {
    Statement::Empty(empty)
  }

  pub fn new_type(type_: TypeDeclaration) -> Self {
    Statement::TypeDeclaration(type_)
  }

  pub fn new_continue(continue_: ContinueStatement) -> Self {
    Statement::Continue(continue_)
  }

  pub fn new_local(local: LocalStatement) -> Self {
    Statement::Local(local)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignExpression {
  pub left: Vec<Expression>,
  pub right: Vec<Expression>,
  pub range: Range,
}

impl AssignExpression {
  pub fn new(left: Vec<Expression>, right: Vec<Expression>, range: Range) -> Self {
    AssignExpression { left, right, range }
  }

  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionStatement {
  pub name: Token,
  pub local: bool,
  pub arguments: Vec<Variable>,
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
    arguments: Vec<Variable>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ContinueStatement {
  pub range: Range,
}

impl ContinueStatement {
  pub fn new(range: Range) -> Self {
    ContinueStatement { range }
  }
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfStatement {
  pub condition: Expression,
  pub then_body: Box<Statement>,
  pub else_if_branches: Vec<ElseIfStatement>,
  pub else_body: Option<Box<Statement>>,
  pub range: Range,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ElseIfStatement {
  pub condition: Expression,
  pub then_branch: Box<Statement>,
  pub range: Range,
}

impl ElseIfStatement {
  pub fn new(condition: Expression, body: Statement, range: Range) -> Self {
    ElseIfStatement { condition, then_branch: Box::new(body), range }
  }
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }
}

impl IfStatement {
  pub fn new(
    condition: Expression,
    then_body: Statement,
    else_if_branches: Vec<ElseIfStatement>,
    else_body: Option<Statement>,
    range: Range,
  ) -> Self {
    IfStatement {
      condition,
      then_body: Box::new(then_body),
      else_if_branches,
      else_body: else_body.map(Box::new),
      range,
    }
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
  pub init: AssignExpresion,
  pub limit: Expression,
  pub step: Option<Expression>,
  pub body: Box<Statement>,
  pub range: Range,
}

impl ForStatement {
  pub fn new(
    init: AssignExpresion,
    limit: Expression,
    step: Option<Expression>,
    body: Statement,
    range: Range,
  ) -> Self {
    ForStatement { init, limit, step, body: Box::new(body), range }
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
pub struct Variable {
  pub name: Token,
  pub ty: Option<Type>,
}

impl Variable {
  pub fn new(name: Token, ty: Option<Type>) -> Self {
    return Variable { name, ty };
  }
  pub fn get_range(&self) -> Range {
    return self.name.range.clone();
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalStatement {
  pub variables: Vec<Variable>,
  pub initializer: Vec<Expression>,
  pub range: Range,
}

impl LocalStatement {
  pub fn new(variables: Vec<Variable>, initializer: Vec<Expression>, range: Range) -> Self {
    LocalStatement { variables, initializer, range }
  }
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignExpresion {
  pub variables: Vec<Expression>,
  pub initializer: Vec<Expression>,
  pub range: Range,
}

impl AssignExpresion {
  pub fn new(variables: Vec<Expression>, initializer: Vec<Expression>, range: Range) -> Self {
    AssignExpresion { variables, initializer, range }
  }
  pub fn get_range(&self) -> Range {
    self.range.clone()
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
  Assign(AssignExpression),
  Variable(Variable),
}

impl Expression {
  pub fn new_literal(value: LiteralExpression) -> Self {
    Expression::Literal(value)
  }

  pub fn new_identifier(name: String, range: Range) -> Self {
    Expression::Identifier(Identifier::new(name, range))
  }

  pub fn new_call(left: Expression, args: Expression) -> Self {
    Expression::Call(CallExpression::new(Box::new(left), Box::new(args)))
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

  pub fn new_member(base: Expression, identifier: Identifier) -> Self {
    Expression::Member(MemberExpression::new(Box::new(base), identifier))
  }

  pub fn new_index(base: Expression, index: Expression, bracket_range: Range) -> Self {
    Expression::Index(IndexExpression::new(Box::new(base), Box::new(index), bracket_range))
  }

  pub fn new_assign(left: Vec<Expression>, right: Vec<Expression>, range: Range) -> Self {
    Expression::Assign(AssignExpression::new(left, right, range))
  }

  pub fn new_unary(operator: UnaryOperator, operand: Expression, range: Range) -> Self {
    Expression::Unary(UnaryExpression::new(operator, Box::new(operand), range))
  }

  pub fn new_binary(operator: BinaryOperator, left: Expression, right: Expression, range: Range) -> Self {
    Expression::Binary(BinaryExpression::new(operator, Box::new(left), Box::new(right), range))
  }

  pub fn new_function(
    arguments: Vec<Variable>,
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
      Expression::Assign(assign) => assign.get_range(),
      Expression::Variable(var) => var.get_range(),
    }
  }

  pub fn is_grouped(&self) -> bool {
    matches!(self, Expression::Grouped(_))
  }

  pub fn is_function(&self) -> bool {
    matches!(self, Expression::Function(_))
  }

  pub fn is_identifier(&self) -> bool {
    matches!(self, Expression::Identifier(_))
  }

  pub fn is_literal(&self) -> bool {
    matches!(self, Expression::Literal(_))
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
  pub left: Box<Expression>,
  pub args: Box<Expression>,
}

impl CallExpression {
  pub fn new(left: Box<Expression>, args: Box<Expression>) -> Self {
    CallExpression { left, args }
  }

  pub fn get_range(&self) -> Range {
    let left_range = self.left.get_range();
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
  pub identifier: Identifier,
}
impl MemberExpression {
  pub fn new(base: Box<Expression>, identifier: Identifier) -> Self {
    MemberExpression { base, identifier }
  }

  pub fn get_range(&self) -> Range {
    let left_range = self.base.get_range();
    let right_range = self.identifier.range.clone();
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
  pub arguments: Vec<Variable>,
  pub return_type: Option<Type>,
  pub body: Box<Statement>,
  pub range: Range,
  pub range_return_type: Option<Range>,
}

impl FunctionExpression {
  pub fn new(
    arguments: Vec<Variable>,
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

  pub fn support_number(&self) -> bool {
    matches!(self, UnaryOperator::Negate | UnaryOperator::Not | UnaryOperator::Hash)
  }

  pub fn support_string(&self) -> bool {
    matches!(self, UnaryOperator::Hash)
  }

  pub fn support_boolean(&self) -> bool {
    matches!(self, UnaryOperator::Not)
  }
  pub fn support_nil(&self) -> bool {
    matches!(self, UnaryOperator::Not)
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

impl BinaryOperator {
  pub fn precedence(&self) -> u8 {
    match self {
      BinaryOperator::Or => 1,
      BinaryOperator::And => 2,
      BinaryOperator::Equal | BinaryOperator::NotEqual => 3,
      BinaryOperator::LessThan
      | BinaryOperator::GreaterThan
      | BinaryOperator::LessThanOrEqual
      | BinaryOperator::GreaterThanOrEqual => 4,
      BinaryOperator::Add | BinaryOperator::Subtract => 5,
      BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulus => 6,
      BinaryOperator::DoubleSlash => 7,
      BinaryOperator::DoubleDot => 8,
    }
  }

  /// Returns `true` if the operator is right-associative.
  pub fn is_right_associative(&self) -> bool {
    matches!(self, BinaryOperator::DoubleDot)
  }

  /// Determines if the current operator has higher precedence than another operator.
  pub fn has_higher_precedence_than(&self, other: &BinaryOperator) -> bool {
    self.precedence() > other.precedence()
  }

  /// Determines if the current operator has equal precedence to another operator.
  pub fn has_equal_precedence_to(&self, other: &BinaryOperator) -> bool {
    self.precedence() == other.precedence()
  }
}

pub struct TypeExpression {
  pub name: Type,
  pub range: Range,
}

impl BinaryOperator {
  pub fn support_number(&self) -> bool {
    matches!(
      self,
      BinaryOperator::Add
        | BinaryOperator::Subtract
        | BinaryOperator::Multiply
        | BinaryOperator::Divide
        | BinaryOperator::Modulus
        | BinaryOperator::DoubleDot
        | BinaryOperator::Equal
        | BinaryOperator::NotEqual
        | BinaryOperator::LessThan
        | BinaryOperator::GreaterThan
        | BinaryOperator::LessThanOrEqual
        | BinaryOperator::GreaterThanOrEqual
    )
  }

  pub fn support_string(&self) -> bool {
    matches!(self, |BinaryOperator::Equal| BinaryOperator::NotEqual | BinaryOperator::DoubleDot)
  }

  pub fn support_boolean(&self) -> bool {
    matches!(self, BinaryOperator::And | BinaryOperator::Or | BinaryOperator::Equal | BinaryOperator::NotEqual)
  }

  pub fn support_nil(&self) -> bool {
    matches!(self, BinaryOperator::Equal | BinaryOperator::NotEqual)
  }
}
