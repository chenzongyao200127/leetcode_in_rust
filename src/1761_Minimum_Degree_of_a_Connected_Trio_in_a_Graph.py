# 1761_Minimum_Degree_of_a_Connected_Trio_in_a_Graph
# https://leetcode.cn/problems/minimum-degree-of-a-connected-trio-in-a-graph/

# 给你一个无向图，整数 n 表示图中节点的数目，edges 数组表示图中的边，其中 edges[i] = [ui, vi] ，表示 ui 和 vi 之间有一条无向边。
# 一个 连通三元组 指的是 三个 节点组成的集合且这三个点之间 两两 有边。
# 连通三元组的度数 是所有满足此条件的边的数目：一个顶点在这个三元组内，而另一个顶点不在这个三元组内。
# 请你返回所有连通三元组中度数的 最小值 ，如果图中没有连通三元组，那么返回 -1 。


from typing import List
# 超时
class Solution:
    def minTrioDegree(self, n: int, edges: List[List[int]]) -> int:
        degres = [0] * (n + 1)
        graph = [[0] * (n + 1) for _ in range(n + 1)]
        for par in edges:
            x = par[0]
            y = par[1]
            degres[x] += 1
            degres[y] += 1
            graph[x][y] += 1
            graph[y][x] += 1
        
        three_nodes = set()
        for x in range(n + 1):
            for y in range(n + 1):
                if graph[x][y] == 1:
                    for z in range(n + 1):
                        if graph[y][z] == 1:
                            for j in range(n + 1):
                                if graph[z][j] == 1 and j == x:
                                    tmp = [x, y, z]
                                    tmp.sort()
                                    nods = (tmp[0], tmp[1], tmp[2])
                                    three_nodes.add(nods)
        ans = float('inf')
        for (x,y,z) in three_nodes:
            ans = min(ans, degres[x] + degres[y] + degres[z] - 6)
        
        if ans == float('inf'):
            return -1
        else:
            return ans

# 超时
class Solution:
    def minTrioDegree(self, n: int, edges: List[List[int]]) -> int:
        degres = [0] * (n + 1)
        graph = [set() for _ in range(n + 1)]
        for par in edges:
            x = par[0]
            y = par[1]
            degres[x] += 1
            degres[y] += 1
            graph[x].add(y)
            graph[y].add(x)
        
        three_nodes = set()
        for x in range(n + 1):
            for y in graph[x]:
                for z in graph[y]:
                    if x in graph[z]:
                        tmp = [x, y, z]
                        tmp.sort()
                        nods = (tmp[0], tmp[1], tmp[2])
                        three_nodes.add(nods)
                        
        ans = float('inf')
        for (x,y,z) in three_nodes:
            ans = min(ans, degres[x] + degres[y] + degres[z] - 6)
        
        if ans == float('inf'):
            return -1
        else:
            return ans
        
        
        
# 优化这段代码的关键是减少不必要的计算和循环。下面是对代码的几个优化建议：

# 1. **数据结构选择**：我们可以直接用一个集合来存储边，这样在检查三元组的存在性时会更加高效。
# 2. **减少重复检查**：由于在寻找三元组时我们可能多次检查相同的三元组，使用集合可以避免这一情况。
# 3. **直接在循环中更新答案**：在寻找三元组时，我们可以直接更新答案，而不是将其存储起来。

# 下面是优化后的代码：


class Solution:
    def minTrioDegree(self, n: int, edges: List[List[int]]) -> int:
        degree = [0] * (n + 1)
        graph = [set() for _ in range(n + 1)]
        edge_set = set()

        for x, y in edges:
            degree[x] += 1
            degree[y] += 1
            graph[x].add(y)
            graph[y].add(x)
            edge_set.add((min(x, y), max(x, y))) # 保证无重复

        ans = float('inf')
        
        for x, y in edge_set:
            for z in graph[x] & graph[y]:
                ans = min(ans, degree[x] + degree[y] + degree[z] - 6)

        return ans if ans != float('inf') else -1
        


                        
s = Solution()
ans = s.minTrioDegree(n = 6, edges = [[1,2],[1,3],[3,2],[4,1],[5,2],[3,6]])
print(ans)

              
s = Solution()
ans = s.minTrioDegree(n = 7, edges = [[1,3],[4,1],[4,3],[2,5],[5,6],[6,7],[7,5],[2,6]])
print(ans)
