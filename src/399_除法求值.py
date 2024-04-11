# 399_除法求值
# https://leetcode.cn/problems/evaluate-division/description/

from collections import defaultdict, deque
from typing import List


# class Solution:
#     def calcEquation(self, equations: List[List[str]], values: List[float], queries: List[List[str]]) -> List[float]:
#         nodes_set = set()
#         for edge in equations:
#             nodes_set.add(edge[0])
#             nodes_set.add(edge[1])

#         n = len(nodes_set)
#         nodes = list(nodes_set)

#         idx2node = {}
#         node2idx = {}
#         for i in range(n):
#             idx2node[i] = nodes[i]
#             node2idx[nodes[i]] = i

#         parents = [x for x in range(n)]
#         parents_value = [1] * n
#         g = [[1.0 for _ in range(n)] for _ in range(n)]

#         def find_set(x):
#             if parents[x] != x:
#                 return find_set(parents[x])
#             else:
#                 return x

#         def unite(x, y):
#             if find_set(x) == find_set(y):
#                 return False
#             else:
#                 parents[y] = find_set(x)
#                 return True

#         for i in range(len(equations)):
#             x = equations[i][0]
#             y = equations[i][1]
#             value = float(values[i])
#             g[node2idx[x]][node2idx[y]] = value
#             g[node2idx[y]][node2idx[x]] = 1/value
#             unite(node2idx[x], node2idx[y])
#             parents_value[node2idx[y]] = value * \
#                 parents_value[node2idx[x]]

#         res = []
#         for query in queries:
#             a = query[0]
#             b = query[1]
#             print(res)
#             if a not in nodes_set or b not in nodes_set:
#                 res.append(-1.0)
#                 continue
#             if a == b:
#                 res.append(1.0)
#                 continue
#             print(idx2node[find_set(node2idx[a])])
#             print(idx2node[find_set(node2idx[b])])
#             if find_set(node2idx[a]) != find_set(node2idx[b]):
#                 res.append(-1.0)
#             else:
#                 res.append(parents_value[node2idx[b]] /
#                            parents_value[node2idx[a]])

#         return res

from typing import List

# 加权并查集


class Solution:
    def calcEquation(self, equations: List[List[str]], values: List[float], queries: List[List[str]]) -> List[float]:
        parent = {}
        rank = {}
        value = {}

        def find(x):
            if parent[x] != x:
                orig = parent[x]
                parent[x] = find(parent[x])
                value[x] *= value[orig]
            return parent[x]

        def union(x, y, ratio):
            rootX = find(x)
            rootY = find(y)
            if rootX != rootY:
                if rank[rootX] > rank[rootY]:
                    parent[rootY] = rootX
                    value[rootY] = value[x] * ratio / value[y]
                elif rank[rootX] < rank[rootY]:
                    parent[rootX] = rootY
                    value[rootX] = value[y] / (value[x] * ratio)
                else:
                    parent[rootY] = rootX
                    value[rootY] = value[x] * ratio / value[y]
                    rank[rootX] += 1

        # Initialize the union-find structure
        for (dividend, divisor), val in zip(equations, values):
            if dividend not in parent:
                parent[dividend] = dividend
                rank[dividend] = 0
                value[dividend] = 1.0
            if divisor not in parent:
                parent[divisor] = divisor
                rank[divisor] = 0
                value[divisor] = 1.0
            union(dividend, divisor, val)

        # Process each query
        results = []
        for dividend, divisor in queries:
            if dividend not in parent or divisor not in parent:
                results.append(-1.0)
            elif find(dividend) != find(divisor):
                results.append(-1.0)
            else:
                results.append(value[divisor] / value[dividend])
        return results


s = Solution()
ans = s.calcEquation(equations=[["a", "e"], ["b", "e"]], values=[
                     4.0, 3.0], queries=[["a", "b"], ["e", "e"], ["x", "x"]])
print(ans)


# 这个思路更符合直觉
class Solution:
    def calcEquation(self, equations: List[List[str]], values: List[float], queries: List[List[str]]) -> List[float]:
        graph = defaultdict(dict)

        # 构建图
        for (dividend, divisor), value in zip(equations, values):
            graph[dividend][divisor] = value
            graph[divisor][dividend] = 1 / value

        def bfs(start, end):
            if start not in graph or end not in graph:
                return -1.0
            queue = deque([(start, 1.0)])
            visited = set()
            while queue:
                current, cur_product = queue.popleft()
                if current == end:
                    return cur_product
                visited.add(current)
                for neighbor in graph[current]:
                    if neighbor not in visited:
                        queue.append((neighbor, cur_product *
                                     graph[current][neighbor]))
            return -1.0

        # 处理查询
        results = []
        for dividend, divisor in queries:
            if dividend == divisor:
                if dividend in graph:
                    results.append(1.0)
                else:
                    results.append(-1.0)
            else:
                results.append(bfs(dividend, divisor))

        return results
