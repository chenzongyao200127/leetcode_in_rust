# 538_Convert_BST_to_Greater_Tree
# https://leetcode.cn/problems/convert-bst-to-greater-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def convertBST(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        tmp = 0
        
        def dfs(node: TreeNode):
            if node is None:
                return
            
            dfs(node.right)
            
            nonlocal tmp
            tmp += node.val
            node.val = tmp
            
            dfs(node.left)
            
        dfs(root)
        
        return root