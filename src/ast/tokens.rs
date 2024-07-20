#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::utils::location::Location;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
  pub kind: TokenKind,
  pub location: Location,
}

impl Token {
  pub fn new(kind: TokenKind, location: Location) -> Token {
    Token { kind, location }
  }

  pub fn new_end_of_file(location: Location) -> Token {
    Token::new(TokenKind::EOF, location)
  }

  pub fn new_identifier(location: Location, identifier: String) -> Token {
    Token::new(TokenKind::Identifier(identifier), location)
  }

  pub fn new_number(location: Location, number: f64) -> Token {
    Token::new(TokenKind::Number(number), location)
  }

  pub fn new_string(location: Location, string: String) -> Token {
    Token::new(TokenKind::String(string), location)
  }

  pub fn new_keyword_or_identifier(location: Location, keyword: String) -> Token {
    match keyword.as_str() {
      "function" => Token::new(TokenKind::Function, location),
      "local" => Token::new(TokenKind::Local, location),
      "if" => Token::new(TokenKind::If, location),
      "then" => Token::new(TokenKind::Then, location),
      "else" => Token::new(TokenKind::Else, location),
      "elseif" => Token::new(TokenKind::ElseIf, location),
      "end" => Token::new(TokenKind::End, location),
      "while" => Token::new(TokenKind::While, location),
      "do" => Token::new(TokenKind::Do, location),
      "for" => Token::new(TokenKind::For, location),
      "in" => Token::new(TokenKind::In, location),
      "repeat" => Token::new(TokenKind::Repeat, location),
      "until" => Token::new(TokenKind::Until, location),
      "return" => Token::new(TokenKind::Return, location),
      "break" => Token::new(TokenKind::Break, location),
      "true" => Token::new(TokenKind::True, location),
      "false" => Token::new(TokenKind::False, location),
      "nil" => Token::new(TokenKind::Nil, location),
      "type" => Token::new(TokenKind::Type, location),
      "enum" => Token::new(TokenKind::Enum, location),
      _ => Token::new(TokenKind::Identifier(keyword), location),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
  // keywords
  Function,
  Local,
  If,
  Then,
  Else,
  ElseIf,
  End,
  While,
  Do,
  For,
  In,
  Repeat,
  Until,
  Return,
  Break,
  True,
  False,
  Nil,
  Type,
  Enum,

  // Operators and Delimiters
  Tilde,        // ~
  Plus,         // +
  Minus,        // -
  Star,         // *
  StarAssign,   // *=
  Slash,        // /
  SlashAssign,  // /=
  Percent,      // %
  Equal,        // ==
  NotEqual,     // ~=
  Less,         // <
  Greater,      // >
  LessEqual,    // <=
  GreaterEqual, // >=
  Assign,       // =
  PlusAssign,   // +=
  MinusAssign,  // -=
  And,          // and
  Or,           // or
  Not,          // not
  Hash,         // #
  Comma,        // ,
  Semicolon,    // ;
  Colon,        // :
  DoubleColon,  // ::
  Dot,          // .
  DoubleDot,    // ..
  TripleDot,    // ...
  LeftParen,    // (
  RightParen,   // )
  LeftBrace,    // {
  RightBrace,   // }
  LeftBracket,  // [
  RightBracket, // ]

  // Literals
  Identifier(String),
  Number(f64),
  String(String),

  // Outros
  EOF, // End of file
}
