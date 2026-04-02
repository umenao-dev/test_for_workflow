def normalize(text)
  text.downcase.gsub(/[^a-z0-9]/, '')
end


def palindrome?(text)
  s = normalize(text)
  left = 0
  right = s.length - 1

  while left < right
    return false if s[left] != s[right]

    left += 1
    right -= 1
  end

  true
end

puts palindrome?('A man, a plan, a canal: Panama') if __FILE__ == $PROGRAM_NAME
