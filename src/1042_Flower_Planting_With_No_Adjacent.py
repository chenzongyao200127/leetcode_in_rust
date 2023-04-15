# 1042_Flower_Planting_With_No_Adjacent
# https://leetcode.cn/problems/flower-planting-with-no-adjacent/

class Solution:
    def gardenNoAdj(self, n: int, paths: List[List[int]]) -> List[int]:
        adj = [[] for _ in range(n)]
        for path in paths:
            adj[path[0] - 1].append(path[1] - 1)
            adj[path[1] - 1].append(path[0] - 1)
            
        ans = [0] * n
        for i in range(n):
            colored = [False] * 5
            
            for vertex in adj[i]:
                colored[ans[vertex]] = True
                
            for j in range(1, 5):
                if not colored[j]:
                    ans[i] = j
                    break
                
        return ans



class Solution:
    def gardenNoAdj(self, n: int, paths: List[List[int]]) -> List[int]:
        graph = [[] for _ in range(n)]
        for x, y in paths:
            x -= 1
            y -= 1
            graph[x].append(y)
            graph[y].append(x)
            
        ans = [0] * n
        for i, v in enumerate(graph):
            temp = [1,2,3,4]
            for x in v:
                if ans[x] and ans[x] in temp:
                    temp.remove(ans[x])
            ans[i] = temp[0]
        return ans