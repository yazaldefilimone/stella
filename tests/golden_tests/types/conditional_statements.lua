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




--- inferir

function is_ok(n)
    if n > 0 then -- assume that n is a number
        return "ok"
    else
        return "not ok :("
    end
end


local result = is_ok(10)

local error_test = is_ok("10")  -- ERROR: Type 'string' is not assignable to type 'number'
