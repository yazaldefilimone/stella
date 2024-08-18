function factorial(number_value: number): number
  if number_value == 0 then
    return 1
  end
  return number_value * factorial(number_value - 1)
end

local factorial_result = factorial(5)
