# 1463_摘樱桃_II
# https://leetcode.cn/problems/cherry-pickup-ii/description/?envType=daily-question&envId=2024-05-07

# 给你一个 rows x cols 的矩阵 grid 来表示一块樱桃地。 grid 中每个格子的数字表示你能获得的樱桃数目。
# 你有两个机器人帮你收集樱桃，机器人 1 从左上角格子 (0,0) 出发，机器人 2 从右上角格子 (0, cols-1) 出发。
# 请你按照如下规则，返回两个机器人能收集的最多樱桃数目：

# 从格子 (i,j) 出发，机器人可以移动到格子 (i+1, j-1)，(i+1, j) 或者 (i+1, j+1) 。
# 当一个机器人经过某个格子时，它会把该格子内所有的樱桃都摘走，然后这个位置会变成空格子，即没有樱桃的格子。
# 当两个机器人同时到达同一个格子时，它们中只有一个可以摘到樱桃。
# 两个机器人在任意时刻都不能移动到 grid 外面。
# 两个机器人最后都要到达 grid 最底下一行。

from typing import List
from functools import cache


# 会超时的递归写法
class Solution:
    def jewelleryValue(self, grid: List[List[int]]) -> int:
        def dfs(i: int, j: int) -> int:
            if i < 0 or j < 0:
                return 0
            return max(dfs(i, j - 1), dfs(i - 1, j)) + grid[i][j]
        return dfs(len(grid) - 1, len(grid[0]) - 1)


class Solution:
    def jewelleryValue(self, grid: List[List[int]]) -> int:
        @cache
        def dfs(i: int, j: int) -> int:
            if i < 0 or j < 0:
                return 0
            return max(dfs(i, j - 1), dfs(i - 1, j)) + grid[i][j]
        return dfs(len(grid) - 1, len(grid[0]) - 1)


class Solution:
    def jewelleryValue(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        f = [[0] * (n + 1) for _ in range(m + 1)]
        for i, row in enumerate(grid):
            for j, x in enumerate(row):
                f[i + 1][j + 1] = max(f[i + 1][j], f[i][j + 1]) + x
        return f[m][n]


class Solution:
    def jewelleryValue(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        f = [[0] * (n + 1) for _ in range(2)]
        for i, row in enumerate(grid):
            for j, x in enumerate(row):
                f[(i + 1) % 2][j + 1] = max(f[(i + 1) % 2][j], f[i % 2][j + 1]) + x
        return f[m % 2][n]


class Solution:
    def jewelleryValue(self, grid: List[List[int]]) -> int:
        n = len(grid[0])
        f = [0] * (n + 1)
        for row in grid:
            for j, x in enumerate(row):
                f[j + 1] = max(f[j], f[j + 1]) + x
        return f[n]


class Solution:
    def jewelleryValue(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        f = grid[0]  # 这里没有拷贝，f 和 grid[0] 都持有同一段内存
        for j in range(1, n):
            f[j] += f[j - 1]
        for i in range(1, m):
            f[0] += grid[i][0]
            for j in range(1, n):
                f[j] = max(f[j - 1], f[j]) + grid[i][j]
        return f[-1]


class Solution:
    def cherryPickup(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        dp = [[[0] * (n + 2) for _ in range(n + 2)] for _ in range(m + 1)]
        for i in range(m - 1, -1, -1):
            for j in range(min(n, i + 1)):
                for k in range(max(j + 1, n - 1 - i), n):
                    dp[i][j + 1][k + 1] = max(
                        dp[i + 1][j][k],
                        dp[i + 1][j][k + 1],
                        dp[i + 1][j][k + 2],
                        dp[i + 1][j + 1][k],
                        dp[i + 1][j + 1][k + 1],
                        dp[i + 1][j + 1][k + 2],
                        dp[i + 1][j + 2][k],
                        dp[i + 1][j + 2][k + 1],
                        dp[i + 1][j + 2][k + 2],
                    ) + grid[i][j] + grid[i][k]
        return dp[0][1][n]

# 滚动数组优化


class Solution:
    def cherryPickup(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        pre = [[0] * (n + 2) for _ in range(n + 2)]
        cur = [[0] * (n + 2) for _ in range(n + 2)]
        for i in range(m - 1, -1, -1):
            for j in range(min(n, i + 1)):
                for k in range(max(j + 1, n - 1 - i), n):
                    cur[j + 1][k + 1] = max(
                        pre[j][k], pre[j][k + 1], pre[j][k + 2],
                        pre[j + 1][k], pre[j + 1][k + 1], pre[j + 1][k + 2],
                        pre[j + 2][k], pre[j + 2][k + 1], pre[j + 2][k + 2],
                    ) + grid[i][j] + grid[i][k]
            pre, cur = cur, pre  # 下一个 i 的 pre 是 cur
        return pre[1][n]
