# 931_Minimum_Falling_Path_Sum
# https://leetcode.cn/problems/minimum-falling-path-sum/

# 给你一个 n x n 的 方形 整数数组 matrix ，请你找出并返回通过 matrix 的下降路径 的 最小和 。

# 下降路径 可以从第一行中的任何元素开始，并从每一行中选择一个元素。
# 在下一行选择的元素和当前行所选元素最多相隔一列（即位于正下方或者沿对角线向左或者向右的第一个元素）。
# 具体来说，位置 (row, col) 的下一个元素应当是 (row + 1, col - 1)、(row + 1, col) 或者 (row + 1, col + 1) 

from typing import List
from collections import deque

# 超出内存限制
class Solution:
    def minFallingPathSum(self, matrix: List[List[int]]) -> int:
        queue = deque([])
        ans = float("inf")
        for i in range(len(matrix)):
            queue.append((0,i,matrix[0][i]))
            while queue:
                x, y, val = queue.popleft()
                if x == len(matrix) - 1:
                    ans = min(ans, val)
                for j in range(-1,2):
                    if 0 <= y + j < len(matrix[0]) and 0 <= x+1 < len(matrix):
                        new_val = val + matrix[x+1][y+j]
                        queue.append([x+1, y+j, new_val])
        
        return ans
    

# 线性DP
class Solution:
    def minFallingPathSum(self, matrix: List[List[int]]) -> int:
        if not matrix:
            return 0

        rows, cols = len(matrix), len(matrix[0])

        for i in range(1, rows):
            for j in range(cols):
                left_up = matrix[i - 1][j - 1] if j > 0 else float("inf")
                up = matrix[i - 1][j]
                right_up = matrix[i - 1][j + 1] if j < cols - 1 else float("inf")
                matrix[i][j] += min(left_up, up, right_up)

        return min(matrix[-1])
