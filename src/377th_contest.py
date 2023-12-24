# 374th_contest
from typing import List

class Solution:
    def numberGame(self, nums: List[int]) -> List[int]:
        nums.sort()
        arr = []
        for _ in range(0, len(nums)//2):
            arr.append(nums[1])
            arr.append(nums[0])
            nums = nums[2:]
        
        return arr
    
class Solution:
    def maximizeSquareArea(self, m: int, n: int, hFences: List[int], vFences: List[int]) -> int:
        hFences.sort()
        vFences.sort()
        a = [1] + hFences + [m]
        b = [1] + vFences + [n]
        MOD = 10 ** 9 + 7
        
        rows = set()
        for i in range(len(a)-1):
            for j in range(i + 1, len(a)):
                rows.add(a[j] - a[i])
        
        cols = set()
        for i in range(len(b)-1):
            for j in range(i + 1, len(b)):
                cols.add(b[j] - b[i])

        union_set = rows & cols
        if not union_set:
            return -1
        else:
            x = max(union_set)
            return (x * x) % MOD       
        

class Solution:
    def minimumCost(self, source: str, target: str, original: List[str], changed: List[str], cost: List[int]) -> int:
        import string

        graph = {c1: {c2: float('inf') for c2 in string.ascii_lowercase} for c1 in string.ascii_lowercase}
        
        for c in string.ascii_lowercase:
            graph[c][c] = 0

        for o, c, co in zip(original, changed, cost):
            if co < graph[o][c]:
                graph[o][c] = co

        for k in string.ascii_lowercase:
            for i in string.ascii_lowercase:
                for j in string.ascii_lowercase:
                    graph[i][j] = min(graph[i][j], graph[i][k] + graph[k][j])

        total_cost = 0
        for s, t in zip(source, target):
            if graph[s][t] == float('inf'):
                return -1 
            total_cost += graph[s][t]

        return total_cost