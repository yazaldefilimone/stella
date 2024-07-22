function outer(x: number): number
    function inner(y: number): number
        return y * 2
    end

    return inner(x) + 1
end

local result: number = outer(5)

local error_test: string = outer(5)  -- ERROR: Type 'number' is not assignable to type 'string'
