# 1617_Count_Subtrees_With_Max_Distance_Between_Cities
# https://leetcode.cn/problems/count-subtrees-with-max-distance-between-cities/solution/tu-jie-on3-mei-ju-zhi-jing-duan-dian-che-am2n/

# 方法一：枚举子集+树的直径
# 前置知识：子集型回溯

# 前置知识：二叉树的直径
# 见 543. 二叉树的直径。
# 推荐从这题开始学习树形 DP。
# 简单来说就是枚举从往左儿子走的最长链和往右儿子走的最长链，这两条链可能会组成直径。枚举所有点作为 x 就能找到答案。
# 每个节点都需要返回「往左走的最长链长度和往右走的最长链长度的最大值」给父节点，这样父节点才知道往这边走的最长链的长度是多少。

# 前置知识：树的直径
# 树的直径可以用两次 DFS 或者树形 DP，我在 周赛 328 的第四题讲了树形 DP 的做法，在二叉树直径的基础上略作修改。

# 本题结合了 78 题和 1245 题：枚举城市的子集（子树），求这棵子树的直径。
# 需要注意的是，枚举的子集不一定是一棵树，可能是森林（多棵树，多个连通块）。
# 我们可以在计算树形 DP 的同时去统计访问过的点，看看是否与子集相等，只有相等才是一棵树。

class Solution:
    def countSubgraphsForEachDiameter(self, n: int, edges: List[List[int]]) -> List[int]:
        # 建树
        g = [[] for _ in range(n)]
        for x, y in edges:
            g[x - 1].append(y - 1)
            g[y - 1].append(x - 1)  # 编号改为从 0 开始

        ans = [0] * (n - 1)
        in_set = [False] * n
        def f(i: int) -> None:
            if i == n:
                vis = [False] * n
                diameter = 0
                for v, b in enumerate(in_set):
                    if not b: continue
                    # 求树的直径
                    def dfs(x: int) -> int:
                        nonlocal diameter
                        vis[x] = True
                        max_len = 0
                        for y in g[x]:
                            if not vis[y] and in_set[y]:
                                ml = dfs(y) + 1
                                diameter = max(diameter, max_len + ml)
                                max_len = max(max_len, ml)
                        return max_len
                    dfs(v)
                    break
                if diameter and vis == in_set:
                    ans[diameter - 1] += 1
                return
            
            # 不选城市 i
            f(i + 1)

            # 选城市  i
            in_set[i] = True
            f(i + 1)
            in_set[i] = False  # 恢复现场
            
        f(0)
        return ans
    
    
class Solution:
    def countSubgraphsForEachDiameter(self, n: int, edges: List[List[int]]) -> List[int]:
        # 建树
        g = [[] for _ in range(n)]
        for x, y in edges:
            g[x - 1].append(y - 1)
            g[y - 1].append(x - 1)  # 编号改为从 0 开始

        ans = [0] * (n - 1)
        
        #  二进制枚举
        for mask in range(3, 1 << n):
            if (mask & (mask - 1)) == 0:  # 需要至少两个点
                continue
            # 求树的直径
            vis = diameter = 0
            
            def dfs(x: int) -> int:
                nonlocal vis, diameter
                vis |= 1 << x  # 标记 x 访问过
                max_len = 0
                for y in g[x]:
                    if (vis >> y & 1) == 0 and mask >> y & 1:  # y 没有访问过且在 mask 中
                        ml = dfs(y) + 1
                        diameter = max(diameter, max_len + ml)
                        max_len = max(max_len, ml)
                return max_len
            dfs(mask.bit_length() - 1)  # 从一个在 mask 中的点开始递归
            if vis == mask:
                ans[diameter - 1] += 1
        return ans


class Solution:
    def countSubgraphsForEachDiameter(self, n: int, edges: List[List[int]]) -> List[int]:
        # 建树
        g = [[] for _ in range(n)]
        for x, y in edges:
            g[x - 1].append(y - 1)
            g[y - 1].append(x - 1)  # 编号改为从 0 开始

        # 计算树上任意两点的距离
        dis = [[0] * n for _ in range(n)]
        def dfs(x: int, fa: int) -> None:
            for y in g[x]:
                if y != fa:
                    dis[i][y] = dis[i][x] + 1  # 自顶向下
                    dfs(y, x)
        for i in range(n):
            dfs(i, -1)  # 计算 i 到其余点的距离

        def dfs2(x: int, fa: int) -> int:
            # 能递归到这，说明 x 可以选
            cnt = 1  # 选 x
            for y in g[x]:
                if y != fa and \
                   (di[y] < d or di[y] == d and y > j) and \
                   (dj[y] < d or dj[y] == d and y > i):  # 满足这些条件就可以选
                    cnt *= dfs2(y, x)  # 每棵子树互相独立，采用乘法原理
            if di[x] + dj[x] > d:  # x 是可选点
                cnt += 1  # 不选 x
            return cnt
        
        ans = [0] * (n - 1)
        for i, di in enumerate(dis):
            for j in range(i + 1, n):
                dj = dis[j]
                d = di[j]
                ans[d - 1] += dfs2(i, -1)
        return ans