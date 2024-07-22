local x: number? = nil -- or Optional<number>

if x == nil then
    x = 10
end

function get_value(y: number?): number
    if y == nil then
        return 0
    else
        return y
    end
end

local value: number = get_value(x)
local error_test: string = get_value(x)  -- ERROR: Type 'number?' is not assignable to type 'string'
