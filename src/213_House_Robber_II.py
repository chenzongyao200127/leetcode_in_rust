# 213_House_Robber_II
# https://leetcode.cn/problems/house-robber-ii/

# 你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都 围成一圈 ，
# 这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警 。
# 给定一个代表每个房屋存放金额的非负整数数组，计算你 在不触动警报装置的情况下 ，今晚能够偷窃到的最高金额。
from typing import List
class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        dp = [0] * (n+1)
        dp[0] = 0
        dp[1] = nums[0]
        for i in range(2, n):
            dp[i] = max(dp[i-2] + nums[i-1], dp[i-1])
        tmp = max(dp[-1], dp[-2])
        dp = [0] * (n+1)
        dp[0] = 0
        dp[1] = 0
        for i in range(2,n+1):
            dp[i] = max(dp[i-2] + nums[i-1], dp[i-1])
        return max(tmp, dp[-1], dp[-2])


class Solution:
    def rob(self, nums: List[int]) -> int:
        def robRange(start: int, end: int) -> int:
            first = nums[start]
            second = max(nums[start], nums[start + 1])
            for i in range(start + 2, end + 1):
                first, second = second, max(first + nums[i], second)
            return second
        
        length = len(nums)
        if length == 1:
            return nums[0]
        elif length == 2:
            return max(nums[0], nums[1])
        else:
            return max(robRange(0, length - 2), robRange(1, length - 1))
