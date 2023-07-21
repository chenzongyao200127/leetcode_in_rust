# 1499_Max_Value_of_Equation
# https://leetcode.cn/problems/max-value-of-equation/

# 给你一个数组 points 和一个整数 k 。数组中每个元素都表示二维平面上的点的坐标，并按照横坐标 x 的值从小到大排序。
# 也就是说 points[i] = [xi, yi] ，并且在 1 <= i < j <= points.length 的前提下， xi < xj 总成立。
# 请你找出 yi + yj + |xi - xj| 的 最大值，其中 |xi - xj| <= k 且 1 <= i < j <= points.length。
# 题目测试数据保证至少存在一对能够满足 |xi - xj| <= k 的点

# 输入：points = [[1,3],[2,0],[5,10],[6,-10]], k = 1
# 输出：4
# 解释：前两个点满足 |xi - xj| <= 1 ，代入方程计算，则得到值 3 + 0 + |1 - 2| = 4 。第三个和第四个点也满足条件，得到值 10 + -10 + |5 - 6| = 1 。
# 没有其他满足条件的点，所以返回 4 和 1 中最大的那个。

# 超时
from typing import List
class Solution:
    def findMaxValueOfEquation(self, points: List[List[int]], k: int) -> int:
        ans = 0
        for i in range(len(points)-1):
            for j in range(i+1, len(points)):
                if abs(points[i][0] - points[j][0]) <= k:
                    ans = max(ans, abs(points[i][0] - points[j][0]) + points[i][1] + points[j][1])
                else:
                    break
        
        return ans
        

# 这是一个典型的动态规划问题。我们首先需要理解这个问题的性质。
# yi + yj + |xi - xj| 可以被理解为 yi - xi + (yj + xj)，
# 在这种情况下我们只需要找出每个 yi - xi 的最大值（因为 xj 和 yj 是固定的）然后和每个 yj + xj 相加。
# 由于输入的 points 是按照 xi 从小到大排序的，我们可以用一个滑动窗口来保持在 xi - k 的范围内 y - x 的最大值。


# 在这个问题中，我们使用双端队列来维护一个窗口，这个窗口内的元素表示的是在 xi - k 范围内 y - x 的最大值。
# 对于每个新的元素 (xj, yj)，我们需要找到满足 xi <= xj - k 的点 (xi, yi) 使得 yj + xj + yi - xi 最大。
# 因此，我们需要从窗口中移除那些不再满足 xi <= xj - k 条件的元素。

# 而我们的队列是按照 x 升序排列的，所以不满足条件的元素总是出现在队列的前端。
# 当队列的前端元素 xi > xj - k 时，我们就从队列前端弹出元素。

# 从队列后端弹出元素是为了保证队列的单调性，使得队列中的元素按照 y - x 从大到小排列。
# 当新的元素 (y - x) 大于队列尾部的元素时，我们就从队列后端弹出元素。

# 总的来说，从队列前端弹出元素是为了保证窗口的范围，从队列后端弹出元素是为了保证队列的单调性。

from collections import deque
class Solution:
    def findMaxValueOfEquation(self, points: List[List[int]], k: int) -> int:
        ans = float("-inf")
        queue = deque()
        for x, y in points:
            while queue and queue[0][1] < x - k:
                queue.popleft()
            ans = max(ans, y + x + queue[0][0])
            queue.append([y-x, x])
            while queue and queue[0][0] < y - x:
                queue.pop()
                
        return ans