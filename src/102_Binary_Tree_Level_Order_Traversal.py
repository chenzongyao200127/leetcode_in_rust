# 102_Binary_Tree_Level_Order_Traversal
# https://leetcode.cn/problems/binary-tree-level-order-traversal/

# Definition for a binary tree node.
class TreeNode(object):
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution(object):
    def levelOrder(self, root):
        """
        :type root: TreeNode
        :rtype: List[List[int]]
        """
        if root is None:
            return []
        
        queue = deque([root])
        result = []
        
        while queue:
            level = len(queue)
            current_level = []
            
            for _ in range(level):
                node = queue.popleft()
                current_level.append(node.val)
                
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
                
            result.append(current_level)
            
        return result
    
    
class Solution(object):
    def levelOrder(self, root):
        """
        :type root: TreeNode
        :rtype: List[List[int]]
        """
        if not root: return []

        res = []
        queue = []
        queue.append([root, 0])
        while queue:
            t, i = queue.pop()
            if i >= len(res): res.append([])
            res[i].append(t.val)
            if t.left:
                queue.insert(0, [t.left, i+1])
            if t.right:
                queue.insert(0, [t.right, i+1])
        
        return res