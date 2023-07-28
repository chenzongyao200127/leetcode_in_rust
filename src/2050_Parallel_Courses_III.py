# 2050_Parallel_Courses_III
# https://leetcode.cn/problems/parallel-courses-iii/description/


# 拓扑排序 + DP
from typing import List
import collections
class Solution:
    def minimumTime(self, n: int, relations: List[List[int]], time: List[int]) -> int:
        graph = collections.defaultdict(list)
        indegrees = [0] * n
        dp = [0] * n
        for pre, next in relations:
            indegrees[next-1] += 1
            graph[pre-1].append(next-1)
        
        queue = collections.deque()
        for i, (v, t) in enumerate(zip(indegrees, time)):
            if v == 0:
                queue.append(i)
                dp[i] = t
        
        while queue:
            cur_node = queue.popleft()
            for next in graph[cur_node]:
                dp[next] = max(dp[next], dp[cur_node] + time[next])
                indegrees[next] -= 1
                if indegrees[next] == 0:
                    queue.append(next)
                    
        return max(dp)