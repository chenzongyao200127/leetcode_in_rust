# 928_尽量减少恶意软件的传播_II
# https://leetcode.cn/problems/minimize-malware-spread-ii/description/
from collections import Counter
from typing import List


class UnionFind:
    def __init__(self, n: int):
        self.parent = list(range(n))
        self.size = [1] * n
        self.setCount = n

    def findset(self, x: int) -> int:
        if self.parent[x] != x:
            self.parent[x] = self.findset(self.parent[x])
        return self.parent[x]

    def unite(self, x: int, y: int) -> bool:
        rootX = self.findset(x)
        rootY = self.findset(y)
        if rootX != rootY:
            if self.size[rootX] < self.size[rootY]:
                rootX, rootY = rootY, rootX
            self.parent[rootY] = rootX
            self.size[rootX] += self.size[rootY]
            self.setCount -= 1
            return True
        return False

    def connected(self, x: int, y: int) -> bool:
        return self.findset(x) == self.findset(y)


class Solution:
    def minMalwareSpread(self, graph: List[List[int]], initial: List[int]) -> int:
        n = len(graph)
        initial.sort()

        min_size = float('inf')
        best_node = -1

        for node in initial:
            uf = UnionFind(n)
            # Build the union-find structure excluding the current node
            for i in range(n):
                if i == node:
                    continue
                for j in range(i + 1, n):
                    if j == node:
                        continue
                    if graph[i][j] == 1:
                        uf.unite(i, j)

            # Calculate the size of the malware spread if 'node' is removed
            infected_size = 0
            seen = set()
            for infected in initial:
                if infected == node:
                    continue
                root = uf.findset(infected)
                if root not in seen:
                    seen.add(root)
                    infected_size = max(infected_size, uf.size[root])

            # Update result if the current node leads to a smaller infected component
            if infected_size < min_size:
                min_size = infected_size
                best_node = node

        return best_node


s = Solution()
ans = s.minMalwareSpread([[1, 0, 0, 0, 0, 0, 0, 0, 0], [0, 1, 0, 0, 0, 0, 0, 0, 0], [0, 0, 1, 0, 1, 0, 1, 0, 0], [0, 0, 0, 1, 0, 0, 0, 0, 0], [
                         0, 0, 1, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 1, 0, 0, 0, 1, 0, 0], [0, 0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 0, 0, 1]], [0, 6, 4])

assert ans == 0


class Solution:
    def minMalwareSpread(self, graph: List[List[int]], initial: List[int]) -> int:
        st = set(initial)
        vis = [False] * len(graph)

        def dfs(x: int) -> int:
            """ Perform DFS to find the size of the component and track the unique infected node if present. """
            stack = [x]
            local_size = 0
            # -1 means no infected node found yet, -2 means multiple infected nodes
            unique_infected = -1

            while stack:
                node = stack.pop()
                if vis[node]:
                    continue
                vis[node] = True
                local_size += 1

                for neighbor, connected in enumerate(graph[node]):
                    if connected == 0:
                        continue
                    if neighbor in st:
                        if unique_infected != -2:
                            if unique_infected == -1:
                                unique_infected = neighbor
                            elif unique_infected != neighbor:
                                unique_infected = -2
                    elif not vis[neighbor]:
                        stack.append(neighbor)

            return local_size, unique_infected

        cnt = Counter()
        for i in range(len(graph)):
            if not vis[i] and i not in st:
                size, node_id = dfs(i)
                if node_id >= 0:
                    cnt[node_id] += size

        # Use a min function with a tuple where we prioritize the largest negative size (smallest size) and then the smallest node_id
        if cnt:
            return min((-size, node_id) for node_id, size in cnt.items())[1]
        return min(initial)
