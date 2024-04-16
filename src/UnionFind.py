# 并查集模板
from typing import List


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


class UnionFind:
    def __init__(self, n):
        self.parent = list(range(n))
        self.rank = [1] * n
        self.size = [1] * n

    def find(self, x):
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x, y):
        rootX = self.find(x)
        rootY = self.find(y)
        if rootX != rootY:
            if self.rank[rootX] > self.rank[rootY]:
                self.parent[rootY] = rootX
                self.size[rootX] += self.size[rootY]
            elif self.rank[rootX] < self.rank[rootY]:
                self.parent[rootX] = rootY
                self.size[rootY] += self.size[rootX]
            else:
                self.parent[rootY] = rootX
                self.size[rootX] += self.size[rootY]
                self.rank[rootX] += 1


class UnionFind:
    def __init__(self, n):
        self.parent = list(range(n))
        self.rank = [1] * n
        self.size = [1] * n

    def find(self, x):
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x, y):
        rootX = self.find(x)
        rootY = self.find(y)
        if rootX != rootY:
            if self.rank[rootX] > self.rank[rootY]:
                self.parent[rootY] = rootX
                self.size[rootX] += self.size[rootY]
            elif self.rank[rootX] < self.rank[rootY]:
                self.parent[rootX] = rootY
                self.size[rootY] += self.size[rootX]
            else:
                self.parent[rootY] = rootX
                self.size[rootX] += self.size[rootY]
                self.rank[rootX] += 1


def minMalwareSpread(graph, initial):
    n = len(graph)
    uf = UnionFind(n)

    # 构建并查集
    for i in range(n):
        for j in range(i + 1, n):
            if graph[i][j] == 1:
                uf.union(i, j)

    # 计算每个连通分量中的感染节点数
    infected_by = {i: set() for i in range(n)}
    for node in initial:
        root = uf.find(node)
        infected_by[root].add(node)

    # 计算移除每个节点后的影响
    result = None
    min_infected = float('inf')

    for node in sorted(initial):  # 排序以处理返回最小索引的要求
        # 计算如果移除这个节点会影响的感染节点数
        contribution = 0
        seen = set()

        for other in initial:
            if other == node:
                continue
            root = uf.find(other)
            if root not in seen:
                if len(infected_by[root]) == 1:  # 只有当一个连通分量中只有一个感染源时，移除当前节点才影响该连通分量
                    contribution += uf.size[root]
                seen.add(root)

        # 更新最小感染数和相应的节点
        if contribution < min_infected:
            min_infected = contribution
            result = node

    return result


# 示例
graph = [[1, 1, 0], [1, 1, 0], [0, 0, 1]]
initial = [0, 1]
print(minMalwareSpread(graph, initial))  # 输出应为 0 或 1
