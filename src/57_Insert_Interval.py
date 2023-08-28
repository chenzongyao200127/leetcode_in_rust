# 57_Insert_Interval
# https://leetcode.cn/problems/insert-interval/description/

# You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] 
# represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.

# Insert newInterval into intervals such that intervals is still sorted in ascending order by 
# starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).

# Return intervals after the insertion.

from typing import List
class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        def merge(intervals: List[List[int]]) -> List[List[int]]:
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
        
        intervals.append(newInterval)
        ans = merge(intervals)
        
        return ans
        
        