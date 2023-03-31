# 404_Sum_of_Left_Leaves
# https://leetcode.cn/problems/sum-of-left-leaves/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def sumOfLeftLeaves(self, root: Optional[TreeNode]) -> int:
        queue = deque([root])
        res = 0
        
        while queue:
            level_length = len(queue)
            
            for _ in range(level_length):
                node = queue.popleft()
                if node.left and node.left.left is None and node.left.right is None:
                    res += node.left.val
                
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
        
        return res    


# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def sumOfLeftLeaves(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        
        if root.left is None and root.right is None:
            return 0
        
        l = self.sumOfLeftLeaves(root.left)
        if root.left and root.left.left is None and root.left.right is None:
            l = root.left.val
            
        r = self.sumOfLeftLeaves(root.right)
        return l + r
