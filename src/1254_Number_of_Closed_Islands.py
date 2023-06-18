# 1254_Number_of_Closed_Islands
# https://leetcode.cn/problems/number-of-closed-islands/solution/tong-ji-feng-bi-dao-yu-de-shu-mu-by-leet-ofh3/

class Solution:
    def closedIsland(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        ans = 0

        for i in range(m):
            for j in range(n):
                if grid[i][j] == 0:
                    qu = deque([(i, j)])
                    # 关键一步
                    grid[i][j] = 1
                    closed = True

                    while qu:
                        cx, cy = qu.popleft()
                        if cx == 0 or cy == 0 or cx == m - 1 or cy == n - 1:
                            closed = False
                        for nx, ny in [(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)]:
                            if 0 <= nx < m and 0 <= ny < n and grid[nx][ny] == 0:
                                grid[nx][ny] = 1
                                qu.append((nx, ny))
                    if closed:
                        ans += 1
        
        return ans
    
    

# DFS
class Solution:
    # 如果一个位置为 0，则以其为起始节点开始进行深度优先搜索，分别向左、上、右、下四个方向进行扩展。
    # 在深度优先搜索的过程中，每个搜索到的 0 都会被重新标记为 1，同时还需检测搜索出来的区域 A 是否「封闭」。
    def closedIsland(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        ans = 0

        def dfs(x: int, y: int) -> bool:
            if x < 0 or y < 0 or x >= m or y >= n:
                return False
            if grid[x][y] != 0:
                return True
            
            grid[x][y] = -1
            ret1, ret2, ret3, ret4 = dfs(x - 1, y), dfs(x + 1, y), dfs(x, y - 1), dfs(x, y + 1)
            return ret1 and ret2 and ret3 and ret4
        
        for i in range(m):
            for j in range(n):
                if grid[i][j] == 0 and dfs(i, j):
                    ans += 1
        
        return ans
    
    
    
# 并查集
class UnionFind:
    def __init__(self, n: int) -> None:
        self.parent = list(range(n))
        self.rank = [0] * n
    
    def uni(self, x: int, y: int) -> None:
        parent_ = self.parent
        rank_ = self.rank

        rootx, rooty = self.find(x), self.find(y)
        if rootx != rooty:
            if rank_[rootx] > rank_[rooty]:
                parent_[rooty] = rootx
            elif rank_[rootx] < rank_[rooty]:
                parent_[rootx] = rooty
            else:
                parent_[rooty] = rootx
                rank_[rootx] += 1

    def find(self, x: int) -> int:
        parent_ = self.parent

        if parent_[x] != x:
            parent_[x] = self.find(parent_[x])
        return parent_[x]

class Solution:
    def closedIsland(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        uf = UnionFind(m * n)

        for i in range(m):
            if grid[i][0] == 0:
                uf.uni(i * n, 0)
            if grid[i][n - 1] == 0:
                uf.uni(i * n + n - 1, 0)
        
        for j in range(1, n - 1):
            if grid[0][j] == 0:
                uf.uni(j, 0)
            if grid[m - 1][j] == 0:
                uf.uni((m - 1) * n + j, 0)
        
        for i in range(m):
            for j in range(n):
                if grid[i][j] == 0:
                    if i > 0 and grid[i - 1][j] == 0:
                        uf.uni(i * n + j, (i - 1) * n + j)
                    if j > 0 and grid[i][j - 1] == 0:
                        uf.uni(i * n + j, i * n + j - 1)

        cnt = set()
        for i in range(m):
            for j in range(n):
                if grid[i][j] == 0:
                    cnt.add(uf.find(i * n + j))
        
        total = len(cnt)
        if uf.find(0) in cnt:
            total -= 1
        return total
