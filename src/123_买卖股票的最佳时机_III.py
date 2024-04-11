# 123_买卖股票的最佳时机_III
# https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iii/description/

from functools import lru_cache
from typing import List

# 给定一个数组，它的第 i 个元素是一支给定的股票在第 i 天的价格。
# 设计一个算法来计算你所能获取的最大利润。你最多可以完成 两笔 交易。
# 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。


from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        memo = {}

        def dfs(day, k, hold):
            if day < 0:
                return -float("inf") if hold else 0

            if k < 0:
                return -float("inf")

            # Memoization check
            if (day, k, hold) in memo:
                return memo[(day, k, hold)]

            price = prices[day]
            if hold:
                # Choice 1: Sell the stock today
                # Choice 2: Do not sell the stock today
                memo[(day, k, hold)] = max(
                    dfs(day - 1, k, False) - price, dfs(day - 1, k, True))
            else:
                # Choice 1: Buy the stock today
                # Choice 2: Do not buy the stock today
                memo[(day, k, hold)] = max(
                    dfs(day - 1, k - 1, True) + price, dfs(day - 1, k, False))

            return memo[(day, k, hold)]

        return dfs(n - 1, 2, False)


class Solution:
    def maxProfit(self, k: int, prices: List[int]) -> int:
        n = len(prices)
        # Initialize the DP array with dimensions [number of days + 1][number of transactions + 2][2 states]
        dp = [[[-float('inf')] * 2 for _ in range(k + 2)] for _ in range(n+1)]

        for j in range(1, k+2):
            dp[0][j][0] = 0

        for idx, p in enumerate(prices):
            for j in range(1, k+2):
                # do not hold stock
                dp[idx+1][j][0] = max(dp[idx][j][0], dp[idx][j-1][1] + p)

                # hold the stock
                dp[idx+1][j][1] = max(dp[idx][j][1], dp[idx][j][0] - p)

        return dp[-1][-1][0]


s = Solution()
prices = [3, 3, 5, 0, 0, 3, 1, 4]
ans = s.maxProfit(2, prices)
print(ans)
assert ans == 6


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        max_transactions = 2

        @lru_cache(maxsize=None)
        def dfs(day: int, transactions_remaining: int, hold: bool) -> int:
            # Base cases
            if day < 0 or transactions_remaining == 0:
                return 0
            if hold:
                # Hold or sell
                do_nothing = dfs(day - 1, transactions_remaining, True)
                sell = dfs(day - 1, transactions_remaining -
                           1, False) + prices[day]
                return max(do_nothing, sell)
            else:
                # Do nothing or buy
                do_nothing = dfs(day - 1, transactions_remaining, False)
                buy = float('-inf') if transactions_remaining <= 0 else dfs(day -
                                                                            1, transactions_remaining, True) - prices[day]
                return max(do_nothing, buy)

        return dfs(n - 1, max_transactions, False)

# Example usage:
# sol = Solution()
# profit = sol.maxProfit([1,2,3,4,5])
# print(profit)
