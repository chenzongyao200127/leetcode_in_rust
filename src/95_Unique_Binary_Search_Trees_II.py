# 95_Unique_Binary_Search_Trees_II
# https://leetcode.cn/problems/unique-binary-search-trees-ii/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def generateTrees(self, n: int) -> List[Optional[TreeNode]]:
        