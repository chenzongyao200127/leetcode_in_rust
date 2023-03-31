# 111_Minimum_Depth_of_Binary_Tree
# https://leetcode.cn/problems/minimum-depth-of-binary-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def minDepth(self, root: Optional[TreeNode]) -> int:
        self.ans = 2000
        if root is None:
            return 0
        
        def dfs(root: TreeNode, level: int):
            if root.left is None and root.right is None:
                self.ans = min(level, self.ans)
            
            if root.left:
                dfs(root.left, level+1)
            if root.right:
                dfs(root.right, level+1)

        # Call the dfs function with the root node and starting level 1
        dfs(root, 1)
        
        return self.ans
    
    
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def minDepth(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0
        
        queue = deque()
        queue.append((root, 1))
        
        while queue:
            item = queue.popleft()
            node = item[0]
            depth = item[1]
            
            if not node.left and not node.right:
                return depth
            
            if node.left:
                queue.append((node.left, depth + 1))
            if node.right:
                queue.append((node.right, depth + 1))
