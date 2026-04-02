def merge_intervals(intervals)
  return [] if intervals.empty?

  sorted = intervals.sort_by { |x| x[0] }
  merged = [sorted[0].dup]

  sorted[1..].each do |start_t, end_t|
    last = merged[-1]
    if start_t <= last[1]
      last[1] = [last[1], end_t].max
    else
      merged << [start_t, end_t]
    end
  end

  merged
end

if __FILE__ == $PROGRAM_NAME
  p merge_intervals([[1, 3], [2, 6], [8, 10], [9, 11]])
end
