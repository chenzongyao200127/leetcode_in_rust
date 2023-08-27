# 56_Merge_Intervals
# https://leetcode.cn/problems/merge-intervals/description/

# Example 1:
# Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
# Output: [[1,6],[8,10],[15,18]]
# Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].

# Example 2:
# Input: intervals = [[1,4],[4,5]]
# Output: [[1,5]]
# Explanation: Intervals [1,4] and [4,5] are considered overlapping.

from typing import List
class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        intervals = sorted(intervals)
        l = intervals[0][0]
        r = intervals[0][1]
        ans = []
        for x, y in intervals:
            if x > r:
                ans.append([l, r])
                l = x
                r = y
            else:
                l = min(l, x)
                r = max(r, y)
        
        ans.append([l, r])
        return ans

s = Solution()
res = s.merge([[1,3],[2,6],[8,10],[15,18]])
print(res)

res = s.merge([[1,4],[4,5]])
print(res)



class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        intervals.sort(key=lambda x: x[0])

        merged = []
        for interval in intervals:
            # 如果列表为空，或者当前区间与上一区间不重合，直接添加
            if not merged or merged[-1][1] < interval[0]:
                merged.append(interval)
            else:
                # 否则的话，我们就可以与上一区间进行合并
                merged[-1][1] = max(merged[-1][1], interval[1])

        return merged