use std::process::exit;

use crate::{
  ast::tokens::{Token, TokenKind},
  diagnostics::diagnostics::Diagnostic,
  utils::location::{Location, Position},
};

pub struct Lexer {
  pub raw: String,   // raw lua code
  pub column: usize, // column of the current character
  pub line: usize,   // line of the current character
  pub cursor: usize, // current character
  pub diagnostics: Diagnostic,
  pub current_position: Position,
}

impl Lexer {
  pub fn new(raw: String) -> Lexer {
    let diagnostics = Diagnostic::new();
    let current_position = Position { line: 1, column: 0 };
    Lexer { raw, column: 0, line: 1, cursor: 0, diagnostics, current_position }
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_trivial();
    if self.is_end() {
      let end_position = self.create_position();
      return Token::create(TokenKind::EOF, self.create_location(end_position));
    }
    let current_token = match self.peek_one() {
      '+' => self.lex_operator(TokenKind::Plus, "+"),
      '-' => self.lex_operator(TokenKind::Minus, "-"),
      '*' => self.lex_operator(TokenKind::Asterisk, "*"),
      '/' => self.lex_operator(TokenKind::Slash, "/"),
      '%' => self.lex_operator(TokenKind::Percent, "%"),
      '=' => self.lex_operator(TokenKind::Equal, "="),
      '<' => self.lex_operator(TokenKind::Less, "<"),
      '0'..='9' => self.lex_number(),
      '"' => self.lex_string(),
      _ => self.lex_identifier(),
    };
    return current_token;
  }

  fn lex_operator(&mut self, kind: TokenKind, text: &str) -> Token {
    self.consume_expect(text);
    let end_position = self.create_position();
    let location = self.create_location(end_position);
    return Token::create(kind, location);
  }

  fn lex_number(&mut self) -> Token {
    let number = self.consume_while(|c| c.is_digit(10));
    let end_position = self.create_position();
    let location = self.create_location(end_position);
    return Token::create_number(location, number.parse::<f64>().unwrap());
  }

  fn lex_string(&mut self) -> Token {
    self.consume_expect("\"");
    // let string = self.consume_while(|c| c != '"');
    self.consume_expect("allets");
    self.consume_expect("\"");
    let end_position = self.create_position();
    let location = self.create_location(end_position);
    return Token::create_string(location, "allets".to_string());
  }

  fn lex_identifier(&mut self) -> Token {
    let identifier = self.consume_while(|c| c.is_alphanumeric() || c == '_');
    let end_position = self.create_position();
    let location = self.create_location(end_position);
    return Token::create_identifier(location, identifier);
  }

  fn lex_keyword_or_identifier(&mut self) -> Token {
    let keyword = self.consume_while(|c| c.is_alphabetic() || c == '_');
    let end_position = self.create_position();
    let location = self.create_location(end_position);
    return Token::create_keyword_or_identifier(location, keyword);
  }

  // helpers

  fn skip_trivial(&mut self) {
    if self.is_end() {
      return;
    }
    /*
    1. if is whitespace, and increase column and cursor
    2. if is newline, increase line and reset column
    */
    let current_char = self.peek_one();
    if current_char == '\n' {
      self.line += 1;
      self.column = 0;
      self.advance_one();
      self.skip_trivial();
    }
    if current_char.is_whitespace() {
      self.column += 1;
      self.advance_one();
      self.skip_trivial();
    }
    return;
  }

  fn peek_one(&self) -> char {
    self.raw[self.cursor..].chars().next().unwrap()
  }

  fn peek_many(&self, count: usize) -> String {
    self.raw[self.cursor..].chars().take(count).collect()
  }

  fn starts_with(&self, s: &str) -> bool {
    self.raw[self.cursor..].starts_with(s)
  }

  fn is_end(&self) -> bool {
    self.cursor >= self.raw.len()
  }

  fn advance_one(&mut self) {
    self.cursor += 1;
    self.column += 1;
  }

  fn advance_many(&mut self, count: usize) {
    self.cursor += count;
    self.column += count;
  }

  fn consume(&mut self) -> char {
    let mut iter = self.raw[self.cursor..].char_indices();
    let (_, cur_char) = iter.next().unwrap();
    let (next_cursor, _) = iter.next().unwrap_or((1, ' '));
    self.cursor += next_cursor;
    cur_char
  }

  fn consume_expect(&mut self, text: &str) {
    if &self.peek_many(text.len()) == text {
      self.advance_many(text.len());
    } else {
      let message = format!("Expected '{}' but got '{}'", text, &self.peek_many(text.len()));
      self.advance_many(text.len());
      let end_position = self.create_position();
      let location = self.create_location(end_position);
      self.diagnostics.add_error(&message, location);
      self.diagnostics.display(&self.raw);
      std::process::exit(1);
    }
  }

  fn consume_while(&mut self, mut test: impl FnMut(char) -> bool) -> String {
    let start_cursor = self.cursor;
    while !self.is_end() && test(self.peek_one()) {
      self.advance_one();
    }
    self.raw[start_cursor..self.cursor].to_string()
  }
  fn create_location(&mut self, end_position: Position) -> Location {
    let previous_position = self.current_position.clone();
    self.current_position = end_position.clone();
    Location { start: previous_position, end: end_position }
  }
  fn create_position(&self) -> Position {
    Position { line: self.line, column: self.column }
  }
}
