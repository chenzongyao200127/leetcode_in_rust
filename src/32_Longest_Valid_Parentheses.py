# 32_Longest_Valid_Parentheses
# https://leetcode.cn/problems/longest-valid-parentheses/description/

# 给你一个只包含 '(' 和 ')' 的字符串，找出最长有效（格式正确且连续）括号子串的长度。
class Solution:
    def longestValidParentheses(self, s: str) -> int:
        if len(s) == 0:
            return 0

        s = list(s)
        dp = [0] * (len(s)+1)
        dp[1] = 0
        for i in range(2, len(s)+1):
            if s[i-1] == "(":
                dp[i] = 0
            else:
                if s[i-2] == "(":
                    dp[i] = dp[i-2] + 2
                else:
                    if (i-2-dp[i-1]) >= 0 and s[i-2-dp[i-1]] == "(":
                        dp[i] = dp[i-1] + 2 + dp[i-2-dp[i-1]]
                    else:
                        dp[i] = 0
        return max(dp)
                