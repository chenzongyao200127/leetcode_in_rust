# 1782_Count_Pairs_Of_Nodes
# https://leetcode.cn/problems/count-pairs-of-nodes/description/


# Input: n = 4, edges = [[1,2],[2,4],[1,3],[2,3],[2,1]], queries = [2,3]
# Output: [6,5]

# Explanation: The calculations for incident(a, b) are shown in the table above.
# The answers for each of the queries are as follows:
# - answers[0] = 6. All the pairs have an incident(a, b) value greater than 2.
# - answers[1] = 5. All the pairs except (3, 4) have an incident(a, b) value greater than 3.

# Example 2:

# Input: n = 5, edges = [[1,5],[1,5],[3,4],[2,5],[1,3],[5,1],[2,3],[2,5]], queries = [1,2,3,4,5]
# Output: [10,10,9,8,6]
from collections import Counter
from typing import List
class Solution:
    def countPairs(self, n: int, edges: List[List[int]], queries: List[int]) -> List[int]:
        deg = [0] * (n + 1)
        for x, y in edges:
            deg[x] += 1
            deg[y] += 1
        
        cnt_e = Counter(tuple(sorted(e)) for e in edges)

        sorted_deg = sorted(deg)
        ans = [0] * len(queries)
        for i, q in enumerate(queries):
            l = 1
            r = n
            while l < r:
                if sorted_deg[l] + sorted_deg[r] <= q:
                    l += 1
                else:
                    ans[i] += r - l
                    r -= 1
        
            for (x, y), c in cnt_e.items():
                if q < deg[x] + deg[y] <= q + c:
                    ans[i] -= 1

        return ans
    

# GPT-4 可读性优化
# GPT-4 可读性优化
from collections import Counter
from typing import List

class Solution:
    def countPairs(self, n: int, edges: List[List[int]], queries: List[int]) -> List[int]:
        # 1. Calculate degree for each node
        degree = self.calculate_degree(n, edges)
        
        # 2. Count occurrences of each edge
        edge_counter = Counter(tuple(sorted(edge)) for edge in edges)
        
        # 3. Process each query
        return [self.process_query(n, query, degree, edge_counter) for query in queries]
    
    def calculate_degree(self, n: int, edges: List[List[int]]) -> List[int]:
        """Calculate degree for each node."""
        deg = [0] * (n + 1)
        for x, y in edges:
            deg[x] += 1
            deg[y] += 1
        return deg
    
    def process_query(self, n:int, query: int, degree: List[int], edge_counter: Counter) -> int:
        """Process each query to calculate pair counts."""
        sorted_degree = sorted(degree)
        count = 0
        l, r = 1, n
        
        while l < r:
            if sorted_degree[l] + sorted_degree[r] <= query:
                l += 1
            else:
                count += r - l
                r -= 1
        
        # Adjust count based on edge occurrences
        for (x, y), occurrences in edge_counter.items():
            if query < degree[x] + degree[y] <= query + occurrences:
                count -= 1

        return count

# 灵神的优化
class Solution:
    def countPairs(self, n: int, edges: List[List[int]], queries: List[int]) -> List[int]:
        deg = [0] * (n + 1)
        cnt_e = dict()  # 比 Counter 快一点
        for x, y in edges:
            if x > y: x, y = y, x
            deg[x] += 1
            deg[y] += 1
            cnt_e[(x, y)] = cnt_e.get((x, y), 0) + 1
        cnt_deg = Counter(deg[1:])

        # 2)
        cnts = [0] * (max(deg) * 2 + 2)
        for deg1, c1 in cnt_deg.items():
            for deg2, c2 in cnt_deg.items():
                if deg1 < deg2:
                    cnts[deg1 + deg2] += c1 * c2
                elif deg1 == deg2:
                    cnts[deg1 + deg2] += c1 * (c1 - 1) // 2

        # 3)
        for (x, y), c in cnt_e.items():
            s = deg[x] + deg[y]
            cnts[s] -= 1
            cnts[s - c] += 1

        # 4) 计算 cnts 的后缀和
        for i in range(len(cnts) - 1, 0, -1):
            cnts[i - 1] += cnts[i]

        for i, q in enumerate(queries):
            queries[i] = cnts[min(q + 1, len(cnts) - 1)]
            
        return queries

        


# `sorted` 和 `sort` 都是用于排序的，但是它们之间存在几个主要的区别：

# 1. **类型**：
#     - `sorted`：是 Python 的内置函数，可以对任何可迭代对象进行排序，例如列表、元组、字符串等，并返回一个新的排序列表。
#     - `sort`：是列表对象的一个方法。它只能被列表使用，而且是在原地排序，即它会修改原列表。

# 2. **返回值**：
#     - `sorted`：总是返回一个排序后的新列表，原始的可迭代对象不会被修改。
#     - `sort`：返回 `None`，并且原列表被修改。

# 3. **适用性**：
#     - `sorted`：可以对任何可迭代对象进行排序。
#     - `sort`：只能用于列表。

# 4. **用法示例**：

#     使用 `sorted`：
#     ```python
#     numbers = (5, 2, 9, 1)
#     sorted_numbers = sorted(numbers)
#     print(numbers)  # 输出：(5, 2, 9, 1)
#     print(sorted_numbers)  # 输出：[1, 2, 5, 9]
#     ```

#     使用 `sort`：
#     ```python
#     numbers = [5, 2, 9, 1]
#     numbers.sort()
#     print(numbers)  # 输出：[1, 2, 5, 9]
#     ```

# 这些区别在选择如何排序数据时很重要。如果你只需要对一个列表进行排序并且希望直接修改它，`sort` 是适当的方法。
# 但是，如果你要排序其他类型的可迭代对象或者想保留原始数据的顺序，你应该使用 `sorted`。