// #![allow(dead_code)]

use std::collections::{BTreeMap, HashSet};

use super::precedence::Precedence;
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
      TokenKind::Function => self.parse_function_declaration(None),
      TokenKind::Type => self.parse_type_declaration(),
      _ => self.parse_expression_statement(),
    };
    self.match_token_and_consume(TokenKind::Semicolon);
    statement
  }

  fn parse_local_declaration(&mut self) -> ast::Statement {
    let local = self.consume_expect_token(TokenKind::Local);
    if self.match_token(&TokenKind::Function) {
      return self.parse_function_declaration(Some(local.range));
    }
    self.parse_local_variable(local.range)
  }

  fn parse_local_variable(&mut self, local_range: Range) -> ast::Statement {
    let mut variables = vec![self.parse_variable()];
    let mut end_range = variables.first().unwrap().get_range();

    while self.match_token_and_consume(TokenKind::Comma).is_some() {
      let variable = self.parse_variable();
      if !self.match_token(&TokenKind::Comma) {
        end_range = variable.get_range();
      }
      variables.push(variable)
    }

    let mut initializer = vec![];

    if self.match_token_and_consume(TokenKind::Assign).is_some() {
      initializer.push(self.parse_expression());
    }

    while self.match_token(&TokenKind::Comma) {
      self.consume_expect_token(TokenKind::Comma);
      let expression = self.parse_expression();

      if !self.match_token(&TokenKind::Comma) {
        end_range = expression.get_range();
      }

      initializer.push(expression)
    }

    let range = create_middle_range(&local_range, &end_range);

    let local = ast::LocalStatement::new(variables, initializer, range);

    return ast::Statement::Local(local);
  }

  fn parse_variables(&mut self) -> Vec<ast::Variable> {
    let mut variables = vec![];
    let peeked = self.lexer.peek_token();

    if !peeked.is_identifier() {
      return variables;
    }

    variables.push(self.parse_variable());

    while self.match_token_and_consume(TokenKind::Comma).is_some() {
      variables.push(self.parse_variable());
    }

    variables
  }
  fn parse_variable(&mut self) -> ast::Variable {
    let name = self.lexer.next_token();

    if !name.is_identifier() {
      self.report_unexpected_token(name);
    }

    let ty = if self.match_token_and_consume(TokenKind::Colon).is_some() {
      Some(self.parse_type(true))
    } else {
      None
    };

    ast::Variable::new(name, ty)
  }

  fn parse_type_declaration(&mut self) -> ast::Statement {
    let range = self.consume_expect_token(TokenKind::Type).range.clone();
    let name = self.consume_token();
    let generics = self.parse_generic_type_names();
    self.consume_expect_token(TokenKind::Assign);
    let initializer = self.parse_type(false);
    ast::Statement::TypeDeclaration(ast::TypeDeclaration::new(name, generics, initializer, range))
  }

  fn parse_function_declaration(&mut self, local_range: Option<Range>) -> ast::Statement {
    let function_keyword = self.consume_expect_token(TokenKind::Function);

    let local = local_range.is_some();

    let start_range = local_range.unwrap_or(function_keyword.range);
    let name = self.consume_token();

    if !name.is_identifier() {
      self.report_unexpected_token(name);
    }

    let generics = self.parse_generic_types();

    self.consume_expect_token(TokenKind::LeftParen);

    let parameters = self.parse_variables();

    self.consume_expect_token(TokenKind::RightParen);

    let mut return_type = None;
    let mut return_type_range = None;

    if self.match_token_and_consume(TokenKind::Colon).is_some() {
      return_type_range = Some(self.lexer.peek_token().range.clone());
      return_type = Some(self.parse_type(true));
    }
    let body = self.parse_block_statement(&[TokenKind::End]);
    let end_range = self.consume_expect_token(TokenKind::End).range;
    let range = create_middle_range(&start_range, &end_range);
    let function =
      ast::FunctionStatement::new(name, local, generics, parameters, return_type, body, range, return_type_range);
    ast::Statement::new_function(function)
  }

  fn parse_expression_statement(&mut self) -> ast::Statement {
    let expression = self.parse_expression();
    ast::Statement::Expression(expression)
  }

  fn parse_if_statement(&mut self) -> ast::Statement {
    let start_range = self.consume_expect_token(TokenKind::If).range;
    let condition = self.parse_expression();
    self.consume_expect_token(TokenKind::Then);
    let then_body = self.parse_block_statement(&[TokenKind::Else, TokenKind::ElseIf, TokenKind::End]);

    let mut else_if: Vec<ast::ElseIfStatement> = Vec::new();

    while self.match_token(&TokenKind::ElseIf) {
      let start_range = self.consume_expect_token(TokenKind::ElseIf).range;
      let condition = self.parse_expression();
      self.consume_expect_token(TokenKind::Then);
      let body = self.parse_block_statement(&[TokenKind::Else, TokenKind::ElseIf, TokenKind::End]);
      let body_range = body.get_range();
      let branch = ast::ElseIfStatement::new(condition, body, create_middle_range(&start_range, &body_range));
      else_if.push(branch);
    }

    let else_body = if self.match_token_and_consume(TokenKind::Else).is_some() {
      Some(self.parse_block_statement(&[TokenKind::End]))
    } else {
      None
    };
    let end_range = self.consume_expect_token(TokenKind::End).range;
    let range = create_middle_range(&start_range, &end_range);
    ast::Statement::If(ast::IfStatement::new(condition, then_body, else_if, else_body, range))
  }

  fn parse_while_statement(&mut self) -> ast::Statement {
    let while_token = self.consume_expect_token(TokenKind::While);
    let condition = self.parse_expression();
    self.consume_expect_token(TokenKind::Do);
    let body = self.parse_block_statement(&[TokenKind::End]);
    self.consume_expect_token(TokenKind::End);
    ast::Statement::While(ast::WhileStatement::new(condition, body, while_token.range))
  }

  fn parse_repeat_statement(&mut self) -> ast::Statement {
    let repeat_token = self.consume_expect_token(TokenKind::Repeat);
    let body = self.parse_block_statement(&[TokenKind::Until]);
    self.consume_expect_token(TokenKind::Until);
    let condition = self.parse_expression();
    ast::Statement::Repeat(ast::RepeatStatement::new(body, condition, repeat_token.range))
  }

  fn parse_for_statement(&mut self) -> ast::Statement {
    let start_range = self.consume_expect_token(TokenKind::For).range;
    // todo: I think this is wrong... :(
    let init = self.parse_simple_assignment();
    self.consume_expect_token(TokenKind::Comma);
    let limit = self.parse_expression();
    let step = if self.match_token_and_consume(TokenKind::Comma).is_some() {
      Some(self.parse_expression())
    } else {
      None
    };
    self.consume_expect_token(TokenKind::Do);
    let body = self.parse_block_statement(&[TokenKind::End]);
    let end_range = self.consume_expect_token(TokenKind::End).range;
    let range = create_middle_range(&start_range, &end_range);
    ast::Statement::For(ast::ForStatement::new(init, limit, step, body, range))
  }

  fn parse_simple_assignment(&mut self) -> ast::AssignExpresion {
    let ident = self.parse_identifier();
    let range = ident.range.clone();
    let ident_expression = ast::Expression::Identifier(ident);
    self.consume_expect_token(TokenKind::Assign);
    let value = self.parse_primary_expression();
    ast::AssignExpresion::new(vec![ident_expression], vec![value], range)
  }

  fn parse_break_statement(&mut self) -> ast::Statement {
    let break_token = self.consume_expect_token(TokenKind::Break);
    ast::Statement::Break(ast::BreakStatement::new(break_token.range))
  }

  fn parse_continue_statement(&mut self) -> ast::Statement {
    let continue_token = self.consume_expect_token(TokenKind::Continue);
    ast::Statement::Continue(ast::ContinueStatement::new(continue_token.range))
  }

  fn parse_return_statement(&mut self) -> ast::Statement {
    let return_token = self.consume_expect_token(TokenKind::Return);
    let values = self.parse_return_values();
    ast::Statement::Return(ast::ReturnStatement::new(values, return_token.range))
  }

  fn parse_function_expression(&mut self) -> ast::Expression {
    let start_range = self.consume_expect_token(TokenKind::Function).range;
    self.consume_expect_token(TokenKind::LeftParen);
    let parameters = self.parse_variables();
    self.consume_expect_token(TokenKind::RightParen);

    let mut return_type = None;
    let mut return_type_range = None;
    if self.match_token_and_consume(TokenKind::Colon).is_some() {
      return_type_range = Some(self.lexer.peek_token().range.clone());
      return_type = Some(self.parse_type(false));
    }

    let body = self.parse_block_statement(&[TokenKind::End]);
    let end_range = self.consume_expect_token(TokenKind::End).range;
    let range = create_middle_range(&start_range, &end_range);

    ast::Expression::new_function(parameters, return_type, body, range, return_type_range)
  }

  fn parse_expression(&mut self) -> ast::Expression {
    self.parse_precedence(Precedence::Assignment)
  }

  // ! this is a recursive function :(... I think is better than the previous one
  fn parse_precedence(&mut self, precedence: Precedence) -> ast::Expression {
    let mut left = if precedence == Precedence::Unary {
      self.parse_unary_expression()
    } else {
      self.parse_precedence(precedence.next())
    };
    while let Some(token) = self.match_any_token(precedence.operators()) {
      let operator = self.token_to_binary_operator(&token);
      let right = self.parse_precedence(precedence.next());
      left = ast::Expression::new_binary(operator, left, right, token.range);
    }

    left
  }

  // fn parse_expression(&mut self) -> ast::Expression {
  //   self.parse_assignment_expression()
  // }

  // fn parse_assignment_expression(&mut self) -> ast::Expression {
  //   let expression = self.parse_or_expression();

  //   if self.match_token(&TokenKind::Assign) {
  //     let assign_token = self.consume_token();
  //     let value = self.parse_expression();
  //     return ast::Expression::new_assign(vec![expression], vec![value], assign_token.range);
  //   }

  //   expression
  // }

  // fn parse_or_expression(&mut self) -> ast::Expression {
  //   self.parse_binary_expression(Self::parse_and_expression, &[TokenKind::Or])
  // }

  // fn parse_and_expression(&mut self) -> ast::Expression {
  //   self.parse_binary_expression(Self::parse_equality_expression, &[TokenKind::And])
  // }

  // fn parse_equality_expression(&mut self) -> ast::Expression {
  //   self.parse_binary_expression(Self::parse_comparison_expression, &[TokenKind::Equal, TokenKind::NotEqual])
  // }

  // fn parse_comparison_expression(&mut self) -> ast::Expression {
  //   self.parse_binary_expression(
  //     Self::parse_term,
  //     &[TokenKind::Less, TokenKind::LessEqual, TokenKind::Greater, TokenKind::GreaterEqual],
  //   )
  // }

  // fn parse_term(&mut self) -> ast::Expression {
  //   self.parse_binary_expression(Self::parse_factor, &[TokenKind::Plus, TokenKind::Minus])
  // }

  // fn parse_factor(&mut self) -> ast::Expression {
  //   self.parse_binary_expression(Self::parse_unary_expression, &[TokenKind::Star, TokenKind::Slash, TokenKind::Percent])
  // }

  // fn parse_binary_expression(
  //   &mut self,
  //   parse_sub_expression: fn(&mut Self) -> ast::Expression,
  //   operator_kinds: &[TokenKind],
  // ) -> ast::Expression {
  //   let mut left = parse_sub_expression(self);

  //   while let Some(token) = self.match_any_token(operator_kinds) {
  //     let operator = self.token_to_binary_operator(&token);
  //     let right = parse_sub_expression(self);
  //     left = ast::Expression::new_binary(operator, left, right, token.range);
  //   }

  //   left
  // }

  fn parse_unary_expression(&mut self) -> ast::Expression {
    if let Some(token) = self.match_any_token(&[TokenKind::Minus, TokenKind::Not, TokenKind::Hash]) {
      let operator = self.token_to_unary_operator(&token);
      let expr = self.parse_unary_expression();
      ast::Expression::new_unary(operator, expr, token.range)
    } else {
      self.parse_primary_expression()
    }
  }

  fn token_to_binary_operator(&self, token: &Token) -> ast::BinaryOperator {
    match token.kind {
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
      _ => self.report_unexpected_token(token.clone()),
    }
  }

  fn token_to_unary_operator(&self, token: &Token) -> ast::UnaryOperator {
    match token.kind {
      TokenKind::Minus => ast::UnaryOperator::Negate,
      TokenKind::Not => ast::UnaryOperator::Not,
      TokenKind::Hash => ast::UnaryOperator::Hash,
      _ => self.report_unexpected_token(token.clone()),
    }
  }

  fn parse_primary_expression(&mut self) -> ast::Expression {
    let token = self.lexer.peek_token();
    let mut expression = match token.kind {
      TokenKind::Number(_) | TokenKind::String(_) => self.parse_literal_expression(),
      TokenKind::Identifier(_) => self.parse_identifier_expression(),
      TokenKind::Nil | TokenKind::True | TokenKind::False => self.parse_literal_expression(),
      TokenKind::LeftParen => self.parse_grouped_expression(),
      TokenKind::Require => self.parse_require_expression(),
      TokenKind::Function => self.parse_function_expression(),
      TokenKind::LeftBrace => self.parse_table_expression(),
      _ => self.report_unexpected_token(token),
    };

    // index expression and member expression
    while self.match_token(&TokenKind::Dot) || self.match_token(&TokenKind::LeftBracket) {
      if self.match_token(&TokenKind::Dot) {
        expression = self.parse_member_expression(expression);
      } else if self.match_token(&TokenKind::LeftBracket) {
        expression = self.parse_index_expression(expression);
      }
    }
    // call expression
    while self.match_token(&TokenKind::LeftParen) {
      expression = self.parse_call_expression(Some(expression));
    }

    // assign expression
    if self.match_token(&TokenKind::Assign) || self.match_token(&TokenKind::Colon) {
      return self.parse_assign_expression(Some(expression));
    }

    // call expression
    // if self.match_token(&TokenKind::LeftParen) {
    //   return self.parse_call_expression(Some(expression));
    // }
    expression
  }

  fn parse_member_expression(&mut self, base: ast::Expression) -> ast::Expression {
    self.consume_expect_token(TokenKind::Dot); // consume '.'
    let member_expression = self.parse_identifier();
    ast::Expression::new_member(base, member_expression)
  }

  fn parse_index_expression(&mut self, base: ast::Expression) -> ast::Expression {
    let start_range = self.consume_expect_token(TokenKind::LeftBracket).range; // consume '['
    let index_expression = self.parse_expression();
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
      let key_or_value = self.parse_expression();
      if self.match_token_and_consume(TokenKind::Assign).is_some() {
        let value = self.parse_expression();
        values.push((key_or_value, Some(value)));
      } else {
        values.push((key_or_value, None));
      }
      if self.match_token(&TokenKind::RightBrace) {
        break;
      }
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

    expressions.push(self.parse_expression());

    while self.match_token(&TokenKind::Comma) {
      self.consume_expect_token(TokenKind::Comma);
      expressions.push(self.parse_expression());
    }
    let right_range = self.consume_expect_token(TokenKind::RightParen).range;

    let range = create_middle_range(&left_range, &right_range);
    ast::Expression::new_grouped(expressions, range)
  }

  fn parse_require_expression(&mut self) -> ast::Expression {
    let require_token = self.consume_expect_token(TokenKind::Require);
    let module_name = self.consume_token();
    let range = require_token.range;
    ast::Expression::new_require(module_name, range)
  }

  fn parse_call_expression(&mut self, left: Option<ast::Expression>) -> ast::Expression {
    let left = left.unwrap_or_else(|| self.parse_expression());
    let args = self.parse_grouped_expression();
    ast::Expression::new_call(left, args)
  }

  fn parse_block_statement(&mut self, end_tokens: &[TokenKind]) -> ast::Statement {
    let mut statements = Vec::new();
    self.skip_comments();
    while !self.contains_token(end_tokens) {
      statements.push(self.parse_statement());
      self.skip_comments();
    }
    ast::Statement::Block(ast::BlockStatement::new(statements))
  }

  fn parse_assign_expression(&mut self, left: Option<ast::Expression>) -> ast::Expression {
    let mut variables = vec![left.unwrap_or_else(|| self.parse_expression())];
    let start_range = variables.first().unwrap().get_range();

    while self.match_token_and_consume(TokenKind::Comma).is_some() {
      variables.push(self.parse_expression());
    }

    let mut end_range = variables.last().unwrap().get_range();

    self.consume_expect_token(TokenKind::Assign); // consume '='

    let mut initializer = vec![self.parse_expression()];

    while self.match_token_and_consume(TokenKind::Comma).is_some() {
      let expression = self.parse_expression();
      if !self.match_token(&TokenKind::Comma) {
        end_range = expression.get_range();
      }
      initializer.push(expression)
    }

    let range = create_middle_range(&start_range, &end_range);

    ast::Expression::new_assign(variables, initializer, range)
  }

  pub fn parse_function_type(&mut self) -> Type {
    self.consume_expect_token(TokenKind::Function);
    self.consume_expect_token(TokenKind::LeftParen);
    let params = self.parse_parameters_with_type();
    self.consume_expect_token(TokenKind::RightParen);
    self.consume_expect_token(TokenKind::Colon);
    let return_type = self.parse_type(true);
    Type::new_function(params, return_type)
  }

  pub fn parse_parameters_with_type(&mut self) -> Vec<Type> {
    let mut parameters = Vec::new();
    while !self.match_token(&TokenKind::RightParen) {
      let _ = self.consume_token();
      self.consume_expect_token(TokenKind::Colon);
      let ty = self.parse_type(false);
      parameters.push(ty);
      self.match_token_and_consume(TokenKind::Comma);
    }
    parameters
  }

  fn parse_nill_type(&mut self) -> Type {
    self.consume_token();
    return Type::Nil;
  }

  fn parse_group_return_type(&mut self) -> Type {
    self.consume_expect_token(TokenKind::LeftParen);
    let mut types = Vec::new();
    while !self.match_token(&TokenKind::RightParen) {
      types.push(self.parse_type(false));
      self.match_token_and_consume(TokenKind::Comma);
    }
    self.consume_expect_token(TokenKind::RightParen);
    Type::new_group(types)
  }

  fn parse_generic_types(&mut self) -> Vec<Type> {
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

  fn parse_generic_type_names(&mut self) -> Vec<String> {
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

  fn parse_return_values(&mut self) -> Vec<ast::Expression> {
    let mut values = Vec::new();
    if self.match_token(&TokenKind::Semicolon) {
      return values;
    }
    values.push(self.parse_expression());
    while self.match_token_and_consume(TokenKind::Comma).is_some() {
      values.push(self.parse_expression());
    }
    values
  }

  fn parse_identifier(&mut self) -> ast::Identifier {
    let token = self.lexer.next_token();
    match token.kind {
      TokenKind::Identifier(name) => ast::Identifier::new(name, token.range),
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_identifier_expression(&mut self) -> ast::Expression {
    let token = self.lexer.next_token();

    if !token.is_identifier() {
      self.report_unexpected_token(token);
    }

    ast::Expression::new_identifier(token.lexeme().to_owned(), token.range)
  }

  fn parse_identifier_type(&mut self) -> Type {
    let token = self.lexer.next_token();
    match token.kind {
      TokenKind::Identifier(name) => {
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
        Type::new(&name, token.range)
      }
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_type(&mut self, allow_parenthesis: bool) -> Type {
    let token = self.lexer.peek_token();
    if !allow_parenthesis && token.kind == TokenKind::LeftParen {
      self.report_unexpected_token(token)
    }
    match token.kind {
      TokenKind::Identifier(_) => self.parse_identifier_type(),
      TokenKind::Nil => self.parse_nill_type(),
      TokenKind::LeftParen => self.parse_group_return_type(),
      TokenKind::Function => self.parse_function_type(),
      TokenKind::LeftBrace => self.parse_table_type(),
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_table_type(&mut self) -> Type {
    self.consume_expect_token(TokenKind::LeftBrace);
    let mut array_elements = HashSet::new();
    let mut map_elements = BTreeMap::new();
    while !self.match_token(&TokenKind::RightBrace) {
      let type_or_key = self.parse_type(false);
      let peeked = self.lexer.peek_token();
      match (&type_or_key, &peeked.kind) {
        (Type::Alias(identifier), &TokenKind::Colon) => {
          self.consume_expect_token(TokenKind::Colon);
          let value_type = self.parse_type(false);
          map_elements.insert(identifier.name.to_string(), value_type);
        }
        _ => {
          array_elements.insert(type_or_key);
        }
      }
      if self.match_token(&TokenKind::RightBrace) {
        break;
      }
      self.consume_expect_token(TokenKind::Comma);
    }

    self.consume_expect_token(TokenKind::RightBrace);
    let array = if array_elements.is_empty() { None } else { Some(array_elements) };
    let map = if map_elements.is_empty() { None } else { Some(map_elements) };
    Type::new_table(array, map)
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

  fn match_any_token(&mut self, kinds: &[TokenKind]) -> Option<Token> {
    let token = self.lexer.peek_token();
    if kinds.contains(&token.kind) {
      Some(self.consume_token())
    } else {
      None
    }
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
