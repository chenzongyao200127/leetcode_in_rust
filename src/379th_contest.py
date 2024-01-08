from typing import List
import math

class Solution:
    def areaOfMaxDiagonal(self, dimensions: List[List[int]]) -> int:
        max_diagonal = 0
        max_area = 0

        for dimension in dimensions:
            length, width = dimension
            diagonal = math.sqrt(length ** 2 + width ** 2)
            area = length * width
            if diagonal > max_diagonal or (diagonal == max_diagonal and area > max_area):
                max_diagonal = diagonal
                max_area = area

        return max_area
    
    
class Solution:
    def minMovesToCaptureTheQueen(self, a: int, b: int, c: int, d: int, e: int, f: int) -> int:
        rook_can_capture = (a == e or b == f) and not ((a == e == c and ((b < d < f) or (f < d < b))) or (b == f == d and ((a < c < e) or (e < c < a))))
        
        bishop_can_capture = False
        if abs(c - e) == abs(d - f):
            step_x = 1 if e > c else -1
            step_y = 1 if f > d else -1
            x, y = c + step_x, d + step_y
            while 0 <= x < 8 and 0 <= y < 8 and (x, y) != (e, f):
                if (x, y) == (a, b):
                    break
                x, y = x + step_x, y + step_y
            else:
                bishop_can_capture = True
                
        if rook_can_capture or bishop_can_capture:
            return 1
        else:
            return 2
        
from collections import Counter

class Solution:
    def maximumSetSize(self, nums1: List[int], nums2: List[int]) -> int:
        n = len(nums1)
        half_n = n // 2

        # 计算两个数组中每个元素的出现次数
        freq1 = Counter(nums1)
        freq2 = Counter(nums2)
        d1 = 0
        d2 = 0
        # 移除各自数组中的重复元素（保留一个）
        for num in set(nums1):
            if freq1[num] > 1:
                d1 += freq1[num] - 1
                freq1[num] = 1
        for num in set(nums2):
            if freq2[num] > 1:
                d2 += freq2[num] - 1
                freq2[num] = 1

        # 计算目前各数组中的元素总数
        current_count1 = sum(freq1.values())
        current_count2 = sum(freq2.values())

        # 如果移除重复元素后已达到目标，直接计算结果
        if current_count1 <= half_n and current_count2 <= half_n:
            result = len(set([num for num in freq1 if freq1[num] > 0]).union(set([num for num in freq2 if freq2[num] > 0])))
            return result

        # 计算两个数组的交集
        intersection = set(nums1).intersection(set(nums2))

        # 移除交集中的元素
        for num in intersection:
            if current_count1 > current_count2:
                freq1[num] = 0
                current_count1 -= 1
            else:
                freq2[num] = 0
                current_count2 -= 1
        
        # 计算最终集合中的唯一元素数量
        result = len(set([num for num in freq1 if freq1[num] > 0]).union(set([num for num in freq2 if freq2[num] > 0])))
        
        if current_count1 > half_n:
            result -= (current_count1 - half_n)
        
        if current_count2 > half_n:
            result -= (current_count2 - half_n)    
            
        return result
                
s = Solution()
ans = s.maximumSetSize(nums1 = [1,1,2,2,3,3], nums2 = [4,4,5,5,6,6])
print(ans)
                    
        
                    
                
                