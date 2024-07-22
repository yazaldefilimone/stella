local function first_element<T>(array: array<T>): T
    return array[1]
end

local numbers: array<number> = {1, 2, 3, 4}
local first_num: number = first_element(numbers)

local strings: array<string> = {"a", "b", "c"}
local first_str: string = first_element(strings)

local error_test: number = first_element(strings)  -- ERROR: Type 'string' is not assignable to type 'number'
