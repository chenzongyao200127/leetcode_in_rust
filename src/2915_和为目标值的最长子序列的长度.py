# https://leetcode.cn/problems/length-of-the-longest-subsequence-that-sums-to-target/

# 超时
from math import inf
from typing import inf
from typing import List


class Solution:
    def lengthOfLongestSubsequence(self, nums: List[int], target: int) -> int:
        max_length = [-1]

        def dfs(index, current_sum, length):
            if current_sum == target:
                max_length[0] = max(max_length[0], length)
                return
            if index == len(nums) or current_sum > target:
                return
            dfs(index + 1, current_sum + nums[index], length + 1)
            dfs(index + 1, current_sum, length)

        dfs(0, 0, 0)
        return max_length[0]


s = Solution()
ans = s.lengthOfLongestSubsequence(nums=[4, 1, 3, 2, 1, 5], target=7)
print(ans)


class Solution:
    def lengthOfLongestSubsequence(self, nums: List[int], target: int) -> int:
        f = [0] + [-inf] * target
        s = 0
        for x in nums:
            s = min(s + x, target)
            for j in range(s, x - 1, -1):
                f[j] = max(f[j], f[j - x] + 1)
        return f[-1] if f[-1] > 0 else -1


class Solution:
    def lengthOfLongestSubsequence(self, nums: List[int], target: int) -> int:
        dp = [0] + [-inf] * target
        current_sum = 0

        for num in nums:
            current_sum = min(current_sum + num, target)
            for j in range(current_sum, num - 1, -1):
                dp[j] = max(dp[j], dp[j - num] + 1)

        return dp[target] if dp[target] > 0 else -1


# Create an instance of the Solution class
s = Solution()
# Call the method with the given parameters
ans = s.lengthOfLongestSubsequence(nums=[4, 1, 3, 2, 1, 5], target=7)
# Print the result
print(ans)
