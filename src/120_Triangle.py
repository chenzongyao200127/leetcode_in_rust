# 120_Triangle
# https://leetcode.cn/problems/triangle/description/

# 给定一个三角形 triangle ，找出自顶向下的最小路径和。

# 每一步只能移动到下一行中相邻的结点上。相邻的结点 在这里指的是 下标 与 上一层结点下标 相同或者等于 上一层结点下标 + 1 的两个结点。也就是说，如果正位于当前行的下标 i ，那么下一步可以移动到下一行的下标 i 或 i + 1 。

from typing import List
class Solution:
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        dp = triangle.copy()
        for i in range(len(triangle)):
            for j in range(len(triangle[i])):
                if j == 0 and i == 0:
                    continue
                elif j == 0:
                    dp[i][j] = dp[i-1][j] + triangle[i][j]
                elif j == len(triangle[i]) -1:
                    dp[i][j] = dp[i-1][j-1] + triangle[i][j]
                else:
                    dp[i][j] = min(dp[i-1][j-1], dp[i-1][j]) + triangle[i][j]
                        
        return min(triangle[-1])