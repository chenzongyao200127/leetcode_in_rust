# 1000_Minimum_Cost_to_Merge_Stones
# https://leetcode.cn/problems/minimum-cost-to-merge-stones/

class Solution:
    def mergeStones(self, stones: List[int], K: int) -> int:
        n = len(stones)
        if (n-1) % (K-1) != 0:  # 无法合并成一堆，返回-1
            return -1
        
        # 计算前缀和
        prefix_sum = [0] * (n+1)
        for i in range(n):
            prefix_sum[i+1] = prefix_sum[i] + stones[i]
        
        # 动态规划求解
        dp = [[float('inf')] * (n+1) for _ in range(n+1)]
        for i in range(1, n+1):
            dp[i][i] = 0
        
        for len_ in range(2, n+1):  # 枚举区间长度
            for i in range(1, n-len_+2):  # 枚举区间起点
                j = i + len_ - 1  # 区间终点
                for k in range(i, j, K-1):  # 枚举最后一次合并的位置
                    dp[i][j] = min(dp[i][j], dp[i][k] + dp[k+1][j])
                if (j-i) % (K-1) == 0:  # 如果区间长度满足合并条件
                    dp[i][j] += prefix_sum[j] - prefix_sum[i-1]
        
        return dp[1][n]
    
# GPT-4 的错误解答:
def mergeStones(stones, K):
    n = len(stones)
    if (n - 1) % (K - 1) != 0:
        return -1

    # 计算前缀和
    prefix_sum = [0] * (n + 1)
    for i in range(n):
        prefix_sum[i + 1] = prefix_sum[i] + stones[i]

    # 初始化dp数组
    dp = [[0] * n for _ in range(n)]

    # 更新dp数组
    for m in range(K, n+1):  # m表示要合并的石头堆数
        for i in range(0, n - m + 1):
            j = i + m - 1
            dp[i][j] = float('inf')
            for k in range(i, j, K-1):
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k + 1][j] + prefix_sum[j + 1] - prefix_sum[i])

    return dp[0][n - 1]
