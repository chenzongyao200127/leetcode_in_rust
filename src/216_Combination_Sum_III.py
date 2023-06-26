# 216_Combination_Sum_III
# https://leetcode.cn/problems/combination-sum-iii/
from typing import List

class Solution:
    def combinationSum3(self, k: int, n: int) -> List[List[int]]:
        def dfs(startIndex, path, current_sum, k, n):
            if current_sum == n and len(path) == k:
                ans.append(path[:])
                return
            
            if current_sum > n or len(path) > k:
                return
            
            for index in range(startIndex + 1, 10):
                path.append(index)
                dfs(index, path, current_sum + index, k, n)  # Fixed parameter order
                path.pop()
        
        ans = []
        dfs(0, [], 0, k, n)  # Fixed parameter order
        return ans
    
    
    
class Solution:
    def combinationSum3(self, k: int, n: int) -> List[List[int]]:
        res, path = [], []
        def backtracking(k, n, start, cur_sum):
            if cur_sum > n:
                return
            if k == 0:
                if cur_sum == n:
                    res.append(path.copy())
                return
            for i in range(start, 10):
                path.append(i)
                cur_sum += i
                backtracking(k - 1, n, i + 1, cur_sum)
                cur_sum -= i
                path.pop()
        backtracking(k, n, 1, 0)
        return res