# 427_Construct_Quad_Tree
# https://leetcode.cn/problems/construct-quad-tree/

# Definition for a QuadTree node.
class Node:
    def __init__(self, val, isLeaf, topLeft, topRight, bottomLeft, bottomRight):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight

class Solution:
    def construct(self, grid: List[List[int]]) -> 'Node':
        n = len(grid)
        
        pre = [[0] * (n + 1) for _ in range(n + 1)]
        for i in range(1, n + 1):
            for j in range(1, n + 1):
                pre[i][j] = pre[i - 1][j] + pre[i][j - 1] - pre[i - 1][j - 1] + grid[i - 1][j - 1]

        def getSum(r0: int, c0: int, r1: int, c1: int) -> int:
            return pre[r1][c1] - pre[r1][c0] - pre[r0][c1] + pre[r0][c0]

        def dfs(r0: int, c0: int, r1: int, c1: int) -> 'Node':
            total = getSum(r0, c0, r1, c1)
            
            if total == 0:
                return Node(False, True)
            
            if total == (r1 - r0) * (c1 - c0):
                return Node(True, True)
            
            return Node(
                True,
                False,
                dfs(r0, c0, (r0 + r1) // 2, (c0 + c1) // 2),
                dfs(r0, (c0 + c1) // 2, (r0 + r1) // 2, c1),
                dfs((r0 + r1) // 2, c0, r1, (c0 + c1) // 2),
                dfs((r0 + r1) // 2, (c0 + c1) // 2, r1, c1),
            )

        return dfs(0, 0, n, n)

class Solution:
    def construct(self, grid: List[List[int]]) -> 'Node':
        def dfs(r0: int, c0: int, r1: int, c1: int) -> 'Node':
            if all(grid[i][j] == grid[r0][c0] for i in range(r0, r1) for j in range(c0, c1)):
                return Node(grid[r0][c0] == 1, True)
            
            return Node(
                True,
                False,
                dfs(r0, c0, (r0 + r1) // 2, (c0 + c1) // 2),
                dfs(r0, (c0 + c1) // 2, (r0 + r1) // 2, c1),
                dfs((r0 + r1) // 2, c0, r1, (c0 + c1) // 2),
                dfs((r0 + r1) // 2, (c0 + c1) // 2, r1, c1),
            )
            
        return dfs(0, 0, len(grid), len(grid))



# class Solution:
#     def construct(self, grid: List[List[int]]) -> 'Node':
#         root = Node(False, False)
        
#         length = len(grid[0])
        
#         if length == 4:
#             return
        
#         all_and = 1
#         all_or = 0
        
#         for i in range(length/2):
#             for j in range(length/2):
#                 all_and = all_and and grid[i][j]
#                 all_or = all_or or grid[i][j]
                
#         if all_and == 1:
#             Node.topLeft = Node(True, True)
#         elif all_or == 0:
#             Node.topLeft = Node(False, True)
#         else:
#             None.topLeft = self.construct(grid[:length/2][:length/2])
    
            
#         all_and = 1
#         all_or = 0
        
#         for i in range(length/2):
#             for j in range(length/2, length):
#                 all_and = all_and and grid[i][j]
#                 all_or = all_or or grid[i][j]
                
#         if all_and == 1:
#             Node.topRight = Node(True, True)
#         elif all_or == 0:
#             Node.topRight = Node(False, True)
#         else:
#             None.topRight = self.construct(grid[length/2][length/2:])
        
        
#         all_and = 1
#         all_or = 0
        
#         for i in range(length/2, length):
#             for j in range(length/2):
#                 all_and = all_and and grid[i][j]
#                 all_or = all_or or grid[i][j]
                
#         if all_and == 1:
#             Node.bottomLeft = Node(True, True)
#         elif all_or == 0:
#             Node.bottomLeft = Node(False, True)
#         else:
#             None.bottomLeft = self.construct(grid[length/2:][:length/2])
        
        
#         all_and = 1
#         all_or = 0
        
#         for i in range(length/2, length):
#             for j in range(length/2, length):
#                 all_and = all_and and grid[i][j]
#                 all_or = all_or or grid[i][j]
                
#         if all_and == 1:
#             Node.bottomLeft = Node(True, True)
#         elif all_or == 0:
#             Node.bottomLeft = Node(False, True)
#         else:
#             None.bottomLeft = self.construct(grid[length/2:][length/2:])
        
        
#         return root
        