# 513_Find_Bottom_Left_Tree_Value
# https://leetcode.cn/problems/find-bottom-left-tree-value/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def findBottomLeftValue(self, root: Optional[TreeNode]) -> int:
        queue = deque([root])
        res = []
        
        while queue:
            level_length = len(queue)
            curr = []
            
            for _ in range(level_length):
                node = queue.popleft()
                
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
                    
                curr.append(node.val)
            
            res.append(curr)
        
        return res[-1][0]    

            
class Solution:
    def findBottomLeftValue(self, root: Optional[TreeNode]) -> int:
        curVal = curHeight = 0
        def dfs(node: Optional[TreeNode], height: int) -> None:
            if node is None:
                return
            height += 1
            dfs(node.left, height)
            dfs(node.right, height)
            nonlocal curVal, curHeight
            if height > curHeight:
                curHeight = height
                curVal = node.val
        dfs(root, 0)
        return curVal
            