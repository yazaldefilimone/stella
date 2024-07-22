#![allow(dead_code)]
use crate::{
  ast::{
    ast,
    tokens::{Token, TokenKind},
  },
  diagnostics::report_and_exit,
  lexer::Lexer,
  types::Type,
};

pub struct Parser {
  lexer: Lexer,
  pub raw: String,
}

impl Parser {
  pub fn new(raw: &str) -> Parser {
    let lexer = Lexer::new(raw.to_string());
    Parser { lexer, raw: raw.to_string() }
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
      TokenKind::Local => self.parse_local_statement(),
      TokenKind::If => self.parse_if_statement(),
      TokenKind::While => self.parse_while_statement(),
      TokenKind::Repeat => self.parse_repeat_statement(),
      TokenKind::For => self.parse_for_statement(),
      TokenKind::Break => self.parse_break_statement(),
      TokenKind::Continue => self.parse_continue_statement(),
      TokenKind::Return => self.parse_return_statement(),
      _ => self.parse_assign_statement(),
    };
    // end ;
    self.match_token_and_consume(&TokenKind::Semicolon);
    return statement;
  }

  fn parse_expression_statement(&mut self) -> ast::ExpressionStatement {
    let token = self.lexer.peek_token();
    match token.kind {
      TokenKind::Number(_) => self.parse_literal_expression(),
      TokenKind::String(_) => self.parse_literal_expression(),
      TokenKind::True => self.parse_literal_expression(),
      TokenKind::False => self.parse_literal_expression(),
      TokenKind::Identifier(_) => {
        let identifier = self.consume_token();
        ast::ExpressionStatement::new_identifier(identifier.lexeme(), identifier.location)
      }
      _ => {
        let mut location = token.location.clone();
        report_and_exit("Invalid expression statement", &mut location, &self.raw.as_str());
      }
    }
  }

  fn parse_literal_expression(&mut self) -> ast::ExpressionStatement {
    let token = self.lexer.next_token();
    match token.kind {
      TokenKind::Number(number) => ast::ExpressionStatement::new_number_literal(number, token.location),
      TokenKind::String(string) => ast::ExpressionStatement::new_string_literal(string, token.location),
      TokenKind::True => ast::ExpressionStatement::new_bool_literal(true, token.location),
      TokenKind::False => ast::ExpressionStatement::new_bool_literal(false, token.location),
      _ => {
        let mut location = token.location.clone();
        report_and_exit("Invalid literal expression", &mut location, &self.raw.as_str());
      }
    }
  }

  fn parse_assign_statement(&mut self) -> ast::Statement {
    let name = self.consume_token();
    self.consume_expect_token(TokenKind::Equal);
    let value = self.parse_expression_statement();
    let location = name.location.clone();
    ast::Statement::AssignStatement(ast::AssignStatement::new(name, value, location))
  }

  // function name(arg1: type1, arg2: type2): type
  // ....
  // end
  fn parse_function_statement(&mut self) -> ast::Statement {
    self.consume_expect_token(TokenKind::Function);
    let name = self.consume_token();
    self.consume_expect_token(TokenKind::LeftParen);
    let arguments = self.parse_arguments();
    self.consume_expect_token(TokenKind::RightParen);
    self.consume_expect_token(TokenKind::Colon);
    let return_type = self.parse_type();
    let body = self.parse_block_statement(TokenKind::End);
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
      _ => todo!("hei, please :) implement me"),
    }
  }

  fn parse_arguments(&mut self) -> Vec<(Token, Type)> {
    let mut arguments = vec![];
    while !self.match_token(&TokenKind::RightParen) {
      let name = self.consume_token();
      self.consume_expect_token(TokenKind::Colon);
      let type_ = self.parse_type();
      arguments.push((name, type_));
    }
    arguments
  }

  fn parse_local_statement(&mut self) -> ast::Statement {
    self.consume_expect_token(TokenKind::Local);
    let name = self.consume_token();
    let mut type_ = None;
    if let Some(_) = self.match_token_and_consume(&TokenKind::Colon) {
      type_ = Some(self.parse_type());
    }
    let mut init = None;
    if let Some(_) = self.match_token_and_consume(&TokenKind::Equal) {
      init = Some(self.parse_expression_statement());
    }

    let location = name.location.clone();

    let local = ast::LocalStatement::new(name, type_, init, location);

    ast::Statement::LocalStatement(local)
  }

  fn parse_block_statement(&mut self, end_token: TokenKind) -> ast::Statement {
    let mut statements = vec![];
    while !self.match_token(&end_token) {
      statements.push(self.parse_statement());
    }
    self.consume_expect_token(TokenKind::End);
    let location = self.lexer.peek_token().location.clone();
    ast::Statement::BlockStatement(ast::BlockStatement::new(statements, location))
  }

  fn parse_if_statement(&mut self) -> ast::Statement {
    todo!("hei, please :) implement me");
  }

  fn parse_while_statement(&mut self) -> ast::Statement {
    todo!("hei, please :) implement me");
  }

  fn parse_repeat_statement(&mut self) -> ast::Statement {
    todo!("hei, please :) implement me");
  }

  fn parse_for_statement(&mut self) -> ast::Statement {
    todo!("hei, please :) implement me");
  }

  fn parse_break_statement(&mut self) -> ast::Statement {
    todo!("hei, please :) implement me");
  }

  fn parse_continue_statement(&mut self) -> ast::Statement {
    todo!("hei, please :) implement me");
  }

  fn parse_return_statement(&mut self) -> ast::Statement {
    todo!("hei, please :) implement me");
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
    return &next_token.kind == kind;
  }

  fn match_token_and_consume(&mut self, kind: &TokenKind) -> Option<Token> {
    if self.match_token(kind) {
      return Some(self.consume_token());
    }
    return None;
  }

  fn is_end(&mut self) -> bool {
    let next_token = self.lexer.peek_token();
    return match next_token.kind {
      TokenKind::EOF => true,
      _ => false,
    };
  }
}
