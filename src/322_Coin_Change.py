# 322_Coin_Change
# https://leetcode.cn/problems/coin-change/

from typing import List

# 给你一个整数数组 coins ，表示不同面额的硬币；以及一个整数 amount ，表示总金额。

# 计算并返回可以凑成总金额所需的 最少的硬币个数 。如果没有任何一种硬币组合能组成总金额，返回 -1 。

# 你可以认为每种硬币的数量是无限的。

# 示例 1：
# 输入：coins = [1, 2, 5], amount = 11
# 输出：3 
# 解释：11 = 5 + 5 + 1

# 示例 2：
# 输入：coins = [2], amount = 3
# 输出：-1

# 示例 3：
# 输入：coins = [1], amount = 0
# 输出：0

# 过了，但是时间很慢
class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        if amount == 0:
            return 0

        coins.sort()
        dp = [100000] * (amount + 1)
        dp[0] = 0
        for i in range(1, amount+1):
            for c in coins:
                if c > i:
                    break
                if i == c:
                    dp[i] = 1
                else:
                    dp[i] = min(dp[i], dp[i-c] + 1)

        if dp[amount] == 100000:
            return -1
        else:
            return dp[amount]



# 简单优化
class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        if amount == 0:
            return 0

        coins.sort(reverse=True)
        dp = [float('inf')] * (amount + 1)
        dp[0] = 0
        for i in range(1, amount+1):
            for c in coins:
                if c > i:
                    continue
                dp[i] = min(dp[i], dp[i-c] + 1)

        if dp[amount] == float('inf'):
            return -1
        else:
            return dp[amount]
        
        
        
# 官解
class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        dp = [float('inf')] * (amount + 1)
        dp[0] = 0
        
        for coin in coins:
            for x in range(coin, amount + 1):
                dp[x] = min(dp[x], dp[x - coin] + 1)
        return dp[amount] if dp[amount] != float('inf') else -1 