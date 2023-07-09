# 130_Surrounded_Regions
# https://leetcode.cn/problems/surrounded-regions/

# 给你一个 m x n 的矩阵 board ，由若干字符 'X' 和 'O' ，找到所有被 'X' 围绕的区域，并将这些区域里所有的 'O' 用 'X' 填充。

# 输入：board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
# 输出：[["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
# 解释：被围绕的区间不会存在于边界上，换句话说，任何边界上的 'O' 都不会被填充为 'X'。 
# 任何不在边界上，或不与边界上的 'O' 相连的 'O' 最终都会被填充为 'X'。如果两个元素在水平或垂直方向相邻，则称它们是“相连”的。

from typing import List, Tuple, Set

class Solution:
    def solve(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        m = len(board)
        n = len(board[0])

        def search(starts: List[Tuple[int, int]]) -> Set[Tuple[int, int]]:
            visited = set()
            
            def dfs(x: int, y: int):
                if (x, y) in visited:
                    return
                visited.add((x, y))
                for nx, ny in ((x, y + 1), (x, y - 1), (x - 1, y), (x + 1, y)):
                    if 0 <= nx < m and 0 <= ny < n and board[x][y] == "O":
                        dfs(nx, ny)
                        
            for x, y in starts:
                dfs(x, y)
            return visited
        
        edges = [(0, i) for i in range(n)] + [(i, 0) for i in range(1, m)] \
            + [(m - 1, i) for i in range(n)] + [(i, n - 1) for i in range(m - 1)]
            
        node_not_to_changed = list(map(list, search(edges)))
        for i in range(m):
            for j in range(n):
                if [i,j] not in node_not_to_changed:
                    board[i][j] = "X"
                    
        return board
        
        

# 官解 DFS
class Solution:
    def solve(self, board: List[List[str]]) -> None:
        if not board:
            return
        
        n, m = len(board), len(board[0])

        def dfs(x, y):
            if not 0 <= x < n or not 0 <= y < m or board[x][y] != 'O':
                return
            
            board[x][y] = "A"
            dfs(x + 1, y)
            dfs(x - 1, y)
            dfs(x, y + 1)
            dfs(x, y - 1)
        
        for i in range(n):
            dfs(i, 0)
            dfs(i, m - 1)
        
        for i in range(m - 1):
            dfs(0, i)
            dfs(n - 1, i)
        
        for i in range(n):
            for j in range(m):
                if board[i][j] == "A":
                    board[i][j] = "O"
                elif board[i][j] == "O":
                    board[i][j] = "X"

import collections
class Solution:
    def solve(self, board: List[List[str]]) -> None:
        if not board:
            return
        
        n, m = len(board), len(board[0])
        que = collections.deque()
        for i in range(n):
            if board[i][0] == "O":
                que.append((i, 0))
                board[i][0] = "A"
            if board[i][m - 1] == "O":
                que.append((i, m - 1))
                board[i][m - 1] = "A"
        for i in range(m - 1):
            if board[0][i] == "O":
                que.append((0, i))
                board[0][i] = "A"
            if board[n - 1][i] == "O":
                que.append((n - 1, i))
                board[n - 1][i] = "A"
        
        while que:
            x, y = que.popleft()
            for mx, my in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
                if 0 <= mx < n and 0 <= my < m and board[mx][my] == "O":
                    que.append((mx, my))
                    board[mx][my] = "A"
        
        for i in range(n):
            for j in range(m):
                if board[i][j] == "A":
                    board[i][j] = "O"
                elif board[i][j] == "O":
                    board[i][j] = "X"
        