# Stella 🚀 - Speedy Lua Type Checker

Stella is a speedy Lua type checker written in Rust.
Make Lua secure and faster.

### TODO

#### Lexer and Parser

- [ ] **Implement support for new tokens**:
  - [ ] Arithmetic operators (e.g., `+`, `-`, `*`, `/`)
  - [ ] Logical operators (e.g., `and`, `or`, `not`)
  - [ ] Comparators (e.g., `==`, `!=`, `<`, `>`, `<=`, `>=`)
  - [ ] Parentheses, brackets, and braces
- [ ] **Improve handling of whitespace and comments**:
  - [ ] Skip line comments (`--`)
  - [ ] Skip block comments (`--[[ ... ]]`)
- [ ] **Support additional literals**:
  - [ ] Boolean literals (`true`, `false`)
  - [ ] Null literals (`nil`)

#### Basic Type Checking

- [ ] **Check Types for Expressions**:
  - [ ] Arithmetic expressions (e.g., `1 + 2`)
  - [ ] Logical expressions (e.g., `true and false`)
  - [ ] Comparison expressions (e.g., `1 == 2`)
- [ ] **Check Types for Statements**:
  - [ ] Variable declarations (e.g., `local x: number = 10`)
  - [ ] Assignment statements (e.g., `x = 5`)
  - [ ] Function calls (e.g., `print("Hello")`)

#### Type Inference

- [ ] **Infer Types for Expressions**:
  - [ ] Infer type for literals (e.g., `10`, `"string"`, `true`)
  - [ ] Infer type for variables based on usage
  - [ ] Infer type for function return values

#### Error Handling

- [ ] **Handle Type Errors**:
  - [ ] Mismatched types (e.g., assigning `string` to `number`)
  - [ ] Undeclared variables
  - [ ] Invalid assignments (e.g., assigning to a function call)
  - [ ] Function arity mismatch (e.g., passing wrong number of arguments)

#### Type Checking for Functions

- [ ] **Check Function Declarations**:
  - [ ] Ensure correct parameter types
  - [ ] Ensure correct return type
- [ ] **Check Function Calls**:
  - [ ] Ensure correct argument types
  - [ ] Ensure correct number of arguments

#### Advanced Type Checking

- [ ] **Support for Composite Types**:
  - [ ] Arrays (e.g., `number[]`)
  - [ ] Tables (e.g., `{ [string]: number }`)
- [ ] **Support for Union Types**:
  - [ ] Union types (e.g., `number | string`)
- [ ] **Support for Generic Types**:
  - [ ] Generic functions (e.g., `function<T>(arg: T): T`)

#### Integration and Testing

- [ ] **Integrate Type Checker with Parser**:
  - [ ] Ensure type checker runs after parsing
  - [ ] Pass AST nodes to type checker for validation
- [ ] **Develop Comprehensive Test Cases**:
  - [ ] Unit tests for basic type checking
  - [ ] Unit tests for type inference
  - [ ] Integration tests for full programs
  - [ ] Error cases to ensure proper error handling

#### Documentation and Cleanup

- [ ] **Improve Documentation**:
  - [ ] Document all public functions in the type checker
  - [ ] Provide examples of common type errors and their resolutions
- [ ] **Code Cleanup**:
  - [ ] Refactor type checker code for readability
  - [ ] Remove any redundant or dead code
