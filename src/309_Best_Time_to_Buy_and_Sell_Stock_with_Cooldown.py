# 309_Best_Time_to_Buy_and_Sell_Stock_with_Cooldown
# https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-cooldown/description/

# 给定一个整数数组prices，其中第  prices[i] 表示第 i 天的股票价格 。​

# 设计一个算法计算出最大利润。在满足以下约束条件下，你可以尽可能地完成更多的交易（多次买卖一支股票）:

# 卖出股票后，你无法在第二天买入股票 (即冷冻期为 1 天)。
# 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
from typing import List

class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        A = -prices[0]    # 有股票
        B = float('-inf') # 无股票 冷静
        C = 0             # 无股票 不冷静
                
        for i in range(1, len(prices)):
            next_A = max(A, C - prices[i])
            next_B = A + prices[i]
            next_C = max(C, B)
            A, B, C = next_A, next_B, next_C
        
        return max(A, B, C)