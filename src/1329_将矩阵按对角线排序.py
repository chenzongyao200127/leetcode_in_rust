# 1329_将矩阵按对角线排序
# https://leetcode.cn/problems/sort-the-matrix-diagonally/description/?envType=daily-question&envId=2024-04-29

from typing import List

# 矩阵对角线 是一条从矩阵最上面行或者最左侧列中的某个元素开始的对角线，沿右下方向一直到矩阵末尾的元素。
# 例如，矩阵 mat 有 6 行 3 列，从 mat[2][0] 开始的 矩阵对角线 将会经过 mat[2][0]、mat[3][1] 和 mat[4][2] 。
# 给你一个 m * n 的整数矩阵 mat ，请你将同一条 矩阵对角线 上的元素按升序排序后，返回排好序的矩阵。


class Solution:
    def diagonalSort(self, mat: List[List[int]]) -> List[List[int]]:
        m, n = len(mat), len(mat[0])
        for k in range(1 - n, m):  # k = i - j
            left_i, right_i = max(k, 0), min(k + n, m)
            a = sorted(mat[i][i - k] for i in range(left_i, right_i))
            for i in range(left_i, right_i):
                mat[i][i - k] = a[i - left_i]
        return mat
