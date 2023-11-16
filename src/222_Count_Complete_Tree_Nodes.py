# 222_Count_Complete_Tree_Nodes
# https://leetcode.cn/problems/count-complete-tree-nodes/description/

# Definition for a binary tree node.
from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def countNodes(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0
        
        # Function to calculate the height of the leftmost path of a subtree
        def height(node: TreeNode) -> int:
            if not node:
                return 0
            return 1 + height(node.left)
        
        left_h = height(root.left)
        right_h = height(root.right)
        
        if left_h == right_h:
            # If the heights are equal, the left subtree is a perfect binary tree
            return (2**left_h) + self.countNodes(root.right)
        else:
            # If the heights are different, the right subtree is a perfect binary tree
            return self.countNodes(root.left) + (2**right_h)

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def countNodes(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0
        
        left = root.left
        right = root.right

        left_dep = 0
        right_dep = 0

        while left:
            left_dep += 1
            left = left.left
            
        while right:
            right_dep += 1
            right =right.right
        
        if left_dep == right_dep:
            return (2<<left_dep)-1
        
        return self.countNodes(root.left) + self.countNodes(root.right) + 1