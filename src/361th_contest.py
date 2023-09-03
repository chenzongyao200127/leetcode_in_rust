# 360th_contest


class Solution:
    def countSymmetricIntegers(self, low: int, high: int) -> int:
        def get_digit_sum(num):
            return sum(int(digit) for digit in str(num))

        def is_symmetric(num):
            s = str(num)
            half_length = len(s) // 2
            return get_digit_sum(s[:half_length]) == get_digit_sum(s[half_length:])

        return sum(is_symmetric(num) for num in range(low, high + 1) if len(str(num)) % 2 == 0)
    
    
class Solution:
    def minimumOperations(self, num: str) -> int:
        def find_last_two_indices(target1, target2):
            idx2 = num.rfind(target2)
            if idx2 == -1:
                return -1, -1
            idx1 = num.rfind(target1, 0, idx2)
            return idx1, idx2

        def count_deletions(idx1, idx2):
            if idx1 == -1 or idx2 == -1:
                return float('inf')
            return len(num) - 1 - idx2 + idx2 - idx1 - 1

        if len(num) == 1:
            return 1 if num != "0" else 0

        single_zero = len(num) - 1 if '0' in num else float('inf')

        idx00 = find_last_two_indices('0', '0')
        idx25 = find_last_two_indices('2', '5')
        idx50 = find_last_two_indices('5', '0')
        idx75 = find_last_two_indices('7', '5')

        return min(single_zero, count_deletions(*idx00), count_deletions(*idx25), count_deletions(*idx50), count_deletions(*idx75), len(num))



from typing import List
class Solution:
    def countInterestingSubarrays(self, nums: List[int], modulo: int, k: int) -> int:      
        prefix = [0] * (len(nums) + 1)
        count = {0: 1}  
        res = 0

        for i in range(len(nums)):
            if nums[i] % modulo == k:
                prefix[i + 1] = prefix[i] + 1
            else:
                prefix[i + 1] = prefix[i]
            
            target = (prefix[i + 1] - k) % modulo
            res += count.get(target, 0)

            count[prefix[i + 1] % modulo] = count.get(prefix[i + 1] % modulo, 0) + 1

        return res  
        
        
# class Solution:
#     def minOperationsQueries(self, n: int, edges: List[List[int]], queries: List[List[int]]) -> List[int]:
#         def find_path(u, parent, tree, target):
#             if u == target:
#                 return [u]
#             for v, _ in tree[u]:
#                 if v != parent:
#                     path = find_path(v, u, tree, target)
#                     if path:
#                         return [u] + path
#             return []


#         tree = [[] for _ in range(n)]
#         for u, v, w in edges:
#             tree[u].append((v, w))
#             tree[v].append((u, w))

#         answer = []
#         for a, b in queries:
#             path = find_path(a, -1, tree, b)

#             if len(path) == 1:
#                 answer.append(0)
#                 continue

#             weights = {}
#             for i in range(len(path) - 1):
#                 for v, w in tree[path[i]]:
#                     if v == path[i + 1]:
#                         weights[w] = weights.get(w, 0) + 1
#                         break

#             max_count = max(weights.values())

#             answer.append(len(path) - 1 - max_count)

#         return answer


from collections import defaultdict
from typing import List

class Solution:
    def minOperationsQueries(self, n: int, edges: List[List[int]], queries: List[List[int]]) -> List[int]:
        # 使用 defaultdict 创建图结构
        graph = defaultdict(list)
        # 为图中每个节点添加邻接点和边的权重
        for u, v, w in edges:
            graph[u].append((v, w))
            graph[v].append((u, w))
        
        # 初始化父节点、节点深度和到父节点的边的权重的列表
        parent = [-1] * n
        depth = [-1] * n
        edge_weight_to_parent = [-1] * n

        # 深度优先搜索函数，用于填充 parent, depth 和 edge_weight_to_parent
        def dfs(node, par, d, w):
            parent[node] = par
            depth[node] = d
            edge_weight_to_parent[node] = w
            for nei, wt in graph[node]:
                if nei != par:
                    dfs(nei, node, d + 1, wt)

        # 从根节点 (0) 开始执行 DFS
        dfs(0, -1, 0, -1)

        # 该函数用于找到从 a 到 b 的路径，并返回这条路径上所有边的权重
        def find_path(a, b):
            path_a_b = []
            while depth[a] > depth[b]:
                path_a_b.append(edge_weight_to_parent[a])
                a = parent[a]
            while depth[b] > depth[a]:
                path_a_b.append(edge_weight_to_parent[b])
                b = parent[b]
            while a != b:
                path_a_b.append(edge_weight_to_parent[a])
                path_a_b.append(edge_weight_to_parent[b])
                a = parent[a]
                b = parent[b]
            return path_a_b

        # 对于每个查询，找到从 a 到 b 的路径，然后计算所需的最小操作次数
        result = []
        for a, b in queries:
            if a == b:
                result.append(0)
                continue
            path_weights = find_path(a, b)
            max_weight_count = max(path_weights.count(wt) for wt in set(path_weights))
            result.append(len(path_weights) - max_weight_count)
        
        # 返回结果列表
        return result

        
s = Solution()
ans = s.minOperationsQueries(n = 7, edges = [[0,1,1],[1,2,1],[2,3,1],[3,4,2],[4,5,2],[5,6,2]], queries = [[0,3],[3,6],[2,6],[0,6]])
print(ans) # [0,0,1,3]

s = Solution()
ans = s.minOperationsQueries(n = 8, edges = [[1,2,6],[1,3,4],[2,4,6],[2,5,3],[3,6,6],[3,0,8],[7,0,2]], queries = [[4,6],[0,4],[6,5],[7,4]])
print(ans) # [1,2,2,3]

s = Solution()
ans = s.minOperationsQueries(n = 1, edges = [], queries = [[0,0]])
print(ans) 
