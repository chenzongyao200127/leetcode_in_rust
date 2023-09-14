# 629_K_Inverse_Pairs_Array
# https://leetcode.cn/problems/k-inverse-pairs-array/description/

# 逆序对的定义如下：对于数组 nums 的第 i 个和第 j 个元素，
# 如果满足 0 <= i < j < nums.length 且 nums[i] > nums[j]，则其为一个逆序对；否则不是。

# 给你两个整数 n 和 k，找出所有包含从 1 到 n 的数字，
# 且恰好拥有 k 个 逆序对 的不同的数组的个数。由于答案可能很大，只需要返回对 109 + 7 取余的结果。
class Solution:
    def kInversePairs(self, n: int, k: int) -> int:
        # 定义一个大整数常量 MOD 用于防止整数溢出
        MOD = 10**9 + 7

        # 初始化一个二维的动态规划表格
        # dp[i][j] 表示前i个数有j个逆序对的方案数
        dp = [[0] * (k+1) for _ in range(n+1)]
        for i in range(n+1):
            dp[i][0] = 1  # 前i个数有0个逆序对的方案数肯定是1
        
        # 采用自底向上的方式构建动态规划表格
        for i in range(1, n+1):
            for j in range(1, k+1):
                # 当前的dp[i][j]值等于dp[i][j-1]与dp[i-1][j]的和
                # 这是因为我们可以考虑当前数字i放在哪个位置，使得逆序对的数量增加
                dp[i][j] = dp[i][j-1] + dp[i-1][j]
                
                # 如果j-i大于等于0，我们需要减去dp[i-1][j-i]
                # 因为我们要避免重复计算某些情况
                if j-i >= 0:
                    dp[i][j] -= dp[i-1][j-i]
                
                # 由于数字可能变得非常大，我们需要模 MOD 进行取余
                dp[i][j] = (dp[i][j] + MOD) % MOD

        return dp[n][k]


        
s = Solution()
ans = s.kInversePairs(3,0)
print(ans)
            

s = Solution()
ans = s.kInversePairs(3,1)
print(ans)            