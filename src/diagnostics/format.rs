use crate::ast::ast::{BinaryOperator, UnaryOperator};

pub fn format_mismatched_types(expected: &str, found: &str) -> String {
  format!("expected `{}`, found `{}`", expected, found)
}

pub fn format_expected_function(name: &str) -> String {
  format!("expected function, found `{}`", name)
}
pub fn format_expected_table(message: &str) -> String {
  format!("expected table, found `{}`", message)
}

pub fn format_undeclared_variable(name: &str) -> String {
  format!("cannot find value `{}` in this scope", name)
}

pub fn format_undeclared_type(name: &str) -> String {
  format!("cannot find type `{}`", name)
}

pub fn format_mismatched_key_type(key: &str) -> String {
  format!("expected `string` key, but found `{}`", key)
}

pub fn format_mismatched_accessor_type(index: &str) -> String {
  format!("expected `number` or `string`, but found `{}`", index)
}

pub fn format_no_field(base: &str, member: &str) -> String {
  format!("no field `{}` on type `{}`", member, base)
}

pub fn format_function_arity_mismatch(expected: usize, found: usize) -> String {
  match expected {
    1 => format!("expected 1 arg, found `{}`", found),
    _ => format!("expected {} args, found {}", expected, found),
  }
}

pub fn format_generic_call_arity_mismatch(expected: usize, found: usize) -> String {
  match expected {
    1 => format!("expected 1 type arg, found `{}`", found),
    _ => format!("expected `{}` type args, found `{}`", expected, found),
  }
}

pub fn format_option_call_arity_mismatch(found: usize) -> String {
  format!("expected one type argument, found `{}`", found)
}

pub fn format_expected_variadic(type_name: &str) -> String {
  format!("expected variadic, found `{}`", type_name)
}

pub fn format_cannot_index_non_array(type_name: &str) -> String {
  format!("cannot index non-sequence `{}`", type_name)
}

pub fn format_field_not_found_in_table(field: &str, table: &str) -> String {
  format!("field `{}` not found in `{}`", field, table)
}

pub fn format_unsupported_operator(left: &str, operator: BinaryOperator, right: &str) -> String {
  use BinaryOperator::*;
  match operator {
    Add => format!("cannot add `{}` and `{}`", left, right),
    Subtract => format!("cannot subtract `{}` from `{}`", left, right),
    Multiply => format!("cannot multiply `{}` and `{}`", left, right),
    Divide => format!("cannot divide `{}` by `{}`", left, right),
    Modulus => format!("cannot take modulus of `{}` and `{}`", left, right),
    Equal => format!("cannot compare `{}` and `{}`", left, right),
    And => format!("cannot perform `and` on `{}` and `{}`", left, right),
    Or => format!("cannot perform `or` on `{}` and `{}`", left, right),
    NotEqual => format!("cannot compare `{}` and `{}`", left, right),
    LessThan => format!("`{}` cannot be less than `{}`", left, right),
    GreaterThan => format!("`{}` cannot be greater than `{}`", left, right),
    LessThanOrEqual => format!("`{}` cannot be less than or equal to `{}`", left, right),
    GreaterThanOrEqual => format!("`{}` cannot be greater than or equal to `{}`", left, right),
    DoubleDot => format!("cannot concatenate `{}` and `{}`", left, right),
    DoubleSlash => format!("cannot divide `{}` by `{}` with `//`", left, right),
  }
}

pub fn format_unsupported_unary_operator(operator: UnaryOperator, right: &str) -> String {
  use UnaryOperator::*;
  match operator {
    Negate => format!("cannot negate `{}`", right),
    Not => format!("cannot apply logical NOT to `{}`", right),
    Hash => format!("cannot compute length of `{}`", right),
  }
}

pub fn format_redeclared_in_same_scope(name: &str) -> String {
  // format!("value `{}` redefined in the same scope", name)
  format!("`{}` redeclared in same scope", name)
}

pub fn format_module_not_found(name: &str) -> String {
  format!("module `{}` not found", name)
}

pub fn format_module_not_exported(name: &str) -> String {
  format!("module `{}` doesn’t export", name)
}

pub fn format_type_mismatch_assignment(expected: &str, found: &str) -> String {
  format!("cannot assign `{}` to `{}`", found, expected)
}

pub fn format_shadow_warning(name: &str) -> String {
  // format!("local value `{}` shadows a global variable", name)
  format!("`{}` shadows an existing variable", name)
}
pub fn format_missing_return_value() -> String {
  "missing return value".into()
}
pub fn format_missing_variable_declaration(name: &str) -> String {
  format!("undeclared variable `{}`", name)
}
pub fn format_nil_assignment(name: &str) -> String {
  format!("cannot assign `nil` to `{}`", name)
}

pub fn format_invalid_indexing(indexed: &str, index: &str) -> String {
  format!("invalid index `{}` in `{}`", index, indexed)
}

pub fn format_nil_access(name: &str) -> String {
  format!("accessed `nil` value `{}`", name)
}

pub fn format_unexpected_type(expected: &str, found: &str) -> String {
  format!("expected `{}`, found `{}`", expected, found)
}

pub fn format_recursive_function(name: &str) -> String {
  format!("recursive function `{}`, possible infinite loop", name)
}

pub fn format_invalid_use_of_varargs() -> String {
  "invalid use of `...`".into()
}

pub fn format_unreachable_code() -> String {
  "unreachable code".into()
}

pub fn format_invalid_literal_in_table() -> String {
  "invalid literal in table".into()
}

pub fn format_incorrect_table_structure(expected: &str, found: &str) -> String {
  format!("expected table `{}`, found `{}`", expected, found)
}

// warning
pub fn format_warning_redundant_type(name: &str, type_name: &str) -> String {
  // re-specifying type `number` for variable `x` is redundant"
  // or
  // redundant type `number` for `x`"
  format!("redundant `{}` for `{}`", type_name, name)
}
pub fn format_warning_unused_variable(name: &str) -> String {
  if name == "..." {
    return format!("unused values in variadic arguments");
  }
  format!("unused value `{}`", name)
}

pub fn format_warning_redeclaration(name: &str) -> String {
  format!("redeclaration of local value `{}`", name)
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
