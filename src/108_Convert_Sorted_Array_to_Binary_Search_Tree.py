# 108_Convert_Sorted_Array_to_Binary_Search_Tree
# https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]: