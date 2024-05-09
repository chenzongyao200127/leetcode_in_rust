# 华为技术暑期实习1面
# s 中最长回文子串

def find_longest_palindrome_str(str):
    if len(str) == 0:
        return ''
    if len(str) == 1:
        return str
    max_len = 0
    start = 0
    for i in range(len(str)):
        if i - max_len >= 1 and str[i - max_len - 1: i + 1] == str[i - max_len - 1: i + 1][::-1]:
            start = i - max_len - 1
            max_len += 2
            continue
        if i - max_len >= 0 and str[i - max_len: i + 1] == str[i - max_len: i + 1][::-1]:
            start = i - max_len
            max_len += 1
    return str[start: start + max_len]


# test case
print(find_longest_palindrome_str('babad'))  # bab
print(find_longest_palindrome_str('cbbd'))  # bb
