# 107_Binary_Tree_Level_Order_Traversal_II
# https://leetcode.cn/problems/binary-tree-level-order-traversal-ii/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def levelOrderBottom(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []
        
        queue = deque([root])
        res = []
        
        while queue:
            level_length = len(queue)
            tmp = []
            
            for _ in range(level_length):
                node = queue.popleft()
                tmp.append(node.val)
                
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
            
            res.append(tmp)
            
        res = res[::-1]
        
        return res
            