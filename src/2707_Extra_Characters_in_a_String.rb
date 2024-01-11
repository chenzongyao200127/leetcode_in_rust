# @param {String} s
# @param {String[]} dictionary
# @return {Integer}

def min_extra_char(s, dictionary)
    d = dictionary.to_set
    n = s.length
    f = Array.new(n + 1, 0)
    (0...n).each do |i|
      f[i + 1] = f[i] + 1 # Case of not choosing a substring
      (0..i).each do |j|
        # Enumerate which substring to choose
        f[i + 1] = [f[i + 1], f[j]].min if d.include?(s[j..i])
      end
    end
    f[n]
  end