#![allow(dead_code, unused_imports)]
// use codspeed::{codspeed::black_box, codspeed_bench, codspeed_main};
// use criterion::{ criterion_group, criterion_main, Criterion};
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

use glob::glob;

pub fn lexer_benchmark(c: &mut Criterion) {
  let paths_names = vec![
    "tests/golden_tests/assign_expression.lua",
    "tests/golden_tests/boolean_logic.lua",
    "tests/golden_tests/conditional_statement.lua",
    "tests/golden_tests/if_statement.lua",
    "tests/golden_tests/nested_functions.lua",
    "tests/golden_tests/simple_declaration.lua",
  ];
  let mut patterns = Vec::new();
  for path_name in paths_names {
    let path = std::path::Path::new(path_name);
    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    patterns.push((file_name.to_owned(), content));
  }
  // checker
  for (file_name, source_code) in patterns.iter() {
    c.bench_function(format!("checker_{}", file_name).as_str(), |b| {
      b.iter(|| {
        let checker = &mut stella::checker::Checker::new(file_name, source_code);
        let check = checker.check(&stella::parser::parser::Parser::new(source_code, file_name).parse_program());
        match check {
          Ok(t) => {
            println!("{:#?}", t);
            black_box(t)
          }
          _ => {
            println!("Error in file {}", file_name);
            black_box(stella::types::Type::Unknown)
          }
        }
      });
    });
  }
}

criterion_group!(benches, lexer_benchmark);
criterion_main!(benches);
