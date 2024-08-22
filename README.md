<samp>

Stella is a type checker for Lua that adds TypeScript-like type safety to your code. It helps catch errors early, ensures your code runs smoothly, and works with your existing Lua code without requiring any changes.

### Installation

#### Install Dependencies

##### On Linux or Mac

```sh
# Install Rust if you haven't already.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

##### Install Stella

```sh
# Install Stella
cargo install stellla_checker

# Check if Stella is installed correctly
stella --version
```

#### Fibonacci

```lua
function fibonacci(sequence_position: number): number
  if sequence_position <= 1 then
    return sequence_position
  end
  return fibonacci(sequence_position - 1) + fibonacci(sequence_position - 2)
end

local fibonacci_result = fibonacci(10)

print(fibonacci_result)

```

```sh
stella check fibonacci.lua
```

or run it:

```sh
stella run fibonacci.lua
#  output:
# done. 0 errors, 0 warnings
# emitting lua code...
# 55
```

#### Binary Search

```lua
type Array<T> = {T}

function binary_search(sorted_array: Array<number>, target_value: number): option<number>
  local low_index = 1
  local high_index = #sorted_array

  while low_index <= high_index do
    local mid_index = math.floor((low_index + high_index) / 2)
    if sorted_array[mid_index] == target_value then
      return mid_index
    elseif sorted_array[mid_index] < target_value then
      low_index = mid_index + 1
    else
      high_index = mid_index - 1
    end
  end
  return nil
end
local target_index = binary_search({1, 3, 5, 7, 9}, 5)
print(target_index)
```

check it:

```sh
stella check binary_search.lua
```

run it:

```sh
stella run binary_search.lua
```

### Complex Examples

```lua
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


--- error
local numbers_error = {1, 2, 3, 4, "hello"}

-- ERROR >>> expected `table<number>`, found `table<string, number>`
--  in `numbers_error`
local incremented_numbers = process_list(numbers_error, increment)

```

```sh
stella check process_list.lua

# let me know if you have any questions or suggestions :) I hope you have a amazing day!
```

- [A Quick Guide](./guide.md)

- [Stella Virtual Machine (maybe coming soon)](https://github.com/yazaldefilimone/stella-compiler)
