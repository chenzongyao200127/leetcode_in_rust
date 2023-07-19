# 188_Best_Time_to_Buy_and_Sell_Stock_IV
# https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iv/

# 给定一个整数数组 prices ，它的第 i 个元素 prices[i] 是一支给定的股票在第 i 天的价格，和一个整型 k 。
# 设计一个算法来计算你所能获取的最大利润。你最多可以完成 k 笔交易。也就是说，你最多可以买 k 次，卖 k 次。
# 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

# 示例 1：
# 输入：k = 2, prices = [2,4,1]
# 输出：2
# 解释：在第 1 天 (股票价格 = 2) 的时候买入，在第 2 天 (股票价格 = 4) 的时候卖出，这笔交易所能获得利润 = 4-2 = 2 。

# 示例 2：
# 输入：k = 2, prices = [3,2,6,5,0,3]
# 输出：7
# 解释：在第 2 天 (股票价格 = 2) 的时候买入，在第 3 天 (股票价格 = 6) 的时候卖出, 这笔交易所能获得利润 = 6-2 = 4 。
# 随后，在第 5 天 (股票价格 = 0) 的时候买入，在第 6 天 (股票价格 = 3) 的时候卖出, 这笔交易所能获得利润 = 3-0 = 3 。
from typing import List
class Solution:
    def maxProfit(self, k: int, prices: List[int]) -> int:
        n = len(prices)
        dp = [[[0] * (k+1) for _ in range(2)] for _ in range(n)]
        dp[0][0][0] = 0
        dp[0][1][0] = float('-inf'), 
        dp[0][1][1] = -prices[0]
        for i in range(1, k+1):
            dp[0][0][i] = float('-inf')
        for i in range(2, k+1):
            dp[0][1][i] = float('-inf')

        for i in range(1, n):
            dp[i][0][0] = dp[i-1][0][0]
            dp[i][1][0] = dp[i-1][0][0]
            for j in range(1, k+1):
                dp[i][0][j] = max(dp[i-1][0][j], dp[i-1][1][j] + prices[i])
                dp[i][1][j] = max(dp[i-1][1][j], dp[i-1][0][j-1] - prices[i])
        
        return max(dp[n-1][0])


# wqs 二分 （ACM）
class Solution:
    def maxProfit(self, k: int, prices: List[int]) -> int:
        if not prices:
            return 0

        n = len(prices)
        # 二分查找的上下界
        left, right = 1, max(prices)
        # 存储答案，如果值为 -1 表示二分查找失败
        ans = -1

        while left <= right:
            # 二分得到当前的斜率（手续费）
            c = (left + right) // 2

            # 使用与 714 题相同的动态规划方法求解出最大收益以及对应的交易次数
            buyCount = sellCount = 0
            buy, sell = -prices[0], 0

            for i in range(1, n):
                if sell - prices[i] >= buy:
                    buy = sell - prices[i]
                    buyCount = sellCount
                if buy + prices[i] - c >= sell:
                    sell = buy + prices[i] - c
                    sellCount = buyCount + 1

            # 如果交易次数大于等于 k，那么可以更新答案
            # 这里即使交易次数严格大于 k，更新答案也没有关系，因为总能二分到等于 k 的
            if sellCount >= k:
                # 别忘了加上 kc
                ans = sell + k * c
                left = c + 1
            else:
                right = c - 1

        # 如果二分查找失败，说明交易次数的限制不是瓶颈
        # 可以看作交易次数无限，直接使用贪心方法得到答案
        if ans == -1:
            ans = sum(max(prices[i] - prices[i - 1], 0) for i in range(1, n))

        return ans