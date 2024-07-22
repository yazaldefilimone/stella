struct Box<T>
    value: T
end

local number_box: Box<number> = Box { value = 10 }
local string_box: Box<string> = Box { value = "Hello" }

local string_box: Box<string> = Box { value = 20 }  -- ERROR: Type 'number' is not assignable to type 'string'

local function get_value<T>(box: Box<T>): T
    return box.value
end

local num: number = get_value(number_box)
local str: string = get_value(string_box)

local error_test: number = get_value(string_box)  -- ERROR: Type 'string' is not assignable to type 'number'
