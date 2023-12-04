# 1038. 从二叉搜索树到更大和树
# https://leetcode.cn/problems/binary-search-tree-to-greater-sum-tree/description/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def bstToGst(self, root: TreeNode) -> TreeNode:
        tmp = 0
        def dfs(node):
            if not node:
                return
            
            dfs(node.right)
            nonlocal tmp
            tmp += node.val
            node.val = tmp
            
            dfs(node.left)
        
        dfs(root)
        
        return root
    