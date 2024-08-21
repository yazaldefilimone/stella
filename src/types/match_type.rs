use super::{FunctionType, GenericType, GroupType, OptionType, TableType, Type, UnionType, VariadicType};

pub fn check_match_table(left: &TableType, right: &TableType) -> bool {
  match (left, right) {
    (TableType { array: Some(left_array), map: None }, TableType { array: Some(right_array), map: None }) => {
      left_array.len() == right_array.len() && left_array.iter().zip(right_array).all(|(l, r)| l.check_match(r))
    }
    (TableType { array: None, map: Some(left_map) }, TableType { array: None, map: Some(right_map) }) => {
      left_map.len() == right_map.len()
        && left_map.iter().all(|(k, v)| right_map.get(k).map_or(false, |rv| v.check_match(rv)))
    }
    (TableType { array: None, map: None }, TableType { array: None, map: None }) => true,
    (TableType { array: None, map: None }, TableType { .. }) => true,
    (TableType { .. }, TableType { array: None, map: None }) => true,
    _ => false,
  }
}

pub fn check_match_function(left: &FunctionType, right: &FunctionType) -> bool {
  if left.params.len() != right.params.len() {
    return false;
  }

  if left.params.iter().zip(&right.params).any(|(l, r)| !l.check_match(r)) {
    return false;
  }
  left.return_type.check_match(&right.return_type)
}

pub fn check_match_generic(left: &GenericType, right: &GenericType) -> bool {
  left.name == right.name
    && left.variables.len() == right.variables.len()
    && left.variables.iter().zip(&right.variables).all(|(l, r)| l == r)
    && left.value.check_match(&right.value)
}

pub fn check_match_group(left: &GroupType, right: &GroupType) -> bool {
  left.types.len() == right.types.len() && left.types.iter().zip(&right.types).all(|(l, r)| l.check_match(r))
}

pub fn check_match_group_with_single_type(left: &GroupType, right: &Type) -> bool {
  left.types.len() == 1 && right.check_match(&left.types[0])
}

pub fn check_match_option(left: &OptionType, right: &OptionType) -> bool {
  left.inner_type.check_match(&right.inner_type)
}

pub fn check_match_option_right(left: &OptionType, right: &Type) -> bool {
  return if right.is_nil() { true } else { left.inner_type.check_match(right) };
}

pub fn check_match_union(left: &Vec<Type>, right: &Vec<Type>) -> bool {
  left.len() == right.len() && left.iter().zip(right).all(|(l, r)| l.check_match(r))
}

pub fn check_match_union_with_single_type(left: &UnionType, right: &Type) -> bool {
  left.types.iter().any(|t| t.check_match(right))
}

pub fn check_match_variadic(left: &VariadicType, right: &VariadicType) -> bool {
  left.inner_type.check_match(&right.inner_type)
}

pub fn check_match_variadic_with_single_type(left: &VariadicType, right: &Type) -> bool {
  return right.is_nil() || left.inner_type.check_match(right);
}
