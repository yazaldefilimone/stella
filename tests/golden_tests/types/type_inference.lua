
function add(a, b)
    return a > b
end

local result = add(5, 10) -- assume that a and b are numbers

local error_test = add(5, "10")  -- ERROR: Type 'string' is not assignable to type 'number'


let age:any;

age = 10
age = "ten" -- ERROR: Type 'string' is not assignable to type 'number'

let name:any;

name = "Stella"
name = 10 -- ERROR: Type 'number' is not assignable to type 'string'

let is_student:any;

is_student = true
is_student = "true" -- ERROR: Type 'string' is not assignable to type 'boolean'



-- generics
function identity<T>(value: T): T
    return value
end

let a:any;

a = identity(10)
a = identity("Hello") -- ERROR: Type 'string' is not assignable to type 'number'

function add_error<T>(a: T, b: T): T
  return a + b -- todo: ERROR: Type 'T' is not assignable to type 'number'
end

let result:any;

result = add_error(10, 20)

result = add_error(10, "20")  -- ERROR: Type 'string' is not assignable to type 'number'



-- generics

function concat<T, U>(left: T, right: U): T & U
    return left .. right
end

concat(10, 20) -- ok :) return constant string "1020"
concat("Hello", "World") -- ok :) return constant string "HelloWorld"
concat(10, "World") -- ok :) return constant string "10World"
concat("Hello", 20) -- ok :) return constant string "Hello20"

-- generics with constraints
function concat_or_add<T: number | string>(left: T, right: T): T
    if type(left) == "number" and type(right) == "number" then
        return left + right
    elseif type(left) == "string" and type(right) == "string" then
        return left .. right
    else
        error("Unsupported types")
    end
end
let result = concat_or_add(10, 20) -- ok :) return constant number 30
let result = concat_or_add("Hello", "World") -- ok :) return constant string "HelloWorld"
let error_test = concat_or_add(10, "World") -- ERROR: Type 'string' is not assignable to type 'number'
