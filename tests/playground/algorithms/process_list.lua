type Fn<T, R> = function(param: T): R

type Array<T> = {T}

type ProcessListType<T> = function(list: Array<T>, apply_fn: Fn<T, T>): Array<T>

local process_list: ProcessListType<number> = function(list, apply_fn)
  local result = {}
  for i = 1, #list do
    table.insert(result, apply_fn(list[i]))
  end
  return result
end

local function increment(n: number): number
  return n + 1
end

local function double(n: number): number
  return n * 2
end

local numbers = {1, 2, 3, 4}

-- Apply the 'increment' function to each number in the list
local incremented_numbers = process_list(numbers, increment) -- ok :)

-- Apply the 'double' function to each number in the list
local doubled_numbers = process_list(numbers, double) -- ok :)
print(doubled_numbers)

-- --- error
-- local numbers_error = {1, 2, 3, 4, "hello"}

-- -- ERROR >>> expected `table<number>`, found `table<string, number>`
-- --  in `numbers_error`
-- local incremented_numbers = process_list(numbers_error, increment)
