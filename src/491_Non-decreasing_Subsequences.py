# 491_Non-decreasing_Subsequences
# https://leetcode.cn/problems/non-decreasing-subsequences/


# 给你一个整数数组 nums ，找出并返回所有该数组中不同的递增子序列，递增子序列中 至少有两个元素 。你可以按 任意顺序 返回答案。
# 数组中可能含有重复元素，如出现两个整数相等，也可以视作递增序列的一种特殊情况。

def findSubsequences(nums):
    ans = set()
    
    def dfs(com, nums):
        if not nums:
            return
        
        for i in range(len(nums)):
            if not com or nums[i] >= com[-1]:
                com.append(nums[i])
                if len(com) > 1:
                    ans.add(tuple(com))
                new_num = nums[i+1:]
                dfs(com, new_num)
                com.pop()
                
    dfs([], nums)
    return [list(subsequence) for subsequence in ans]

print(findSubsequences([1, 2, 3, 1, 1, 1, 1]))

from typing import List


# 这个实现采用了迭代的方法，与之前提供的深度优先搜索（DFS）方法不同。
# 代码使用一个字典 index_dict 来存储每个数字在 res 中最后出现的索引。
# 对于 nums 中的每个数字，代码首先查找其在 index_dict 中的索引，然后复制当前的 res（不包括当前数字的索引之后的部分）。
# 接着，代码将当前数字添加到复制的子序列中，并添加到 res。
class Solution:
    def findSubsequences(self, nums: List[int]) -> List[List[int]]:
        res = []
        index_dict = {}
        for num in nums:
            pre_index = len(res)
            if num in index_dict:
                res_ = res[index_dict[num]:]
            else:
                res_ = res.copy()
                res.append([num, ])
            index_dict[num] = pre_index
            for r in res_:
                if num >= r[-1]:
                    n_list = r.copy()
                    n_list.append(num)
                    res.append(n_list)
        return [l for l in res if len(l) > 1]