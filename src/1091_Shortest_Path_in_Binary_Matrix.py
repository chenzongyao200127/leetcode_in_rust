# 1091_Shortest_Path_in_Binary_Matrix
# https://leetcode.cn/problems/shortest-path-in-binary-matrix/solution/er-jin-zhi-ju-zhen-zhong-de-zui-duan-lu-553kt/
from collections import deque

# BFS
class Solution:
    def shortestPathBinaryMatrix(self, grid: List[List[int]]) -> int:
        if grid[0][0] == 1:
            return -1
        
        n = len(grid)
        dist = [[inf] * n for _ in range(n)]
        dist[0][0] = 1
        
        queue = deque([(0, 0)])
        while queue:
            x, y = queue.popleft()
            
            if x == y == n-1:
                return dist[x][y]
            
            for dx in range(-1, 2):
                for dy in range(-1, 2):
                    if x + dx < 0 or x + dx >= n or y + dy < 0 or y + dy >= n:  # 越界
                        continue
                    if (grid[x + dx][y + dy] == 1 or dist[x + dx][y + dy] <= dist[x][y] + 1):   # 单元格值不为 0 或已被访问
                        continue
                    dist[x + dx][y + dy] = dist[x][y] + 1
                    queue.append((x + dx, y + dy))
        
        return -1
            
                        
# 在二维矩阵中搜索，什么时候用BFS，什么时候用DFS？
# 1.如果只是要找到某一个结果是否存在，那么DFS会更高效。
# 因为DFS会首先把一种可能的情况尝试到底，才会回溯去尝试下一种情况，只要找到一种情况，就可以返回了。
# 但是BFS必须所有可能的情况同时尝试，在找到一种满足条件的结果的同时，也尝试了很多不必要的路径；

# 2.如果是要找所有可能结果中最短的，那么BFS回更高效。
# 因为DFS是一种一种的尝试，在把所有可能情况尝试完之前，无法确定哪个是最短，
# 所以DFS必须把所有情况都找一遍，才能确定最终答案（DFS的优化就是剪枝，不剪枝很容易超时）。
# 而BFS从一开始就是尝试所有情况，所以只要找到第一个达到的那个点，
# 那就是最短的路径，可以直接返回了，其他情况都可以省略了，所以这种情况下，BFS更高效。
       
# BFS解法中的visited为什么可以全局使用？
# BFS是在尝试所有的可能路径，哪个最快到达终点，哪个就是最短。
# 那么每一条路径走过的路不同，visited（也就是这条路径上走过的点）也应该不同，那么为什么visited可以全局使用呢？
# 因为我们要找的是最短路径，那么如果在此之前某个点已经在visited中，
# 也就是说有其他路径在小于或等于当前步数的情况下，到达过这个点，证明到达这个点的最短路径已经被找到。
# 那么显然这个点没必要再尝试了，因为即便去尝试了，最终的结果也不会是最短路径了，所以直接放弃这个点即可。


from collections import deque

class Solution:
    def shortestPathBinaryMatrix(self, grid: List[List[int]]) -> int:
        if grid[0][0] != 0:
            return -1
        
        length = len(grid)
        if length == 1:
            return 1
        
        que = deque()
        visited = {}
        que.appendleft((0,0))
        visited[(0,0)] = True
        start = 1
        while que:
            for _ in range(len(que)):
                ind, con = que.pop()
                for pos_h, pos_v in [(-1,-1), (-1,0), (-1,1), (0,1), (1,1), (1,0), (1,-1), (0,-1)]:
                    new_ind = ind + pos_h
                    new_con = con + pos_v
                    
                    if 0 <= new_ind < length and 0 <= new_con < length and grid[new_ind][new_con] == 0 and not visited.get((new_ind, new_con)):
                        if new_ind == length - 1 and new_con == length - 1:
                            return start + 1
                        
                        que.appendleft((new_ind, new_con))
                        visited[(new_ind, new_con)] = True
            start += 1
        return -1
