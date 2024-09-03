

function fibonacci(sequence_position: number): number
  if sequence_position <= 1 then
    return sequence_position
  end
  return fibonacci(sequence_position - 1) + fibonacci(sequence_position - 2)
end

local fibonacci_result = fibonacci(10)

print(fibonacci_result)
