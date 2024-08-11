use crate::types::Type;
pub fn create_print_type() -> Type {
  let print_function = Type::new_function(vec![Type::Unknown], Type::Nil);
  return print_function;
}
