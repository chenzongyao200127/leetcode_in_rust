# 207_Course_Schedule
# https://leetcode.cn/problems/course-schedule/


from typing import List

# 你这个学期必须选修 numCourses 门课程，记为 0 到 numCourses - 1 。

# 在选修某些课程之前需要一些先修课程。 先修课程按数组 prerequisites 给出，
# 其中 prerequisites[i] = [ai, bi] ，表示如果要学习课程 ai 则 必须 先学习课程  bi 。

# 例如，先修课程对 [0, 1] 表示：想要学习课程 0 ，你需要先完成课程 1 。
# 请你判断是否可能完成所有课程的学习？如果可以，返回 true ；否则，返回 false 。

# 示例 1：
# 输入：numCourses = 2, prerequisites = [[1,0]]
# 输出：true
# 解释：总共有 2 门课程。学习课程 1 之前，你需要完成课程 0 。这是可能的。

# 示例 2：
# 输入：numCourses = 2, prerequisites = [[1,0],[0,1]]
# 输出：false
# 解释：总共有 2 门课程。学习课程 1 之前，你需要先完成​课程 0 ；并且学习课程 0 之前，你还应先完成课程 1 。这是不可能的。

# 这个问题可以通过拓扑排序来解决。拓扑排序是一种对有向无环图（DAG）进行排序的算法。

# 首先，我们可以使用一个列表indegree来记录每门课程的入度（即有多少先修课程）。然后，我们使用一个邻接表adjacency来表示课程之间的依赖关系，
# 其中adjacency[i]表示课程i的后续课程列表。

# 接下来，我们从入度为0的课程开始，将其加入一个队列queue中。然后，我们不断从队列中取出课程，并处理其后续课程。
# 对于取出的课程a，我们遍历其后续课程列表adjacency[a]，将每门课程的入度减1。如果某门课程的入度减为0，则将其加入队列中。

# 最后，我们判断是否所有课程都被学习完。如果是，则返回true；否则，返回false。
from collections import deque

def canFinish(numCourses: int, prerequisites: List[List[int]]) -> bool:
    indegree = [0] * numCourses  # 记录每门课程的入度（有多少先修课程）
    adj = [[] for _ in range(numCourses)]  # 邻接表，表示课程之间的依赖关系
    
    for pair in prerequisites:
        course, prerequisite = pair
        indegree[course] += 1  # 更新课程的入度
        adj[prerequisite].append(course)  # 添加依赖关系
    
    queue = deque()  # 创建队列用于拓扑排序
    for i in range(numCourses):
        if indegree[i] == 0:
            queue.append(i)  # 将入度为0的课程加入队列
    
    while queue:
        course = queue.popleft()
        numCourses -= 1
        for next_course in adj[course]:
            indegree[next_course] -= 1  # 从取出的课程开始处理其后续课程
            if indegree[next_course] == 0:
                queue.append(next_course)  # 如果后续课程的入度为0，则加入队列
    
    return numCourses == 0  # 判断是否所有课程都被学习完
    
print(canFinish(numCourses = 2, prerequisites = [[1,0],[0,1]]))
    
    