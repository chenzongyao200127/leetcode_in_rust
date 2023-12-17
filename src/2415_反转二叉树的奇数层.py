# 2415_反转二叉树的奇数层
# https://leetcode.cn/problems/reverse-odd-levels-of-binary-tree/description/



# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
from typing import Optional
        
class Solution:
    def reverseOddLevels(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root:
            return root

        def reverse(l, r, isNeedReverse):
            if not l or not r:
                return
            
            if isNeedReverse:
                l.val, r.val = r.val, l.val
            
            reverse(l.left, r.right, not isNeedReverse)
            reverse(l.right, r.left, not isNeedReverse)
        
        reverse(root.left, root.right, True)
        return root