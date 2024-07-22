function identity<T>(value: T): T
    return value
end

local a: number = identity(10)
local b: string = identity("Hello")

local error_test: number = identity("Hello")  -- ERROR: Type 'string' is not assignable to type 'number'

function add_error<T>(a: T, b: T): T
  return a + b -- todo: ERROR: Type 'T' is not assignable to type 'number'
end

local result: number = add_error(10, 20)

result = add_error(10, "20")  -- ERROR: Type 'string' is not assignable to type 'number'
