# 741_摘樱桃
# https://leetcode.cn/problems/cherry-pickup/description/?envType=daily-question&envId=2024-05-06

# 你一个 n x n 的网格 grid ，代表一块樱桃地，每个格子由以下三种数字的一种来表示：

# 0 表示这个格子是空的，所以你可以穿过它。
# 1 表示这个格子里装着一个樱桃，你可以摘到樱桃然后穿过它。
# -1 表示这个格子里有荆棘，挡着你的路。

# 请你统计并返回：在遵守下列规则的情况下，能摘到的最多樱桃数：
# 从位置 (0, 0) 出发，最后到达 (n - 1, n - 1) ，只能向下或向右走，并且只能穿越有效的格子（即只可以穿过值为 0 或者 1 的格子）；
# 当到达 (n - 1, n - 1) 后，你要继续走，直到返回到 (0, 0) ，只能向上或向左走，并且只能穿越有效的格子；
# 当你经过一个格子且这个格子包含一个樱桃时，你将摘到樱桃并且这个格子会变成空的（值变为 0 ）；
# 如果在 (0, 0) 和 (n - 1, n - 1) 之间不存在一条可经过的路径，则无法摘到任何一个樱桃。

from typing import List
from math import inf


class Solution:
    def cherryPickup(self, grid: List[List[int]]) -> int:
        n = len(grid)
        f = [[[-inf] * (n + 1) for _ in range(n + 1)]
             for _ in range(n * 2 - 1)]
        f[0][1][1] = grid[0][0]
        for t in range(1, n * 2 - 1):
            for j in range(max(t - n + 1, 0), min(t + 1, n)):
                if grid[t - j][j] < 0:
                    continue
                for k in range(j, min(t + 1, n)):
                    if grid[t - k][k] < 0:
                        continue
                    f[t][j + 1][k + 1] = max(f[t - 1][j + 1][k + 1], f[t - 1][j + 1][k], f[t - 1][j][k + 1], f[t - 1][j][k]) + \
                        grid[t - j][j] + (grid[t - k][k] if k != j else 0)
        return max(f[-1][n][n], 0)
