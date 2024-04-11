# 684_冗余连接
# https://leetcode.cn/problems/redundant-connection/description/
from typing import List


class UnionFind:
    def __init__(self, n: int):
        self.parent = list(range(n))

    def find_set(self, x: int) -> int:
        if self.parent[x] != x:
            return self.find_set(self.parent[x])
        else:
            return x

    def unite(self, x: int, y: int) -> int:
        x = self.find_set(x)
        y = self.find_set(y)
        if x != y:
            self.parent[y] = x
        else:
            return False


class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        n = len(edges)
        uf = UnionFind(n+1)
        res = []
        for edge in edges:
            x = edge[0]
            y = edge[1]
            if uf.find_set(x) != uf.find_set(y):
                uf.unite(x, y)
            else:
                res = [x, y]
        return res
