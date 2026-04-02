local function count_vowels(text)
  local counts = {a = 0, e = 0, i = 0, o = 0, u = 0}

  for ch in text:lower():gmatch('.') do
    if counts[ch] ~= nil then
      counts[ch] = counts[ch] + 1
    end
  end

  return counts
end

local function print_counts(counts)
  for _, k in ipairs({'a', 'e', 'i', 'o', 'u'}) do
    print(k .. ': ' .. counts[k])
  end
end

print_counts(count_vowels('Algorithm and Automation'))
