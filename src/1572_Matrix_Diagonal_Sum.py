# 1572_Matrix_Diagonal_Sum
# https://leetcode.cn/problems/matrix-diagonal-sum/

# Given a square matrix mat, return the sum of the matrix diagonals.

# Only include the sum of all the elements on the primary diagonal and all the
# elements on the secondary diagonal that are not part of the primary diagonal.

from typing import List

class Solution:
    def diagonalSum(self, mat: List[List[int]]) -> int:
        n = len(mat)
        ans = 0 
        for i in range(n):
            ans += mat[i][i]
            ans += mat[i][n-1-i]
            if i == n-1-i:
                ans -= mat[i][i]
        
        return ans