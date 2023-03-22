# 563_Binary_Tree_Tilt
# https://leetcode.cn/problems/binary-tree-tilt/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def findTilt(self, root: Optional[TreeNode]) -> int:
        def childSum(root) -> int:
            if root == None:
                return 0
            
            ret = 0
            if root.left == None and root.right == None:
                return root.val
            ret += childSum(root.left)
            ret += childSum(root.right)
            
            return ret + root.val
        
        ans = 0
        if root == None:
            return 0
        ans += abs(childSum(root.left) - childSum(root.right))
        ans += self.findTilt(root.left)
        ans += self.findTilt(root.right)
        
        return ans
        
# 事实上，我们可以在计算子树权值和的时候将坡度进行累加，从而将复杂度降为O(n)
class Solution:
    def __init__(self):
        self.ans = 0

    def findTilt(self, root: TreeNode) -> int:
        self.dfs(root)
        return self.ans

    def dfs(self, node):
        if not node:
            return 0
        sum_left = self.dfs(node.left)
        sum_right = self.dfs(node.right)
        self.ans += abs(sum_left - sum_right) #全局变量
        return sum_left + sum_right + node.val
