#![allow(dead_code, unused_imports)]

use glob::glob;
use insta::assert_ron_snapshot;
use std::fs;
use std::path::Path;
use stella::ast::tokens::{Token, TokenKind};
use stella::lexer::Lexer;

fn read_test_files(pattern: &str) -> Vec<String> {
  let mut files = Vec::new();
  for entry in glob(pattern).expect("Failed to read glob pattern") {
    match entry {
      Ok(path) => {
        let content = fs::read_to_string(path).expect("Failed to read file");
        files.push(content);
      }
      Err(e) => println!("{:?}", e),
    }
  }
  files
}

fn create_tokens(source_code: &str) -> Vec<Token> {
  let mut lexer = Lexer::new(source_code.to_string());
  let mut tokens = vec![];
  loop {
    let token = lexer.next_token();
    if token.kind == TokenKind::EOF {
      tokens.push(token);
      break;
    }
    tokens.push(token);
  }
  return tokens;
}

#[test]
fn test_lexer_snapshot() {
  let test_files = read_test_files("tests/golden_tests/lexer/*.lua");
  println!("test_lexer_snapshot: {:?}", test_files);
  for source_code in test_files.iter() {
    let tokens = create_tokens(source_code);
    assert_ron_snapshot!("lexer_tokens", tokens);
  }
}

// Funções para o parser e o type checker seguirão a mesma estrutura
