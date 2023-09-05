# 312_Burst_Balloons
# https://leetcode.cn/problems/burst-balloons/

# You are given n balloons, indexed from 0 to n - 1. Each balloon is painted with a number 
# on it represented by an array nums. You are asked to burst all the balloons.

# If you burst the ith balloon, you will get nums[i - 1] * nums[i] * nums[i + 1] coins. 
# If i - 1 or i + 1 goes out of bounds of the array, then treat it as if there is a balloon with a 1 painted on it.

# Return the maximum coins you can collect by bursting the balloons wisely.

 
# Example 1:
# Input: nums = [3,1,5,8]
# Output: 167
# Explanation:
# nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
# coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167

# Example 2:
# Input: nums = [1,5]
# Output: 10

# from typing import List
# from pprint import pprint
# 错误解答
# class Solution:
#     def maxCoins(self, nums: List[int]) -> int:
#         nums = [1] + nums + [1]
#         l = len(nums)
#         dp = [[1] * l for _ in range(l)]
        
#         pprint(dp)

#         for i in range(1, l-1):
#             dp[i-1][i] = nums[i-1] * nums[i] * nums[i+1]
        
#         pprint(dp)
        
#         for i in range(l-2):
#             for j in range(l):
#                 for k in range(i+1, j):
#                     dp[i][j] = max(dp[i][j], dp[i][k] + dp[k][j] + nums[i] * nums[j] * nums[k])
        
#         pprint(dp)
                    
#         return dp[0][-1]
        
        
        
from typing import List

class Solution:
    def maxCoins(self, nums: List[int]) -> int:
        nums = [1] + nums + [1]
        l = len(nums)
        dp = [[0] * l for _ in range(l)]

        for gap in range(2, l):
            for i in range(l-gap):
                j = i + gap
                for k in range(i+1, j):
                    dp[i][j] = max(dp[i][j], dp[i][k] + dp[k][j] + nums[i] * nums[k] * nums[j])

        return dp[0][-1]

        
s = Solution()
ans = s.maxCoins([3,1,5,8])
print(ans)
        