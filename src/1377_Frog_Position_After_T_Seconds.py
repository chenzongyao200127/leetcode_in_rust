# 1377. Frog Position After T Seconds
# https://leetcode.cn/problems/frog-position-after-t-seconds/

class Solution:
    def frogPosition(self, n: int, edges: List[List[int]], t: int, target: int) -> float:
        G = [[] for i in range(n + 1)]
        for i, j in edges:
            G[i].append(j)
            G[j].append(i)
        seen = [0] * (n + 1)

        def dfs(i, t):
            nxt = len(G[i])
            if i > 1:
                nxt -= 1
            if nxt == 0 or t == 0:
                return 1.0 if i == target else 0.0
            seen[i] = 1
            for j in G[i]:
                if not seen[j]:
                    p = dfs(j, t - 1)
                    if p > 0:
                        return p / nxt
            return 0.0

        return dfs(1, t)