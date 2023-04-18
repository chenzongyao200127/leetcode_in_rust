# 1026_Maximum_Difference_Between_Node_and_Ancestor
# https://leetcode.cn/problems/maximum-difference-between-node-and-ancestor/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        

class Solution:
    def maxAncestorDiff(self, root: Optional[TreeNode]) -> int:
        res = []
        
        def dfs(node: TreeNode, val: int):
            if node is None:
                return
            
            nonlocal res
            res.append(abs(val - node.val))
            if node.left:
                dfs(node.left, val)
            if node.right:
                dfs(node.right, val)
                
        def preorder(root: TreeNode):
            if root is None:
                return
            
            dfs(root, root.val)
            
            preorder(root.left)
            preorder(root.right)
            
        
        preorder(root)
        return max(res)


# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxAncestorDiff(self, root: Optional[TreeNode]) -> int:

        maxDiff = 0
        def dfs(node):

            nonlocal maxDiff
            if not node:
                return 0, 0
        
            if not node.left and not node.right:
                return node.val, node.val

            retMin, retMax = node.val, node.val
            if node.left:
                minL, maxL  = dfs(node.left)
                maxDiff = max(maxDiff, abs(node.val-minL), abs(node.val- maxL))
                retMin = min(minL, retMin)
                retMax = max(maxL, retMax)
            
            if node.right:
                minR, maxR  = dfs(node.right)
                maxDiff = max(maxDiff, abs(node.val-minR), abs(node.val- maxR))
                retMin = min(minR, retMin)
                retMax = max(maxR, retMax)
            

            return retMin, retMax
        
        dfs(root)
        return maxDiff


