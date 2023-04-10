# 235_Lowest_Common_Ancestor_of_a_Binary_Search_Tree
# https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        res = root
        
        def findAncestor(node: TreeNode, p: TreeNode, q: TreeNode) -> 'TreeNode':
            if p.val == node.val or q.val == node.val or (p.val - node.val) * (q.val - node.val) < 0 :
                nonlocal res
                res = node
            
            if p.val < node.val and q.val < node.val:
                findAncestor(node.left, p, q)
            
            if p.val > node.val and q.val > node.val:
                findAncestor(node.right, p, q)
            
        findAncestor(root, p, q)
        
        return res
    
    
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        # iterative way using BST attribute 
        if not root: return 
        while root:
            if root.val > q.val and root.val > p.val:
                root = root.left
            elif root.val < q.val and root.val < p.val:
                root = root.right 
            else:
                return root



class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        # normal way:
        if not root or root == p or root == q: 
            return root
        
        left = self.lowestCommonAncestor(root.left, p, q)
        right = self.lowestCommonAncestor(root.right, p, q)
        
        if left and right: return root
        if left and not right: return left
        if not left and right: return right