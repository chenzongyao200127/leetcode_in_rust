# 2276_统计区间中的整数数目
# https://leetcode.cn/problems/count-integers-in-intervals/description/?envType=daily-question&envId=2023-12-16

from sortedcontainers import SortedDict


class CountIntervals:
    def __init__(self):
        # Initialize a SortedDict to store intervals.
        # SortedDict is used to maintain the intervals in sorted order.
        self.mp = SortedDict()
        # Initialize a counter to keep track of the total count of unique elements in all intervals.
        self.cnt = 0

    def add(self, left: int, right: int) -> None:
        # Find the position in the sorted dictionary where 'right' would fit.
        interval_index = self.mp.bisect_right(right)

        # Adjust the index if it's not the first element.
        if interval_index != 0:
            interval_index -= 1

        # Merge overlapping intervals and adjust the total count accordingly.
        while interval_index < len(self.mp) and self.mp.keys()[interval_index] <= right \
                and self.mp.values()[interval_index] >= left:
            l, r = self.mp.items()[interval_index]
            left = min(left, l)
            right = max(right, r)
            # Decrease the count by the size of the current interval as it will be merged.
            self.cnt -= r - l + 1
            # Remove the current interval as it is being merged into the new interval.
            self.mp.popitem(interval_index)

            # Recalculate the position for the merged interval.
            interval_index = self.mp.bisect_right(right)
            if interval_index != 0:
                interval_index -= 1

        # Add the new/merged interval's size to the total count.
        self.cnt += right - left + 1
        # Store the new/merged interval in the sorted dictionary.
        self.mp[left] = right

    def count(self) -> int:
        # Return the total count of unique elements in all intervals.
        return self.cnt
