# 221_Maximal_Square
# https://leetcode.cn/problems/maximal-square/description/


# 在一个由 '0' 和 '1' 组成的二维矩阵内，找到只包含 '1' 的最大正方形，并返回其面积。
from typing import List
class Solution:
    def maximalSquare(self, matrix: List[List[str]]) -> int:
        m = len(matrix)
        n = len(matrix[0])
        ans = 0
        dp = [[0] * n for _ in range(m)]
        for i in range(n):
            if matrix[0][i] == "1":
                dp[0][i] = 1
                ans = 1
        for i in range(m):
            if matrix[i][0] == "1":
                dp[i][0] = 1
                ans = 1
        for i in range(1, m):
            for j in range(1, n):
                if matrix[i][j] == "1":
                    dp[i][j] = min(dp[i-1][j-1], dp[i-1][j], dp[i][j-1]) + 1
                    ans = max(ans, dp[i][j])

        return ans * ans
                    
sol = Solution()
ans = sol.maximalSquare([["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]])
print(ans)  