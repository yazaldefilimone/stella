#![allow(dead_code)]

use crate::ast::ast;
use crate::ast::tokens::{Token, TokenKind};
use crate::diagnostics::report_and_exit;
use crate::lexer::Lexer;
use crate::types::Type;

pub struct Parser {
  lexer: Lexer,
  raw: String,
}

impl Parser {
  pub fn new(raw: &str) -> Self {
    Self { lexer: Lexer::new(raw.to_string()), raw: raw.to_string() }
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
    let token = self.lexer.peek_token();
    let statement = match token.kind {
      TokenKind::Local => self.parse_variable_declaration(),
      TokenKind::If => self.parse_if_statement(),
      TokenKind::While => self.parse_while_statement(),
      TokenKind::Repeat => self.parse_repeat_statement(),
      TokenKind::For => self.parse_for_statement(),
      TokenKind::Break => self.parse_break_statement(),
      TokenKind::Continue => self.parse_continue_statement(),
      TokenKind::Return => self.parse_return_statement(),
      TokenKind::Function => self.parse_function_statement(),
      _ => self.parse_assign_or_call_statement(),
    };
    self.match_token_and_consume(TokenKind::Semicolon);
    statement
  }

  fn parse_expression_statement(&mut self) -> ast::Expression {
    self.parse_or_expression()
  }

  fn parse_or_expression(&mut self) -> ast::Expression {
    let mut expression = self.parse_and_expression();
    while let Some(operator) = self.parse_binary_operator() {
      let right_expression = self.parse_and_expression();
      let binary_expression = ast::BinaryExpression::new(operator, Box::new(expression), Box::new(right_expression));
      expression = ast::Expression::BinaryExpression(binary_expression);
    }
    expression
  }

  fn parse_and_expression(&mut self) -> ast::Expression {
    let mut expression = self.parse_unary_expression();
    while let Some(operator) = self.parse_binary_operator() {
      let right_expression = self.parse_unary_expression();
      let binary_expression = ast::BinaryExpression::new(operator, Box::new(expression), Box::new(right_expression));
      expression = ast::Expression::BinaryExpression(binary_expression);
    }
    expression
  }

  fn parse_binary_operator(&mut self) -> Option<ast::BinaryOperator> {
    let token = self.lexer.peek_token();
    let operator_token = match token.kind {
      TokenKind::Plus => Some(ast::BinaryOperator::Add),
      TokenKind::Minus => Some(ast::BinaryOperator::Subtract),
      TokenKind::Star => Some(ast::BinaryOperator::Multiply),
      TokenKind::Slash => Some(ast::BinaryOperator::Divide),
      TokenKind::Percent => Some(ast::BinaryOperator::Modulus),
      TokenKind::And => Some(ast::BinaryOperator::And),
      TokenKind::Or => Some(ast::BinaryOperator::Or),
      TokenKind::Equal => Some(ast::BinaryOperator::Equal),
      TokenKind::NotEqual => Some(ast::BinaryOperator::NotEqual),
      TokenKind::Less => Some(ast::BinaryOperator::LessThan),
      TokenKind::Greater => Some(ast::BinaryOperator::GreaterThan),
      TokenKind::LessEqual => Some(ast::BinaryOperator::LessThanOrEqual),
      TokenKind::GreaterEqual => Some(ast::BinaryOperator::GreaterThanOrEqual),
      TokenKind::DoubleDot => Some(ast::BinaryOperator::DoubleDot),
      _ => None,
    };
    if operator_token.is_some() {
      self.lexer.next_token();
    }
    operator_token
  }

  fn parse_unary_operator(&mut self) -> Option<ast::UnaryOperator> {
    let token = self.lexer.peek_token();
    let operator_token = match token.kind {
      TokenKind::Minus => Some(ast::UnaryOperator::Negate),
      TokenKind::Not => Some(ast::UnaryOperator::Not),
      TokenKind::Hash => Some(ast::UnaryOperator::Not),
      _ => None,
    };
    if operator_token.is_some() {
      self.lexer.next_token();
    }
    return operator_token;
  }

  fn parse_unary_expression(&mut self) -> ast::Expression {
    if let Some(operator) = self.parse_unary_operator() {
      let expression = self.parse_expression_statement();
      let unary_expression = ast::UnaryExpression::new(operator, Box::new(expression));
      return ast::Expression::UnaryExpression(unary_expression);
    }

    return self.parse_primary_expression();
  }

  fn parse_primary_expression(&mut self) -> ast::Expression {
    let token = self.lexer.peek_token();
    match token.kind {
      TokenKind::Number(_) | TokenKind::String(_) => self.parse_literal_expression(),
      TokenKind::True | TokenKind::False => self.parse_literal_expression(),
      TokenKind::Identifier(_) => self.parse_call_expression_or_identifier(),
      TokenKind::LeftParen => self.parse_grouped_expression(),
      TokenKind::LeftBrace => self.parse_table_construction(),
      _ => {
        let mut location = token.location.clone();
        report_and_exit("Expected a primary expression", &mut location, &self.raw.as_str());
      }
    }
  }

  fn parse_grouped_expression(&mut self) -> ast::Expression {
    let left_paren = self.consume_expect_token(TokenKind::LeftParen);
    let mut expressions = vec![];
    while !self.match_token(&TokenKind::RightParen) {
      expressions.push(self.parse_expression_statement());
      self.match_token_and_consume(TokenKind::Comma);
    }
    self.consume_expect_token(TokenKind::RightParen);
    let location = left_paren.location;
    ast::Expression::GroupedExpression(ast::GroupedExpression::new(expressions, location))
  }

  fn parse_table_construction(&mut self) -> ast::Expression {
    todo!("Please implement the parse_table_construction method.")
  }

  fn parse_call_expression_or_identifier(&mut self) -> ast::Expression {
    let token = self.lexer.next_token();
    if self.match_token(&TokenKind::LeftParen) {
      return self.parse_call_expression(token);
    }
    if let TokenKind::Identifier(_) = token.kind {
      return ast::Expression::new_identifier(token.lexeme(), token.location);
    }

    let mut location = token.location;
    report_and_exit("Invalid identifier expression", &mut location, &self.raw.as_str());
  }

  fn parse_identifier(&mut self) -> ast::Expression {
    let token = self.lexer.next_token();
    match token.kind {
      TokenKind::Identifier(name) => ast::Expression::new_identifier(name, token.location),
      _ => {
        let mut location = token.location;
        report_and_exit("Invalid identifier", &mut location, &self.raw.as_str());
      }
    }
  }

  fn parse_literal_expression(&mut self) -> ast::Expression {
    let token = self.lexer.next_token();
    match token.kind {
      TokenKind::Number(number) => ast::Expression::new_number_literal(number, token.location),
      TokenKind::String(string) => ast::Expression::new_string_literal(string, token.location),
      TokenKind::True => ast::Expression::new_bool_literal(true, token.location),
      TokenKind::False => ast::Expression::new_bool_literal(false, token.location),
      _ => {
        let mut location = token.location;
        report_and_exit("Invalid literal expression", &mut location, &self.raw.as_str());
      }
    }
  }

  fn parse_assign_or_call_statement(&mut self) -> ast::Statement {
    let token = self.lexer.next_token();
    match self.lexer.peek_token().kind {
      TokenKind::Assign => self.parse_assign_statement(token),
      _ => {
        let expression = self.parse_call_expression(token);
        ast::Statement::CallStatement(ast::CallStatement::new(expression))
      }
    }
  }

  fn parse_call_expression(&mut self, ident: Token) -> ast::Expression {
    if let TokenKind::Identifier(_) = ident.kind {
      let location = ident.location.clone();
      let arguments = self.parse_expression_statement();
      ast::Expression::new_call_expression(ident, arguments, location)
    } else {
      let mut location = ident.location.clone();
      let report_message = format!("Expected a function call, found '{:?}'", ident.kind);
      report_and_exit(&report_message, &mut location, &self.raw.as_str());
    }
  }

  fn parse_assign_statement(&mut self, name: Token) -> ast::Statement {
    self.consume_expect_token(TokenKind::Assign);
    let value = self.parse_expression_statement();
    let location = name.location.clone();
    ast::Statement::AssignStatement(ast::AssignStatement::new(name, value, location))
  }

  // suport anonymous functions only as expressions and expect error when used as a statement
  fn parse_function_statement(&mut self) -> ast::Statement {
    self.consume_expect_token(TokenKind::Function);
    let name = self.consume_token();
    self.consume_expect_token(TokenKind::LeftParen);
    let arguments = self.parse_arguments_with_option_type();
    self.consume_expect_token(TokenKind::RightParen);
    let mut return_type = None;

    if self.match_token(&TokenKind::Colon) {
      self.consume_expect_token(TokenKind::Colon);
      return_type = Some(self.parse_type());
    }

    let body = self.parse_block_statement(&[TokenKind::End]);

    self.consume_expect_token(TokenKind::End);
    let location = name.location.clone();
    ast::Statement::FunctionStatement(ast::FunctionStatement::new(
      name,
      arguments,
      return_type,
      body,
      location,
    ))
  }

  fn parse_type(&mut self) -> Type {
    let token = self.lexer.next_token();
    match token.kind {
      TokenKind::Identifier(name) => Type::new_type(name),
      _ => todo!("Please implement the parse_type method."),
    }
  }

  fn parse_arguments_with_option_type(&mut self) -> Vec<(Token, Option<Type>)> {
    let mut arguments = vec![];
    while !self.match_token(&TokenKind::RightParen) {
      let name = self.consume_token();
      let mut type_ = None;
      if let Some(_) = self.match_token_and_consume(TokenKind::Colon) {
        type_ = Some(self.parse_type());
      }
      arguments.push((name, type_));
    }
    arguments
  }

  fn parse_variable_declaration(&mut self) -> ast::Statement {
    let local_token = self.match_token_and_consume(TokenKind::Local);
    let name = self.consume_token();
    let mut type_ = None;
    if self.match_token(&TokenKind::Colon) {
      self.consume_expect_token(TokenKind::Colon);
      type_ = Some(self.parse_type());
    }
    let mut init = None;
    if self.match_token(&TokenKind::Assign) {
      self.consume_expect_token(TokenKind::Assign);
      init = Some(self.parse_expression_statement());
    }

    let local = local_token.is_some();
    let location = name.location.clone();
    let variable_declaration = ast::VariableDeclaration::new(name, local, type_, init, location);
    ast::Statement::VariableDeclaration(variable_declaration)
  }

  fn parse_block_statement(&mut self, end_tokens: &[TokenKind]) -> ast::Statement {
    let mut statements = vec![];
    let location = self.lexer.peek_token().location.clone();
    while !self.contains_token(end_tokens) {
      let statement = self.parse_statement();
      statements.push(statement);
    }
    ast::Statement::BlockStatement(ast::BlockStatement::new(statements, location))
  }

  fn parse_if_statement(&mut self) -> ast::Statement {
    let if_token = self.consume_expect_token(TokenKind::If);
    let condition = self.parse_expression_statement();
    self.consume_expect_token(TokenKind::Then);
    let body = self.parse_block_statement(&[TokenKind::End, TokenKind::Else]);
    let else_body = if self.match_token(&TokenKind::Else) {
      self.consume_expect_token(TokenKind::Else);
      let block = self.parse_block_statement(&[TokenKind::End]);
      Some(Box::new(block))
    } else {
      None
    };
    self.consume_expect_token(TokenKind::End);
    let if_statement = ast::IfStatement { condition, body: Box::new(body), else_body, location: if_token.location };
    ast::Statement::IfStatement(if_statement)
  }

  fn parse_while_statement(&mut self) -> ast::Statement {
    let while_token = self.consume_expect_token(TokenKind::While);
    let condition = self.parse_expression_statement();
    self.consume_expect_token(TokenKind::Do);
    let body = self.parse_block_statement(&[TokenKind::End]);
    self.consume_expect_token(TokenKind::End);
    let location = while_token.location;
    ast::Statement::WhileStatement(ast::WhileStatement { condition, body: Box::new(body), location })
  }

  fn parse_repeat_statement(&mut self) -> ast::Statement {
    let repeat_token = self.consume_expect_token(TokenKind::Repeat);
    let body = self.parse_block_statement(&[TokenKind::Until]);
    let condition = self.parse_expression_statement();
    self.consume_expect_token(TokenKind::Until);
    let location = repeat_token.location;
    let repeat_statement = ast::RepeatStatement::new(body, condition, location);
    ast::Statement::RepeatStatement(repeat_statement)
  }

  fn parse_for_statement(&mut self) -> ast::Statement {
    let for_token = self.consume_expect_token(TokenKind::For);
    let variable = self.parse_identifier();
    self.consume_expect_token(TokenKind::Assign);
    let init = self.parse_expression_statement();
    self.consume_expect_token(TokenKind::Comma);
    let limit = self.parse_expression_statement();
    let mut step = None;
    if let Some(_) = self.match_token_and_consume(TokenKind::Comma) {
      step = Some(self.parse_expression_statement());
    }
    self.consume_expect_token(TokenKind::Do);
    let body = self.parse_block_statement(&[TokenKind::End]);
    self.consume_expect_token(TokenKind::End);
    let location = for_token.location;
    let for_statement = ast::ForStatement { variable, init, limit, step, body: Box::new(body), location };
    ast::Statement::ForStatement(for_statement)
  }

  fn parse_break_statement(&mut self) -> ast::Statement {
    let break_token = self.consume_expect_token(TokenKind::Break);
    let location = break_token.location;
    ast::Statement::BreakStatement(ast::BreakStatement { location })
  }

  fn parse_continue_statement(&mut self) -> ast::Statement {
    todo!("Please implement the parse_continue_statement method.")
  }

  fn parse_return_statement(&mut self) -> ast::Statement {
    let return_token = self.consume_expect_token(TokenKind::Return);
    let location = return_token.location;
    let value = self.parse_return_value();
    let return_statement = ast::ReturnStatement { location, value };
    ast::Statement::ReturnStatement(return_statement)
  }

  fn parse_return_value(&mut self) -> Vec<ast::Expression> {
    let mut values = vec![];
    while !self.contains_token(&[TokenKind::End]) {
      if self.match_token(&TokenKind::Comma) {
        self.consume_expect_token(TokenKind::Comma);
      }
      values.push(self.parse_expression_statement());
    }
    values
  }

  fn consume_expect_token(&mut self, kind: TokenKind) -> Token {
    let token = self.lexer.next_token();
    if token.kind != kind {
      let mut location = token.location.clone();
      let message = format!("Expected '{:?}' but found '{:?}'", kind, token.kind);
      report_and_exit(message.as_str(), &mut location, &self.raw.as_str());
    }
    token
  }

  fn consume_token(&mut self) -> Token {
    self.lexer.next_token()
  }

  fn match_token(&mut self, kind: &TokenKind) -> bool {
    let next_token = self.lexer.peek_token();
    next_token.kind == *kind
  }

  fn contains_token(&mut self, kinds: &[TokenKind]) -> bool {
    if self.is_end() {
      return false;
    }
    let next_token = self.lexer.peek_token();
    kinds.iter().any(|kind| &next_token.kind == kind)
  }

  fn match_token_and_consume(&mut self, kind: TokenKind) -> Option<Token> {
    if self.is_end() {
      return None;
    }
    if self.match_token(&kind) {
      Some(self.consume_token())
    } else {
      None
    }
  }

  fn is_end(&mut self) -> bool {
    let next_token = self.lexer.peek_token();
    matches!(next_token.kind, TokenKind::EOF)
  }
}
