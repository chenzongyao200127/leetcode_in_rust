# 630_Course_Schedule_III
# https://leetcode.cn/problems/course-schedule-iii/


# 示例 1：
# 输入：courses = [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]
# 输出：3
# 解释：
# 这里一共有 4 门课程，但是你最多可以修 3 门：
# 首先，修第 1 门课，耗费 100 天，在第 100 天完成，在第 101 天开始下门课。
# 第二，修第 3 门课，耗费 1000 天，在第 1100 天完成，在第 1101 天开始下门课程。
# 第三，修第 2 门课，耗时 200 天，在第 1300 天完成。
# 第 4 门课现在不能修，因为将会在第 3300 天完成它，这已经超出了关闭日期。

# 示例 2：
# 输入：courses = [[1,2]]
# 输出：1

# 示例 3：
# 输入：courses = [[3,2],[4,3]]
# 输出：0


# During iteration, say I want to add the current course, 
# currentTotalTime being total time of all courses taken till now, 
# but adding the current course might exceed my deadline or it doesn’t.

# 1. If it doesn’t, then I have added one new course. 
# Increment the currentTotalTime with duration of current course.

# 2. If it exceeds deadline, I can swap current course with current courses that has biggest duration.
# * No harm done and I might have just reduced the currentTotalTime, right?
# * What preprocessing do I need to do on my course processing order so that this swap is always legal?

from typing import List
import heapq
from pprint import pprint

class Solution:
    def scheduleCourse(self, courses: List[List[int]]) -> int:
        pq = []
        courses = [course for course in courses if course[0] <= course[1]]
        courses.sort(key=lambda c: c[1])
        
        if not courses:
            return 0

        cur_time = 0

        for course in courses:
            duration, ddl = course[0], course[1]
            if cur_time + duration <= ddl:
                cur_time += duration
                heapq.heappush(pq, -duration)
            else:
                d = -pq[0]
                if duration < d:
                    cur_time = cur_time - d + duration
                    heapq.heappop(pq)
                    heapq.heappush(pq, -duration)

        return len(pq)
                    
# # 官解
# class Solution:
#     def scheduleCourse(self, courses: List[List[int]]) -> int:
#         courses.sort(key=lambda c: c[1])

#         q = list()
#         # 优先队列中所有课程的总时间
#         total = 0

#         for ti, di in courses:
#             if total + ti <= di:
#                 total += ti
#                 # Python 默认是小根堆
#                 heapq.heappush(q, -ti)
#             elif q and -q[0] > ti:
#                 total -= -q[0] - ti
#                 heapq.heappop(q)
#                 heapq.heappush(q, -ti)

#         return len(q)

            
        
s = Solution()
ans = s.scheduleCourse(courses = [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]])
print(ans)

s = Solution()
ans = s.scheduleCourse(courses = [[1,2]])
print(ans)
        
s = Solution()
ans = s.scheduleCourse(courses = [[3,2],[4,3]])
print(ans)

s = Solution()
ans = s.scheduleCourse(courses = [[1,2],[2,3]])
print(ans)

s = Solution()
ans = s.scheduleCourse(courses = [[5,15],[3,19],[6,7],[2,10],[5,16],[8,14],[10,11],[2,19]])
print(ans)
                            
s = Solution()
ans = s.scheduleCourse(courses = [[7,17],[3,12],[10,20],[9,10],[5,20],[10,19],[4,18]])
print(ans)
                                                            