# 416_Partition_Equal_Subset_Sum
# https://leetcode.cn/problems/partition-equal-subset-sum/

from typing import List

# 错误解答 超时 指数时间复杂度
# 给你一个 只包含正整数 的 非空 数组 nums 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。
class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        pre_sum = sum(nums)
        if pre_sum % 2 != 0:
            return False

        target = pre_sum // 2
        nums.sort(reverse=True)

        def dfs(cur_sum, start):
            if cur_sum == target:
                return True

            for i in range(start, len(nums)):
                if cur_sum + nums[i] > target:
                    continue
                if dfs(cur_sum + nums[i], i + 1):
                    return True

            return False

        return dfs(0, 0)


# DP
class Solution:
    def canPartition(self, nums):
        total = sum(nums)
        if total % 2 != 0:
            return False
        
        target = total // 2
        dp = [False] * (target + 1)
        dp[0] = True
        
        for num in nums:
            for i in range(target, num - 1, -1):
                dp[i] = dp[i] or dp[i - num]
        
        return dp[target]

