# 491_Non-decreasing_Subsequences
# https://leetcode.cn/problems/non-decreasing-subsequences/

class Solution:
    def findSubsequences(self, nums: List[int]) -> List[List[int]]:
        ans = []
        n = len(nums)
        def dfs(com, nums, n):
            if len(com) == n:
                ans.append(com[:])
            
            for i in range(len(nums)):
                # 关键一步， 去重
                if i > 0 and nums[i] == nums[i - 1]:
                    continue
                if nums[i] >= nums[i - 1]:
                    com.append(nums[i])
                new_num = nums[:i] + nums[i+1:]
                dfs(com, new_num, n)
                com.pop()
        
        nums.sort()
        dfs([], nums, n)
        return ans
        