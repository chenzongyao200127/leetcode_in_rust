# 1631_Path_With_Minimum_Effort
# https://leetcode.cn/problems/path-with-minimum-effort/description/

# 并查集模板
class UnionFind:
    def __init__(self, n: int):
        # Initialize the parent list where each element is its own parent
        self.parent = list(range(n))
        # Initialize the size list, where each element's size is 1 initially
        self.size = [1] * n
        # Store the number of elements in the UnionFind
        self.n = n
        # Initialize the count of distinct sets
        self.setCount = n
    
    def findset(self, x: int) -> int:
        # Find the root of the set to which 'x' belongs
        # If 'x' is its own parent, it is the root
        if self.parent[x] == x:
            return x
        # Path compression: update the parent of 'x' to its root
        self.parent[x] = self.findset(self.parent[x])
        return self.parent[x]
    
    def unite(self, x: int, y: int) -> bool:
        # Find the roots of the sets to which 'x' and 'y' belong
        x, y = self.findset(x), self.findset(y)
        # If they are already in the same set, return False
        if x == y:
            return False
        # Union by size: make the larger set the parent
        if self.size[x] < self.size[y]:
            x, y = y, x
        # Merge the sets
        self.parent[y] = x
        # Update the size of the root of the merged set
        self.size[x] += self.size[y]
        # Decrease the number of distinct sets
        self.setCount -= 1
        return True
    
    def connected(self, x: int, y: int) -> bool:
        # Check if 'x' and 'y' are in the same set
        x, y = self.findset(x), self.findset(y)
        return x == y

from typing import List

class Solution:
    def minimumEffortPath(self, heights: List[List[int]]) -> int:
        m, n = len(heights), len(heights[0])
        edges = list()
        for i in range(m):
            for j in range(n):
                iden = i * n + j
                if i > 0:
                    edges.append((iden - n, iden, abs(heights[i][j] - heights[i - 1][j])))
                if j > 0:
                    edges.append((iden - 1, iden, abs(heights[i][j] - heights[i][j - 1])))
        
        edges.sort(key=lambda e: e[2])

        uf = UnionFind(m * n)
        ans = 0
        for x, y, v in edges:
            uf.unite(x, y)
            if uf.connected(0, m * n - 1):
                ans = v
                break
        
        return ans
    