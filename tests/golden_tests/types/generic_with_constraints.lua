function add<T: number>(a: T, b: T): T
    return a + b
end

local sum: number = add(10, 20)

function concat_or_add<T: number | string>(a: T, b: T): T
    if type(a) == "number" and type(b) == "number" then
        return a + b
    elseif type(a) == "string" and type(b) == "string" then
        return a .. b
    else
        error("Unsupported types")
    end
end

local result1: number = concat_or_add(10, 20)
local result2: string = concat_or_add("hello", "world")

local error_test = add("10", "20")  -- ERROR: Type 'string' is not constrained to type 'number'
