# 123. 买卖股票的最佳时机 III
# https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iii/

from typing import List

# 给定一个数组，它的第 i 个元素是一支给定的股票在第 i 天的价格。
# 设计一个算法来计算你所能获取的最大利润。你最多可以完成 两笔 交易。
# 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

# 示例 1:

# 输入：prices = [3,3,5,0,0,3,1,4]
# 输出：6
# 解释：在第 4 天（股票价格 = 0）的时候买入，在第 6 天（股票价格 = 3）的时候卖出，这笔交易所能获得利润 = 3-0 = 3 。
#      随后，在第 7 天（股票价格 = 1）的时候买入，在第 8 天 （股票价格 = 4）的时候卖出，这笔交易所能获得利润 = 4-1 = 3 。


# 示例 2：
# 输入：prices = [1,2,3,4,5]
# 输出：4
# 解释：在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。   
#      注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。   
#      因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。

# 示例 3：
# 输入：prices = [7,6,4,3,1] 
# 输出：0 
# 解释：在这个情况下, 没有交易完成, 所以最大利润为 0。

# 示例 4：
# 输入：prices = [1]
# 输出：0

# 未进行过任何操作；
# 只进行过一次买操作；
# 进行了一次买操作和一次卖操作，即完成了一笔交易；
# 在完成了一笔交易的前提下，进行了第二次买操作；
# 完成了全部两笔交易。

# i 代表第几天（0 到 n-1，其中 n 是天数）
# j 代表是否持有股票（0 代表不持有，1 代表持有）
# k 代表已经进行过几次交易（0, 1, 2，其中 2 代表至多交易两次）
# 因此，dp[i][j][k] 表示在第 i 天，是否持有股票，已经交易 k 次后的最大利润。

# 初始化 DP 表：
# 对于 dp[0][0][0]，即第 0 天，不持有股票，且没有进行过交易，此时的最大利润为 0；
# 对于 dp[0][0][1]，dp[0][0][2]，即第 0 天，不持有股票，但已经进行过 1 次或 2 次交易，这种情况是不存在的，初始化为负无穷大；
# 对于 dp[0][1][0]，即第 0 天，持有股票，但还没有进行过交易，也是不存在的，初始化为负无穷大；
# 对于 dp[0][1][1]，dp[0][1][2]，即第 0 天，持有股票，且已经进行过 1 次或 2 次交易，此时的最大利润应该就是负的第 0 天的股票价格。

# dp[i][0][0]：没有交易过，所以一直是 0；
# dp[i][0][1]：可能是今天卖出或者之前就卖出的，取最大值即可，dp[i][0][1] = max(dp[i-1][0][1], dp[i-1][1][1] + prices[i])；
# dp[i][0][2]：可能是今天卖出或者之前就卖出的，取最大值即可，dp[i][0][2] = max(dp[i-1][0][2], dp[i-1][1][2] + prices[i])；
# dp[i][1][0]：没有交易过但却有股票，这种情况是不可能出现的，所以还是负无穷；
# dp[i][1][1]：可能是今天买入，也可能是之前就买入的，取最大值即可，dp[i][1][1] = max(dp[i-1][1][1], dp[i-1][0][0] - prices[i])；
# dp[i][1][2]：可能是今天买入，也可能是之前就买入的，取最大值即可，dp[i][1][2] = max(dp[i-1][1][2], dp[i-1][0][1] - prices[i])。

class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        dp = [[[0]*3 for _ in range(2)] for _ in range(n)]
        dp[0][0][0], dp[0][0][1], dp[0][0][2] = 0, float('-inf'), float('-inf')
        dp[0][1][0], dp[0][1][1], dp[0][1][2] = float('-inf'), -prices[0], float('-inf')

        for i in range(1, n):
            dp[i][0][0] = dp[i-1][0][0]
            dp[i][0][1] = max(dp[i-1][0][1], dp[i-1][1][1] + prices[i])
            dp[i][0][2] = max(dp[i-1][0][2], dp[i-1][1][2] + prices[i])
            dp[i][1][0] = dp[i-1][1][0]
            dp[i][1][1] = max(dp[i-1][1][1], dp[i-1][0][0] - prices[i])
            dp[i][1][2] = max(dp[i-1][1][2], dp[i-1][0][1] - prices[i])

        return max(dp[n-1][0][0], dp[n-1][0][1], dp[n-1][0][2])


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        first_purchase = -float('inf')
        first_sell = 0
        second_purchase = -float('inf')
        second_sell = 0
        for price in prices:
            if -price > first_purchase:
                first_purchase = -price
            if price + first_purchase > first_sell:
                first_sell = price + first_purchase
            if first_sell - price > second_purchase:
                second_purchase = first_sell - price
            if price + second_purchase > second_sell:
                second_sell = price + second_purchase
        return max(second_sell, first_sell, 0)
