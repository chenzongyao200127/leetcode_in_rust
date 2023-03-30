# 637_Average_of_Levels_in_Binary_Tree
# https://leetcode.cn/problems/average-of-levels-in-binary-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def averageOfLevels(self, root: Optional[TreeNode]) -> List[float]:
        queue = deque([root])
        res = []
        
        while queue:
            level_length = len(queue)
            total = 0
            
            for _ in range(level_length):
                node = queue.popleft()
                
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
                
                total += node.val
            
            res.append(total/level_length)
            
        return res
    
    
# Example
class Solution:
    def averageOfLevels(self, root: Optional[TreeNode]) -> List[float]:
        # M2: DFS
        def dfs(root, level=0):
            if not root:
                return
            if len(ans) == level:
                ans.append([])
            ans[level].append(root.val)
            dfs(root.left, level+1)
            dfs(root.right, level+1)
        ans = []
        dfs(root)
        return [sum(a)/len(a) for a in ans]
            