# 122_买卖股票的最佳时机_II
# https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/description/

from typing import List

# 初始版本


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        s = 0
        res = 0
        n = len(prices)

        if n == 2:
            return prices[-1] - prices[0] if prices[-1] > prices[0] else 0

        while s < n-1 and prices[s] > prices[s+1]:
            s += 1

        while s < n - 1:
            if prices[s] < prices[s + 1]:
                res += (prices[s+1] - prices[s])
            s += 1

        return res

# 优化


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        nums = []
        for i in range(len(prices)-1):
            nums.append(prices[i+1] - prices[i])

        nums = filter(lambda x: x >= 0, nums)
        return sum(nums)


# GPT-4
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        max_profit = 0
        for i in range(1, len(prices)):
            if prices[i] > prices[i - 1]:
                max_profit += prices[i] - prices[i - 1]
        return max_profit
