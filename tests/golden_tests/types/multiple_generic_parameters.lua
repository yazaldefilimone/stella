function pair<A, B>(a: A, b: B): {first: A, second: B}
    return {first = a, second = b}
end

local p = pair(1, "one")
local first: number = p.first
local second: string = p.second

local error_test: number = p.second   -- ERROR: Type 'string' is not assignable to type 'number'
