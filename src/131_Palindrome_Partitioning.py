# 131_Palindrome_Partitioning
# https://leetcode.cn/problems/palindrome-partitioning/

# 给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是 回文串 。返回 s 所有可能的分割方案。
# 回文串 是正着读和反着读都一样的字符串。

# 示例 1：
# 输入：s = "aab"

# 输出：[["a","a","b"],["aa","b"]]
# 示例 2：

# 输入：s = "a"
# 输出：[["a"]]

# 提示：

# 1 <= s.length <= 16
# s 仅由小写英文字母组成

class Solution(object):
    def minCut(self, s):
        N = len(s)
        dp = [N] * N
        for i in range(N):
            if self.isPalindrome(s[0 : i + 1]):
                dp[i] = 0
                continue
            for j in range(i):
                if self.isPalindrome(s[j + 1 : i + 1]):
                    dp[i] = min(dp[i], dp[j] + 1)
        return dp[N - 1]
    
    def isPalindrome(self, s):
        return s == s[::-1]

# 时间优化
class Solution:
    def minCut(self, s: str) -> int:
        s = list(s)
        n = len(s)
        dp = [[True] * n for _ in range(n)]

        for i in range(n-1, -1, -1):
            for j in range(i+1, n):
                dp[i][j] = (dp[i+1][j-1] and s[i] == s[j])
                
        f = [float("inf")] * n
        for i in range(n):
            if dp[0][i]:
                f[i] = 0
            else:
                for j in range(i, -1, -1):
                    if dp[j][i]:
                        f[i] = min(f[i], f[j-1] + 1)

        return f[-1]
