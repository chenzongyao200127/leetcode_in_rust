# 547_Number_of_Provinces
# https://leetcode.cn/problems/number-of-provinces/

# 有 n 个城市，其中一些彼此相连，另一些没有相连。
# 如果城市 a 与城市 b 直接相连，且城市 b 与城市 c 直接相连，那么城市 a 与城市 c 间接相连。

# 省份 是一组直接或间接相连的城市，组内不含其他没有相连的城市。

# 给你一个 n x n 的矩阵 isConnected ，其中 isConnected[i][j] = 1 表示第 i 个城市和第 j 个城市直接相连，
# 而 isConnected[i][j] = 0 表示二者不直接相连。

# 返回矩阵中 省份 的数量。

from typing import List


def findCircleNum(isConnected: List[List[int]]) -> int:
    n = len(isConnected)
    provinces = [[0]]
    def findCityInProvices(city, provinces):
        print(provinces)
        for i in range(len(provinces)):
            if city in provinces[i]:
                return i
        return -1
    
    def merge_lists_with_common_elements(lst):
        result = []
        while lst:
            first, *lst = lst
            first = set(first)

            lf = -1
            while len(lst) != lf:
                lf = len(lst)
                rest = []
                for other in lst:
                    if first.intersection(other):  
                        first |= set(other)
                    else:
                        rest.append(other)
                lst = rest

            result.append(list(first))
        return result
    
    for i in range(n):
        idx = findCityInProvices(i, provinces)
        if idx == -1:
            tmp_provinces = [i]
            for j in range(i, n):
                if isConnected[i][j] == 1:
                    tmp_provinces.append(j)
            provinces.append(tmp_provinces[:])
        else:
            for j in range(i, n):
                if isConnected[i][j] == 1:
                    if j not in provinces[idx]:
                        provinces[idx].append(j)
    
    return len(merge_lists_with_common_elements(provinces))

print(findCircleNum([[1,0,0,1],[0,1,1,0],[0,1,1,1],[1,0,1,1]]))



# 当两个列表有共享的值时，我们可以使用集合（set）的并集来将它们合并。这是一个示例代码：
def merge_lists_with_common_elements(provinces):
    result = []
    while provinces:
        # first, *provinces = provinces 这种语法在 Python 中被称为列表解包（list unpacking）。
        # 在这个例子中，first 会被赋值为 provinces 列表的第一个元素，
        # 然后 *provinces 会获取列表剩余的部分。注意，这会修改原来的 provinces 变量。
        first, *provinces = provinces
        first = set(first)

        lf = -1
        while len(provinces) != lf:
            lf = len(provinces)
            rest = []
            for other in provinces:
                # 然后遍历 provinces 中的其他每个列表。
                # 对于每个 other 列表，如果它与 first 有交集（即它们有共享的元素），
                # 就将 other 的元素添加到 first 中。否则，就将 other 添加到 rest 中。
                if first.intersection(other): 
                    # 如果存在交集（即 if 条件为 True），那么 first |= set(other) 这行代码将会执行。
                    # 这行代码使用了 |= 操作符，它是一个集合特有的 "in-place union" 操作符。
                    # 它将 other 集合中的所有元素添加到 first 集合中，如果这些元素在 first 中尚不存在的话。 
                    first |= set(other)
                else:
                    rest.append(other)
            # 将 rest 赋值给 provinces，以便在下一次外部的 while 循环中处理 rest 中的列表。
            provinces = rest

        result.append(list(first))
    return result

lists = [[0, 3], [1, 1, 2, 3], [4, 5]]
print(merge_lists_with_common_elements(lists))  # 输出：[[0, 1, 2, 3], [4, 5]]

# 并查集（Union-Find）是一种树形的数据结构，用于处理一些不交集（Disjoint Sets）的合并及查询问题。
# 有一个联合-查找算法（union-find algorithm）定义了两个用于此数据结构的操作：

# Find：确定元素属于哪一个子集。它可以被用来确定两个元素是否属于同一子集。
# Union：将两个子集合并成同一个集合。

# 因为它支持在近乎常数时间内做出这些操作，所以并查集在一些算法中非常有用，比如 Kruskal's 算法。

# 数据结构实现
# 并查集通过数组实现，数组的每个索引代表每个元素，数组元素的值代表该元素的父节点。
# 如果一个元素的父节点就是它自己，那么这个元素就是集合的代表。

# 下面我们以一个简单的例子来说明这个数据结构：

# 考虑有这样一组元素 {0, 1, 2, 3, 4, 5, 6, 7, 8, 9} ，我们通过一些操作，
# 使得它们的集合划分为 {0, 1, 2}, {3, 4}, {5, 6, 7}, {8}, {9}。

# 首先，初始化时，每个元素自己就是一个独立的集合，父节点就是它自己：

# 元素  	0	1	2	3	4	5	6	7	8	9
# 父节点	0	1	2	3	4	5	6	7	8	9

# 然后我们执行 Union(0, 1), Union(1, 2), Union(3, 4), Union(5, 6), Union(6, 7) ，
# 集合变为了 {0, 1, 2}, {3, 4}, {5, 6, 7}, {8}, {9}。

# 元素  	0	1	2	3	4	5	6	7	8	9
# 父节点	0	0	1	3	3	5	5	6	8	9

# 接着，我们可以通过 Find 操作来检查任意两个元素是否属于同一集合。
# 例如 Find(0, 2) 和 Find(5, 7) 返回 True ， Find(0, 3) 返回 False。
# 这是并查集的基础操作，然而在实际应用中，为了提高效率，
# 我们还会使用路径压缩（Path Compression）和按秩合并（Union by Rank）等优化手段。


# 方法三：并查集
# 计算连通分量数的另一个方法是使用并查集。
# 初始时，每个城市都属于不同的连通分量。遍历矩阵 isConnected，如果两个城市之间有相连关系，则它们属于同一个连通分量，对它们进行合并。

# 遍历矩阵 isConnected的全部元素之后，计算连通分量的总数，即为省份的总数。
class Solution:
    def findCircleNum(self, isConnected: List[List[int]]) -> int:
        # find 函数用于查找节点的根节点
        def find(index: int) -> int:
            # 如果节点的父节点不是自己，说明它不是根节点，需要向上查找
            if parent[index] != index:
                # 递归查找父节点，并进行路径压缩，让当前节点直接指向根节点，降低查找效率
                parent[index] = find(parent[index])
            # 返回最终找到的根节点
            return parent[index]
        
        # union 函数用于将两个节点合并为同一集合，即让一个节点的根节点变为另一个节点的根节点
        def union(index1: int, index2: int):
            parent[find(index1)] = find(index2)
        
        # 获取城市（节点）的数量
        cities = len(isConnected)
        # 初始化每个节点的父节点为其自身，即每个城市都是一个独立的省份
        parent = list(range(cities))
        
        # 遍历每一对城市，如果两个城市间有直接的道路相连，则将他们合并为同一集合（省份）
        for i in range(cities):
            for j in range(i + 1, cities):
                if isConnected[i][j] == 1:
                    union(i, j)
        
        # 遍历每个城市，如果城市的父节点是其自身，则说明该城市是一个省份的首府，统计这样的城市数量，即为总的省份数量
        provinces = sum(parent[i] == i for i in range(cities))
        
        return provinces
    
# DFS
class Solution:
    def findCircleNum(self, isConnected: List[List[int]]) -> int:
        def dfs(i: int):
            for j in range(cities):
                if isConnected[i][j] == 1 and j not in visited:
                    visited.add(j)
                    dfs(j)
        
        cities = len(isConnected)
        visited = set()
        provinces = 0

        for i in range(cities):
            if i not in visited:
                dfs(i)
                provinces += 1
        
        return provinces
    
# BFS
import collections

class Solution:
    def findCircleNum(self, isConnected: List[List[int]]) -> int:
        cities = len(isConnected)
        visited = set()
        provinces = 0
        
        for i in range(cities):
            if i not in visited:
                Q = collections.deque([i])
                while Q:
                    j = Q.popleft()
                    visited.add(j)
                    for k in range(cities):
                        if isConnected[j][k] == 1 and k not in visited:
                            Q.append(k)
                provinces += 1
        
        return provinces