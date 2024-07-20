#![allow(dead_code)]

use crate::diagnostics::Diagnostic;

pub struct Checker {}

impl Checker {
  pub fn check(&self) -> Result<(), Diagnostic> {
    Ok(())
  }
}
