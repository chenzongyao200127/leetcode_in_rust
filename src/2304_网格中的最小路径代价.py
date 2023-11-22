# 2304_网格中的最小路径代价
# https://leetcode.cn/problems/minimum-path-cost-in-a-grid/description/

from typing import List

class Solution:
    def minPathCost(self, grid: List[List[int]], moveCost: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])
        
        dp = [[float("inf")] * n for _ in range(m)]
        for i in range(n):
            dp[0][i] = grid[0][i]
            
        for i in range(1, m):
            for t in range(n):
                for s in range(n):
                    value = grid[i-1][s]
                    dp[i][t] = min(dp[i][t], dp[i-1][s] + moveCost[value][t] + grid[i][t])

        return min(dp[-1])
        
s = Solution()
ans = s.minPathCost(grid = [[5,3],[4,0],[2,1]], moveCost = [[9,8],[1,5],[10,12],[18,6],[2,4],[14,3]])
print(ans)