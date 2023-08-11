# 174_Dungeon_Game
# https://leetcode.cn/problems/dungeon-game/

# 恶魔们抓住了公主并将她关在了地下城 dungeon 的 右下角 。地下城是由 m x n 个房间组成的二维网格。
# 我们英勇的骑士最初被安置在 左上角 的房间里，他必须穿过地下城并通过对抗恶魔来拯救公主。

# 骑士的初始健康点数为一个正整数。如果他的健康点数在某一时刻降至 0 或以下，他会立即死亡。

# 有些房间由恶魔守卫，因此骑士在进入这些房间时会失去健康点数（若房间里的值为负整数，则表示骑士将损失健康点数）；
# 其他房间要么是空的（房间里的值为 0），要么包含增加骑士健康点数的魔法球（若房间里的值为正整数，则表示骑士将增加健康点数）。

# 为了尽快解救公主，骑士决定每次只 向右 或 向下 移动一步。

# 返回确保骑士能够拯救到公主所需的最低初始健康点数。

# # 注意：任何房间都可能对骑士的健康点数造成威胁，也可能增加骑士的健康点数，包括骑士进入的左上角房间以及公主被监禁的右下角房间。

# 这段 Python 代码是解决 "地牢游戏"（LeetCode 174题）的动态规划解法。地牢游戏是一个求解最小初始点数的问题。

# 我们先来了解一下问题，给定一个二维网格，每一个格子有一个值，正数代表你会获得健康值，负数代表你会失去健康值。在任何时刻，如果你的健康值小于等于0，那么你就死了。你从左上角出发，目标是到达右下角，求你至少需要多少初始健康值才能够成功到达。

# 以下是代码的逐行解释：
# 1. `n, m = len(dungeon), len(dungeon[0])`: 获取地牢的行数 n 和列数 m。
# 2. `BIG = 10**9`: 初始化一个很大的数值 BIG 作为默认值。
# 3. `dp = [[BIG] * (m + 1) for _ in range(n + 1)]`: 初始化动态规划矩阵 dp，dp[i][j] 代表从格子 (i,j) 到右下角所需的最小初始健康值。
# 4. `dp[n][m - 1] = dp[n - 1][m] = 1`: 对最下面一行和最右边一列的外部一格设为1，作为边界条件，表示在到达终点前最少需要1点生命值。
# 5. 双层逆向遍历，从倒数第二行和列开始，到第0行和第0列结束。
# 6. `minn = min(dp[i + 1][j], dp[i][j + 1])`：首先计算在当前格子 (i,j) 向右或向下移动所需的最小初始健康值。
# 7. `dp[i][j] = max(minn - dungeon[i][j], 1)`：然后计算在当前格子 (i,j) 所需的最小初始健康值，因为生命值不能小于1，所以要和1取最大值。
# 8. 最后返回 dp[0][0]，这就是从左上角出发至少需要多少初始健康值才能够成功到达右下角。

# 动态规划的思想是从后向前计算，对每个格子都计算出从这个格子出发至少需要多少健康值，然后选择一个值最小的路径走。

from typing import List
class Solution:
    def calculateMinimumHP(self, dungeon: List[List[int]]) -> int:
        n, m = len(dungeon), len(dungeon[0])
        BIG = 10**9
        dp = [[BIG] * (m + 1) for _ in range(n + 1)]
        dp[n][m - 1] = dp[n - 1][m] = 1
        for i in range(n - 1, -1, -1):
            for j in range(m - 1, -1, -1):
                minn = min(dp[i + 1][j], dp[i][j + 1])
                dp[i][j] = max(minn - dungeon[i][j], 1)

        return dp[0][0]
    
# 在这个问题中，使用逆向动态规划比正向动态规划更为直观，原因如下：

# 1. **目标明确**：在地牢游戏中，我们的目标是从起点到达终点，这看似是一个正向问题。
# 但是，我们的生命值必须始终大于0，这就引入了一个新的条件：
# 在每一个点上，我们需要考虑的是，从这个点到达终点所需的最小初始生命值。
# 这个值受到从这一点开始的所有可能路径的影响。
# 由于我们不知道哪一条路径是最优的，所以难以从起点开始正向推导。

# 2. **未来决策对当前决策的影响**：在每一步，我们的决策（即生命值的选择）会对未来的步骤产生影响。
# 在正向DP中，我们需要考虑所有可能的未来路径，然而这些信息在当前步骤并不全都可用。
# 反过来，如果我们从终点开始进行逆向DP，
# 那么在每个步骤，我们都可以基于"未来"（实际上已经计算出来的DP值）做出最优的决策。

# 3. **边界条件的处理**：从逆向DP的角度看，我们只需要考虑到达终点的最小生命值为1，
# 而从正向DP的角度看，我们需要考虑所有可能的生命值和所有可能的路径，
# 这使得边界条件和初始状态的处理变得复杂。

# 因此，在这个问题中，逆向DP提供了一种更为直观、易于实现的方法。