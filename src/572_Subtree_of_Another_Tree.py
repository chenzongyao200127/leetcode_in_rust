# 572_Subtree_of_Another_Tree
# https://leetcode.cn/problems/subtree-of-another-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        def isSame(t1: Optional[TreeNode], t2: Optional[TreeNode]) -> bool:
            if not t1 and not t2:
                return True
            if not t1 or not t2:
                return False
            if t1.val == t2.val:
                return isSame(t1.left, t2.left) and isSame(t1.right, t2.right)
            return False
        
        if not root:
            return False
        if isSame(root, subRoot):
            return True
        else:
            return self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)
