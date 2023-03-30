# 515_Find_Largest_Value_in_Each_Tree_Row
# https://leetcode.cn/problems/find-largest-value-in-each-tree-row/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def largestValues(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []
        
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
        
        ans = []
        
        for level in res:
            level.sort()
            ans.append(level[-1])
        
        return ans
            
        
class Solution:
    def largestValues(self, root: Optional[TreeNode]) -> List[int]:
        ans = []
        if not root:
            return ans
        que = deque([root])
        
        while que:
            size = len(que)
            temp = []
            for _ in range(size):
                node = que.popleft()
                temp.append(node.val)
                if node.left:
                    que.append(node.left)
                if node.right:
                    que.append(node.right)
            ans.append(temp)
            
        return [max(i) for i in ans]