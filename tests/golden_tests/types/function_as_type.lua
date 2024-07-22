type FnSumType = function(a: number, b: number): number
type FnConcatType = function(a: string, b: string): string
type FnType = FnSumType | FnConcatType


function sum(n: number, fn: FnSumType): number
    return fn(n, n)
end
sum(10, function(a, b) -- infer that a and b are numbers
    return a + b
end)

function concat(s: string, fn: FnConcatType): string
    return fn(s, s)
end
concat("Hello", function(a, b)
    return a .. b
end)

local error_test = sum(10, function(a, b)
    return a .. b
end) -- ERROR: Type 'string' is not assignable to type 'number'



-- with generics
type FnSumTypeG<T> = function(a: T, b: T): T


function sum<T>(n: T, fn: FnSumTypeG<T>): T
    return fn(n, n)
end


sum(10, function(a, b) -- infer that a and b are numbers
    return a + b
end)

sum("Hello", function(a, b) -- infer that a and b are strings
    return a .. b
end)

local error_test = sum(10, function(a, b)
    return a .. b
end) -- ERROR: Type 'string' is not assignable to type 'number'



function sum_two(n: number, m: number): number
    return n + m
end


sum(10, sum_two) --ok :)



sum("Hello", sum_two) -- ERROR: Type 'string' is not assignable to type 'number'
