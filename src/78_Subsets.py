# 78_Subsets
# https://leetcode.cn/problems/subsets/

class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        ans = [[]]
        
        def dfs(com, nums):
            if not nums:
                return
            
            for i in range(len(nums)):
                com.append(nums[i])
                new_nums = nums[i+1:]
                ans.append(com[:])
                dfs(com, new_nums)
                com.pop()
                
        dfs([],nums)
        return ans
    
    
class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        res = []
        temp_res = []
        
        def backtrack(nums,start):
            res.append(temp_res[:])
            if start >= len(nums):
                return
            
            for i in range(start,len(nums)):
                num = nums[i]
                temp_res.append(num)
                backtrack(nums,i+1)
                temp_res.pop()
                
        backtrack(nums,0)
        return res

            