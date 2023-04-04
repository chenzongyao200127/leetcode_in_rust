# 98_Validate_Binary_Search_Tree
# https://leetcode.cn/problems/validate-binary-search-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        order = []
        
        def dfs(node: TreeNode):
            if node is None:
                return node
            
            dfs(node.left)
            
            nonlocal order
            order.append(node.val)
            
            dfs(node.right)
            
        dfs(root)
        order_vec = sorted(order)
        
        return order_vec == order and len(set(order)) == len(order)
    
    
class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        def is_valid_bst_helper(node: TreeNode, min_val: float, max_val: float) -> bool:
            if node is None:
                return True
            
            if not min_val < node.val < max_val:
                return False
            
            return (is_valid_bst_helper(node.left, min_val, node.val) and
                    is_valid_bst_helper(node.right, node.val, max_val))
        
        return is_valid_bst_helper(root, float('-inf'), float('inf'))
    

class Solution:
    def isValidBST(self, root: Optional[TreeNode], left=-inf, right=inf) -> bool:
        if not root:
            return True
        
        x = root.val
        
        return left < x < right and self.isValidBST(root.left, left, x) and self.isValidBST(root.right, x, right)