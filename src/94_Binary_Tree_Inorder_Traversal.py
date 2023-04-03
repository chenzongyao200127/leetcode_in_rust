# 94_Binary_Tree_Inorder_Traversal
# https://leetcode.cn/problems/binary-tree-inorder-traversal/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        
        def dfs(node: TreeNode):
            if node is None:
                return
            
            dfs(node.left)
            res.append(node.val)
            dfs(node.right)
            
        dfs(root)
        
        return res