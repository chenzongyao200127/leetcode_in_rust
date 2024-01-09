# 100030_Minimum_Moves_to_Spread_Stones_Over_Grid
# https://leetcode.cn/problems/minimum-moves-to-spread-stones-over-grid/description/

# 给你一个大小为 3 * 3 ，下标从 0 开始的二维整数矩阵 grid ，分别表示每一个格子里石头的数目。
# 网格图中总共恰好有 9 个石头，一个格子里可能会有 多个 石头。

# 每一次操作中，你可以将一个石头从它当前所在格子移动到一个至少有一条公共边的相邻格子。

# 请你返回每个格子恰好有一个石头的 最少移动次数 。

# 输入：grid = [[1,1,0],[1,1,1],[1,2,1]]
# 输出：3
# 解释：让每个格子都有一个石头的一个操作序列为：
# 1 - 将一个石头从格子 (2,1) 移动到 (2,2) 。
# 2 - 将一个石头从格子 (2,2) 移动到 (1,2) 。
# 3 - 将一个石头从格子 (1,2) 移动到 (0,2) 。
# 总共需要 3 次操作让每个格子都有一个石头。
# 让每个格子都有一个石头的最少操作次数为 3 。


# 输入：grid = [[1,3,0],[1,0,0],[1,0,3]]
# 输出：4
# 解释：让每个格子都有一个石头的一个操作序列为：
# 1 - 将一个石头从格子 (0,1) 移动到 (0,2) 。
# 2 - 将一个石头从格子 (0,1) 移动到 (1,1) 。
# 3 - 将一个石头从格子 (2,2) 移动到 (1,2) 。
# 4 - 将一个石头从格子 (2,2) 移动到 (2,1) 。
# 总共需要 4 次操作让每个格子都有一个石头。
# 让每个格子都有一个石头的最少操作次数为 4 。


# 方法一：枚举全排列
from collections import deque
from typing import List
from itertools import permutations


class Solution:
    def minimumMoves(self, grid: List[List[int]]) -> int:
        # 初始化起点和终点列表
        start_points = []
        end_points = []

        # 遍历网格，寻找起点和终点
        for i, row in enumerate(grid):
            for j, value in enumerate(row):
                # 当格子的值大于1时，根据其值确定起点的数量
                if value > 1:
                    start_points.extend([(i, j)] * (value - 1))
                # 当格子的值为0时，将其作为一个终点
                elif value == 0:
                    end_points.append((i, j))

        # 初始化最小移动次数为无穷大
        min_moves = float("inf")

        # 遍历所有起点的可能排列
        for start_permutation in permutations(start_points):
            current_moves = 0
            # 计算从当前起点到终点的移动距离
            for (x1, y1), (x2, y2) in zip(start_permutation, end_points):
                current_moves += abs(x1 - x2) + abs(y1 - y2)
            # 更新最小移动次数
            min_moves = min(min_moves, current_moves)

        return min_moves


# BFS


class Solution:
    def minimumMoves(self, grid: List[List[int]]) -> int:
        N = 3  # 网格的大小

        # 定义目标状态：即每个位置恰好有一个石头的状态
        target = tuple(sorted((i, j) for i in range(N) for j in range(N)))
        stones = []

        # 遍历网格，找到所有石头的位置
        for i in range(N):
            for j in range(N):
                # 对于多个石头在同一位置的情况，记录所有石头的位置
                for k in range(grid[i][j]):
                    stones.append((i, j))

        # 初始化石头的起始状态
        start = tuple(stones)

        # 使用BFS搜索，queue中存储当前的状态和到达该状态所需的步数
        queue = deque([(start, 0)])

        # 使用集合记录已经访问过的状态，避免重复搜索
        visited = set([start])

        # 当队列非空时，继续搜索
        while queue:
            # 从队列头部取出一个状态
            state, steps = queue.popleft()
            # 如果该状态为目标状态，返回所需步数
            if state == target:
                return steps

            # 遍历当前状态中的每一个石头
            for i, (x, y) in enumerate(state):
                # 尝试四个方向的移动
                for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                    # 计算石头移动后的新位置
                    nx, ny = x + dx, y + dy

                    # 检查新位置是否在网格内，且该位置没有其他石头
                    if 0 <= nx < N and 0 <= ny < N and (nx, ny) not in state:
                        # 创建新的状态，并更新移动后的石头位置
                        newState = list(state)
                        newState[i] = (nx, ny)

                        # 对新状态的石头位置进行排序，确保状态的唯一性
                        newState = tuple(sorted(newState))

                        # 如果新状态没有被访问过，将其加入队列和访问集合
                        if newState not in visited:
                            visited.add(newState)
                            queue.append((newState, steps + 1))

        # 如果没有找到解，返回-1
        return -1
