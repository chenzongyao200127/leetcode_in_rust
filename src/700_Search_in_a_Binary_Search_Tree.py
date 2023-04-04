# 700_Search_in_a_Binary_Search_Tree
# https://leetcode.cn/problems/search-in-a-binary-search-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def searchBST(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
        if root is None:
            return []
        
        res = None
        
        def dfs(node: TreeNode, val: int):
            if node is None:
                return
            
            if node.val == val:
                nonlocal res
                res = node
            
            dfs(node.left, val)
            dfs(node.right, val)
            
        dfs(root, val)
        
        return res
    

class Solution:
    def searchBST(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
        while root:
            if root.val < val:
                root = root.right
            elif root.val > val:
                root = root.left
            else:
                return root
            
        return None        
