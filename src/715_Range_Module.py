# 715_Range_Module
# https://leetcode.cn/problems/range-module/description/

# A Range Module is a module that tracks ranges of numbers. 
# Design a data structure to track the ranges represented as half-open intervals and query about them.

# A half-open interval [left, right) denotes all the real numbers x where left <= x < right.

# Implement the RangeModule class:

# RangeModule() Initializes the object of the data structure.

# void addRange(int left, int right) Adds the half-open interval [left, right), 
# tracking every real number in that interval. 
# Adding an interval that partially overlaps with currently tracked numbers 
# should add any numbers in the interval [left, right) that are not already tracked.

# boolean queryRange(int left, int right) Returns true if every real 
# number in the interval [left, right) is currently being tracked, and false otherwise.

# void removeRange(int left, int right) Stops tracking every real 
# number currently being tracked in the half-open interval [left, right).
 

# Example 1:

# Input
# ["RangeModule", "addRange", "removeRange", "queryRange", "queryRange", "queryRange"]
# [[], [10, 20], [14, 16], [10, 14], [13, 15], [16, 17]]
# Output
# [null, null, null, true, false, true]

# Explanation
# RangeModule rangeModule = new RangeModule();
# rangeModule.addRange(10, 20);
# rangeModule.removeRange(14, 16);
# rangeModule.queryRange(10, 14); // return True,(Every number in [10, 14) is being tracked)
# rangeModule.queryRange(13, 15); // return False,(Numbers like 14, 14.03, 14.17 in [13, 15) are not being tracked)
# rangeModule.queryRange(16, 17); // return True, (The number 16 in [16, 17) is still being tracked, despite the remove operation)
 

# Constraints:

# 1 <= left < right <= 109
# At most 104 calls will be made to addRange, queryRange, and removeRange.

# 超出内存限制 维护有序区间-线段树
class RangeModule:

    def __init__(self):
        self.set = set()


    def addRange(self, left: int, right: int) -> None:
        for i in range(left, right):
            self.set.add(i)

    def queryRange(self, left: int, right: int) -> bool:
        for i in range(left, right):
            if i not in self.set:
                return False
        return True

    def removeRange(self, left: int, right: int) -> None:
        for i in range(left, right):
            if i in self.set:
                self.set.remove(i)
            



# Your RangeModule object will be instantiated and called as such:
# obj = RangeModule()
# obj.addRange(left,right)
# param_2 = obj.queryRange(left,right)
# obj.removeRange(left,right)

# To implement the RangeModule class as described, you can use an interval tree, a segment tree,
# or a set of intervals. For simplicity and efficiency, 
# I'll use a set of intervals sorted by their starting points. 

# This approach allows for relatively straightforward 
# implementations of addRange, queryRange, and removeRange functions.
# 维护有序集合
class RangeModule:
    def __init__(self):
        self.intervals = []

    def addRange(self, left, right):
        new_intervals = []
        i = 0
        while i < len(self.intervals):
            if self.intervals[i][1] < left:
                new_intervals.append(self.intervals[i])
            elif self.intervals[i][0] > right:
                new_intervals.append((left, right))
                left, right = self.intervals[i]
            else:
                # 这个操作有点秀
                left = min(left, self.intervals[i][0])
                right = max(right, self.intervals[i][1])
            i += 1
            
        new_intervals.append((left, right))
        self.intervals = new_intervals

    def queryRange(self, left, right):
        for start, end in self.intervals:
            if start <= left and end >= right:
                return True
        return False

    def removeRange(self, left, right):
        new_intervals = []
        for start, end in self.intervals:
            if start < left:
                new_intervals.append((start, min(end, left)))
            if end > right:
                new_intervals.append((max(start, right), end))
        self.intervals = new_intervals


# 线段树        
class SegmentTreeNode:
    def __init__(self, start, end):
        self.start = start
        self.end = end
        self.track = False
        self.left = self.right = None

class RangeModule:
    def __init__(self):
        self.root = SegmentTreeNode(0, 10**9)  # Assuming a large enough range.

    def update(self, node, start, end, track):
        if start >= node.end or end <= node.start:
            return
        if start <= node.start and end >= node.end:
            node.track = track
            node.left = node.right = None  # Remove children as this node is updated.
        else:
            mid = (node.start + node.end) // 2
            if not node.left:
                node.left = SegmentTreeNode(node.start, mid)
                node.right = SegmentTreeNode(mid, node.end)
            if node.track is not None:
                node.left.track = node.right.track = node.track
                node.track = None
            self.update(node.left, start, end, track)
            self.update(node.right, start, end, track)

    def query(self, node, start, end):
        if start >= node.end or end <= node.start:
            return True
        if start <= node.start and end >= node.end:
            return node.track
        if not node.left:
            return node.track
        mid = (node.start + node.end) // 2
        return (self.query(node.left, start, min(end, mid)) and
                self.query(node.right, max(start, mid), end))

    def addRange(self, left, right):
        self.update(self.root, left, right, True)

    def queryRange(self, left, right):
        return self.query(self.root, left, right)

    def removeRange(self, left, right):
        self.update(self.root, left, right, False)


# the fast solution
from bisect import bisect_left, bisect_right

class RangeModule:

    def __init__(self):
        # Initialize an empty list to store the active ranges
        self.rage = []

    def addRange(self, left: int, right: int) -> None:
        # Find the insertion points for left and right in the list
        l = bisect_left(self.rage, left)
        r = bisect_right(self.rage, right)
        
        # Prepare a list to merge with existing ranges
        merge = []
        if l & 1 == 0:
            # If l is even, left is outside the current ranges and should be included
            merge.append(left)
        if r & 1 == 0:
            # If r is even, right is outside the current ranges and should be included
            merge.append(right)
        
        # Update the list by merging new range
        self.rage = self.rage[:l] + merge + self.rage[r:]

    def queryRange(self, left: int, right: int) -> bool:
        # Find the positions of left and right in the list
        l = bisect_right(self.rage, left)
        r = bisect_left(self.rage, right)

        # Check if left and right are in the same range and that range is active
        if l == r and l % 2:
            return True
        return False

    def removeRange(self, left: int, right: int) -> None:
        # Find the insertion points for left and right in the list
        l = bisect_left(self.rage, left)
        r = bisect_right(self.rage, right)

        # Prepare a list to merge with existing ranges
        merge = []
        if l & 1 == 1:
            # If l is odd, left is inside a current range and should be the new start
            merge.append(left)
        if r & 1 == 1:
            # If r is odd, right is inside a current range and should be the new end
            merge.append(right)

        # Update the list by merging the new ranges
        self.rage = self.rage[:l] + merge + self.rage[r:]

# Example of usage:
# rangeModule = RangeModule()
# rangeModule.addRange(10, 20)
# print(rangeModule.queryRange(10, 15))  # True
# rangeModule.removeRange(14, 16)
# print(rangeModule.queryRange(10, 15))  # False
