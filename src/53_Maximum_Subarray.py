# 53_Maximum_Subarray
# https://leetcode.cn/problems/maximum-subarray/

from typing import List

# 超时
class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        pre_sum = [0] + [sum(nums[:i+1]) for i in range(len(nums))]
        print(pre_sum)
        ans = -100000
        for i in range(len(nums), 0, -1):
            # print(pre_sum[:i])
            ans = max(ans, pre_sum[i] - min(pre_sum[:i]))
        
        return ans

class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        ans = -inf
        min_pre_sum = pre_sum = 0
        for x in nums:
            pre_sum += x  # 当前的前缀和
            ans = max(ans, pre_sum - min_pre_sum)  # 减去前缀和的最小值
            min_pre_sum = min(min_pre_sum, pre_sum)  # 维护前缀和的最小值
        return ans

# DP
class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        if not nums:
            return 0

        # Initialize the dp array
        dp = [0] * len(nums)
        dp[0] = nums[0]

        # Fill the dp array
        for i in range(1, len(nums)):
            dp[i] = max(nums[i], dp[i-1] + nums[i])
        
        # Return the maximum value in dp
        return max(dp)

    
        

        
s = Solution()
ans = s.maxSubArray([5,4,-1,7,8])
print(ans)