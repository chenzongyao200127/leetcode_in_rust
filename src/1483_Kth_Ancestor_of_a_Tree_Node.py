# 1483_Kth_Ancestor_of_a_Tree_Node
# https://leetcode.cn/problems/kth-ancestor-of-a-tree-node/

# You are given a tree with n nodes numbered from 0 to n - 1 
# in the form of a parent array parent where parent[i] is the parent of ith node. 
# The root of the tree is node 0. Find the kth ancestor of a given node.

# The kth ancestor of a tree node is the kth node in the path from that node to the root node.

# Implement the TreeAncestor class:

# TreeAncestor(int n, int[] parent) 
# Initializes the object with the number of nodes in the tree and the parent array.
# int getKthAncestor(int node, int k) 
# return the kth ancestor of the given node node. If there is no such ancestor, return -1.

# Input
# ["TreeAncestor", "getKthAncestor", "getKthAncestor", "getKthAncestor"]
# [[7, [-1, 0, 0, 1, 1, 2, 2]], [3, 1], [5, 2], [6, 3]]
# Output
# [null, 1, 0, -1]

# Explanation
# TreeAncestor treeAncestor = new TreeAncestor(7, [-1, 0, 0, 1, 1, 2, 2]);
# treeAncestor.getKthAncestor(3, 1); // returns 1 which is the parent of 3
# treeAncestor.getKthAncestor(5, 2); // returns 0 which is the grandparent of 5
# treeAncestor.getKthAncestor(6, 3); // returns -1 because there is no such ancestor

# The queries must be answered efficiently to avoid time limit exceeded verdict.
# Use sparse table (dynamic programming application) to travel the tree upwards in a fast way.

class TreeAncestor:

    def __init__(self, n: int, parent: List[int]):
        m = n.bit_length() - 1
        pa = [[p] + [-1] * m for p in parent]
        for i in range(m):
            for x in range(n):
                if (p := pa[x][i]) != -1:
                    pa[x][i + 1] = pa[p][i]
        self.pa = pa

    def getKthAncestor(self, node: int, k: int) -> int:
        for i in range(k.bit_length()):
            if (k >> i) & 1:    # k 的二进制从低到高第 i 位是 1
                node = self.pa[node][i]
                if node < 0: break
        return node
    
    # 另一种写法，不断去掉 k 的最低位的 1
    def getKthAncestor2(self, node: int, k: int) -> int:
        while k and node != -1:
            lb = k & -k # 计算 k 的最低有效位（least significant bit, LSB），即 k 的二进制表示中最右边的 1。
            node = self.pa[node][lb.bit_length() - 1]
            k ^= lb # 异或运算删除 k 的最低有效位（LSB）。
        return node
    

# Your TreeAncestor object will be instantiated and called as such:
# obj = TreeAncestor(n, parent)
# param_1 = obj.getKthAncestor(node,k)


class TreeAncestor:

    def __init__(self, n: int, parent: List[int]):
        self.dpt = {}
        self.son = {}
        self.pa = {}
        rep = {x for x in parent}
        for i in range(n):
            if i not in rep:
                h = 0
                p = i 
                nums = []
                while p != -1:
                    self.pa[p] = [i,h]
                    nums.append(p)
                    h += 1
                    p = parent[p]
                self.son[i] = nums
    
        # print(self.son)
        # print(self.pa)      
             
    def getKthAncestor(self, node: int, k: int) -> int:
        a,h = self.pa[node]
        nums = self.son[a]
        if len(nums)-h <= k:return -1
        return nums[h+k]
    
    
    
# 【模板】最近公共祖先
class TreeAncestor:
    def __init__(self, edges: List[List[int]]):
        n = len(edges) + 1
        m = n.bit_length()
        g = [[] for _ in range(n)]
        for x, y in edges:  # 节点编号从 0 开始
            g[x].append(y)
            g[y].append(x)

        depth = [0] * n
        pa = [[-1] * m for _ in range(n)]
        def dfs(x: int, fa: int) -> None:
            pa[x][0] = fa
            for y in g[x]:
                if y != fa:
                    depth[y] = depth[x] + 1
                    dfs(y, x)
        dfs(0, -1)

        for i in range(m - 1):
            for x in range(n):
                if (p := pa[x][i]) != -1:
                    pa[x][i + 1] = pa[p][i]
        self.depth = depth
        self.pa = pa

    def get_kth_ancestor(self, node: int, k: int) -> int:
        for i in range(k.bit_length()):
            if (k >> i) & 1:  # k 二进制从低到高第 i 位是 1
                node = self.pa[node][i]
        return node

    # 返回 x 和 y 的最近公共祖先（节点编号从 0 开始）
    def get_lca(self, x: int, y: int) -> int:
        if self.depth[x] > self.depth[y]:
            x, y = y, x
        # 使 y 和 x 在同一深度
        y = self.get_kth_ancestor(y, self.depth[y] - self.depth[x])
        if y == x:
            return x
        for i in range(len(self.pa[x]) - 1, -1, -1):
            px, py = self.pa[x][i], self.pa[y][i]
            if px != py:
                x, y = px, py  # 同时上跳 2**i 步
        return self.pa[x][0]
