# 122_Best_Time_to_Buy_and_Sell_Stock_II
# https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/

# 给你一个整数数组 prices ，其中 prices[i] 表示某支股票第 i 天的价格。

# 在每一天，你可以决定是否购买和/或出售股票。你在任何时候 最多 只能持有 一股 股票。你也可以先购买，然后在 同一天 出售。

# 返回 你能获得的 最大 利润 。


from typing import List
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        ans = 0
        for i in range(len(prices)-1):
            if prices[i+1] - prices[i] >= 0:
                ans += (prices[i+1] - prices[i])
        
        return ans