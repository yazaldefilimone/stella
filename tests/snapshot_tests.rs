#![allow(dead_code, unused_imports)]

use glob::glob;
use insta::assert_ron_snapshot;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use stella::ast::tokens::{Token, TokenKind};
use stella::lexer::Lexer;
use stella::parser::parser::Parser;

fn read_test_files_with_pattern(pattern: &str) -> Vec<(String, String)> {
  let mut patterns = Vec::new();
  let glob_pattern = glob(pattern).expect("Failed to read glob pattern");
  for entry in glob_pattern {
    let path = entry.expect("Failed to read file");
    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
    let content = fs::read_to_string(path).expect("Failed to read file");
    patterns.push((file_name.to_owned(), content));
  }
  patterns
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

fn create_parser(source_code: &str) -> Parser {
  let parser = Parser::new(source_code);
  return parser;
}
fn format_file_name_with_module(file_name: &str, module: &str) -> String {
  let file_name = format!("{}_{}", module, file_name).replace(".lua", "");
  return file_name;
}
fn setings_snapshot() -> insta::Settings {
  let mut settings = insta::Settings::clone_current();
  settings.set_prepend_module_to_snapshot(false);
  settings.set_omit_expression(true);
  settings
}

#[test]
fn test_lexer_snapshot() {
  let test_files = read_test_files_with_pattern("tests/golden_tests/*.lua");
  let settings = setings_snapshot();
  settings.bind(|| {
    for (file_name, source_code) in test_files.iter() {
      let tokens = create_tokens(source_code);
      let file_name = format_file_name_with_module(file_name, "lexer");
      assert_ron_snapshot!(file_name.clone(), tokens);
    }
  });
}

#[test]
fn test_parser_snapshot() {
  let test_files = read_test_files_with_pattern("tests/golden_tests/*.lua");
  let settings = setings_snapshot();
  settings.bind(|| {
    for (file_name, source_code) in test_files.iter() {
      let mut parser = create_parser(source_code);
      let file_name = format_file_name_with_module(file_name, "parser");
      let program = parser.parse_program();
      assert_ron_snapshot!(file_name.clone(), program);
    }
  });
}

#[test]
fn test_type_checker_snapshot() {
  let test_files = read_test_files_with_pattern("tests/golden_tests/*.lua");
  println!("test_type_checker_snapshot: {:?}", test_files);
  // hei, please :) implement me
}
