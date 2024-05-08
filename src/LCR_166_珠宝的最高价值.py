# LCR_166_珠宝的最高价值
# https://leetcode.cn/problems/li-wu-de-zui-da-jie-zhi-lcof/description/

from typing import List

# 现有一个记作二维矩阵 frame 的珠宝架，其中 frame[i][j] 为该位置珠宝的价值。拿取珠宝的规则为：

# 只能从架子的左上角开始拿珠宝
# 每次可以移动到右侧或下侧的相邻位置
# 到达珠宝架子的右下角时，停止拿取
# 注意：珠宝的价值都是大于 0 的。除非这个架子上没有任何珠宝，比如 frame = [[0]]。


class Solution:
    def jewelleryValue(self, grid: List[List[int]]) -> int:
        # Get the dimensions of the grid
        num_rows, num_cols = len(grid), len(grid[0])

        # Initialize a 2-row DP table to store the maximum jewellery value collectable
        dp = [[0] * (num_cols + 1) for _ in range(2)]

        # Iterate over each cell in the grid
        for row_index, row in enumerate(grid):
            for col_index, value in enumerate(row):
                # Use modulo operation to alternate between the two rows in the dp table
                current_row = (row_index + 1) % 2
                previous_row = row_index % 2
                # Update the dp table with the maximum value collectable at the current cell
                dp[current_row][col_index + 1] = max(
                    dp[current_row][col_index], dp[previous_row][col_index + 1]) + value

        # The result is in the last cell of the row corresponding to the last input row
        return dp[num_rows % 2][num_cols]
