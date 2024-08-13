#![allow(dead_code)]

use crate::ast::ast;
use crate::ast::tokens::{Token, TokenKind};
use crate::diagnostics::report::report_and_exit;
use crate::lexer::Lexer;
use crate::types::Type;
use crate::utils::range::{create_middle_range, Range};

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  raw: &'a str,
}

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
      TokenKind::Type => self.parse_type_declaration(),
      _ => self.parse_assign_expression_or_call_expression(),
    };
    self.match_token_and_consume(TokenKind::Semicolon);
    statement
  }

  // declarations
  //
  fn parse_local_declaration(&mut self) -> ast::Statement {
    self.consume_expect_token(TokenKind::Local);
    if self.match_token(&TokenKind::Function) {
      return self.parse_function_statement(true);
    }
    self.parse_variable_declaration(true)
  }

  fn parse_type_declaration(&mut self) -> ast::Statement {
    let range = self.consume_expect_token(TokenKind::Type).range.clone();
    let name = self.consume_token();
    let generics = self.parse_simple_generic_str();
    self.consume_expect_token(TokenKind::Assign);
    // we can accept function type
    let initilizer = self.parse_type(false);
    ast::Statement::TypeDeclaration(ast::TypeDeclaration::new(name, generics, initilizer, range))
  }
  fn parse_variable_declaration(&mut self, local: bool) -> ast::Statement {
    let values = self.parse_variable_and_type(None);
    let mut initializer = None;
    if self.match_token_and_consume(TokenKind::Assign).is_some() {
      initializer = Some(self.parse_expression_statement());
    }
    ast::Statement::VariableDeclaration(ast::VariableDeclaration::new(values, local, initializer))
  }

  fn parse_function_statement(&mut self, local: bool) -> ast::Statement {
    let start = self.consume_expect_token(TokenKind::Function).range;
    let name = self.consume_token();
    let generics = self.parse_generic_type();
    self.consume_expect_token(TokenKind::LeftParen);
    let arguments = self.parse_arguments_with_option_type();
    self.consume_expect_token(TokenKind::RightParen);

    let mut return_type = None;

    let mut range_return_type = None;
    if self.match_token(&TokenKind::Colon) {
      self.consume_expect_token(TokenKind::Colon);
      range_return_type = Some(self.lexer.peek_token().range.clone());
      return_type = Some(self.parse_type(true))
    }
    let body = self.parse_block_statement(&[TokenKind::End]);
    let end_range = self.consume_expect_token(TokenKind::End).range;
    let range = create_middle_range(&start, &end_range);
    ast::Statement::Function(ast::FunctionStatement::new(
      name,
      local,
      generics,
      arguments,
      return_type,
      body,
      range,
      range_return_type,
    ))
  }

  // flow control
  //

  fn parse_if_statement(&mut self) -> ast::Statement {
    let start_range = self.consume_expect_token(TokenKind::If).range;
    let condition = self.parse_expression_statement();
    self.consume_expect_token(TokenKind::Then);
    let then_body = self.parse_block_statement(&[TokenKind::Else, TokenKind::End]);
    let else_body = if self.match_token_and_consume(TokenKind::Else).is_some() {
      Some(self.parse_block_statement(&[TokenKind::End]))
    } else {
      None
    };
    let end_range = self.consume_expect_token(TokenKind::End).range;
    let range = create_middle_range(&start_range, &end_range);
    ast::Statement::If(ast::IfStatement::new(condition, then_body, else_body, range))
  }

  fn parse_while_statement(&mut self) -> ast::Statement {
    let while_token = self.consume_expect_token(TokenKind::While);
    let condition = self.parse_expression_statement();
    self.consume_expect_token(TokenKind::Do);
    let body = self.parse_block_statement(&[TokenKind::End]);
    self.consume_expect_token(TokenKind::End);
    ast::Statement::While(ast::WhileStatement::new(condition, body, while_token.range))
  }

  fn parse_repeat_statement(&mut self) -> ast::Statement {
    let repeat_token = self.consume_expect_token(TokenKind::Repeat);
    let body = self.parse_block_statement(&[TokenKind::Until]);
    self.consume_expect_token(TokenKind::Until);
    let condition = self.parse_expression_statement();
    ast::Statement::Repeat(ast::RepeatStatement::new(body, condition, repeat_token.range))
  }

  fn parse_for_statement(&mut self) -> ast::Statement {
    let start_range = self.consume_expect_token(TokenKind::For).range;
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
    let end_range = self.consume_expect_token(TokenKind::End).range;
    let range = create_middle_range(&start_range, &end_range);
    ast::Statement::For(ast::ForStatement::new(variable, init, limit, step, body, range))
  }

  fn parse_break_statement(&mut self) -> ast::Statement {
    let break_token = self.consume_expect_token(TokenKind::Break);
    ast::Statement::Break(ast::BreakStatement::new(break_token.range))
  }

  fn parse_continue_statement(&mut self) -> ast::Statement {
    // Implementação ausente
    unimplemented!()
  }

  fn parse_return_statement(&mut self) -> ast::Statement {
    let return_token = self.consume_expect_token(TokenKind::Return);
    let values = self.parse_return_value();
    ast::Statement::Return(ast::ReturnStatement::new(values, return_token.range))
  }

  // expressions
  //
  fn parse_function_expression(&mut self) -> ast::Expression {
    let start_range = self.consume_expect_token(TokenKind::Function).range;
    self.consume_expect_token(TokenKind::LeftParen);
    let arguments = self.parse_arguments_with_option_type();
    self.consume_expect_token(TokenKind::RightParen);

    let mut return_type = None;
    let mut range_return_type = None;
    if self.match_token_and_consume(TokenKind::Colon).is_some() {
      range_return_type = Some(self.lexer.peek_token().range.clone());
      return_type = Some(self.parse_type(false));
    }

    let body = self.parse_block_statement(&[TokenKind::End]);
    let end_range = self.consume_expect_token(TokenKind::End).range;
    let range = create_middle_range(&start_range, &end_range);

    ast::Expression::new_function(arguments, return_type, body, range, range_return_type)
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
    while let Some((operator, range)) = self.parse_operator() {
      let right_expression = parse_sub_expression(self);
      let binary_exp = ast::BinaryExpression::new(operator, Box::new(expression), Box::new(right_expression), range);
      expression = ast::Expression::Binary(binary_exp);
    }
    expression
  }

  fn parse_unary_expression(&mut self) -> ast::Expression {
    let unary_range = self.lexer.peek_token().range.clone();
    if let Some(operator) = self.parse_unary_operator() {
      let expression = self.parse_expression_statement();
      let unary_expression = ast::UnaryExpression::new(operator, Box::new(expression), unary_range);
      ast::Expression::Unary(unary_expression)
    } else {
      self.parse_primary_expression()
    }
  }
  fn parse_primary_expression(&mut self) -> ast::Expression {
    let token = self.lexer.peek_token();
    let mut expression = match token.kind {
      TokenKind::Number(_) | TokenKind::String(_) => self.parse_literal_expression(),
      TokenKind::Identifier(_) => self.parse_identifier(),
      TokenKind::Nil | TokenKind::True | TokenKind::False => self.parse_literal_expression(),
      TokenKind::LeftParen => self.parse_grouped_expression(),
      TokenKind::Require => self.parse_require_expression(),
      TokenKind::Function => self.parse_function_expression(),
      TokenKind::LeftBrace => self.parse_table_expression(),
      _ => self.report_unexpected_token(token),
    };

    while self.match_token(&TokenKind::Dot) || self.match_token(&TokenKind::LeftBracket) {
      if self.match_token(&TokenKind::Dot) {
        expression = self.parse_member_expression(expression);
      } else if self.match_token(&TokenKind::LeftBracket) {
        expression = self.parse_index_expression(expression);
      }
    }
    expression
  }

  fn parse_member_expression(&mut self, base: ast::Expression) -> ast::Expression {
    self.consume_expect_token(TokenKind::Dot); // consume '.'
    let member_expression = self.parse_expression_statement();
    ast::Expression::new_member(base, member_expression)
  }

  fn parse_index_expression(&mut self, base: ast::Expression) -> ast::Expression {
    let start_range = self.consume_expect_token(TokenKind::LeftBracket).range; // consume '['
    let index_expression = self.parse_expression_statement();
    let end_range = self.consume_expect_token(TokenKind::RightBracket).range; // consume ']'

    let bracket_range = create_middle_range(&start_range, &end_range);

    ast::Expression::Index(ast::IndexExpression {
      base: Box::new(base),
      index: Box::new(index_expression),
      bracket_range,
    })
  }

  fn parse_table_expression(&mut self) -> ast::Expression {
    let left_range = self.consume_expect_token(TokenKind::LeftBrace).range;
    let mut values = vec![];
    while !self.match_token(&TokenKind::RightBrace) {
      let value_or_key = self.parse_expression_statement();
      if self.match_token_and_consume(TokenKind::Assign).is_some() {
        let value = self.parse_expression_statement();
        values.push((value_or_key, Some(value)));
      } else {
        values.push((value_or_key, None));
      }
      if self.match_token(&TokenKind::RightBrace) {
        break;
      }
      // skip comma
      self.consume_expect_token(TokenKind::Comma);
    }
    let right_range = self.consume_expect_token(TokenKind::RightBrace).range;
    let range = create_middle_range(&left_range, &right_range);
    ast::Expression::new_table(values, range)
  }

  fn parse_literal_expression(&mut self) -> ast::Expression {
    let token = self.lexer.next_token();
    match token.kind {
      TokenKind::Number(value) => ast::Expression::new_literal(ast::LiteralExpression::new_number(value, token.range)),
      TokenKind::String(value) => ast::Expression::new_literal(ast::LiteralExpression::new_string(value, token.range)),
      TokenKind::True => ast::Expression::new_literal(ast::LiteralExpression::new_bool(true, token.range)),
      TokenKind::False => ast::Expression::new_literal(ast::LiteralExpression::new_bool(false, token.range)),
      TokenKind::Nil => ast::Expression::new_literal(ast::LiteralExpression::new_nil(token.range)),
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_grouped_expression(&mut self) -> ast::Expression {
    let left_range = self.consume_expect_token(TokenKind::LeftParen).range;
    let mut expressions = vec![];

    if self.match_token(&TokenKind::RightParen) {
      let right_range = self.consume_expect_token(TokenKind::RightParen).range;
      return ast::Expression::new_grouped(expressions, create_middle_range(&left_range, &right_range));
    }

    expressions.push(self.parse_expression_statement());

    while self.match_token(&TokenKind::Comma) {
      self.consume_expect_token(TokenKind::Comma);
      expressions.push(self.parse_expression_statement());
    }
    let right_range = self.consume_expect_token(TokenKind::RightParen).range;

    let range = create_middle_range(&left_range, &right_range);
    return ast::Expression::new_grouped(expressions, range);
  }

  fn parse_require_expression(&mut self) -> ast::Expression {
    let require_token = self.consume_expect_token(TokenKind::Require);
    let module_name = self.consume_token();
    let range = require_token.range;
    ast::Expression::new_require(module_name, range)
  }

  fn parse_assign_expression_or_call_expression(&mut self) -> ast::Statement {
    let token = self.lexer.peek_token();
    match token.kind {
      TokenKind::Identifier(_) => self.parse_assign_statement_or_call_statement(token),
      TokenKind::LeftBrace => {
        let table_expression = self.parse_table_expression();
        ast::Statement::Expression(table_expression)
      }
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_call_expression(&mut self, ident: Token) -> ast::Expression {
    let args = self.parse_grouped_expression();
    ast::Expression::new_call(ident, args)
  }

  // statements
  //
  fn parse_block_statement(&mut self, end_tokens: &[TokenKind]) -> ast::Statement {
    let mut statements = Vec::new();
    // skip block comments
    self.skip_comments();
    while !self.contains_token(end_tokens) {
      statements.push(self.parse_statement());
      // skip block comments
      self.skip_comments();
    }
    ast::Statement::Block(ast::BlockStatement::new(statements))
  }

  fn parse_assign_statement_or_call_statement(&mut self, ident_token: Token) -> ast::Statement {
    self.lexer.next_token(); // consume identifier
    if self.match_token(&TokenKind::Assign) || self.match_token(&TokenKind::Colon) {
      self.parse_assign_statement(ident_token)
    } else if self.match_token(&TokenKind::LeftParen) {
      let expression = self.parse_call_expression(ident_token);
      ast::Statement::Expression(expression)
    } else {
      // support print "hello"...
      let peeked = self.lexer.peek_token();
      if ident_token.kind == TokenKind::Identifier("print".to_string()) && peeked.is_string() {
        let args = self.parse_expression_statement();
        let range = args.get_range();
        let args = ast::Expression::new_grouped(vec![args], range);
        let expression = ast::Expression::new_call(ident_token, args);
        ast::Statement::Expression(expression)
      } else {
        self.report_unexpected_token(ident_token)
      }
    }
  }

  fn parse_assign_statement(&mut self, ident_token: Token) -> ast::Statement {
    let values = self.parse_variable_and_type(Some(ident_token));
    self.consume_expect_token(TokenKind::Assign);
    let init = self.parse_expression_statement();
    ast::Statement::Assign(ast::AssignStatement::new(values, init))
  }

  fn parse_expression_statement(&mut self) -> ast::Expression {
    self.parse_or_expression()
  }

  // utils functions
  //

  pub fn parse_arguments_with_type(&mut self) -> Vec<Type> {
    let mut arguments = Vec::new();
    while !self.match_token(&TokenKind::RightParen) {
      // todo: what I should do here?
      let _ = self.consume_token();
      self.consume_expect_token(TokenKind::Colon);
      let ty = self.parse_type(false);
      arguments.push(ty);
      self.match_token_and_consume(TokenKind::Comma);
    }
    arguments
  }

  pub fn parse_function_type(&mut self) -> Type {
    self.consume_expect_token(TokenKind::Function);
    self.consume_expect_token(TokenKind::LeftParen);
    let params = self.parse_arguments_with_type();
    self.consume_expect_token(TokenKind::RightParen);
    self.consume_expect_token(TokenKind::Colon);
    let return_type = self.parse_type(true);
    return Type::new_function(params, return_type);
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

  fn parse_generic_type(&mut self) -> Vec<Type> {
    let mut types = Vec::new();

    if !self.match_token(&TokenKind::Less) {
      return types;
    }

    self.consume_expect_token(TokenKind::Less);
    while !self.match_token(&TokenKind::Greater) {
      let ty = self.parse_type(false);
      types.push(ty);
      self.match_token_and_consume(TokenKind::Comma);
    }
    self.consume_expect_token(TokenKind::Greater);
    types
  }

  fn parse_simple_generic_str(&mut self) -> Vec<String> {
    if !self.match_token(&TokenKind::Less) {
      return vec![];
    }
    self.consume_expect_token(TokenKind::Less);
    let mut generics = vec![];
    while !self.match_token(&TokenKind::Greater) {
      let name = self.consume_token();
      generics.push(name.lexeme().to_string());
      self.match_token_and_consume(TokenKind::Comma);
    }
    self.consume_expect_token(TokenKind::Greater);
    generics
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

  fn parse_operator(&mut self) -> Option<(ast::BinaryOperator, Range)> {
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
    return Some((operator, token.range));
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

  fn parse_identifier(&mut self) -> ast::Expression {
    let token = self.lexer.next_token();

    if self.match_token(&TokenKind::LeftParen) {
      return self.parse_call_expression(token);
    }

    match token.kind {
      TokenKind::Identifier(name) => ast::Expression::new_identifier(name, token.range),
      _ => self.report_unexpected_token(token),
    }
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

  fn parse_type(&mut self, excepted_paren: bool) -> Type {
    let token = self.lexer.peek_token();
    if !excepted_paren && token.kind == TokenKind::LeftParen {
      self.report_unexpected_token(token)
    }
    match token.kind {
      TokenKind::Identifier(name) => {
        self.lexer.next_token();
        if self.match_token(&TokenKind::Less) {
          self.consume_expect_token(TokenKind::Less);
          let mut types = Vec::new();
          while !self.match_token(&TokenKind::Greater) {
            let ty = self.parse_type(false);
            types.push(ty);
            self.match_token_and_consume(TokenKind::Comma);
          }
          let right_range = self.consume_expect_token(TokenKind::Greater).range;
          let range = create_middle_range(&token.range, &right_range);
          return Type::new_generic_call(name, types, range);
        }

        Type::new_type(&name, token.range)
      }
      TokenKind::LeftParen => self.parse_grup_return_type(),
      TokenKind::Function => self.parse_function_type(),
      _ => self.report_unexpected_token(token),
    }
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
    report_and_exit(&message, &mut token.range.clone(), &self.raw, &self.lexer.file_name)
  }
}
