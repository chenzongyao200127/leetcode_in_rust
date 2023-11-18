# 2736_Maximum_Sum_Queries
# https://leetcode.cn/problems/maximum-sum-queries/description/

from typing import List

# 超时
class Solution:
    def maximumSumQueries(self, nums1: List[int], nums2: List[int], queries: List[List[int]]) -> List[int]:
        answer = []
    
        for query in queries:
            xi, yi = query
            max_value = -1

            for j in range(len(nums1)):
                if nums1[j] >= xi and nums2[j] >= yi:
                    max_value = max(max_value, nums1[j] + nums2[j])

            answer.append(max_value)

        return answer

