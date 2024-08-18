type Array<T> = {T}

function bubble_sort(array: Array<number>): Array<number>
  local array_length = #array
  for i = 1, array_length do
    for current_index = 1, array_length - i do
      if array[current_index] > array[current_index + 1] then
        local temp_value = array[current_index]
        array[current_index] = array[current_index + 1]
        array[current_index + 1] = temp_value
      end
    end
  end
  return array
end

local sorted_array = bubble_sort({5, 3, 8, 4, 2})
