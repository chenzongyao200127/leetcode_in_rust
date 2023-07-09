# 417_Pacific_Atlantic_Water_Flow
# https://leetcode.cn/problems/pacific-atlantic-water-flow/

from typing import List


# 有一个 m × n 的矩形岛屿，与 太平洋 和 大西洋 相邻。 
# “太平洋” 处于大陆的左边界和上边界，而 “大西洋” 处于大陆的右边界和下边界。

# 这个岛被分割成一个由若干方形单元格组成的网格。
# 给定一个 m x n 的整数矩阵 heights ， heights[r][c] 表示坐标 (r, c) 上单元格 高于海平面的高度 。

# 岛上雨水较多，如果相邻单元格的高度 小于或等于 当前单元格的高度，雨水可以直接向北、南、东、西流向相邻单元格。
# 水可以从海洋附近的任何单元格流入海洋。

# 返回网格坐标 result 的 2D 列表 ，其中 result[i] = [ri, ci] 表示雨水从单元格 (ri, ci) 流动 既可流向太平洋也可流向大西洋 。

# 输入: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
# 输出: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]

from collections import deque


from typing import List
from collections import deque

def pacificAtlantic(heights: List[List[int]]) -> List[List[int]]:
    m = len(heights)
    n = len(heights[0])
    able = []
    dirs = [(-1,0), (1,0), (0,-1), (0,1)]
    for i in range(m):
        for j in range(n):
            pac_flag = False
            atla_flag = False
            queue = deque([(i, j, heights[i][j])])
            visit = set()
            while queue:
                i, j, height = queue.popleft()
                visit.add((i,j))
                if i == 0 or j == 0:
                    pac_flag = True
                if i == m-1 or j == n-1:
                    atla_flag = True
                if pac_flag and atla_flag:
                    able.append([i,j])
                    break
                for di, dj in dirs:
                    new_i = di + i
                    new_j = dj + j
                    if 0 <= new_i < m and 0 <= new_j < n and (new_i, new_j) not in visit:
                        if heights[new_i][new_j] <= height:
                            queue.append((new_i, new_j, heights[new_i][new_j]))
    return able

# 注意这段代码实际上并不能正确地解决这个问题。
# 因为这个问题需要从边缘开始向内部进行遍历，即从能流到海洋的点开始，找到那些可以从这些点通过一系列上流节点到达的内部节点。
# 你的代码实际上是尝试从每个内部节点出发，找到能否到达两个海洋，但你的代码并没有正确地实现这一点，因为你的代码并没有正确地处理水流的方向性。
# 实际上，对于这个问题，通常需要分别从两个海洋出发，进行两次深度优先搜索或广度优先搜索，然后找到那些能被两次搜索都访问到的节点。

print(pacificAtlantic([[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]))
# 输出: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
                    
                        
                        
# GPT-4        
# 这是一个解决"Pacific Atlantic Water Flow"问题的更好的方法，
# 它使用广度优先搜索（BFS）策略，从四个边界开始向内部进行搜索，这种方法比原来的方法更直观，也更符合这个问题的物理特性。
from typing import List, Tuple, Set
from collections import deque

def pacificAtlantic(heights: List[List[int]]) -> List[List[int]]:
    if not heights: return []
    m, n = len(heights), len(heights[0])
    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    
    def bfs(start_points):
        queue = deque(start_points)
        visited = set(start_points)
        while queue:
            x, y = queue.popleft()
            for dx, dy in directions:
                nx, ny = x + dx, y + dy
                new_point = (nx, ny)
                if nx < 0 or ny < 0 or nx == m or ny == n or new_point in visited or heights[nx][ny] < heights[x][y]:
                    continue
                queue.append(new_point)
                visited.add(new_point)
        return visited

    pacific_start_points = [(i, 0) for i in range(m)] + [(0, j) for j in range(1, n)]
    atlantic_start_points = [(i, n-1) for i in range(m)] + [(m-1, j) for j in range(n-1)]

    pacific_reachable = bfs(pacific_start_points)
    atlantic_reachable = bfs(atlantic_start_points)

    return list(pacific_reachable & atlantic_reachable)

        
        
print(pacificAtlantic([[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]))
# 输出: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
        
        
class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        m, n = len(heights), len(heights[0])

        def search(starts: List[Tuple[int, int]]) -> Set[Tuple[int, int]]:
            visited = set()
            
            def dfs(x: int, y: int):
                if (x, y) in visited:
                    return
                visited.add((x, y))
                for nx, ny in ((x, y + 1), (x, y - 1), (x - 1, y), (x + 1, y)):
                    if 0 <= nx < m and 0 <= ny < n and heights[nx][ny] >= heights[x][y]:
                        dfs(nx, ny)
                        
            for x, y in starts:
                dfs(x, y)
            return visited

        pacific = [(0, i) for i in range(n)] + [(i, 0) for i in range(1, m)]
        atlantic = [(m - 1, i) for i in range(n)] + [(i, n - 1) for i in range(m - 1)]
        return list(map(list, search(pacific) & search(atlantic)))
