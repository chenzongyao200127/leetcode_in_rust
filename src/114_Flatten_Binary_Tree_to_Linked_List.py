# 114_Flatten_Binary_Tree_to_Linked_List
# https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
# If you notice carefully in the flattened tree, each node's right child points to the next node of a pre-order traversal.
class Solution:
    def flatten(self, root: Optional[TreeNode]) -> None:
        """
        Do not return anything, modify root in-place instead.
        """

        
        