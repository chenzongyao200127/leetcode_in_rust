# 375_Guess_Number_Higher_or_Lower_II
# https://leetcode.cn/problems/guess-number-higher-or-lower-ii/

# 我们正在玩一个猜数游戏，游戏规则如下：
# 我从 1 到 n 之间选择一个数字。
# 你来猜我选了哪个数字。
# 如果你猜到正确的数字，就会 赢得游戏 。
# 如果你猜错了，那么我会告诉你，我选的数字比你的 更大或者更小 ，并且你需要继续猜数。
# 每当你猜了数字 x 并且猜错了的时候，你需要支付金额为 x 的现金。如果你花光了钱，就会 输掉游戏 。
# 给你一个特定的数字 n ，返回能够 确保你获胜 的最小现金数，不管我选择那个数字 。
from pprint import pprint
class Solution:
    def getMoneyAmount(self, n: int) -> int:
        dp = [[float('inf')] * n for _ in range(n)]
        
        for start in range(n):
            dp[start][start] = 0
        
        for start in range(n-1):
            for end in range(start+1, n):
                dp[start][end] = start
        
        for start in range(n-2):
            for end in range(start+2, n):
                dp[start][end] = start+1 
        
        for start in range(n-2, -1, -1):
            for end in range(start+1, n):
                min_val = float('inf')
                # 遍历所有可能的猜测k
                for k in range(start, end):
                    # 关键
                    worst_case = k + 1 + max(dp[start][k-1], dp[k+1][end])
                    min_val = min(min_val, worst_case)
                dp[start][end] = min_val
        
        return dp[0][-1]
    
s = Solution()
ans = s.getMoneyAmount(n = 10)
print(ans)



class Solution:
    def getMoneyAmount(self, n: int) -> int:
        # 定义dp函数
        dp = [[0] * (n + 1) for _ in range(n + 1)]
        
        # 第一层for循环为斜对角计算的起始位置，即列从1到n
        for k in range(1, n + 1):
            # 行始终从1开始，列从1到n
            i, j = 1, k
            
            # 按照斜对角for循环
            while j <= n:
                # 最小问题，对于j与i差距小于2的情况，可以直接得到结果
                if j == i:
                    dp[i][j] = 0
                elif j - i <= 1:
                    dp[i][j] = i
                elif j - i <= 2:
                    dp[i][j] = i + 1
                else:
                    # 对于更大问题，只能依赖更小问题的计算，此处为F(i,j)递推公式的实现
                    dp[i][j] = float('inf')
                    for l in range(i + 1, j):
                        dp[i][j] = min(dp[i][j], l + max(dp[i][l - 1], dp[l + 1][j]))
                
                # 斜对角循环方式即为向右向下移动1
                i += 1
                j += 1
        
        return dp[1][n]

s = Solution()
ans = s.getMoneyAmount(n = 10)
print(ans)


class Solution:
    def getMoneyAmount(self, n: int) -> int:
        # 初始化二维dp数组
        dp = [[0] * n for _ in range(n)]
        
        # 从后向前遍历开始位置
        for start in range(n-2, -1, -1):
            for end in range(start+1, n):
                min_val = float('inf')
                # 遍历所有可能的猜测k
                for k in range(start, end):
                    worst_case = k + 1 + max(dp[start][k-1], dp[k+1][end])
                    min_val = min(min_val, worst_case)
                dp[start][end] = min_val
        
        return dp[0][-1]
    
s = Solution()
ans = s.getMoneyAmount(n = 10)
print(ans)
