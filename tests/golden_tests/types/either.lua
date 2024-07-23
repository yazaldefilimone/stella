
type Either<T, U> = {
  left: T,
  right: U
}

local ex = function(value: number): Either<string, number>
  if value > 10 then
  return Either.left("Too big")
  else
    return Either.right(value)
  end
end


let result: Either<string, number> = ex(20)
if result.left then
  print("Error: " .. result.left)
elseif result.right then
  print("Result: " .. result.right)
end
