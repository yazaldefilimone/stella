function outer(x: number): number
    function inner(y: number): number
        return x + y
    end
    return inner(x * 2)
end

print(outer(10))
