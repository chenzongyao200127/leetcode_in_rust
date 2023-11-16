# 162_寻找峰值
# https://leetcode.cn/problems/find-peak-element/description/

from typing import List

class Solution:
    def findPeakElement(self, nums: List[int]) -> int:
        # edge case
        length = len(nums)
        if length == 1:
            return 0
        
        if nums[0] > nums[1]:
            return 0
        
        if nums[-2] < nums[-1]:
            return length - 1
        
        l = 0
        r = 2
        while r < length:
            if nums[l] < nums[l+1] and nums[r] < nums[r-1]:
                return l + 1
            else:
                l += 1
                r += 1
        

class Solution:
    def findPeakElement(self, nums: List[int]) -> int:
        length = len(nums)

        # Edge case for a single element
        if length == 1:
            return 0

        left, right = 0, length - 1
        while left < right:
            mid = (left + right) // 2

            # Check if the middle element is a peak
            if nums[mid] > nums[mid + 1]:
                right = mid
            else:
                left = mid + 1

        return left
    
# Rust     
# impl Solution {
#     pub fn find_peak_element(nums: Vec<i32>) -> i32 {
#         if nums.len() == 1 {
#             return 0;
#         }       

#         let mut l = 0;
#         let mut r = nums.len()-1;

#         while l < r {
#             let mid = l + (r - l) / 2;
#             if nums[mid] > nums[mid + 1] {
#                 r = mid;
#             } else {
#                 l = mid + 1;
#             }
#         }

#         l as i32
#     }
# }    
        