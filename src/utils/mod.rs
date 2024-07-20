pub mod location;

pub fn match_number(character: char) -> bool {
  "1234567890.".contains(character)
}
