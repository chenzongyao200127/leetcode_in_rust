# 576_Out_of_Boundary_Paths
# https://leetcode.cn/problems/out-of-boundary-paths/

# 给你一个大小为 m x n 的网格和一个球。球的起始坐标为 [startRow, startColumn] 。
# 你可以将球移到在四个方向上相邻的单元格内（可以穿过网格边界到达网格之外）。你 最多 可以移动 maxMove 次球。

# 给你五个整数 m、n、maxMove、startRow 以及 startColumn ，找出并返回可以将球移出边界的路径数量。
# 因为答案可能非常大，返回对 109 + 7 取余 后的结果。

# 超过最大递归深度
from functools import lru_cache
class Solution:
    def findPaths(self, m: int, n: int, maxMove: int, startRow: int, startColumn: int) -> int:
        MOD = 10**9 + 7
        dp = [[0] * n for _ in range(m)]
        for i in range(m):
            dp[i][0] += 1
            dp[i][-1] += 1
        for i in range(n):
            dp[0][i] += 1
            dp[-1][i] += 1

        @lru_cache(maxsize=None)
        def dfs(i, j , move):
            if move == 1:
                return dp[i][j]
            
            tmp = dfs(i, j, 1)
            if i >= 1:
                tmp += dfs(i-1, j, move-1)
            if i <= m-2:
                tmp += dfs(i+1, j, move-1)
            if j >= 1:
                tmp += dfs(i, j-1, move-1)
            if j <= n-2:
                tmp += dfs(i, j+1, move-1)
            return tmp

        return dfs(startRow, startColumn, maxMove) % MOD



from functools import lru_cache
class Solution:
    def findPaths(self, m: int, n: int, maxMove: int, startRow: int, startColumn: int) -> int:
        MOD = 10**9 + 7
        directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        
        @lru_cache(None)
        def dfs(i, j, remain):
            if remain < 0:
                return 0
            if i < 0 or i >= m or j < 0 or j >= n:
                return 1
            count = 0
            for dx, dy in directions:
                count = (count + dfs(i + dx, j + dy, remain - 1)) % MOD
            return count
        
        return dfs(startRow, startColumn, maxMove) % MOD

# 这两个函数在实现上有一些不同，这些不同可能会影响到它们的递归深度。
# 1. **不同的终止条件**：在你的函数中，递归只在 `move == 1` 时停止。
# 这意味着如果 `maxMove` 是一个很大的值，那么递归深度可能会非常高。
# 而在我给你的函数中，递归在 `remain < 0` 或球已经移出边界时停止。
# 这意味着即使 `maxMove` 是一个很大的值，递归深度也不会超过 `maxMove`。

# 2. **不同的记忆化策略**：在你的函数中，你在每一步都对 `dfs` 的结果进行记忆化。
# 这意味着你的函数可能会存储大量的中间结果，而这些结果可能并不会被重复使用。
# 而在我给你的函数中，只有在递归返回时才会对结果进行记忆化。
# 这意味着只有那些实际被使用的结果才会被存储。

# 3. **不同的搜索策略**：在你的函数中，你在每一步都对四个方向进行搜索。
# 这可能会导致大量的重复搜索，从而增加递归深度。而在我给你的函数中，
# 我使用了一种深度优先的搜索策略，这可以减少重复搜索，从而减少递归深度。

# 总的来说，这两个函数在设计上都使用了递归和记忆化，但是在具体的实现上有所不同。
# 这些不同可能会影响到它们的性能和递归深度。在实际编程中，正确地选择和设计算法是非常重要的。

# 动态规划的状态由移动次数、行和列决定，定义 dp[i][j][k] 表示球移动 i 次之后位于坐标 (j,k) 的路径数量。
# 当 i=0时，球一定位于起始坐标 (startRow,startColumn)
class Solution:
    def findPaths(self, m: int, n: int, maxMove: int, startRow: int, startColumn: int) -> int:
        MOD = 10**9 + 7  # 定义模数，用于处理可能出现的大数溢出问题

        outCounts = 0  # 初始化移出边界的路径数为0
        # 初始化动态规划数组，表示在移动 i 次后，球处于 (j, k) 的路径数
        dp = [[[0] * n for _ in range(m)] for _ in range(maxMove + 1)]
        dp[0][startRow][startColumn] = 1  # 在移动0次时，球处于起始位置的路径数为1

        # 遍历每一次移动
        for i in range(maxMove):
            # 遍历网格的每一行
            for j in range(m):
                # 遍历网格的每一列
                for k in range(n):
                    # 如果当前位置有路径可以到达
                    if dp[i][j][k] > 0:
                        # 遍历球的四个可能的移动方向
                        for j1, k1 in [(j - 1, k), (j + 1, k), (j, k - 1), (j, k + 1)]:
                            # 如果新的位置仍在网格内
                            if 0 <= j1 < m and 0 <= k1 < n:
                                # 更新新的位置在下一次移动后的路径数
                                dp[i + 1][j1][k1] = (dp[i + 1][j1][k1] + dp[i][j][k]) % MOD
                            else:  # 如果新的位置在网格外
                                # 更新移出边界的路径数
                                outCounts = (outCounts + dp[i][j][k]) % MOD

        return outCounts  # 返回总的移出边界的路径数