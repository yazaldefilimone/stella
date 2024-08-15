<samp>

**Stella** is a type checker for Lua. It helps you catch type errors before your code runs, making your Lua code safer and more reliable.

1. Generics

<table>
<tr>
<td><strong>Stella</strong></td> <td><strong>Lua</strong></td>
</tr>

<tr>
<td>

```lua
type fn<T, R> = function(n: T, b: T): R;

local do_thing: fn<number, string> = function (n, b)
  local a = n + 10
  return "hei, stella checker :)"
end
```

</td>
<td>

```lua
local do_thing = function (n, b)
  local a = n + 10
  return "hei, stella checker :)"
end
```

</td>
</tr>
</table>

2. Function Types

<table>
<tr>
<td><strong>Stella</strong></td> <td><strong>Lua</strong></td>
</tr>

<tr>
<td>

```lua
type Apply<T> = function(num: T): T;

type ApplyTwiceType<T> = function(n: number, fn: Apply<T>): T;

local apply_twice: ApplyTwiceType<number> = function(num, fn)
    print(fn)
    return fn(fn(num))
end

local function inc(n: number): number
    return n + 1
end

local result = apply_twice(3, inc)
```

</td>
<td>

```lua
local apply_twice = function(num, fn)
    print(fn)
    return fn(fn(num))
end

local function inc(n)
    return n + 1
end

local result = apply_twice(3, inc)
```

</td>
</tr>
</table>

3. optionals

<table>
<tr>
<td><strong>Stella</strong></td> <td><strong>Lua</strong></td>
</tr>

<tr>
<td>

```lua
function divide(a: number, b: number): option<number>
    if b == 0 then
        return nil
    end
    return a / b
end

local result: option<number> = divide(10, 0)
```

</td>
<td>

```lua
function divide(a, b)
    if b == 0 then
        return nil
    end
    return a / b
end

local result = divide(10, 0)
```

</td>
</tr>
</table>

4. unions

<table>
<tr>
<td><strong>Stella</strong></td> <td><strong>Lua</strong></td>
</tr>

<tr>
<td>

```lua
type Either<T, U> = union<T, U>

function get_value(flag: boolean): Either<number, string>
    if flag then
        return 42
    else
        return "forty-two"
    end
end

local value: Either<number, string> = get_value(true)
```

</td>
<td>

```lua
function get_value(flag)
    if flag then
        return 42
    else
        return "forty-two"
    end
end

local value = get_value(true)
```

</td>
</tr>
</table>

<samp>


```sh
cargo build --release

# Run the type checker
#

./stella check tests/golden_tests/nested_functions.lua


# you can see ast
#
./stella compile tests/golden_tests/nested_functions.lua

```

- [Stella Virtual Machine](https://github.com/yazaldefilimone/stella-compiler)
