local x: number = 10
local y: number = 20
local result: number = x + y

local a: boolean = true
local b: boolean = false
local comparison: boolean = a and b

local error_test: number = x + a  -- ERROR: Type 'boolean' is not suported for '+' operator
