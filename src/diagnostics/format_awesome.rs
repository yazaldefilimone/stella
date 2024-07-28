use crate::ast::ast::BinaryOperator;

pub fn format_mismatched_types(expected: &str, found: &str) -> String {
  format!(
    "uh-oh, expected '{}', but found '{}'. are you sure? 😅",
    expected, found
  )
}

pub fn format_undeclared_variable(name: &str) -> String {
  format!("hmm, can't find the variable '{}'. did you declare it? 🕵️‍♂️", name)
}

pub fn format_invalid_assignment(name: &str) -> String {
  format!("oops, can't assign to '{}'. something's not right! 🚫", name)
}

pub fn format_function_arity_mismatch(expected: usize, found: usize) -> String {
  format!(
    "expected {} arguments, but got {} instead. check your function call! 🤔",
    expected, found
  )
}

pub fn format_unsupported_operator(left: &str, right: &str, op: &BinaryOperator) -> String {
  format!(
    "can't use '{}' between '{}' and '{}'. that's not allowed! 🛑",
    op.to_string(),
    left,
    right
  )
}

pub fn format_redeclared_in_same_scope(name: &str) -> String {
  format!("'{}' is already declared. try using a different name! 🌱", name)
}

pub fn format_module_not_found(name: &str) -> String {
  format!("module '{}' not found. did you spell it right? 🧐", name)
}

pub fn format_module_not_exported(name: &str) -> String {
  format!("module '{}' doesn't export anything. maybe you forgot? 🤷", name)
}

pub fn format_type_mismatch_assignment(expected: &str, found: &str) -> String {
  format!("can't assign '{}' to '{}'. they're not compatible! 💡", found, expected)
}

pub fn format_missing_return_value() -> String {
  format!("did you forget to return a value in your function? 🌀")
}

pub fn format_nil_assignment(name: &str) -> String {
  format!("can't assign 'nil' to '{}'. it needs a real value! 🌟", name)
}

pub fn format_invalid_indexing(indexed: &str, index: &str) -> String {
  format!("can't index '{}' with '{}'. something's wrong! 🤔", indexed, index)
}

pub fn format_nil_access(name: &str) -> String {
  format!("uh-oh, '{}' is nil. you can't use it like that! ❌", name)
}

pub fn format_unexpected_type(expected: &str, found: &str) -> String {
  format!(
    "expected type '{}', but found '{}'. that's surprising! 🤨",
    expected, found
  )
}

pub fn format_recursive_function(name: &str) -> String {
  format!("recursive function '{}'? make sure it ends! 🔄", name)
}

pub fn format_invalid_use_of_varargs() -> String {
  format!("invalid use of '...'. check how you're using varargs! 🌠")
}

pub fn format_global_shadowing(name: &str) -> String {
  format!("warning: you're shadowing the global variable '{}'. careful! 🌥️", name)
}

pub fn format_unreachable_code() -> String {
  format!("looks like there's unreachable code here. did you mean to do that? 🚧")
}

pub fn format_invalid_literal_in_table() -> String {
  format!("invalid literal in table. make sure everything fits together! 🧩")
}

pub fn format_incorrect_table_structure(expected: &str, found: &str) -> String {
  format!(
    "expected table structure '{}', but got '{}'. double-check your table! 🛠️",
    expected, found
  )
}

pub fn format_unused_variable(name: &str) -> String {
  format!(
    "hey, the variable '{}' isn't being used. don't forget about it! 🐾",
    name
  )
}
