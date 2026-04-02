def two_sum(nums, target)
  seen = {}

  nums.each_with_index do |num, idx|
    need = target - num
    return [seen[need], idx] if seen.key?(need)

    seen[num] = idx
  end

  nil
end

if __FILE__ == $PROGRAM_NAME
  values = [2, 7, 11, 15]
  p two_sum(values, 9)
  p two_sum(values, 18)
end
