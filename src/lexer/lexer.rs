use crate::ast::tokens::{Token, TokenKind};
use crate::diagnostics::report::report_and_exit;
use crate::utils::match_number;
use crate::utils::range::Range;

pub struct Lexer<'a> {
  raw: &'a str,
  column: usize,
  line: usize,
  cursor: usize,
  range_start: usize,
  peeked_token: Option<Token>,
  pub file_name: &'a str,
}

impl<'a> Lexer<'a> {
  pub fn new(raw: &'a str, file_name: &'a str) -> Lexer<'a> {
    Lexer { raw, column: 0, range_start: 0, line: 1, cursor: 0, peeked_token: None, file_name }
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
    self.update_current_range();
    if self.is_end() {
      return Token::new(TokenKind::EOF, self.create_range());
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
      '.' => self.read_dot(),
      _ => {
        let mut range = self.create_range();
        let message = format!("Invalid character '{}'", current_char);
        report_and_exit(&message, &mut range, &self.raw, &self.file_name);
      }
    }
  }

  fn read_dot(&mut self) -> Token {
    if self.starts_with("...") {
      self.advance_many(3);
      return Token::new(TokenKind::TripleDot, self.create_range());
    } else {
      self.read_check_ahead("..", TokenKind::Dot, TokenKind::DoubleDot)
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
    let range = self.create_range();
    Token::new(kind, range)
  }

  fn read_check_ahead(&mut self, expected: &str, single_kind: TokenKind, double_kind: TokenKind) -> Token {
    if self.starts_with(expected) {
      self.advance_many(expected.len());
      let range = self.create_range();
      return Token::new(double_kind, range);
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
    let range = self.create_range();
    Token::new_block_comment(range, text)
  }

  fn read_line_comment(&mut self) -> Token {
    self.consume_expect("--");
    let text = self.read_while(|c| c != '\n');
    let range = self.create_range();
    Token::new_comment(range, text)
  }

  fn read_keyword_or_identifier(&mut self) -> Token {
    let text = self.read_while(|c| c.is_ascii_alphabetic() || c == '_' || c == '$' || c.is_ascii_digit());
    let range = self.create_range();
    Token::new_keyword_or_identifier(range, text)
  }

  fn read_number(&mut self) -> Token {
    let number = self.read_while(match_number);
    let range = self.create_range();
    Token::new_number(range, number)
  }

  fn read_string(&mut self) -> Token {
    self.consume_expect("\"");
    let string = self.read_while(|c| c != '"');
    self.consume_expect_with_custom_error("\"", "unterminated string");
    let range = self.create_range();
    Token::new_string(range, string)
  }

  fn read_while(&mut self, mut test: impl FnMut(char) -> bool) -> String {
    let range_start = self.cursor;
    while !self.is_end() && test(self.peek_one()) {
      self.advance_one();
    }
    self.raw[range_start..self.cursor].to_string()
  }

  fn read_until(&mut self, end: &str) -> String {
    let range_start = self.cursor;
    while !self.is_end() && !self.starts_with(end) {
      let c = self.peek_one();
      if c == '\n' {
        self.advance_new_line();
      } else {
        self.advance_one();
      }
    }
    self.raw[range_start..self.cursor].to_string()
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

  fn create_range(&mut self) -> Range {
    let start = self.range_start;
    let end = self.cursor;
    self.range_start = self.cursor;
    Range { start, end }
  }

  fn update_current_range(&mut self) {
    self.range_start = self.cursor;
  }

  fn consume_expect(&mut self, text: &str) {
    if self.starts_with(text) {
      self.advance_many(text.len());
    } else {
      let range = self.create_range();
      panic!("expected '{}' at {:?}", text, range);
    }
  }

  fn consume_expect_with_custom_error(&mut self, text: &str, error_message: &str) {
    if self.starts_with(text) {
      self.advance_many(text.len());
    } else {
      let location = self.create_range();
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
