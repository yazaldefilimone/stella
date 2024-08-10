pub fn format_mismatched_types(expected: &str, found: &str) -> String {
  format!("expected `{}`, found `{}`", expected, found)
}

pub fn format_expected_function(name: &str) -> String {
  format!("expected function, found `{}`", name)
}

pub fn format_undeclared_variable(name: &str) -> String {
  format!("cannot find value `{}` in this scope", name)
}

pub fn format_undeclared_type(name: &str) -> String {
  format!("cannot find type `{}` in this scope", name)
}

pub fn format_function_arity_mismatch(expected: usize, found: usize) -> String {
  if expected > 1 {
    format!("expected `{}` arguments, found `{}`", expected, found)
  } else {
    format!("expected `{}` argument, found `{}`", expected, found)
  }
}

pub fn format_unsupported_operator(left: &str, right: &str, oper: &str) -> String {
  format!("unsupported operator `{}` for `{}` and `{}`", oper, left, right)
}

pub fn format_redeclared_in_same_scope(name: &str) -> String {
  format!("redeclared in same scope: `{}`", name)
}

pub fn format_module_not_found(name: &str) -> String {
  format!("module not found: `{}.lua`", name)
}

pub fn format_module_not_exported(name: &str) -> String {
  format!("module `{}` doesn`t export", name)
}

pub fn format_type_mismatch_assignment(expected: &str, found: &str) -> String {
  format!("can`t assign `{}` to `{}`", expected, found)
}

pub fn format_missing_variable_declaration() -> String {
  format!("missing variable name in declaration or assignment")
}
pub fn format_warning_shadow_warning(name: &str) -> String {
  format!("local variable `{}` shadows global variable", name)
}

pub fn format_missing_return_value() -> String {
  format!("missing return value")
}

pub fn format_nil_assignment(name: &str) -> String {
  format!("can`t assign `nil` to `{}`", name)
}

pub fn format_invalid_indexing(indexed: &str, index: &str) -> String {
  format!("invalid indexing: `{}` with `{}`", indexed, index)
}

pub fn format_nil_access(name: &str) -> String {
  format!("attempted to access nil value `{}`", name)
}

pub fn format_unexpected_type(expected: &str, found: &str) -> String {
  format!("unexpected type `{}`, found `{}`", expected, found)
}

pub fn format_recursive_function(name: &str) -> String {
  format!("recursive function `{}`, check for infinite recursion", name)
}

pub fn format_invalid_use_of_varargs() -> String {
  format!("invalid use of `...`")
}

pub fn format_unreachable_code() -> String {
  format!("unreachable code detected")
}

pub fn format_invalid_literal_in_table() -> String {
  format!("invalid literal in table")
}

pub fn format_incorrect_table_structure(expected: &str, found: &str) -> String {
  format!("incorrect table structure, expected `{}`, found `{}`", expected, found)
}

// warning

pub fn format_warning_unused_variable(name: &str) -> String {
  format!("unused value `{}`", name)
}

pub fn format_warning_redeclaration(name: &str) -> String {
  format!("redeclaration of local variable `{}`", name)
}

pub fn format_warning_uninitialized_variable(name: &str) -> String {
  format!("value `{}` used before initialization", name)
}

pub fn format_warning_undeclared_global(name: &str) -> String {
  format!("usage of undeclared global variable `{}`", name)
}

pub fn format_warning_execution_order(name: &str) -> String {
  format!("usage of variable `{}` may depend on execution order", name)
}

pub fn format_warning_variable_declared_not_initialized(name: &str) -> String {
  format!("value `{}` declared but not initialized", name)
}

pub fn format_warning_scope_end(name: &str) -> String {
  format!("value `{}` used outside its valid scope", name)
}

pub fn format_warning_shadowed_variable(name: &str) -> String {
  format!("value `{}` shadows an existing value", name)
}

pub fn format_warning_deprecated_function(name: &str) -> String {
  format!("value `{}` is deprecated", name)
}

pub fn format_warning_possible_nil_access(name: &str) -> String {
  format!("possible nil access to `{}`", name)
}

pub fn format_warning_implicit_conversion(from: &str, to: &str) -> String {
  format!("implicit conversion from `{}` to `{}`", from, to)
}

pub fn format_warning_unreachable_code() -> String {
  format!("unreachable code detected")
}

pub fn format_warning_missing_return_in_function(name: &str) -> String {
  format!("value `{}` may not return a value", name)
}

pub fn format_warning_suspect_empty_block() -> String {
  format!("empty block detected")
}

pub fn format_warning_duplicate_case_in_switch(case: &str) -> String {
  format!("duplicate case `{}` in switch", case)
}

pub fn format_warning_large_function(name: &str) -> String {
  format!("value `{}` is too large, consider refactoring", name)
}

pub fn format_warning_unused_import(module: &str) -> String {
  format!("module `{}` imported but not used", module)
}

pub fn format_warning_variable_never_assigned(name: &str) -> String {
  format!("value `{}` is declared but never assigned", name)
}

pub fn format_warning_potential_floating_point_error() -> String {
  format!("potential floating point precision issue")
}

pub fn format_warning_unoptimized_code_segment() -> String {
  format!("unoptimized code segment detected")
}
