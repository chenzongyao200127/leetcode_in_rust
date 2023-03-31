# 112_Path_Sum
# https://leetcode.cn/problems/path-sum/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if root is None:
            return False
        self.flag = False
        
        def dfs(node: TreeNode, target: int):
            if node is None:
                return
            
            if node.right is None and node.left is None and node.val == target:
                self.flag = True

            # Use 'node' instead of 'root' for left and right children
            dfs(node.left, target - node.val)
            dfs(node.right, target - node.val)
            
        dfs(root, targetSum)
        return self.flag
    
            
            
# DFS
class Solution:
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if not root:
            return False
        
        que = []
        que.append((root,root.val))
        
        while que:
            new,path = que.pop()
            
            if not new.left and not new.right and path == targetSum:
                return True
            
            if new.left:
                que.append((new.left,path+new.left.val))
            if new.right:
                que.append((new.right,path+new.right.val))
                
        return False