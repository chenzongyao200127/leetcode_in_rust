# 309_买卖股票的最佳时机含冷冻期
# https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-cooldown/

from functools import lru_cache
from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)

        @lru_cache(maxsize=None)
        def dfs(day, hold):
            if day < 0:
                return -float("inf") if hold else 0

            if hold:
                return max(dfs(day-1, True), dfs(day-2, False) - prices[day])
            else:
                return max(dfs(day-1, False), dfs(day-1, True) + prices[day])

        return dfs(n-1, False)


