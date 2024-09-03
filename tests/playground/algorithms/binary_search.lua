
type Array<T> = {T}

function binary_search(sorted_array: Array<number>, target_value: number): option<number>
  local low_index = 1

  local high_index = #sorted_array

  while low_index <= high_index do
    local mid_index = math.floor((low_index + high_index) / 2)
    if sorted_array[mid_index] == target_value then
      return mid_index
    elseif sorted_array[mid_index] < target_value then
      low_index = mid_index + 1
    else
      high_index = mid_index - 1
    end
  end
  return nil
end


age = 19

local age = 20 -- shadowing warning

local target_index = binary_search({1, "10", 5, 7, 9}, 5)
print("target_index: ", target_index)
