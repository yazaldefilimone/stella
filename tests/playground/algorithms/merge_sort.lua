
type Array<T> = {T}

function merge_sort(array: Array<number>): Array<number>
  if #array <= 1 then
    return array
  end

  local mid_index = math.floor(#array / 2)
  local left_array = merge_sort({unpack(array, 1, mid_index)})
  local right_array = merge_sort({unpack(array, mid_index + 1)})

  return merge(left_array, right_array)
end

function merge(left_array: Array<number>, right_array: Array<number>): Array<number>
  local sorted_result = {}
  local left_index, right_index = 1, 1

  while left_index <= #left_array and right_index <= #right_array do
    if left_array[left_index] <= right_array[right_index] then
      table.insert(sorted_result, left_array[left_index])
      left_index = left_index + 1
    else
      table.insert(sorted_result, right_array[right_index])
      right_index = right_index + 1
    end
  end

  while left_index <= #left_array do
    table.insert(sorted_result, left_array[left_index])
    left_index = left_index + 1
  end

  while right_index <= #right_array do
    table.insert(sorted_result, right_array[right_index])
    right_index = right_index + 1
  end

  return sorted_result
end

local sorted_array = merge_sort({5, 3, 8, 4, 2})
