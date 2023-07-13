# 684. Redundant Connection
# https://leetcode.cn/problems/redundant-connection/

# 树可以看成是一个连通且 无环 的 无向 图。

# 给定往一棵 n 个节点 (节点值 1～n) 的树中添加一条边后的图。添加的边的两个顶点包含在 1 到 n 中间，
# 且这条附加的边不属于树中已存在的边。图的信息记录于长度为 n 的二维数组 edges ，edges[i] = [ai, bi] 表示图中在 ai 和 bi 之间存在一条边。

# 请找出一条可以删去的边，删除后可使得剩余部分是一个有着 n 个节点的树。如果有多个答案，则返回数组 edges 中最后出现的那个。

# 输入: edges = [[1,2], [1,3], [2,3]]
# 输出: [2,3]

# 输入: edges = [[1,2], [2,3], [3,4], [1,4], [1,5]]
# 输出: [1,4]
from typing import List

# 方法一：并查集
# 在一棵树中，边的数量比节点的数量少 1。如果一棵树有 n 个节点，则这棵树有 n−1 条边。
# 这道题中的图在树的基础上多了一条附加的边，因此边的数量也是 n。

# 树是一个连通且无环的无向图，在树中多了一条附加的边之后就会出现环，因此附加的边即为导致环出现的边。

# 可以通过并查集寻找附加的边。初始时，每个节点都属于不同的连通分量。
# 遍历每一条边，判断这条边连接的两个顶点是否属于相同的连通分量。

# 如果两个顶点属于不同的连通分量，则说明在遍历到当前的边之前，
# 这两个顶点之间不连通，因此当前的边不会导致环出现，合并这两个顶点的连通分量。

# 如果两个顶点属于相同的连通分量，则说明在遍历到当前的边之前，
# 这两个顶点之间已经连通，因此当前的边导致环出现，为附加的边，将当前的边作为答案返回。


class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        def find(idx: int) -> int:
            if parent[idx] != idx:
                parent[idx] = find(parent[idx])
            return parent[idx]
        
        def union(idx1: int, idx2: int):
            parent[find(idx1)] = parent[find(idx2)]
        
        n = len(edges)
        parent = list(range(n+1))
        
        for x, y in edges:
            if find(x) != find(y):
                union(x, y)
            else:
                return [x,y]
        
        return []
        
        

class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        n = len(edges)
        parent = list(range(n + 1))

        def find(index: int) -> int:
            if parent[index] != index:
                parent[index] = find(parent[index])
            return parent[index]
        
        def union(index1: int, index2: int):
            parent[find(index1)] = find(index2)

        for node1, node2 in edges:
            if find(node1) != find(node2):
                union(node1, node2)
            else:
                return [node1, node2]
        
        return []

        