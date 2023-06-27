# 47_Permutations_II
# https://leetcode.cn/problems/permutations-ii/


# 给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。

# 超时
class Solution:
    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
        ans = []
        n = len(nums)
        def dfs(com, nums, n):
            if len(com) == n:
                if com not in ans:
                    ans.append(com[:])
            
            for i in range(len(nums)):
                com.append(nums[i])
                new_num = nums[:i] + nums[i+1:]
                dfs(com, new_num, n)
                com.pop()
        
        dfs([], nums, n)
        return ans
    
    
# 优化：
class Solution:
    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
        ans = []
        n = len(nums)
        def dfs(com, nums, n):
            if len(com) == n:
                ans.append(com[:])
            
            for i in range(len(nums)):
                # 关键一步， 去重
                if i > 0 and nums[i] == nums[i - 1]:
                    continue
                com.append(nums[i])
                new_num = nums[:i] + nums[i+1:]
                dfs(com, new_num, n)
                com.pop()
        
        nums.sort()
        dfs([], nums, n)
        return ans
    
    
class Solution:
    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
        ans=[]
        used=[False]*len(nums)
        nums.sort()
        path=[]
        def trackback():
            if len(path)==len(nums):
                ans.append(path.copy())
                return 
            for i in range(len(nums)):
                if used[i]:
                    continue
                if i>0 and nums[i]==nums[i-1] and not used[i-1]:
                    continue
                path.append(nums[i])
                used[i]=True
                trackback()
                used[i]=False
                path.pop()
                
        trackback()
        return ans
