# 399_Evaluate_Division
# https://leetcode.cn/problems/evaluate-division/

from typing import List

# 给你一个变量对数组 equations 和一个实数值数组 values 作为已知条件，
# 其中 equations[i] = [Ai, Bi] 和 values[i] 共同表示等式 Ai / Bi = values[i] 。每个 Ai 或 Bi 是一个表示单个变量的字符串。

# 另有一些以数组 queries 表示的问题，其中 queries[j] = [Cj, Dj] 表示第 j 个问题，请你根据已知条件找出 Cj / Dj = ? 的结果作为答案。

# 返回 所有问题的答案 。如果存在某个无法确定的答案，则用 -1.0 替代这个答案。
# 如果问题中出现了给定的已知条件中没有出现的字符串，也需要用 -1.0 替代这个答案。

# 注意：输入总是有效的。你可以假设除法运算中不会出现除数为 0 的情况，且不存在任何矛盾的结果。

# 示例 1：
# 输入：equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
# 输出：[6.00000,0.50000,-1.00000,1.00000,-1.00000]
# 解释：
# 条件：a / b = 2.0, b / c = 3.0
# 问题：a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
# 结果：[6.0, 0.5, -1.0, 1.0, -1.0 ]


# 示例 2：
# 输入：equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
# 输出：[3.75000,0.40000,5.00000,0.20000]


# 示例 3：
# 输入：equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
# 输出：[0.50000,2.00000,-1.00000,-1.00000]
 


# 拓扑排序是一个有向无环图（DAG, Directed Acyclic Graph）的所有顶点的线性序列，满足以下两个条件：

# 每个顶点出现且只出现一次；
# 若存在一条从顶点 A 到顶点 B 的路径，那么在序列中顶点 A 出现在顶点 B 的前面。
# 拓扑排序常用于任务调度和课程表等问题中，解决的核心问题是依赖问题，比如任务B依赖于任务A，那么在执行任务时，需要先执行任务A，再执行任务B。

# 算法步骤：

# 从 DAG 图中选择一个 没有前驱（即入度为0）的顶点并输出。
# 从图中删除该顶点和所有以它为起点的有向边。
# 重复步骤1和2，直到当前的 DAG 图为空或当前图中不存在无前驱的顶点为止。后一种情况说明有向图中必然存在环。
# 常见算法：

# Kahn算法： 该算法的主要思想是，每次从图中删除入度为 0 的节点，直到没有入度为 0 的节点。如果还有节点没有被删除，那么图中存在环，否则就已经生成了一种拓扑排序。

# 深度优先搜索： 对于图中的任意一个节点，它在搜索过程结束后，都会生成一个列表，列表中的节点顺序就是拓扑排序的一种可能结果。
# 使用深度优先搜索进行拓扑排序需要对存在环的情况进行特殊处理。

# 注意： 拓扑排序的结果可能不唯一，因为可能存在多个合理的任务执行顺序。此外，只有当图中没有环时，才可能存在拓扑排序。

# 拓扑排序是对有向无环图（DAG）的一种重要操作，它的主要应用场景包括任务调度、代码编译等，都需要利用拓扑排序来确定执行或处理的顺序。

# 输入：equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
# 输出：[3.75000,0.40000,5.00000,0.20000]

# 一边查询一边修改结点指向是并查集的特色。
class Solution:
    def calcEquation(self, equations: List[List[str]], values: List[float], queries: List[List[str]]) -> List[float]:
        nodes = []
        for equation in equations:
            if equation[0] not in nodes:
                nodes.append(equation[0])
            if equation[1] not in nodes:
                nodes.append(equation[1])
                
        n = len(nodes)
        map = [[0] * n for _ in range(n)]
        def findIdxByEqution(name):
            for i in range(n):
                if nodes[i] == name:
                    return i
            return -1
        
        for i in range(len(values)):
            x = findIdxByEqution(equations[i][0])
            y = findIdxByEqution(equations[i][1])
            val = values[i]
            map[x][y] = val
            map[y][x] = 1/val
        
        def dfs(s, t, val):
            if s == t:
                return val
            
            for i in range(n):
                if map[s][i] != 0:
                    tmp = map[s][i]
                    map[s][i] = 0
                    res = dfs(i, t, val * tmp)
                    map[s][i] = tmp
                    if res != -1:
                        return res
                    
            return -1
        
        ans = []
        for query in queries:
            x = findIdxByEqution(query[0])
            y = findIdxByEqution(query[1])
            if x == -1 or y == -1:
                ans.append(-1.0)
            else:
                ans.append(dfs(x,y,1.0))
            
        return ans
        