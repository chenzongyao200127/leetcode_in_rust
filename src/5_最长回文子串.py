# 5. 最长回文子串
# https://leetcode.cn/problems/longest-palindromic-substring/description/

# 给你一个字符串 s，找到 s 中最长的回文子串

# 如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。

class Solution:
    def longestPalindrome(self, s: str) -> str:
        if len(s) == 0:
            return ''
        if len(s) == 1:
            return s
        max_len = 0
        start = 0
        for i in range(len(s)):
            # 以 i 为中心的回文串
            if i - max_len >= 1 and s[i - max_len - 1: i + 1] == s[i - max_len - 1: i + 1][::-1]:
                start = i - max_len - 1
                max_len += 2
                continue
            if i - max_len >= 0 and s[i - max_len: i + 1] == s[i - max_len: i + 1][::-1]:
                start = i - max_len
                max_len += 1
        return s[start: start + max_len]


# test case
print(Solution().longestPalindrome('babad'))  # bab
print(Solution().longestPalindrome('cbbd'))  # bb
print(Solution().longestPalindrome('a'))  # a
print(Solution().longestPalindrome('ac'))  # a
print(Solution().longestPalindrome('bb'))  # bb
print(Solution().longestPalindrome('ccc'))  # ccc

# DP
# 对于每个子串 s[i:j+1]（其中 j = i + length - 1），
# 如果 s[i] 和 s[j] 相同，并且内部子串 s[i+1:j] 是回文（即 dp[i+1][j-1] 为 True），则 s[i:j+1] 也是回文。


class Solution:
    def longestPalindrome(self, s: str) -> str:
        n = len(s)
        if n <= 1:
            return s

        # 初始化一个n*n的二维列表，全部填充False
        dp = [[False] * n for _ in range(n)]
        start = 0
        max_len = 1

        # 单个字符是回文
        for i in range(n):
            dp[i][i] = True

        # 检查长度为2的子串
        for i in range(n - 1):
            if s[i] == s[i + 1]:
                dp[i][i + 1] = True
                start = i
                max_len = 2

        # 检查长度大于2的子串
        for length in range(3, n + 1):          # 子串长度
            for i in range(n - length + 1):     # 子串起始点
                j = i + length - 1              # 子串结束点
                if s[i] == s[j] and dp[i + 1][j - 1]:
                    dp[i][j] = True
                    start = i
                    max_len = length

        # 返回最长的回文子串
        return s[start:start + max_len]


# 测试代码
sol = Solution()
print(sol.longestPalindrome("babad"))  # 输出: "bab" 或 "aba"
print(sol.longestPalindrome("cbbd"))   # 输出: "bb"


# 中心拓展法
class Solution:
    def longestPalindrome(self, s: str) -> str:
        if len(s) < 2:
            return s

        def expandAroundCenter(left: int, right: int) -> str:
            while left >= 0 and right < len(s) and s[left] == s[right]:
                left -= 1
                right += 1
            # 注意，跳出循环时，left和right都超出了边界，所以实际的回文子串是s[left+1:right]
            return s[left + 1:right]

        longest_palindrome = ""

        for i in range(len(s)):
            # 考虑奇数长度的回文
            palindrome1 = expandAroundCenter(i, i)
            if len(palindrome1) > len(longest_palindrome):
                longest_palindrome = palindrome1

            # 考虑偶数长度的回文
            palindrome2 = expandAroundCenter(i, i + 1)
            if len(palindrome2) > len(longest_palindrome):
                longest_palindrome = palindrome2

        return longest_palindrome
