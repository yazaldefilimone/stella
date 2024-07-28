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

  pub fn new_number(location: Location, number: String) -> Token {
    Token::new(TokenKind::Number(number), location)
  }

  pub fn new_string(location: Location, string: String) -> Token {
    Token::new(TokenKind::String(string), location)
  }

  pub fn new_comment(location: Location, comment: String) -> Token {
    Token::new(TokenKind::Comment(comment), location)
  }

  pub fn new_block_comment(location: Location, comment: String) -> Token {
    Token::new(TokenKind::BlockComment(comment), location)
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
      "continue" => Token::new(TokenKind::Continue, location),
      "and" => Token::new(TokenKind::And, location),
      "or" => Token::new(TokenKind::Or, location),
      "not" => Token::new(TokenKind::Not, location),
      "require" => Token::new(TokenKind::Require, location),
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
  Continue,
  Require,

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

  // Literals, inclue types
  Identifier(String),
  Number(String),
  String(String),

  // Comments
  Comment(String),
  BlockComment(String),
  // Outros
  EOF, // End of file
}

impl Token {
  pub fn lexeme(&self) -> String {
    match &self.kind {
      TokenKind::Identifier(name) => name.clone(),
      TokenKind::Number(number) => number.clone(),
      TokenKind::String(string) => string.clone(),
      _ => panic!("Invalid token"),
    }
  }
  pub fn is_comment(&self) -> bool {
    match &self.kind {
      TokenKind::Comment(_) | TokenKind::BlockComment(_) => true,
      _ => false,
    }
  }
}

impl TokenKind {
  pub fn to_string(&self) -> String {
    let text = match self {
      TokenKind::Identifier(name) => name.as_str(),
      TokenKind::Number(number) => number.as_str(),
      TokenKind::String(string) => string.as_str(),
      TokenKind::EOF => "EOF",
      TokenKind::Function => "function",
      TokenKind::Local => "local",
      TokenKind::If => "if",
      TokenKind::Then => "then",
      TokenKind::Else => "else",
      TokenKind::ElseIf => "elseif",
      TokenKind::End => "end",
      TokenKind::While => "while",
      TokenKind::Do => "do",
      TokenKind::For => "for",
      TokenKind::In => "in",
      TokenKind::Repeat => "repeat",
      TokenKind::Until => "until",
      TokenKind::Return => "return",
      TokenKind::Break => "break",
      TokenKind::True => "true",
      TokenKind::False => "false",
      TokenKind::Nil => "nil",
      TokenKind::Type => "type",
      TokenKind::Enum => "enum",
      TokenKind::Continue => "continue",
      TokenKind::Assign => "=",
      TokenKind::PlusAssign => "+=",
      TokenKind::MinusAssign => "-=",
      TokenKind::StarAssign => "*=",
      TokenKind::SlashAssign => "/=",
      TokenKind::NotEqual => "~=",
      TokenKind::LessEqual => "<=",
      TokenKind::GreaterEqual => ">=",
      TokenKind::DoubleDot => "..",
      TokenKind::TripleDot => "...",
      TokenKind::LeftParen => "(",
      TokenKind::RightParen => ")",
      TokenKind::LeftBrace => "{{",
      TokenKind::RightBrace => "}}",
      TokenKind::LeftBracket => "[",
      TokenKind::RightBracket => "]",
      TokenKind::Comma => ",",
      TokenKind::Semicolon => ";",
      TokenKind::Colon => ":",
      TokenKind::DoubleColon => "::",
      TokenKind::Dot => ".",
      TokenKind::Tilde => "~",
      TokenKind::Hash => "#",
      TokenKind::Plus => "+",
      TokenKind::Minus => "-",
      TokenKind::Star => "*",
      TokenKind::Slash => "/",
      TokenKind::Not => "not",
      TokenKind::Percent => "%",
      TokenKind::Equal => "==",
      TokenKind::Less => "<",
      TokenKind::Greater => ">",
      TokenKind::Or => "or",
      TokenKind::And => "and",
      TokenKind::Comment(_) => "comment",
      TokenKind::BlockComment(_) => "block comment",
      TokenKind::Require => "require",
    };
    return text.to_string();
  }
}
