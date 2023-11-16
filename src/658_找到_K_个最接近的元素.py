# 658_找到_K_个最接近的元素
# https://leetcode.cn/problems/find-k-closest-elements/description/

from typing import List

class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
        
        # Creating a list of tuples containing the original index and the absolute difference from x
        indexed_diff = [(idx, abs(n - x)) for idx, n in enumerate(arr)]

        # Sorting the list first by the difference, then by the original index in case of a tie
        indexed_diff.sort(key=lambda elem: (elem[1], elem[0]))

        # Getting the first k elements, retrieving their original values, and sorting the result
        closest_elements = [arr[idx] for idx, _ in indexed_diff[:k]]
        closest_elements.sort()

        return closest_elements
        
s = Solution()
ans = s.findClosestElements(arr = [1,2,3,4,5], k = 4, x = 3)
print(ans)


class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
        arr.sort(key=lambda v: abs(v - x))
        return sorted(arr[:k])
    
from bisect import bisect_left

class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
        r = bisect_left(arr, x)    
        l = r - 1
        
        for _ in range(k):
            if l < 0:
                r += 1
            elif r >= len(arr) or x - arr[l] <= arr[r] - x:
                l -= 1
            else:
                r += 1
        
        return arr[l+1: r]
                
        
class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
        # left, right = 0, len(arr) - k
        # while left < right:
        #     mid = (left + right) // 2
        #     if abs(arr[mid] - x) >= abs(arr[mid + k] - x):
        #         left = mid + 1
        #     else:
        #         right = mid 
        # return arr[left: left + k]
        n = len(arr)
        
        # 最大的起点为n-k，这样才能保证选取长度为k的连续子数组
        left, right = 0, n - k
        while left < right:
            mid = (left + right) // 2
            # mid与mid+k分别为当前的左右端点
            if x - arr[mid] <= arr[mid+k] - x:
                right = mid
            else:
                left = mid + 1
        return arr[left:left+k]        