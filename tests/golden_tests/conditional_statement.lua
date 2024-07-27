-- local age: number = 25

-- if age >= 18 then
--     print("You are an adult.")
-- else
--     print("You are a minor.")
-- end


function test(t: boolean)
  if t then
    return 10
  else
    return "Hello"
  end
end


local result: number = test(true)
local n: string = test(false)
