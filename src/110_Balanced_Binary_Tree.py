# 110_Balanced_Binary_Tree
# https://leetcode.cn/problems/balanced-binary-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        
        def treeHight(node: TreeNode) -> int:
            if node is None: 
                return 0
            
            return max(treeHight(node.left) + 1, treeHight(node.right) + 1)
        
        falg = True
        
        def dfs(node: TreeNode):
            if node is None:
                return
            
            if abs(treeHight(node.left) - treeHight(node.right)) <= 1:
                dfs(node.left)
                dfs(node.right)
            else:
                nonlocal falg
                falg = False
                return
            
        dfs(root)
        
        return falg
    
    
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:

        def judge(node):
            if not node:
                return 0, True
            
            deep_left, flag = judge(node.left)
            
            if not flag:
                return 0, False
            
            deep_right, flag = judge(node.right)
            
            if not flag:
                return 0, False
            
            return 1 + max(deep_left, deep_right), abs(deep_left - deep_right) < 2

        _, res = judge(root)

        return res
    
    
class Solution:
    def isBalanced(self, root: TreeNode) -> bool:
        def height(root: TreeNode) -> int:
            if not root:
                return 0
            
            leftHeight = height(root.left)
            rightHeight = height(root.right)
            
            if leftHeight == -1 or rightHeight == -1 or abs(leftHeight - rightHeight) > 1:
                return -1
            else:
                return max(leftHeight, rightHeight) + 1

        return height(root) >= 0