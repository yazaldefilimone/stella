function greet(name: string): string
    return "Hello, " .. name
end

local message: string = greet("Alice")

function is_even(n: number): boolean
    return n % 2 == 0
end

local even: boolean = is_even(10)
