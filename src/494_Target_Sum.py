# 494_Target_Sum
# https://leetcode.cn/problems/target-sum/description/

# 给你一个整数数组 nums 和一个整数 target 。

# 向数组中的每个整数前添加 '+' 或 '-' ，然后串联起所有整数，可以构造一个 表达式 ：

# 例如，nums = [2, 1] ，可以在 2 之前添加 '+' ，在 1 之前添加 '-' ，然后串联起来得到表达式 "+2-1" 。
# 返回可以通过上述方法构造的、运算结果等于 target 的不同 表达式 的数目。

from typing import List

class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:
        offset = sum(nums)
        if target > offset or target < -offset: 
            return 0

        for n in nums:
            target += n

        dp = [[0] * (target + 1) for _ in range(len(nums) + 1)]

        dp[0][0] = 1
        
        for i in range(1, len(nums) + 1):
            for j in range(0, target + 1):
                if j >= nums[i-1] * 2:
                    dp[i][j] = dp[i-1][j - nums[i-1] * 2] + dp[i-1][j]
                else:
                    dp[i][j] = dp[i-1][j]
        
        return dp[-1][-1]

# 优化

from pprint import pprint
def findTargetSumWays(nums: List[int], target: int) -> int:
    offset = sum(nums)
    if target > offset or target < -offset: 
        return 0

    for n in nums:
        target += n

    dp = [0] * (target + 1)

    dp[0] = 1
    
    for i in range(1, len(nums) + 1):
        tmp = dp.copy()
        for j in range(0, target + 1):
            if j >= nums[i-1] * 2:
                dp[j] += tmp[j - nums[i-1] * 2]
        
    return dp[-1]

print(findTargetSumWays(nums = [1,1,1,1,1], target = 3))
