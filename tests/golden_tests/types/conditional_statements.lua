local x: number = 10
local y: number = 20
local max: number

if x > y then
    max = x
else
    max = y
end

-- ERROR: Expected 'boolean', found 'number' in conditional expression
local error_test: boolean = if x then x else y
-- ERROR: Type 'boolean' is not assignable to type 'number'
