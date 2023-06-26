# 39_Combination_Sum
# https://leetcode.cn/problems/combination-sum/


# Given an array of distinct integers candidates and a target integer target, 
# return a list of all unique combinations of candidates where the chosen numbers sum to target. 
# You may return the combinations in any order.

# The same number may be chosen from candidates an unlimited number of times. 
# Two combinations are unique if the frequency of at least one of the chosen numbers is different.

# The test cases are generated such that the number of unique combinations that sum up to 
# target is less than 150 combinations for the given input.

# 存在重复
def combinationSum(candidates, target):
    com = []
    ans = []
    def dfs(com, candidates, target):
        nonlocal ans
        if sum(com) == target:
            ans.append(com[:])  # Create a copy of 'com' before adding it to 'ans'
            print(ans)
            return
        
        if sum(com) > target:
            return
        
        for num in candidates:
            com.append(num)
            dfs(com, candidates, target)
            com.pop()
    
    dfs(com, candidates, target)
    return ans

print(combinationSum([2,3,6,7],7))    
    
from typing import List

class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        def dfs(startIndex, path, current_sum, candidates, target):
            if current_sum == target:
                ans.append(path[:])
                return
            
            if current_sum > target:
                return
            
            for index in range(startIndex, len(candidates)):
                num = candidates[index]
                path.append(num)
                dfs(index, path, current_sum + num, candidates, target)
                path.pop()
        
        ans = []
        dfs(0, [], 0, candidates, target)
        return ans
    
    
# 示例：
class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates.sort()
        n = len(candidates)
        res = []
        
        def backtrack(i, tmp_sum, tmp):
            if tmp_sum > target or i == n:
                return 
            if tmp_sum == target:
                res.append(tmp)
                return 
            for j in range(i, n):
                if tmp_sum + candidates[j] > target:
                    break
                backtrack(j,tmp_sum + candidates[j],tmp+[candidates[j]])
                
        backtrack(0, 0, [])
        return res
