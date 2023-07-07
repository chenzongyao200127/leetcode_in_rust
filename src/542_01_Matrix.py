# 542_01_Matrix
# https://leetcode.cn/problems/01-matrix/

# 给定一个由 0 和 1 组成的矩阵 mat ，请输出一个大小相同的矩阵，其中每一个格子是 mat 中对应位置元素到最近的 0 的距离。

# 两个相邻元素间的距离为 1 。

from typing import List


# 边界条件的处理问题：在DFS函数中，你正在检查（x+dx, y+dy）是否在矩阵的边界内，并且mat[x+dx][y+dy]是否等于0。
# 如果不满足这些条件，你将调用DFS（x+dx，y+dy，mat）。这可能会导致索引超出范围的错误，
# 因为你在调用DFS之前并没有检查（x+dx, y+dy）是否在边界内。

# 死循环问题：这个DFS算法并没有一个机制来避免访问已经访问过的点，
# 如果一个点四周都是1，那么DFS将陷入无限循环。

# 算法效率问题：对于每个等于1的元素，你都进行了DFS搜索。
# 这使得算法的时间复杂度非常高，如果矩阵中有大量的1，那么算法将变得非常慢。
# Wrong!
def wrongupdateMatrix(mat: List[List[int]]) -> List[List[int]]:
    m = len(mat)
    n = len(mat[0])
    ans = [[0 for _ in range(n)] for _ in range(m)]
    
    def dfs(x, y, mat):
        dirs = [(-1,0), (1,0), (0,1), (0,-1)]
        for dx, dy in dirs:
            if 0 <= x+dx < m and 0 <= y+dy < n and mat[x+dx][y+dy] == 0:
                return 1
            else:
                return 1 + dfs(x+dx, y+dy, mat)
        
    for i in range(m):
        for j in range(n):
            if mat[i][j] == 0:
                ans[i][j] = 0
            else:
                ans[i][j] = dfs(i, j, mat)
    
    return ans
    
    
# 要解决这个问题，一种更好的方法是使用广度优先搜索（BFS）而不是深度优先搜索（DFS）。
# 首先，对于矩阵中的每个0，将它们全部放入队列中，
# 然后从这些0开始进行广度优先搜索，更新它们邻居的距离。
# 这样做只需要遍历矩阵两次，时间复杂度为O(m*n)，其中m和n分别是矩阵的行数和列数。

from collections import deque    


# 熟悉「最短路」的读者应该知道，我们所说的「超级零」实际上就是一个「超级源点」。
# 在最短路问题中，如果我们要求多个源点出发的最短路时，一般我们都会建立一个「超级源点」连向所有的源点，
# 用「超级源点」到终点的最短路等价多个源点到终点的最短路。

# 官解
class Solution:
    def updateMatrix(self, matrix: List[List[int]]) -> List[List[int]]:
        m, n = len(matrix), len(matrix[0])
        dist = [[0] * n for _ in range(m)]
        zeroes_pos = [(i, j) for i in range(m) for j in range(n) if matrix[i][j] == 0]
        # 将所有的 0 添加进初始队列中
        q = deque(zeroes_pos)
        seen = set(zeroes_pos)

        # 广度优先搜索
        while q:
            i, j = q.popleft()
            for ni, nj in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
                if 0 <= ni < m and 0 <= nj < n and (ni, nj) not in seen:
                    dist[ni][nj] = dist[i][j] + 1
                    q.append((ni, nj))
                    seen.add((ni, nj))
        
        return dist



def updateMatrix(mat: List[List[int]]) -> List[List[int]]:
    m = len(mat)
    n = len(mat[0])
    dirs = [(-1,0), (1,0), (0,1), (0,-1)]
    ans = [[float("inf") for _ in range(n)] for _ in range(m)]
    queue = deque([])
    
    # Initialize queue with all 0s in the matrix
    for i in range(m):
        for j in range(n):
            if mat[i][j] == 0:
                ans[i][j] = 0
                queue.append((i, j, 0))
    
    # Process queue
    while queue:
        x, y, val = queue.popleft()
        for dx, dy in dirs:
            nx, ny = dx + x, dy + y
            if 0 <= nx < m and 0 <= ny < n and ans[nx][ny] > val + 1:
                ans[nx][ny] = val + 1
                queue.append((nx, ny, val + 1))
    
    print(ans)
    return ans
                
updateMatrix([[0,0,0],[0,1,0],[1,1,1]])


# 方法二：动态规划
# 在距离最短的前提下可能有四种方法：
# 只有 水平向左移动 和 竖直向上移动；
# 只有 水平向左移动 和 竖直向下移动；
# 只有 水平向右移动 和 竖直向上移动；
# 只有 水平向右移动 和 竖直向下移动。

# 例如下面这一个矩阵包含四个 0。从中心位置的 1 移动到这四个 0，就需要使用四种不同的方法：
# 0 _ _ _ 0
# _ _ _ _ _
# _ _ 1 _ _
# _ _ _ _ _
# 0 _ _ _ 0

class Solution:
    def updateMatrix(self, matrix: List[List[int]]) -> List[List[int]]:
        m, n = len(matrix), len(matrix[0])
        
        # 初始化动态规划的数组，所有的距离值都设置为一个很大的数
        dist = [[10**9] * n for _ in range(m)]
        
        # 如果 (i, j) 的元素为 0，那么距离为 0
        for i in range(m):
            for j in range(n):
                if matrix[i][j] == 0:
                    dist[i][j] = 0
        
        # 只有 水平向左移动 和 竖直向上移动，注意动态规划的计算顺序
        for i in range(m):
            for j in range(n):
                if i - 1 >= 0:
                    dist[i][j] = min(dist[i][j], dist[i - 1][j] + 1)
                if j - 1 >= 0:
                    dist[i][j] = min(dist[i][j], dist[i][j - 1] + 1)
            
        # 只有 水平向左移动 和 竖直向下移动，注意动态规划的计算顺序
        for i in range(m - 1, -1, -1):
            for j in range(n):
                if i + 1 < m:
                    dist[i][j] = min(dist[i][j], dist[i + 1][j] + 1)
                if j - 1 >= 0:
                    dist[i][j] = min(dist[i][j], dist[i][j - 1] + 1)
                    
        # 只有 水平向右移动 和 竖直向上移动，注意动态规划的计算顺序
        for i in range(m):
            for j in range(n - 1, -1, -1):
                if i - 1 >= 0:
                    dist[i][j] = min(dist[i][j], dist[i - 1][j] + 1)
                if j + 1 < n:
                    dist[i][j] = min(dist[i][j], dist[i][j + 1] + 1)
                    
        # 只有 水平向右移动 和 竖直向下移动，注意动态规划的计算顺序
        for i in range(m - 1, -1, -1):
            for j in range(n - 1, -1, -1):
                if i + 1 < m:
                    dist[i][j] = min(dist[i][j], dist[i + 1][j] + 1)
                if j + 1 < n:
                    dist[i][j] = min(dist[i][j], dist[i][j + 1] + 1)
                    
        return dist
    
    
# DP 优化
class Solution:
    def updateMatrix(self, matrix: List[List[int]]) -> List[List[int]]:
        m, n = len(matrix), len(matrix[0])
        
        # 初始化动态规划的数组，所有的距离值都设置为一个很大的数
        dist = [[10**9] * n for _ in range(m)]
        
        # 如果 (i, j) 的元素为 0，那么距离为 0
        for i in range(m):
            for j in range(n):
                if matrix[i][j] == 0:
                    dist[i][j] = 0
                    
        # 只有 水平向左移动 和 竖直向上移动，注意动态规划的计算顺序
        for i in range(m):
            for j in range(n):
                if i - 1 >= 0:
                    dist[i][j] = min(dist[i][j], dist[i - 1][j] + 1)
                if j - 1 >= 0:
                    dist[i][j] = min(dist[i][j], dist[i][j - 1] + 1)
                    
        # 只有 水平向右移动 和 竖直向下移动，注意动态规划的计算顺序
        for i in range(m - 1, -1, -1):
            for j in range(n - 1, -1, -1):
                if i + 1 < m:
                    dist[i][j] = min(dist[i][j], dist[i + 1][j] + 1)
                if j + 1 < n:
                    dist[i][j] = min(dist[i][j], dist[i][j + 1] + 1)
                    
        return dist