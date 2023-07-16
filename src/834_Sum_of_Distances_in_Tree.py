# 834_Sum_of_Distances_in_Tree
# https://leetcode.cn/problems/sum-of-distances-in-tree/

# 给定一个无向、连通的树。树中有 n 个标记为 0...n-1 的节点以及 n-1 条边 。
# 给定整数 n 和数组 edges ， edges[i] = [ai, bi]表示树中的节点 ai 和 bi 之间有一条边。
# 返回长度为 n 的数组 answer ，其中 answer[i] 是树中第 i 个节点与所有其他节点之间的距离之和。

import collections
from typing import List

class Solution:
    def sumOfDistancesInTree(self, n: int, edges: List[List[int]]) -> List[int]:
        graph = collections.defaultdict(list)
        count = [1] * n  # 初始化所有节点的子节点数量为1（包括节点自身）
        res = [0] * n  # 初始化结果列表

        # 根据边的信息创建邻接表，表示树的结构
        for u, v in edges:
            graph[u].append(v)
            graph[v].append(u)

        # 深度优先搜索（dfs）函数，用于计算从根节点（默认为0号节点）到其他所有节点的距离之和，并计算每个节点的子节点数量
        def dfs(node = 0, parent = None):
            for child in graph[node]:
                if child != parent:  # 不处理父节点
                    dfs(child, node)  # 递归处理子节点
                    count[node] += count[child]  # 将子节点的子节点数量加到当前节点上
                    res[node] += res[child] + count[child]  # 计算当前节点到其他所有节点的距离之和

        # 第二次深度优先搜索，利用已经获得的count和res信息，计算每个节点到其他所有节点的距离之和
        def dfs2(node = 0, parent = None):
            for child in graph[node]:
                if child != parent:  # 不处理父节点
                    # 利用父节点的结果和子节点数量，计算子节点到其他所有节点的距离之和
                    res[child] = res[node] - count[child] + n - count[child]
                    dfs2(child, node)  # 递归处理子节点

        dfs()  # 第一次深度优先搜索，初始化count和res数组
        dfs2()  # 第二次深度优先搜索，利用count和res数组，得到最终结果
        return res  # 返回所有节点到其他所有节点的距离之和
