# 100281_矩阵中的最大得分
# https://leetcode.cn/problems/maximum-difference-score-in-a-grid/

from typing import List


class Solution:
    def maxScore(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        # DP 表示从 (0, 0) 到 (i, j) 的起点最小值
        dp = [[-float('inf')] * n for _ in range(m)]
        res = -float('inf')
        for i in range(m):
            for j in range(n):
                tmp = float('inf')
                if i > 0:
                    tmp = min(tmp, dp[i-1][j])
                if j > 0:
                    tmp = min(tmp, dp[i][j-1])

                # 注意至少需要移动一次
                res = max(res, grid[i][j] - tmp)

                # 小技巧，更新res后，再更新dp
                dp[i][j] = min(tmp, grid[i][j])

        return res


class Solution:
    def maxScore(self, grid: List[List[int]]) -> int:
        rows, cols = len(grid), len(grid[0])

        # DP table to store the minimum value encountered from (0, 0) to (i, j)
        dp = [[-float('inf')] * cols for _ in range(rows)]

        max_difference = -float('inf')

        for i in range(rows):
            for j in range(cols):
                # Minimum value from the top or left neighbor, initialized to infinity
                min_from_neighbors = float('inf')

                # Check from the top neighbor if exists
                if i > 0:
                    min_from_neighbors = min(min_from_neighbors, dp[i-1][j])

                # Check from the left neighbor if exists
                if j > 0:
                    min_from_neighbors = min(min_from_neighbors, dp[i][j-1])

                # Calculate the difference from the current cell value to the minimum from neighbors
                # Note: grid[i][j] - float('inf') is handled implicitly because min_from_neighbors
                # will not be infinity after the first row and first column
                if min_from_neighbors != float('inf'):
                    max_difference = max(
                        max_difference, grid[i][j] - min_from_neighbors)

                # Update the DP table with the minimum value between the current cell and min_from_neighbors
                dp[i][j] = min(min_from_neighbors, grid[i][j])

        return max_difference
