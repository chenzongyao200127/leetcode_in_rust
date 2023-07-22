# 368_Largest_Divisible_Subset
# https://leetcode.cn/problems/largest-divisible-subset/

# 给你一个由 无重复 正整数组成的集合 nums ，请你找出并返回其中最大的整除子集 answer ，
# 子集中每一元素对 (answer[i], answer[j]) 都应当满足：
# answer[i] % answer[j] == 0 ，或
# answer[j] % answer[i] == 0
# 如果存在多个有效解子集，返回其中任何一个均可。
    
from typing import List
from collections import defaultdict

class Solution:
    def largestDivisibleSubset(self, nums: List[int]) -> List[int]:
        nums.sort()
        dp = defaultdict(list)
        for i in range(len(nums)):
            tmp = []
            for j in range(i):
                if nums[i] % nums[j] == 0:
                    if len(dp[j]) > len(tmp):
                        tmp = dp[j]
            dp[i] = tmp + [nums[i]]
        ans = []
        for _, v in dp.items():
            if len(v) > len(ans):
                ans = v
        return ans