# 664_Strange_Printer
# https://leetcode.cn/problems/strange-printer/

# 有台奇怪的打印机有以下两个特殊要求：
# 打印机每次只能打印由 同一个字符 组成的序列。
# 每次可以在从起始到结束的任意位置打印新字符，并且会覆盖掉原来已有的字符。
# 给你一个字符串 s ，你的任务是计算这个打印机打印它需要的最少打印次数。

# 示例 1：
# 输入：s = "aaabbb"
# 输出：2
# 解释：首先打印 "aaa" 然后打印 "bbb"。

# 示例 2：
# 输入：s = "aba"
# 输出：2
# 解释：首先打印 "aaa" 然后在第二个位置打印 "b" 覆盖掉原来的字符 'a'。
from pprint import pprint
class Solution:
    def strangePrinter(self, s: str) -> int:
        n = len(s)
        dp = [[float("inf")] * n for _ in range(n)]
        for i in range(n):
            dp[i][i] = 1
        
        for len in range(2, n+1):  # length of substring
            for start in range(n - len + 1):  # starting index
                end = start + len - 1  # ending index
                if s[start] == s[end]:
                    dp[start][end] = dp[start][end-1]
                else:
                    for k in range(start, end):
                        dp[start][end] = min(dp[start][end], dp[start][k] + dp[k+1][end])
        
        return dp[0][n-1]



s = Solution()
ans = s.strangePrinter("aaabbb")
print(ans)

s = Solution()
ans = s.strangePrinter("aba")
print(ans)
