# 1462_课程表_IV
# https://leetcode.cn/problems/course-schedule-iv/description/

# 输入：numCourses = 2, prerequisites = [[1,0]], queries = [[0,1],[1,0]]
# 输出：[false,true]
# 解释：课程 0 不是课程 1 的先修课程，但课程 1 是课程 0 的先修课程。

# 输入：numCourses = 2, prerequisites = [], queries = [[1,0],[0,1]]
# 输出：[false,false]
# 解释：没有先修课程对，所以每门课程之间是独立的。

# 输入：numCourses = 3, prerequisites = [[1,2],[1,0],[2,0]], queries = [[1,0],[1,2]]
# 输出：[true,true]

# from typing import List

# class Solution:
#     def checkIfPrerequisite(self, numCourses: int, prerequisites: List[List[int]], queries: List[List[int]]) -> List[bool]:
#         prereq_map = {}
#         for i in range(numCourses):
#             prereq_map[i] = []
        
#         for pair in prerequisites:
#             course, pre_req_class = pair[0], pair[1]
#             prereq_map[course].append(pre_req_class)

#         def search(pre, cur, visited):
#             if pre == cur:
#                 return True
            
#             if pre in prereq_map[cur]:
#                 return True

#             for c in prereq_map[cur]:
#                 if c not in visited:
#                     visited.add(c)
#                     if search(pre, c, visited):
#                         return True
            
#             return False
        
#         res = []
#         for pair in queries:
#             pre_req, course = pair[0], pair[1]
#             res.append(search(course, pre_req, set()))
            
#         return res

from typing import List

class Solution:
    def checkIfPrerequisite(self, numCourses: int, prerequisites: List[List[int]], queries: List[List[int]]) -> List[bool]:
        # 使用集合而不是列表来存储每个课程的先决条件
        prereq_map = {}
        for i in range(numCourses):
            prereq_map[i] = set()
        
        for pair in prerequisites:
            course, pre_req_class = pair[0], pair[1]
            prereq_map[course].add(pre_req_class)

        def search(pre, cur, visited):
            if pre == cur:
                return True
            
            if pre in prereq_map[cur]:
                return True

            for c in prereq_map[cur]:
                if c not in visited:
                    visited.add(c)
                    if search(pre, c, visited):
                        return True
            
            return False
        
        res = []
        for pair in queries:
            pre_req, course = pair[0], pair[1]
            res.append(search(course, pre_req, set()))
            
        return res

                

                        
# Here's an optimized version of the given code:

# 1. Instead of using two separate loops to initialize the `prereq_map` and populate it, we can merge them.
# 2. The variable names are a bit misleading. Let's use clearer names.
# 3. We can use a Floyd-Warshall algorithm to preprocess the prerequisites, 
#    determining the transitive closure, which will then make queries fast.


from typing import List

class Solution:
    def checkIfPrerequisite(self, numCourses: int, prerequisites: List[List[int]], queries: List[List[int]]) -> List[bool]:
        # Initialize the adjacency matrix with False
        matrix = [[False for _ in range(numCourses)] for _ in range(numCourses)]
        
        for course, pre_req in prerequisites:
            matrix[course][pre_req] = True

        # Floyd-Warshall Algorithm to find the transitive closure
        for k in range(numCourses):
            for i in range(numCourses):
                for j in range(numCourses):
                    matrix[i][j] = matrix[i][j] or (matrix[i][k] and matrix[k][j])
        
        return [matrix[course][pre_req] for course, pre_req in queries]

# Explanation:

# 1. The adjacency matrix `matrix` is used to represent the prerequisites between courses. 
# If `matrix[i][j]` is True, then `j` is a prerequisite of `i`.

# 2. Floyd-Warshall algorithm is used to find the transitive closure of the graph, 
# meaning after executing the algorithm, if `matrix[i][j]` is True, 
# then there exists a path from node `i` to node `j` considering all intermediate prerequisites.

# 3. Finally, we can now answer each query in constant time by just looking up the adjacency matrix.

from collections import deque
from typing import List

class Solution:
    def checkIfPrerequisite(
        self, n: int, prerequisites: List[List[int]], queries: List[List[int]]
    ) -> List[bool]:
        
        # f[i][j] 表示从课程 i 到课程 j 是否存在路径
        f = [[False] * n for _ in range(n)]
        
        # g[i] 是一个列表，保存了从课程 i 出发可以直接到达的课程
        g = [[] for _ in range(n)]
        
        # indeg[i] 代表指向课程 i 的先决条件的数量
        indeg = [0] * n
        
        # 根据给定的先决条件关系建立图结构并计算每个课程的入度
        for a, b in prerequisites:
            g[a].append(b)
            indeg[b] += 1

        # 初始化队列，包括那些没有先决条件（即入度为0）的课程
        q = deque(i for i, x in enumerate(indeg) if x == 0)

        # 拓扑排序
        while q:
            i = q.popleft()
            for j in g[i]:
                f[i][j] = True
                
                # 对于每个课程 h，如果 h 到 i 存在路径，且 i 到 j 存在路径
                # 则 h 到 j 也存在路径
                for h in range(n):
                    f[h][j] = f[h][j] or f[h][i]
                    
                # 由于 j 课程的一个先决条件已被处理，因此减少它的入度
                indeg[j] -= 1
                
                # 如果 j 的入度为0，将其加入队列以进行处理
                if indeg[j] == 0:
                    q.append(j)

        # 根据查询列表返回结果
        return [f[a][b] for a, b in queries]

# 时间复杂度 O(n^2)
# 空间复杂度 O(n^2)


s = Solution()
ans = s.checkIfPrerequisite(n = 2, prerequisites = [[1,0]], queries = [[0,1],[1,0]])
print(ans)        
                        
                
s = Solution()
ans = s.checkIfPrerequisite(n = 2, prerequisites = [], queries = [[1,0],[0,1]])
print(ans)

s = Solution()
ans = s.checkIfPrerequisite(n = 3, prerequisites = [[1,2],[1,0],[2,0]], queries = [[1,0],[1,2]])
print(ans)