# 687_Longest_Univalue_Path
# https://leetcode.cn/problems/longest-univalue-path/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def longestUnivaluePath(self, root: Optional[TreeNode]) -> int:
        self.ans = 0
        def arrow_length(node: TreeNode):
            # TODO
        arrow_length(root)
        return self.ans

            
