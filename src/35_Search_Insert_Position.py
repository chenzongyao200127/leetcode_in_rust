# 35_Search_Insert_Position
# https://leetcode.cn/problems/search-insert-position/

from typing import List

class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        l = 0
        r = len(nums) - 1
        mid = l + r // 2
        while nums[mid] != target:
            if nums[mid] < target:
                r = mid
            if nums[mid] > target:
                l = mid
            mid = l + r // 2
            if mid - l == 1 or r - mid == 1:
                return l + 1
        return mid
    
# There are a few issues with your binary search logic:

# 1. You initialize `r` to `len(nums)`, but it should be `len(nums) - 1` since list indices are 0-based.
# 2. The calculation for `mid` has the wrong precedence for the addition and division operations. It should be `mid = (l + r) // 2`.
# 3. In the condition where `nums[mid] < target`, you should move the left boundary `l`, not the right boundary `r`. Conversely, when `nums[mid] > target`, you should move the right boundary `r`, not the left boundary `l`.
# 4. The check `if mid - l == 1 or r - mid == 1` is not very intuitive. Instead, a better stopping condition would be `while l <= r`.
# 5. After the loop finishes, if the target is not found, we can return the `l` index as the position where `target` should be inserted.

class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        l = 0
        r = len(nums) - 1  # Fix 1

        while l <= r:  # Fix 4
            mid = l + (r - l) // 2  # Fix 2

            if nums[mid] == target:
                return mid
            elif nums[mid] < target:  # Fix 3
                l = mid + 1
            else:
                r = mid - 1

        return l  # Fix 5
