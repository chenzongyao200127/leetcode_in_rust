# 2258_Escape_the_Spreading_Fire
# https://leetcode.cn/problems/escape-the-spreading-fire/description/

from typing import List, Tuple

class Solution:
    def maximumMinutes(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])
        
        def check(t: int) -> bool:
            fires = [(i, j) for i, row in enumerate(grid) for j, x in enumerate(row) if x == 1]
            on_fire = set(fires)
            
            def fire_spread():
                nonlocal fires
                tmp = fires
                fires = []
                for i, j in tmp:
                    for x, y in (i-1, j), (i+1, j), (i, j-1), (i, j+1):
                        if 0 <= x < m and 0 <= y < n and grid[x][y] == 0 and (x, y) not in on_fire:
                            on_fire.add((x, y))
                            fires.append((x, y))
                            
            while t and fires:
                fire_spread()
                t -= 1
                
            if (0, 0) in on_fire:
                return False
        
            queue = [(0, 0)]
            visited = set(queue)
            while queue:
                tmp = queue
                queue = []
                for i, j in tmp:
                    if (i, j) in on_fire:
                        continue
                    for x, y in (i-1, j), (i+1, j), (i, j-1), (i, j+1):
                        if 0 <= x < m and 0 <= y < n and grid[x][y] == 0 and (x, y) not in on_fire and (x, y) not in visited:
                            if x == m-1 and y == n-1:    
                                return True
                            visited.add((x, y))
                            queue.append((x, y))
                fire_spread()
            return False
        
        l = -1
        r = m * n + 1
        while l + 1 < r:
            mid = (l + r) // 2
            if check(mid):
                l = mid
            else:
                r = mid
        return l if l < m*n else 10 ** 9
        

# s = Solution()
# ans = s.maximumMinutes(grid = [[0,2,0,0,0,0,0],[0,0,0,2,2,1,0],[0,2,0,0,1,2,0],[0,0,2,2,2,0,2],[0,0,0,0,0,0,0]])
# print(ans)

# Example usage
s = Solution()
ans = s.maximumMinutes([[0,2,0,0,1],[0,2,0,2,2],[0,2,0,0,0],[0,0,2,2,0],[0,0,0,0,0]])
print(ans)



# 方法二：直接计算
class Solution:
    def maximumMinutes(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        
        def bfs(q: List[Tuple[int, int]]) -> (int, int, int):
            time = [[-1] * n for _ in range(m)]
            for i, j in q:
                time[i][j] = 0
            t = 1
            while q:
                tmp = q
                q = []
                for i, j in tmp:
                    for x,y in (i-1, j), (i+1, j), (i, j-1), (i, j+1):
                        if 0 <= x < m and 0 <= y < n and grid[x][y] == 0 and time[x][y] < 0:
                            time[x][y] = t
                            q.append((x, y))
                t += 1
            return time[-1][-1], time[-1][-2], time[-2][-1]
        
        man_to_house_time, m1, m2 = bfs([(0, 0)])
        if man_to_house_time < 0:
            return -1
        
        fires = [(i, j) for i, row in enumerate(grid) for j, x in enumerate(row) if x == 1]
        fire_to_house_time, f1, f2 = bfs(fires)
        if fire_to_house_time < 0:
            return 10 ** 9
        
        d = fire_to_house_time - man_to_house_time
        if d < 0:
            return -1
        
        if m1 != -1 and m1 + d < f1 or \
           m2 != -1 and m2 + d < f2:
               return d
        return d - 1     
            



# # GPT-4 错误解答
# from collections import deque

# class Solution:
#     def maximumMinutes(self, grid: List[List[int]]) -> int:
#         m, n = len(grid), len(grid[0])
#         directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]
        
#         # Precalculate fire spread for each minute
#         fire_time = [[float('inf')] * n for _ in range(m)]
#         fire_front = deque([(i, j) for i in range(m) for j in range(n) if grid[i][j] == 1])
#         time = 0
#         while fire_front:
#             for _ in range(len(fire_front)):
#                 i, j = fire_front.popleft()
#                 if fire_time[i][j] > time:
#                     fire_time[i][j] = time
#                     for di, dj in directions:
#                         ni, nj = i + di, j + dj
#                         if 0 <= ni < m and 0 <= nj < n and grid[ni][nj] == 0:
#                             fire_front.append((ni, nj))
#             time += 1
        
#         # Binary search to find the maximum time
#         def can_escape(start_time: int) -> bool:
#             if fire_time[0][0] <= start_time:
#                 return False
#             q = deque([(0, 0)])
#             visited = {(0, 0)}
#             while q:
#                 i, j = q.popleft()
#                 if i == m - 1 and j == n - 1:
#                     return True
#                 for di, dj in directions:
#                     ni, nj = i + di, j + dj
#                     if 0 <= ni < m and 0 <= nj < n and grid[ni][nj] == 0 and (ni, nj) not in visited:
#                         if fire_time[ni][nj] > start_time:
#                             visited.add((ni, nj))
#                             q.append((ni, nj))
#             return False

#         left, right = 0, m * n
#         while left < right:
#             mid = (left + right) // 2
#             if can_escape(mid):
#                 left = mid + 1
#             else:
#                 right = mid

#         return left - 1 if left - 1 < m * n else 10 ** 9


from typing import List

class Solution:
    def maximumMinutes(self, grid: List[List[int]]) -> int:
        """
        Calculates the maximum number of minutes one can safely wait before
        escaping from a grid where fire is spreading each minute.
        """
        m, n = len(grid), len(grid[0])

        def spread_fire(fires, on_fire):
            """
            Simulates the spread of fire for one minute.
            """
            next_fires = []
            for i, j in fires:
                for x, y in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]:
                    if 0 <= x < m and 0 <= y < n and grid[x][y] == 0 and (x, y) not in on_fire:
                        on_fire.add((x, y))
                        next_fires.append((x, y))
            return next_fires

        def is_escape_possible(t):
            """
            Checks if it is possible to escape the grid safely after waiting for 't' minutes.
            """
            # Initialize fire locations and spread fire for 't' minutes
            fires = [(i, j) for i in range(m) for j in range(n) if grid[i][j] == 1]
            on_fire = set(fires)
            
            for _ in range(t):
                fires = spread_fire(fires, on_fire)
                if not fires:  # No more fire to spread
                    break

            # Check if starting cell is on fire
            if (0, 0) in on_fire:
                return False

            # Perform BFS to find a safe path
            queue = [(0, 0)]
            visited = set(queue)
            
            while queue:
                i, j = queue.pop(0)
                if (i, j) in on_fire:
                    continue

                for x, y in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]:
                    if 0 <= x < m and 0 <= y < n and grid[x][y] == 0 and (x, y) not in on_fire and (x, y) not in visited:
                        if x == m-1 and y == n-1:    
                            return True
                        visited.add((x, y))
                        queue.append((x, y))
                
                fires = spread_fire(fires, on_fire)  # Spread fire after each step

            return False

        # Binary search to find the maximum safe waiting time
        left, right = -1, m * n + 1
        while left + 1 < right:
            mid = (left + right) // 2
            if is_escape_possible(mid):
                left = mid
            else:
                right = mid

        return left if left < m * n else 10 ** 9
        

# Example usage
s = Solution()
ans = s.maximumMinutes([[0,2,0,0,1],[0,2,0,2,2],[0,2,0,0,0],[0,0,2,2,0],[0,0,0,0,0]])
print(ans)
