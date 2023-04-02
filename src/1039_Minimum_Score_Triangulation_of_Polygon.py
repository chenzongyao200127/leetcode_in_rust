# 1039_Minimum_Score_Triangulation_of_Polygon
# https://leetcode.cn/problems/minimum-score-triangulation-of-polygon/submissions/

class Solution:
    def minScoreTriangulation(self, values: List[int]) -> int:
        n = len(values)
        dp = [[0] * n for _ in range(n)]

        # 初始化 dp 数组
        for i in range(n - 1):
            dp[i][i + 1] = 0

        # 逐步增加多边形的大小
        for l in range(2, n):
            for i in range(n - l):
                j = i + l
                dp[i][j] = float('inf')
                # 遍历中间顶点 k
                for k in range(i + 1, j):
                    dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j] + values[i] * values[k] * values[j])

        return dp[0][n - 1]


class Solution:
    def minScoreTriangulation(self, values: List[int]) -> int:
        @cache
        def dfs(i: int, j: int) -> int:
            if i + 1 == j:
                return 0
            return min(dfs(i, k) + dfs(k, j) + values[i] * values[k] * values[j] for k in range(i + 1, j))

        return dfs(0, len(values) - 1)
    

class Solution:
    def minScoreTriangulation(self, v: List[int]) -> int:
        n = len(v)
        f = [[0] * n for _ in range(n)]
        for i in range(n - 3, -1, -1):
            for j in range(i + 2, n):
                f[i][j] = min(f[i][k] + f[k][j] + v[i] * v[j] * v[k]
                              for k in range(i + 1, j))
        return f[0][-1]