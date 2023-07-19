# 264_Ugly_Number_II
# https://leetcode.cn/problems/ugly-number-ii/description/

# 给你一个整数 n ，请你找出并返回第 n 个 丑数 。

# 丑数 就是只包含质因数 2、3 和/或 5 的正整数。

class Solution:
    def nthUglyNumber(self, n: int) -> int:
        dp = [1] * n
        p2, p3, p5 = 0, 0, 0

        for i in range(1, n):
            dp[i] = min(dp[p2] * 2, dp[p3] * 3, dp[p5] * 5)
            if dp[i] == dp[p2] * 2:
                p2 += 1
            if dp[i] == dp[p3] * 3:
                p3 += 1
            if dp[i] == dp[p5] * 5:
                p5 += 1

        return dp[n-1]
    
import heapq
# 最小堆
class Solution:
    def nthUglyNumber(self, n: int) -> int:
        factors = [2, 3, 5]
        seen = {1}
        heap = [1]

        for i in range(n - 1):
            curr = heapq.heappop(heap)
            for factor in factors:
                if (nxt := curr * factor) not in seen:
                    seen.add(nxt)
                    heapq.heappush(heap, nxt)

        return heapq.heappop(heap)