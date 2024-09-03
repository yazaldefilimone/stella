function binary_search(sorted_array, target_value)
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

local target_index = binary_search({1, 3, 5, 7, 9}, 5)

print("target_index: ", target_index)
