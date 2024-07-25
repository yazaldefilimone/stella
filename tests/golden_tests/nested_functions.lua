function outer(x: number): number
    function inner(y: number): string
        return "Hello, " .. "World"
    end
    return inner(x * 2)
end

local result: number = outer(10)
