# 33_Search_in_Rotated_Sorted_Array
# https://leetcode.cn/problems/search-in-rotated-sorted-array/

from typing import List

# 二分 x 2
class Solution:
    def search(self, nums: List[int], target: int) -> int:
        n = len(nums)
        
        def findMinIdx(nums):
            l = 0
            r = n-1
            while l < r:
                mid = (r - l) // 2 + l
                if nums[mid] > nums[r]:
                    l = mid + 1
                else:
                    r = mid
            return l
        
        minIdx = findMinIdx(nums)
        
        # print(minIdx)
        
        def binSearch(nums):
            if not nums:
                return -1
            # print(nums)
            
            if nums[-1] < target or nums[0] > target:
                return -1
            
            l = 0
            r = len(nums) - 1
            
            while l < r:
                mid = (r - l) // 2 + l
                # print(l,r,mid)
                if nums[mid] < target:
                    l = mid + 1
                else:
                    r = mid
                    
            if nums[l] == target:
                return l
            else:
                return -1
            
        ans1 = binSearch(nums[:minIdx])
        ans2 = binSearch(nums[minIdx:])
        
        if ans1 == -1 and ans2 == -1:
            return -1
        elif ans1 == -1:
            return minIdx + ans2
        else:
            return ans1

s = Solution()
ans = s.search([4,5,6,7,0,1,2], 3)  
print(ans)      
        

from typing import List

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        if not nums:
            return -1

        left, right = 0, len(nums) - 1

        while left <= right:
            mid = (left + right) // 2

            if nums[mid] == target:
                return mid

            # Check if the left half is sorted
            if nums[left] <= nums[mid]:
                if nums[left] <= target < nums[mid]:
                    right = mid - 1
                else:
                    left = mid + 1
                    
            # Right half is sorted
            else:
                if nums[mid] < target <= nums[right]:
                    left = mid + 1
                else:
                    right = mid - 1

        return -1

s = Solution()
ans = s.search([4, 5, 6, 7, 0, 1, 2], 3)
print(ans)
        