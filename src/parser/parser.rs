#![allow(dead_code)]

use crate::ast::ast;
use crate::ast::tokens::{Token, TokenKind};
use crate::diagnostics::report::report_and_exit;
use crate::lexer::Lexer;
use crate::types::Type;
use crate::utils::location::{get_middle_location, Location};

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  raw: &'a str,
}

type ParseExpression = fn(&mut Parser) -> ast::Expression;

impl<'a> Parser<'a> {
  pub fn new(raw: &'a str, file_name: &'a str) -> Self {
    Self { lexer: Lexer::new(raw, file_name), raw }
  }
  pub fn parse_program(&mut self) -> ast::Program {
    let mut program = ast::Program::new();
    while !self.is_end() {
      let statement = self.parse_statement();
      program.statements.push(statement);
    }
    program
  }

  fn parse_statement(&mut self) -> ast::Statement {
    self.skip_comments();
    if self.is_end() {
      return ast::Statement::Empty(ast::EmptyStatement {});
    }

    let token = self.lexer.peek_token();
    let statement = match token.kind {
      TokenKind::Local => self.parse_local_declaration(),
      TokenKind::If => self.parse_if_statement(),
      TokenKind::While => self.parse_while_statement(),
      TokenKind::Repeat => self.parse_repeat_statement(),
      TokenKind::For => self.parse_for_statement(),
      TokenKind::Break => self.parse_break_statement(),
      TokenKind::Continue => self.parse_continue_statement(),
      TokenKind::Return => self.parse_return_statement(),
      TokenKind::Function => self.parse_function_statement(false),
      _ => self.parse_assign_or_call(),
    };

    self.match_token_and_consume(TokenKind::Semicolon);
    statement
  }

  fn parse_local_declaration(&mut self) -> ast::Statement {
    self.consume_expect_token(TokenKind::Local);
    if self.match_token(&TokenKind::Function) {
      return self.parse_function_statement(true);
    }
    self.parse_variable_declaration(true)
  }

  fn parse_expression_statement(&mut self) -> ast::Expression {
    self.parse_or_expression()
  }

  fn parse_or_expression(&mut self) -> ast::Expression {
    // self.parse_binary_expression(Self::parse_and_expression, vec![BinaryOperator::Or])
    self.parse_binary_expression(Self::parse_and_expression)
  }

  fn parse_and_expression(&mut self) -> ast::Expression {
    // self.parse_binary_expression(Self::parse_unary_expression, vec![BinaryOperator::And])
    self.parse_binary_expression(Self::parse_unary_expression)
  }

  fn parse_binary_expression(&mut self, parse_sub_expression: fn(&mut Self) -> ast::Expression) -> ast::Expression {
    let mut expression = parse_sub_expression(self);
    while let Some((operator, location)) = self.parse_operator() {
      let right_expression = parse_sub_expression(self);
      let binary_exp = ast::BinaryExpression::new(operator, Box::new(expression), Box::new(right_expression), location);
      expression = ast::Expression::Binary(binary_exp);
    }
    expression
  }

  fn parse_operator(&mut self) -> Option<(ast::BinaryOperator, Location)> {
    let token = self.lexer.peek_token();
    let operator = match token.kind {
      TokenKind::Plus => ast::BinaryOperator::Add,
      TokenKind::Minus => ast::BinaryOperator::Subtract,
      TokenKind::Star => ast::BinaryOperator::Multiply,
      TokenKind::Slash => ast::BinaryOperator::Divide,
      TokenKind::Percent => ast::BinaryOperator::Modulus,
      TokenKind::And => ast::BinaryOperator::And,
      TokenKind::Or => ast::BinaryOperator::Or,
      TokenKind::Equal => ast::BinaryOperator::Equal,
      TokenKind::NotEqual => ast::BinaryOperator::NotEqual,
      TokenKind::Less => ast::BinaryOperator::LessThan,
      TokenKind::Greater => ast::BinaryOperator::GreaterThan,
      TokenKind::LessEqual => ast::BinaryOperator::LessThanOrEqual,
      TokenKind::GreaterEqual => ast::BinaryOperator::GreaterThanOrEqual,
      TokenKind::DoubleDot => ast::BinaryOperator::DoubleDot,
      TokenKind::DoubleSlash => ast::BinaryOperator::DoubleSlash,
      _ => return None,
    };
    self.lexer.next_token();
    return Some((operator, token.location));
  }

  fn parse_unary_expression(&mut self) -> ast::Expression {
    let unary_location = self.lexer.peek_token().location.clone();
    if let Some(operator) = self.parse_unary_operator() {
      let expression = self.parse_expression_statement();
      let unary_expression = ast::UnaryExpression::new(operator, Box::new(expression), unary_location);
      ast::Expression::Unary(unary_expression)
    } else {
      self.parse_primary_expression()
    }
  }

  fn parse_unary_operator(&mut self) -> Option<ast::UnaryOperator> {
    let token = self.lexer.peek_token();
    let operator = match token.kind {
      TokenKind::Minus => ast::UnaryOperator::Negate,
      TokenKind::Not => ast::UnaryOperator::Not,
      TokenKind::Hash => ast::UnaryOperator::Hash,
      _ => return None,
    };
    self.lexer.next_token();
    Some(operator)
  }

  fn parse_primary_expression(&mut self) -> ast::Expression {
    let token = self.lexer.peek_token();
    match token.kind {
      TokenKind::Number(_) | TokenKind::String(_) => self.parse_literal_expression(),
      TokenKind::True | TokenKind::False => self.parse_literal_expression(),
      TokenKind::Identifier(_) => self.parse_identifier(),
      TokenKind::LeftParen => self.parse_grouped_expression(),
      TokenKind::Require => self.parse_require_expression(),
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_literal_expression(&mut self) -> ast::Expression {
    let token = self.lexer.next_token();
    match token.kind {
      TokenKind::Number(value) => {
        ast::Expression::new_literal(ast::LiteralExpression::new_number(value, token.location))
      }
      TokenKind::String(value) => {
        ast::Expression::new_literal(ast::LiteralExpression::new_string(value, token.location))
      }
      TokenKind::True => ast::Expression::new_literal(ast::LiteralExpression::new_bool(true, token.location)),
      TokenKind::False => ast::Expression::new_literal(ast::LiteralExpression::new_bool(false, token.location)),
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_identifier(&mut self) -> ast::Expression {
    let token = self.lexer.next_token();

    if self.match_token(&TokenKind::LeftParen) {
      return self.parse_call_expression(token);
    }
    match token.kind {
      TokenKind::Identifier(name) => ast::Expression::new_identifier(name, token.location),
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_grouped_expression(&mut self) -> ast::Expression {
    let left_location = self.consume_expect_token(TokenKind::LeftParen).location;
    if self.match_token(&TokenKind::RightParen) {
      let right_location = self.consume_expect_token(TokenKind::RightParen).location;
      let location = get_middle_location(&left_location, &right_location);
      return ast::Expression::new_grouped(vec![], location);
    }

    let mut expressions = Vec::new();

    expressions.push(self.parse_expression_statement());

    while self.match_token(&TokenKind::Comma) {
      self.consume_expect_token(TokenKind::Comma);
      expressions.push(self.parse_expression_statement());
    }

    let right_location = self.consume_expect_token(TokenKind::RightParen).location;
    let location = get_middle_location(&left_location, &right_location);
    return ast::Expression::new_grouped(expressions, location);
  }

  fn parse_require_expression(&mut self) -> ast::Expression {
    let require_token = self.consume_expect_token(TokenKind::Require);
    let module_name = self.consume_token();
    let location = require_token.location;
    ast::Expression::new_require(module_name, location)
  }

  fn parse_assign_or_call(&mut self) -> ast::Statement {
    let token = self.lexer.peek_token();
    match token.kind {
      TokenKind::Identifier(_) => self.parse_possible_assign_or_call(token),
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_possible_assign_or_call(&mut self, ident_token: Token) -> ast::Statement {
    self.lexer.next_token(); // consume identifier
                             // if next is `=` or `:` then it's an assign statement
    if self.match_token(&TokenKind::Assign) || self.match_token(&TokenKind::Colon) {
      self.parse_assign_statement(ident_token)
    } else if self.match_token(&TokenKind::LeftParen) {
      let expression = self.parse_call_expression(ident_token);
      ast::Statement::Expression(expression)
    } else {
      self.report_unexpected_token(ident_token)
    }
  }

  fn parse_assign_statement(&mut self, ident_token: Token) -> ast::Statement {
    let values = self.parse_variable_and_type(Some(ident_token));
    self.consume_expect_token(TokenKind::Assign);
    let init = self.parse_expression_statement();
    ast::Statement::Assign(ast::AssignStatement::new(values, init))
  }

  fn parse_variable_and_type(&mut self, token: Option<Token>) -> Vec<(Token, Option<Type>)> {
    let mut names: Vec<(Token, Option<Type>)> = Vec::new();

    let token = if token.is_some() { token.unwrap() } else { self.consume_token() };

    let mut name = (token, None);
    if self.match_token_and_consume(TokenKind::Colon).is_some() {
      name.1 = Some(self.parse_type(false));
    };
    names.push(name);
    while self.match_token_and_consume(TokenKind::Comma).is_some() {
      name = (self.consume_token(), None);
      if self.match_token_and_consume(TokenKind::Colon).is_some() {
        name.1 = Some(self.parse_type(false));
      };
      names.push(name);
    }
    names
  }

  fn parse_call_expression(&mut self, ident: Token) -> ast::Expression {
    let location = ident.location.clone();
    let args = self.parse_expression_statement();
    ast::Expression::new_call(ident, args, location)
  }

  fn parse_function_statement(&mut self, local: bool) -> ast::Statement {
    self.consume_expect_token(TokenKind::Function);
    let name = self.consume_token();

    self.consume_expect_token(TokenKind::LeftParen);
    let arguments = self.parse_arguments_with_option_type();
    self.consume_expect_token(TokenKind::RightParen);

    let mut return_type = None;

    if self.match_token(&TokenKind::Colon) {
      self.consume_expect_token(TokenKind::Colon);
      return_type = Some(self.parse_type(true))
    }

    let body = self.parse_block_statement(&[TokenKind::End]);
    self.consume_expect_token(TokenKind::End);
    let location = name.location.clone();

    ast::Statement::Function(ast::FunctionStatement::new(name, local, arguments, return_type, body, location))
  }

  fn parse_type(&mut self, is_function_return: bool) -> Type {
    let token = self.lexer.peek_token();
    if !is_function_return && token.kind == TokenKind::LeftParen {
      self.report_unexpected_token(token)
    }
    match token.kind {
      TokenKind::Identifier(name) => {
        self.lexer.next_token();
        Type::new_type(&name)
      }
      TokenKind::LeftParen => self.parse_grup_return_type(),
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_grup_return_type(&mut self) -> Type {
    self.consume_expect_token(TokenKind::LeftParen);
    let mut types = Vec::new();
    while !self.match_token(&TokenKind::RightParen) {
      types.push(self.parse_type(false));
      self.match_token_and_consume(TokenKind::Comma);
    }
    self.consume_expect_token(TokenKind::RightParen);
    Type::new_grup(types)
  }

  fn parse_arguments_with_option_type(&mut self) -> Vec<(Token, Option<Type>)> {
    let mut arguments = Vec::new();
    while !self.match_token(&TokenKind::RightParen) {
      let name = self.consume_token();
      let mut tty = None;
      if self.match_token_and_consume(TokenKind::Colon).is_some() {
        tty = Some(self.parse_type(false));
      };
      arguments.push((name, tty));
      self.match_token_and_consume(TokenKind::Comma);
    }
    arguments
  }

  fn parse_variable_declaration(&mut self, local: bool) -> ast::Statement {
    let valyes = self.parse_variable_and_type(None);
    let mut initializer = None;
    if self.match_token_and_consume(TokenKind::Assign).is_some() {
      initializer = Some(self.parse_expression_statement());
    }
    ast::Statement::VariableDeclaration(ast::VariableDeclaration::new(valyes, local, initializer))
  }

  fn parse_block_statement(&mut self, end_tokens: &[TokenKind]) -> ast::Statement {
    let mut statements = Vec::new();
    let location = self.lexer.peek_token().location.clone();
    while !self.contains_token(end_tokens) {
      statements.push(self.parse_statement());
    }
    ast::Statement::Block(ast::BlockStatement::new(statements, location))
  }

  fn parse_if_statement(&mut self) -> ast::Statement {
    let if_token = self.consume_expect_token(TokenKind::If);
    let condition = self.parse_expression_statement();
    self.consume_expect_token(TokenKind::Then);
    let then_body = self.parse_block_statement(&[TokenKind::Else, TokenKind::End]);
    let else_body = if self.match_token_and_consume(TokenKind::Else).is_some() {
      Some(self.parse_block_statement(&[TokenKind::End]))
    } else {
      None
    };
    self.consume_expect_token(TokenKind::End);
    ast::Statement::If(ast::IfStatement::new(condition, then_body, else_body, if_token.location))
  }

  fn parse_while_statement(&mut self) -> ast::Statement {
    let while_token = self.consume_expect_token(TokenKind::While);
    let condition = self.parse_expression_statement();
    self.consume_expect_token(TokenKind::Do);
    let body = self.parse_block_statement(&[TokenKind::End]);
    self.consume_expect_token(TokenKind::End);
    ast::Statement::While(ast::WhileStatement::new(condition, body, while_token.location))
  }

  fn parse_repeat_statement(&mut self) -> ast::Statement {
    let repeat_token = self.consume_expect_token(TokenKind::Repeat);
    let body = self.parse_block_statement(&[TokenKind::Until]);
    self.consume_expect_token(TokenKind::Until);
    let condition = self.parse_expression_statement();
    ast::Statement::Repeat(ast::RepeatStatement::new(body, condition, repeat_token.location))
  }

  fn parse_for_statement(&mut self) -> ast::Statement {
    let for_token = self.consume_expect_token(TokenKind::For);
    let variable = self.parse_identifier();
    self.consume_expect_token(TokenKind::Assign);
    let init = self.parse_expression_statement();
    self.consume_expect_token(TokenKind::Comma);
    let limit = self.parse_expression_statement();
    let step = if self.match_token_and_consume(TokenKind::Comma).is_some() {
      Some(self.parse_expression_statement())
    } else {
      None
    };
    self.consume_expect_token(TokenKind::Do);
    let body = self.parse_block_statement(&[TokenKind::End]);
    self.consume_expect_token(TokenKind::End);
    ast::Statement::For(ast::ForStatement::new(variable, init, limit, step, body, for_token.location))
  }

  fn parse_break_statement(&mut self) -> ast::Statement {
    let break_token = self.consume_expect_token(TokenKind::Break);
    ast::Statement::Break(ast::BreakStatement::new(break_token.location))
  }

  fn parse_continue_statement(&mut self) -> ast::Statement {
    // Implementação ausente
    unimplemented!()
  }

  fn parse_return_statement(&mut self) -> ast::Statement {
    let return_token = self.consume_expect_token(TokenKind::Return);
    let values = self.parse_return_value();
    ast::Statement::Return(ast::ReturnStatement::new(values, return_token.location))
  }

  fn parse_return_value(&mut self) -> Vec<ast::Expression> {
    let mut values = Vec::new();
    if self.match_token(&TokenKind::Semicolon) {
      return values;
    }
    values.push(self.parse_expression_statement());
    while self.match_token_and_consume(TokenKind::Comma).is_some() {
      values.push(self.parse_expression_statement());
    }
    values
  }

  fn consume_expect_token(&mut self, kind: TokenKind) -> Token {
    let token = self.lexer.next_token();
    if token.kind != kind {
      let message = format!("expected '{}' but found '{}'", kind.to_string(), token.kind.to_string());
      self.report_error(message, token);
    }
    token
  }

  fn consume_token(&mut self) -> Token {
    self.lexer.next_token()
  }

  fn match_token(&mut self, kind: &TokenKind) -> bool {
    self.lexer.peek_token().kind == *kind
  }

  fn match_token_and_consume(&mut self, kind: TokenKind) -> Option<Token> {
    if self.match_token(&kind) {
      Some(self.consume_token())
    } else {
      None
    }
  }

  fn contains_token(&mut self, kinds: &[TokenKind]) -> bool {
    if self.is_end() {
      return false;
    }
    let next_token = self.lexer.peek_token();
    kinds.iter().any(|kind| &next_token.kind == kind)
  }

  fn is_end(&mut self) -> bool {
    self.match_token(&TokenKind::EOF)
  }

  fn skip_comments(&mut self) {
    while self.lexer.peek_token().is_comment() {
      self.lexer.next_token();
    }
  }

  fn report_unexpected_token(&self, token: Token) -> ! {
    let message = format!("unexpected token '{}'", token.kind.to_string());
    self.report_error(message, token)
  }

  fn report_error(&self, message: String, token: Token) -> ! {
    report_and_exit(&message, &mut token.location.clone(), &self.raw, &self.lexer.file_name)
  }
}
