use crate::ast::tokens::{Token, TokenKind};
use crate::diagnostics::report::report_and_exit;
use crate::utils::location::{Location, Position};
use crate::utils::match_number;

pub struct Lexer<'a> {
  raw: &'a str,
  column: usize,
  line: usize,
  cursor: usize,
  start_cursor: usize,
  pub current_position: Position,
  peeked_token: Option<Token>,
  pub file_name: &'a str,
}

impl<'a> Lexer<'a> {
  pub fn new(raw: &'a str, file_name: &'a str) -> Lexer<'a> {
    Lexer {
      raw,
      column: 0,
      start_cursor: 0,
      line: 1,
      cursor: 0,
      current_position: Position { line: 1, column: 0 },
      peeked_token: None,
      file_name,
    }
  }

  pub fn peek_token(&mut self) -> Token {
    if self.peeked_token.is_none() {
      self.peeked_token = Some(self.read_next_token());
    }
    self.peeked_token.clone().unwrap()
  }

  pub fn next_token(&mut self) -> Token {
    if let Some(token) = self.peeked_token.take() {
      return token;
    }
    self.read_next_token()
  }

  fn read_next_token(&mut self) -> Token {
    self.skip_whitespace();
    self.update_current_position();
    if self.is_end() {
      return Token::new(TokenKind::EOF, self.create_location());
    }

    let current_char = self.peek_one();
    match current_char {
      '+' => self.read_check_ahead("+=", TokenKind::Plus, TokenKind::PlusAssign),
      '-' => self.read_comment_or_minus(),
      '*' => self.read_check_ahead("*=", TokenKind::Star, TokenKind::StarAssign),
      '=' => self.read_check_ahead("==", TokenKind::Assign, TokenKind::Equal),
      '~' => self.read_check_ahead("~=", TokenKind::Tilde, TokenKind::NotEqual),
      '<' => self.read_check_ahead("<=", TokenKind::Less, TokenKind::LessEqual),
      '>' => self.read_check_ahead(">=", TokenKind::Greater, TokenKind::GreaterEqual),
      '(' => self.read_simple_token(TokenKind::LeftParen),
      ')' => self.read_simple_token(TokenKind::RightParen),
      '%' => self.read_simple_token(TokenKind::Percent),
      '#' => self.read_simple_token(TokenKind::Hash),
      '.' => self.read_check_ahead("..", TokenKind::Dot, TokenKind::DoubleDot),
      ',' => self.read_simple_token(TokenKind::Comma),
      ':' => self.read_simple_token(TokenKind::Colon),
      ';' => self.read_simple_token(TokenKind::Semicolon),
      '{' => self.read_simple_token(TokenKind::LeftBrace),
      '}' => self.read_simple_token(TokenKind::RightBrace),
      '[' => self.read_simple_token(TokenKind::LeftBracket),
      ']' => self.read_simple_token(TokenKind::RightBracket),
      '"' => self.read_string(),
      '/' => self.read_slash(),
      '0'..='9' => self.read_number(),
      'a'..='z' | 'A'..='Z' | '_' => self.read_keyword_or_identifier(),
      _ => {
        let mut location = self.create_location();
        let message = format!("Invalid character '{}'", current_char);
        report_and_exit(&message, &mut location, &self.raw, &self.file_name);
      }
    }
  }

  fn read_slash(&mut self) -> Token {
    if self.starts_with("//") {
      self.read_check_ahead("//", TokenKind::DoubleSlash, TokenKind::DoubleSlash)
    } else {
      self.read_check_ahead("/=", TokenKind::Slash, TokenKind::SlashAssign)
    }
  }

  fn read_simple_token(&mut self, kind: TokenKind) -> Token {
    self.advance_one();
    let location = self.create_location();
    Token::new(kind, location)
  }

  fn read_check_ahead(&mut self, expected: &str, single_kind: TokenKind, double_kind: TokenKind) -> Token {
    if self.starts_with(expected) {
      self.advance_many(expected.len());
      let location = self.create_location();
      return Token::new(double_kind, location);
    }
    self.read_simple_token(single_kind)
  }

  fn read_comment_or_minus(&mut self) -> Token {
    if self.starts_with("--") {
      self.read_comment()
    } else {
      self.read_check_ahead("-=", TokenKind::Minus, TokenKind::MinusAssign)
    }
  }

  fn read_comment(&mut self) -> Token {
    if self.starts_with("--[[") {
      self.read_block_comment()
    } else {
      self.read_line_comment()
    }
  }

  fn read_block_comment(&mut self) -> Token {
    self.consume_expect("--[[");
    let text = self.read_until("--]]");
    self.consume_expect_with_custom_error("--]]", "unexpected end of block comment");
    let location = self.create_location();
    Token::new_block_comment(location, text)
  }

  fn read_line_comment(&mut self) -> Token {
    self.consume_expect("--");
    let text = self.read_while(|c| c != '\n');
    let location = self.create_location();
    Token::new_comment(location, text)
  }

  fn read_keyword_or_identifier(&mut self) -> Token {
    let text = self.read_while(|c| c.is_ascii_alphabetic() || c == '_');
    let location = self.create_location();
    Token::new_keyword_or_identifier(location, text)
  }

  fn read_number(&mut self) -> Token {
    let number = self.read_while(match_number);
    let location = self.create_location();
    Token::new_number(location, number)
  }

  fn read_string(&mut self) -> Token {
    self.consume_expect("\"");
    let string = self.read_while(|c| c != '"');
    self.consume_expect_with_custom_error("\"", "unterminated string");
    let location = self.create_location();
    Token::new_string(location, string)
  }

  fn read_while(&mut self, mut test: impl FnMut(char) -> bool) -> String {
    let start_cursor = self.cursor;
    while !self.is_end() && test(self.peek_one()) {
      self.advance_one();
    }
    self.raw[start_cursor..self.cursor].to_string()
  }

  fn read_until(&mut self, end: &str) -> String {
    let start_cursor = self.cursor;
    while !self.is_end() && !self.starts_with(end) {
      let c = self.peek_one();
      if c == '\n' {
        self.advance_new_line();
      } else {
        self.advance_one();
      }
    }
    self.raw[start_cursor..self.cursor].to_string()
  }

  fn advance_one(&mut self) {
    if let Some(c) = self.raw[self.cursor..].chars().next() {
      self.cursor += c.len_utf8();
      self.column += 1;
      if c == '\n' {
        self.line += 1;
        self.column = 0;
      }
    }
  }

  fn create_location(&mut self) -> Location {
    let start = self.current_position.clone();
    let range_start = self.start_cursor;
    let range_end = self.cursor;
    self.start_cursor = self.cursor;
    let end = Position { line: self.line, column: self.column };
    Location { start, end, rage_start: range_start, rage_end: range_end }
  }

  fn update_current_position(&mut self) {
    self.current_position = Position { line: self.line, column: self.column };
    self.start_cursor = self.cursor;
  }

  fn consume_expect(&mut self, text: &str) {
    if self.starts_with(text) {
      self.advance_many(text.len());
    } else {
      let location = self.create_location();
      panic!("expected '{}' at {:?}", text, location);
    }
  }

  fn consume_expect_with_custom_error(&mut self, text: &str, error_message: &str) {
    if self.starts_with(text) {
      self.advance_many(text.len());
    } else {
      let location = self.create_location();
      panic!("{} at {:?}", error_message, location);
    }
  }

  fn is_end(&self) -> bool {
    self.cursor >= self.raw.len()
  }

  fn peek_one(&self) -> char {
    self.raw[self.cursor..].chars().next().unwrap_or('\0')
  }

  fn advance_many(&mut self, count: usize) {
    for _ in 0..count {
      self.advance_one();
    }
  }

  fn starts_with(&self, s: &str) -> bool {
    self.raw[self.cursor..].starts_with(s)
  }

  fn advance_new_line(&mut self) {
    self.line += 1;
    self.column = 0;
    self.cursor += 1;
  }

  fn skip_whitespace(&mut self) {
    while !self.is_end() && self.peek_one().is_whitespace() {
      if self.peek_one() == '\n' {
        self.advance_new_line();
      } else {
        self.advance_one();
      }
    }
  }
}
