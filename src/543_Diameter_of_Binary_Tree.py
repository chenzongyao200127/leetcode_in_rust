# 543_Diameter_of_Binary_Tree
# https://leetcode.cn/problems/diameter-of-binary-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:        
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        def depth(node: TreeNode) -> int:
            if not node:
                return 0 
            left_depth = depth(node.left)
            right_depth = depth(node.right)
            self.ans = max(self.ans, left_depth + right_depth)
            return max(left_depth, right_depth) + 1
        
        self.ans = 0
        depth(root)
        return self.ans
        