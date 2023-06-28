# 31_Next_Permutation
# https://leetcode.cn/problems/next-permutation/


class Solution:
    def nextPermutation(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        suf = -float('inf')
        a = -1
        for i in range(len(nums)-1, -1, -1):
            cur = nums[i]
            if suf > cur:
                a = i
                break
            suf = cur
        if a == -1:
            nums.sort()
            return
        
        for i in range(len(nums)-1, a, -1):
            if nums[i] > nums[a]:
                b = i
                break
            
        nums[a], nums[b] = nums[b], nums[a]
        nums[a+1:] = sorted(nums[a+1:])
        
        
from typing import List

class Solution:
    def nextPermutation(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        def find_decreasing_suffix_start(nums: List[int]) -> int:
            for i in range(len(nums) - 1, 0, -1):
                if nums[i - 1] < nums[i]:
                    return i
            return 0

        def find_swap_index(nums: List[int], start: int) -> int:
            for i in range(len(nums) - 1, start - 1, -1):
                if nums[i] > nums[start - 1]:
                    return i

        suffix_start = find_decreasing_suffix_start(nums)

        if suffix_start == 0:
            nums.sort()
            return

        swap_index = find_swap_index(nums, suffix_start)
        nums[suffix_start - 1], nums[swap_index] = nums[swap_index], nums[suffix_start - 1]
        nums[suffix_start:] = sorted(nums[suffix_start:])        
        
        
        
class Solution:
    def nextPermutation(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        n=len(nums)
        if n==1: 
            return
        else:
            min1=1000
            flag=False
            for i in range(n-2,-1,-1):
                for j in range(i+1,n):
                    if nums[i]<nums[j]:
                        if min1>nums[j]:
                            min1=nums[j]
                            k=j
                        flag=True
                if flag: break
            if min1!=1000:
                nums[i],nums[k]=nums[k],nums[i]
                nums1=nums[i+1:n].copy()
                nums1.sort()
                nums[i+1:n]=nums1
            else:
                nums.sort()
           
            