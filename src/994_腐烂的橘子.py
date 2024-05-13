# 994_腐烂的橘子
# https://leetcode.cn/problems/rotting-oranges/

from typing import List


class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        rows, cols = len(grid), len(grid[0])

        # Initialize the queue with rotten oranges
        queue = []
        fresh_oranges = 0
        for i in range(rows):
            for j in range(cols):
                if grid[i][j] == 2:
                    queue.append((i, j))
                elif grid[i][j] == 1:
                    fresh_oranges += 1

        # Directions: up, right, down, left
        directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]

        minutes = 0
        while queue and fresh_oranges:
            minutes += 1
            for _ in range(len(queue)):
                i, j = queue.pop(0)
                for dx, dy in directions:
                    ni, nj = i + dx, j + dy
                    if 0 <= ni < rows and 0 <= nj < cols and grid[ni][nj] == 1:
                        grid[ni][nj] = 2
                        fresh_oranges -= 1
                        queue.append((ni, nj))

        return -1 if fresh_oranges else minutes
