#![allow(dead_code)]

use std::{
  fs,
  path::{Path, PathBuf},
};
pub mod location;

pub fn match_number(character: char) -> bool {
  "1234567890.".contains(character)
}
pub fn highlight_text_with_red(text: &str) -> String {
  format!("\x1b[31m{}\x1b[0m", text)
}

pub fn highlight_text_with_yellow(text: &str) -> String {
  format!("\x1b[33m{}\x1b[0m", text)
}

pub fn highlight_text_with_green(text: &str) -> String {
  format!("\x1b[32m{}\x1b[0m", text)
}

pub fn highlight_text_with_blue(text: &str) -> String {
  format!("\x1b[34m{}\x1b[0m", text)
}

pub fn highlight_text_with_magenta(text: &str) -> String {
  format!("\x1b[35m{}\x1b[0m", text)
}

pub fn highlight_text_with_cyan(text: &str) -> String {
  format!("\x1b[36m{}\x1b[0m", text)
}

pub fn highlight_text_with_white(text: &str) -> String {
  format!("\x1b[97m{}\x1b[0m", text)
}

pub fn highlight_text_with_gray(text: &str) -> String {
  format!("\x1b[90m{}\x1b[0m", text)
}

pub fn get_full_path(path: &str) -> String {
  if path.starts_with('/') {
    return PathBuf::from(path).as_path().to_str().unwrap().to_string();
  }
  let path = Path::new(path);
  let asb = fs::canonicalize(path);
  if asb.is_err() {
    return path.to_str().unwrap().to_string();
  }
  let path = asb.unwrap();
  return path.to_str().unwrap().to_string();
}
