# 144_Binary_Tree_Preorder_Traversal
# https://leetcode.cn/problems/binary-tree-preorder-traversal/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def preorderTraversal(self, root: TreeNode) -> List[int]:
        def preorder(root: TreeNode):
            if not root:
                return
            
            res.append(root.val)
            
            preorder(root.left)
            preorder(root.right)
        
        res = list()
        preorder(root)
        return res
    
# 迭代写法    
class Solution:
    def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        if not root:
            return []
        
        res = []
        stack = [root]
        
        while stack:
            node = stack.pop()
            res.append(node.val)
            
            if node.right:
                stack.append(node.right)
            if node.left:
                stack.append(node.left)
                
        return res