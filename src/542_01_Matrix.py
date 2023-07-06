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
def updateMatrix(mat: List[List[int]]) -> List[List[int]]:
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
def updateMatrix(mat: List[List[int]]) -> List[List[int]]:
    m = len(mat)
    n = len(mat[0])
    ans = [[float("inf") for _ in range(n)] for _ in range(m)]
    
    
    
    return ans
                
                
updateMatrix([[0,0,0],[0,1,0],[0,0,0]])