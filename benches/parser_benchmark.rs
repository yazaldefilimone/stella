#![allow(dead_code, unused_imports)]
// use codspeed::{codspeed::black_box, codspeed_bench, codspeed_main};
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use glob::glob;

pub fn lexer_benchmark(c: &mut Criterion) {
  let mut patterns = Vec::new();
  let glob_pattern = glob("tests/golden_tests/*.lua").expect("Failed to read glob pattern");
  for entry in glob_pattern {
    let path = entry.expect("Failed to read file");
    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    patterns.push((file_name.to_owned(), content));
  }

  for (file_name, source_code) in patterns.iter() {
    c.bench_function(format!("parser_{}", file_name).as_str(), |b| {
      b.iter(|| {
        black_box(stella_checker::parser::parser::Parser::new(source_code, file_name).parse_program());
      });
    });
  }
}

criterion_group!(benches, lexer_benchmark);
criterion_main!(benches);
