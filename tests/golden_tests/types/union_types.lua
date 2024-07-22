local x: number | string = 10
x = "Now I'm a string"

function print_value(value: number | string)
    if type(value) == "number" then
        print("Number: " .. value)
    else
        print("String: " .. value)
    end
end

print_value(x)

x = true  -- ERROR: Type 'boolean' is not assignable to type 'number | string'

function test_union(a: number | boolean)
    if type(a) == "number" then
        return a + 1
    else
        return not a
    end
end

local test1: number = test_union(10)
local test2: boolean = test_union(false)
local error_test: string = test_union(10)  -- ERROR: Type 'number | boolean' is not assignable to type 'string'
