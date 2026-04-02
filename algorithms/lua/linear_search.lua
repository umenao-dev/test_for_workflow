local function linear_search(arr, target)
  for i = 1, #arr do
    if arr[i] == target then
      return i
    end
  end
  return -1
end

local function demo()
  local nums = {5, 8, 2, 9, 3}
  print(linear_search(nums, 9))
  print(linear_search(nums, 7))
end


demo()
