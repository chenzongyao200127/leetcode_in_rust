# 90_Subsets_II
# https://leetcode.cn/problems/subsets-ii/

class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        ans = [[]]
        
        def dfs(com, nums):
            if not nums:
                return
            
            for i in range(len(nums)):
                if i > 0 and nums[i] == nums[i-1]:
                    continue
                com.append(nums[i])
                new_nums = nums[i+1:]
                ans.append(com[:])
                dfs(com, new_nums)
                com.pop()
        
        nums.sort()
        dfs([],nums)
        return ans
    
    
    
class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        path=[]
        res=[]
        nums.sort()
        used=[0]*len(nums)

        def travel(nums,start_index,used):
            res.append(path[:])
            if start_index>=len(nums):
                return 
            
            for i in range(start_index,len(nums)):
                if i>0 and nums[i]==nums[i-1] and used[i-1]==0:
                    continue
                used[i]=1

                path.append(nums[i])

                travel(nums,i+1,used)
                path.pop()
                used[i]=0
        travel(nums,0,used)
        return res    