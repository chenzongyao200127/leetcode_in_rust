# 450_Delete_Node_in_a_BST
# https://leetcode.cn/problems/delete-node-in-a-bst/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def deleteNode(self, root: Optional[TreeNode], key: int) -> Optional[TreeNode]:
        if root is None:
            return root
        
        if root.val == key:
            root = root.right
        
        
        def dfs(node: TreeNode, key: int):
            if node is None:
                return
            
            if node.left and node.left.val == key:
                node.left,  = node.left.right