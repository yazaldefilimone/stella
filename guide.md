<samp>

### A Quick Guide

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

### Simple exemple!

Here’s how you can write a simple `"Hello, world!"` in Stella:

```lua
function main()
  return "Hello, world!"
end
```

To check it:

```sh
stella check hello.lua
```

Stella checks for any type errors before running your Lua code.

### Pure Lua

Stella can run on pure Lua code without type annotations, catching errors and inferring types on the fly. Here’s an example:

```lua
function binary_search(sorted_array, target_value)
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
```

Stella will infer the types and warn you if there’s an issue, even without explicit type annotations.

### Using Generics in Stella

Stella supports generics, allowing you to create flexible and reusable code. Here’s an example of using generics:

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

### Unions and Option Types

Stella also supports union and option types, which allow you to represent multiple possible types for a value. This helps in scenarios where a variable might hold different types at runtime.

**Example with Union Types:**

```lua
type StringOrNumber = union<string, number>

local function process_value(value: StringOrNumber)
  if type(value) == "number" then
    return value + 1
  else
    return value .. " is a string"
  end
end

print(process_value(10))   -- Outputs: 11
print(process_value("Lua")) -- Outputs: Lua is a string
```

**Example with Option Types:**

```lua

local function find_value(key: string, data: table): option<string>
  return data[key] or nil
end

local result = find_value("name", { name = "Stella", age = 1 })
print(result) -- Outputs: Stella

local missing = find_value("missing", { name = "Stella", age = 1 })
print(missing) -- Outputs: nil
```

### Diagnostics

Stella helps catch errors such as variable shadowing and type mismatches. Here’s an example:

```lua
local function add_numbers(a: number, b: number): number
  local a = 10  -- WARNING >>> `a` shadows an existing variable
  return a + b
end
```

Running Stella on this code would give you a warning about shadowing the variable `a`.

```sh
stella check your_code.lua
```
