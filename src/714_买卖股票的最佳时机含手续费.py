# 714_买卖股票的最佳时机含手续费
# https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/

from typing import List
from functools import lru_cache


class Solution:
    def maxProfit(self, prices: List[int], fee: int) -> int:
        n = len(prices)

        @lru_cache(maxsize=None)
        def dfs(day, hold):
            if day < 0:
                return -float("inf") if hold else 0

            if hold:
                return max(dfs(day-1, True), dfs(day-1, False) - prices[day])
            else:
                return max(dfs(day-1, False), dfs(day-1, True) + prices[day] - fee)

        return dfs(n-1, False)


class Solution:
    def maxProfit(self, prices: List[int], fee: int) -> int:
        n = len(prices)
        dp = [[-float('inf') for _ in range(2)] for _ in range(n+1)]
        dp[0][0] = 0

        for day, p in enumerate(prices):
            dp[day+1][0] = max(dp[day][0], dp[day][1] + p - fee)
            dp[day+1][1] = max(dp[day][1], dp[day][0] - p)

        return dp[-1][0]
