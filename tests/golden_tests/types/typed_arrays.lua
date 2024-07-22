local numbers: array<number> = {1, 2, 3, 4, 5}
local strings: array<string> = {"a", "b", "c"}

for i: number = 1, #numbers do
    print(numbers[i])
end

for j: number = 1, #strings do
    print(strings[j])
end

numbers[1] = "string"  -- ERROR: type "string" is not assignable to type "number"

local mixed_array: array<number | string> = {1, "two", 3}
mixed_array[2] = 2  -- Ok...

mixed_array[2] = true  -- ERROR: type "boolean" is not assignable to type "number | string"
