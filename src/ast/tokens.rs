use crate::utils::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub location: Location,
}

impl Token {
  pub fn create(kind: TokenKind, location: Location) -> Token {
    Token { kind, location }
  }

  pub fn create_end_of_file(location: Location) -> Token {
    Token::create(TokenKind::EOF, location)
  }

  pub fn create_identifier(location: Location, identifier: String) -> Token {
    Token::create(TokenKind::Identifier(identifier), location)
  }

  pub fn create_number(location: Location, number: f64) -> Token {
    Token::create(TokenKind::Number(number), location)
  }

  pub fn create_string(location: Location, string: String) -> Token {
    Token::create(TokenKind::String(string), location)
  }

  pub fn create_keyword_or_identifier(location: Location, keyword: String) -> Token {
    match keyword.as_str() {
      "function" => Token::create(TokenKind::Function, location),
      "local" => Token::create(TokenKind::Local, location),
      "if" => Token::create(TokenKind::If, location),
      "then" => Token::create(TokenKind::Then, location),
      "else" => Token::create(TokenKind::Else, location),
      "elseif" => Token::create(TokenKind::ElseIf, location),
      "end" => Token::create(TokenKind::End, location),
      "while" => Token::create(TokenKind::While, location),
      "do" => Token::create(TokenKind::Do, location),
      "for" => Token::create(TokenKind::For, location),
      "in" => Token::create(TokenKind::In, location),
      "repeat" => Token::create(TokenKind::Repeat, location),
      "until" => Token::create(TokenKind::Until, location),
      "return" => Token::create(TokenKind::Return, location),
      "break" => Token::create(TokenKind::Break, location),
      "true" => Token::create(TokenKind::True, location),
      "false" => Token::create(TokenKind::False, location),
      "nil" => Token::create(TokenKind::Nil, location),
      "type" => Token::create(TokenKind::Type, location),
      "enum" => Token::create(TokenKind::Enum, location),
      _ => Token::create(TokenKind::Identifier(keyword), location),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
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
  Plus,         // +
  Minus,        // -
  Asterisk,     // *
  Slash,        // /
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
