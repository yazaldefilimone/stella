#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::utils::range::Range;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Token {
  pub kind: TokenKind,
  pub range: Range,
}

impl Token {
  pub fn new(kind: TokenKind, range: Range) -> Token {
    Token { kind, range }
  }

  pub fn new_end_of_file(range: Range) -> Token {
    Token::new(TokenKind::EOF, range)
  }

  pub fn new_identifier(range: Range, identifier: String) -> Token {
    Token::new(TokenKind::Identifier(identifier), range)
  }

  pub fn new_number(range: Range, number: String) -> Token {
    Token::new(TokenKind::Number(number), range)
  }

  pub fn new_string(range: Range, string: String) -> Token {
    Token::new(TokenKind::String(string), range)
  }

  pub fn new_comment(range: Range, comment: String) -> Token {
    Token::new(TokenKind::Comment(comment), range)
  }

  pub fn new_block_comment(range: Range, comment: String) -> Token {
    Token::new(TokenKind::BlockComment(comment), range)
  }

  pub fn new_keyword_or_identifier(range: Range, keyword: String) -> Token {
    match keyword.as_str() {
      "function" => Token::new(TokenKind::Function, range),
      "local" => Token::new(TokenKind::Local, range),
      "if" => Token::new(TokenKind::If, range),
      "then" => Token::new(TokenKind::Then, range),
      "else" => Token::new(TokenKind::Else, range),
      "elseif" => Token::new(TokenKind::ElseIf, range),
      "end" => Token::new(TokenKind::End, range),
      "while" => Token::new(TokenKind::While, range),
      "do" => Token::new(TokenKind::Do, range),
      "for" => Token::new(TokenKind::For, range),
      "in" => Token::new(TokenKind::In, range),
      "repeat" => Token::new(TokenKind::Repeat, range),
      "until" => Token::new(TokenKind::Until, range),
      "return" => Token::new(TokenKind::Return, range),
      "break" => Token::new(TokenKind::Break, range),
      "true" => Token::new(TokenKind::True, range),
      "false" => Token::new(TokenKind::False, range),
      "nil" => Token::new(TokenKind::Nil, range),
      "type" => Token::new(TokenKind::Type, range),
      "enum" => Token::new(TokenKind::Enum, range),
      "continue" => Token::new(TokenKind::Continue, range),
      "and" => Token::new(TokenKind::And, range),
      "or" => Token::new(TokenKind::Or, range),
      "not" => Token::new(TokenKind::Not, range),
      "require" => Token::new(TokenKind::Require, range),
      _ => Token::new(TokenKind::Identifier(keyword), range),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
  DoubleSlash,  // //
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
  pub fn lexeme(&self) -> &str {
    match &self.kind {
      TokenKind::Identifier(name) => name,
      TokenKind::Number(number) => number,
      TokenKind::String(string) => string,
      _ => panic!("Invalid token"),
    }
  }

  pub fn is_string(&self) -> bool {
    match &self.kind {
      TokenKind::String(_) => true,
      _ => false,
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
  pub fn to_string(&self) -> &str {
    let text = match self {
      TokenKind::Identifier(name) => name,
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
      TokenKind::LeftBrace => "{",
      TokenKind::RightBrace => "}",
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
      TokenKind::DoubleSlash => "//",
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
    return text;
  }
}
