def quick_sort(arr)
  return arr if arr.length <= 1

  pivot = arr[arr.length / 2]
  left = []
  mid = []
  right = []

  arr.each do |x|
    if x < pivot
      left << x
    elsif x > pivot
      right << x
    else
      mid << x
    end
  end

  quick_sort(left) + mid + quick_sort(right)
end

p quick_sort([3, 6, 8, 10, 1, 2, 1]) if __FILE__ == $PROGRAM_NAME
