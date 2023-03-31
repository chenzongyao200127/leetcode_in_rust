# 129_Sum_Root_to_Leaf_Numbers
# https://leetcode.cn/problems/sum-root-to-leaf-numbers/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def sumNumbers(self, root: Optional[TreeNode]) -> int:
        res = 0
        
        def dfs(root: TreeNode, path_sum: int):
            nonlocal res
            
            if not root:
                return
            
            path_sum = path_sum * 10 + root.val
            
            if not root.left and not root.right:
                res += path_sum
                
            dfs(root.left, path_sum)
            dfs(root.right, path_sum)
            
        dfs(root, 0)
        
        return res
         
  
        
# class Solution:
#     def sumNumbers(self, root: Optional[TreeNode]) -> int:
#         ans = []
#         path = []
        
#         def dfs(root: TreeNode):
#             if not root:
#                 return
            
#             path.append(root.val)
            
#             if not root.left and not root.right:
#                 ans.append(path[:])
                
#             dfs(root.left)
#             dfs(root.right)
            
#             path.pop()

#         dfs(root)
        
#         res = 0
#         for path in ans:
#             tmp = 0
#             for i in range(len(path)):
#                 tmp += path[i] * 10**(len(path) - i - 1)
#             res += tmp
        
#         return res
            
        
class Solution:
    def sumNumbers(self, root: Optional[TreeNode]) -> int:
        def helper(root, prefix, result):
            if not root:
                return
            
            prefix = prefix * 10 + root.val
            
            if not root.left and not root.right:
                result.append(prefix)
                return
            
            helper(root.left, prefix, result)
            helper(root.right, prefix, result)
            
        result = []
        helper(root, 0, result)
        return sum(result)
    

class Solution:
    def sumNumbers(self, root: TreeNode) -> int:
        if not root:
            return 0

        total = 0
        nodeQueue = collections.deque([root])
        numQueue = collections.deque([root.val])
        
        while nodeQueue:
            node = nodeQueue.popleft()
            num = numQueue.popleft()
            left, right = node.left, node.right
            if not left and not right:
                total += num
            else:
                if left:
                    nodeQueue.append(left)
                    numQueue.append(num * 10 + left.val)
                if right:
                    nodeQueue.append(right)
                    numQueue.append(num * 10 + right.val)

        return total
