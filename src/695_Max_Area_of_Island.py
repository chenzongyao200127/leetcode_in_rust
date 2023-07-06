

from typing import List

class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        ans = 0
        m = len(grid)
        n = len(grid[0])

        def dfs(x, y, grid):
            size = 1
            grid[x][y] = 0
            for (dx, dy) in directions:
                if 0 <= dx + x < m and 0 <= dy + y < n and grid[dx + x][dy + y] == 1:
                    size += dfs(dx + x, dy + y, grid)
            return size

        for i in range(m):
            for j in range(n):
                if grid[i][j] == 1:
                    ans = max(ans, dfs(i, j, grid))

        return ans