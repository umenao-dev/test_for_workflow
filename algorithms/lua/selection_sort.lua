local function selection_sort(arr)
  for i = 1, #arr do
    local min_idx = i
    for j = i + 1, #arr do
      if arr[j] < arr[min_idx] then
        min_idx = j
      end
    end
    arr[i], arr[min_idx] = arr[min_idx], arr[i]
  end
  return arr
end

local function print_table(t)
  for i = 1, #t do
    io.write(t[i], i == #t and "\n" or " ")
  end
end

local values = {64, 25, 12, 22, 11}
print_table(selection_sort(values))
