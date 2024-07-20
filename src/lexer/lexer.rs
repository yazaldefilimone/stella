use crate::{
  ast::tokens::{Token, TokenKind},
  diagnostics::report_error,
  utils::{
    location::{Location, Position},
    match_number,
  },
};

pub struct Lexer {
  raw: String,   // lua code
  column: usize, // column of the current character
  line: usize,   // line of the current character
  cursor: usize, // current character
  pub current_position: Position,
  peeked_token: Option<Token>,
}

impl Lexer {
  pub fn new(raw: String) -> Lexer {
    let current_position = Position { line: 1, column: 0 };
    let peeked_token = None;
    Lexer { raw, column: 0, line: 1, cursor: 0, current_position, peeked_token }
  }

  pub fn peek_token(&mut self) -> Token {
    if self.peeked_token.is_none() {
      self.peeked_token = Some(self.read_next_token());
    }
    let token = self.peeked_token.clone().unwrap();
    return token;
  }

  pub fn next_token(&mut self) -> Token {
    if self.peeked_token.is_none() {
      return self.read_next_token();
    }
    let token = self.peeked_token.clone().unwrap();
    self.peeked_token = None;
    return token;
  }
  fn read_next_token(&mut self) -> Token {
    self.skip_whitespace();
    if self.is_end() {
      return Token::new(TokenKind::EOF, self.create_location());
    }
    let current_char = self.peek_one();
    let current_token = match current_char {
      '+' => self.read_check_ahead("+=", TokenKind::Plus, TokenKind::PlusAssign),
      '-' => self.read_check_ahead("-=", TokenKind::Minus, TokenKind::MinusAssign),
      '*' => self.read_check_ahead("*=", TokenKind::Star, TokenKind::StarAssign),
      '/' => self.read_check_ahead("/=", TokenKind::Slash, TokenKind::SlashAssign),
      '=' => self.read_check_ahead("=", TokenKind::Assign, TokenKind::Equal),
      '~' => self.read_check_ahead("~=", TokenKind::Tilde, TokenKind::NotEqual),
      '<' => self.read_check_ahead("=", TokenKind::Less, TokenKind::LessEqual),
      '>' => self.read_check_ahead("=", TokenKind::Greater, TokenKind::GreaterEqual),
      '(' => self.read_simple_token(TokenKind::LeftParen),
      ')' => self.read_simple_token(TokenKind::RightParen),
      '%' => self.read_simple_token(TokenKind::Percent),
      '#' => self.read_simple_token(TokenKind::Hash),
      '.' => self.read_check_ahead(".", TokenKind::Dot, TokenKind::DoubleDot),
      ',' => self.read_simple_token(TokenKind::Comma),
      ':' => self.read_simple_token(TokenKind::Colon),
      ';' => self.read_simple_token(TokenKind::Semicolon),
      '{' => self.read_simple_token(TokenKind::LeftBrace),
      '}' => self.read_simple_token(TokenKind::RightBrace),
      '[' => self.read_simple_token(TokenKind::LeftBracket),
      ']' => self.read_simple_token(TokenKind::RightBracket),
      '"' => self.read_string(),
      '0'..='9' => self.read_number(),
      'a'..='z' | 'A'..='Z' | '_' => self.read_keyword_or_identifier(),
      _ => {
        let mut location = self.create_location();
        let message = format!("Invalid character '{}'", current_char);
        report_error(&message, &mut location, &self.raw);
      }
    };

    return current_token;
  }

  fn read_simple_token(&mut self, kind: TokenKind) -> Token {
    self.advance_one();
    let location = self.create_location();
    Token::new(kind, location)
  }

  fn read_check_ahead(&mut self, next: &str, single_kind: TokenKind, double_kind: TokenKind) -> Token {
    if self.starts_with(next) {
      self.advance_many(next.len());
      let location = self.create_location();
      Token::new(double_kind, location)
    } else {
      self.advance_one();
      let location = self.create_location();
      Token::new(single_kind, location)
    }
  }

  fn read_keyword_or_identifier(&mut self) -> Token {
    let text = self.read_while(|c| c.is_alphabetic() || c == '_');
    let location = self.create_location();
    Token::new_keyword_or_identifier(location, text)
  }

  fn read_number(&mut self) -> Token {
    let number = self.read_while(|character| match_number(character));
    let location = self.create_location();
    Token::new_number(location, number)
  }

  fn read_string(&mut self) -> Token {
    self.consume_expect("\"");
    let string = self.read_while(|c| c != '"');
    self.consume_expect_with_custom_error("\"", "undetermined string");
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

  fn advance_one(&mut self) {
    self.cursor += 1;
    self.column += 1;
  }

  fn create_location(&mut self) -> Location {
    let start = self.current_position.clone();
    self.current_position = Position { line: self.line, column: self.column };
    Location { start, end: self.current_position.clone() }
  }

  fn consume_expect(&mut self, text: &str) {
    if &self.peek_many(text.len()) == text {
      self.advance_many(text.len());
    } else {
      let location = self.create_location();
      panic!("Expected '{}' at {:?}", text, location);
    }
  }

  fn consume_expect_with_custom_error(&mut self, text: &str, error_message: &str) {
    if &self.peek_many(text.len()) == text {
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
    if self.is_end() {
      return '\0';
    }
    self.raw[self.cursor..].chars().next().unwrap()
  }

  fn peek_many(&self, count: usize) -> String {
    if self.is_end() {
      return "".to_string();
    }
    self.raw[self.cursor..].chars().take(count).collect()
  }

  fn advance_many(&mut self, count: usize) {
    self.cursor += count;
    self.column += count;
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
    let current_char = self.peek_one();
    if self.is_end() || !current_char.is_whitespace() {
      return;
    }
    if current_char == '\n' {
      self.advance_new_line();
      return self.skip_whitespace();
    }
    if current_char.is_whitespace() {
      self.advance_one();
      return self.skip_whitespace();
    }
    self.current_position = Position { line: self.line, column: self.column };
  }
}
