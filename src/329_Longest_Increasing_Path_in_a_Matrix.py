# 329_Longest_Increasing_Path_in_a_Matrix
# https://leetcode.cn/problems/longest-increasing-path-in-a-matrix/

# 给定一个 m x n 整数矩阵 matrix ，找出其中 最长递增路径 的长度。
# 对于每个单元格，你可以往上，下，左，右四个方向移动。 你 不能 在 对角线 方向上移动或移动到 边界外（即不允许环绕）。

from typing import List
class Solution:
    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:
        