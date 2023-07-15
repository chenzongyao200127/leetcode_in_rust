# 685_Redundant_Connection_II
# https://leetcode.cn/problems/redundant-connection-ii/

# 在本问题中，有根树指满足以下条件的 有向 图。该树只有一个根节点，所有其他节点都是该根节点的后继。
# 该树除了根节点之外的每一个节点都有且只有一个父节点，而根节点没有父节点。

# 输入一个有向图，该图由一个有着 n 个节点（节点值不重复，从 1 到 n）的树及一条附加的有向边构成。
# 附加的边包含在 1 到 n 中的两个不同顶点间，这条附加的边不属于树中已存在的边。

# 结果图是一个以边组成的二维数组 edges 。 每个元素是一对 [ui, vi]，用以表示 有向 图中连接顶点 ui 和顶点 vi 的边，其中 ui 是 vi 的一个父节点。

# 返回一条能删除的边，使得剩下的图是有 n 个节点的有根树。若有多个答案，返回最后出现在给定二维数组的答案。


# 我们首先初始化并查集，然后遍历每条边，对于每条边，我们判断这条边的终点是否已经有父节点，如果有，那么这条边就是多余的；
# 否则，我们判断这条边的起点和终点是否在同一个集合中，如果在，那么这条边也是多余的。
# 否则，我们就将这条边的终点加入到起点的集合中。

# 请注意，在初始化并查集时，我们将每个节点的父节点初始化为自己，而在遍历边时，我们将每个节点的父节点更新为这条边的起点。
# 这是因为在这个问题中，我们知道每个节点除了根节点外都有且只有一个父节点，所以我们可以直接将每个节点的父节点设置为这条边的起点。


# 使用并查集查找无向图中的环
# 并查集（Union-Find）是一种数据结构，用于处理一些不相交集合（Disjoint Sets）的合并及查询问题。常常在使用中以森林来表示。
# 并查集常用于解决连通性问题，例如检查一个无向图是否包含环。在无向图中，如果一个边的两个顶点指向的是同一个集合，那么添加这条边会在图中创建一个环。
# 以下是使用并查集检测无向图中是否存在环的步骤：

# 初始化并查集：为图中的每个顶点创建一个集合，使得每个集合都只包含一个元素（即该顶点自身）。
# 遍历图中的每条边。对于每条边，使用并查集的“查找”操作，查找两个顶点所在的集合。
# 如果两个顶点在同一个集合中，那么这条边会创建一个环，因此图中存在环。
# 如果两个顶点不在同一个集合中，那么合并这两个集合。
# 如果遍历完所有的边都没有发现环，那么图中不存在环。
class UnionFind:
    def __init__(self, n):
        self.parent = list(range(n))

    def find(self, x):
        if x != self.parent[x]:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x, y):
        self.parent[self.find(x)] = self.find(y)

def contains_cycle(graph):
    uf = UnionFind(len(graph))
    for u, v in graph:
        if uf.find(u) == uf.find(v):
            return True
        uf.union(u, v)
    return False

# 对于有向图，我们不能直接使用并查集来检测环，因为边的方向性会导致并查集无法正确地识别环。
# 在有向图中，我们通常使用深度优先搜索（DFS）或者拓扑排序来检测环。
# 以下是使用深度优先搜索检测有向图中是否存在环的步骤：
# 从任意一个节点开始，进行深度优先搜索。
# 在搜索过程中，我们用两种颜色来标记节点：灰色和黑色。一开始，所有节点都是白色。当我们首次访问一个节点时，我们将其标记为灰色。
# 当我们访问完一个节点的所有邻居后，我们将其标记为黑色。
# 如果在搜索过程中，我们遇到了一个灰色节点，那么这意味着我们找到了一个环。
# 以下是一个使用 Python 实现的示例：

def contains_cycle(graph):
    WHITE, GRAY, BLACK = 0, 1, 2
    color = {node: WHITE for node in graph}

    def dfs(node):
        color[node] = GRAY
        for neighbor in graph[node]:
            if color[neighbor] == GRAY:
                return True
            if color[neighbor] == WHITE and dfs(neighbor):
                return True
        color[node] = BLACK
        return False

    for node in graph:
        if color[node] == WHITE and dfs(node):
            return True
    return False

