# 623_Add_One_Row_to_Tree
# https://leetcode.cn/problems/add-one-row-to-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def addOneRow(self, root: Optional[TreeNode], val: int, depth: int) -> Optional[TreeNode]:
        if depth == 1:
            return TreeNode(val, root)     
        
        queue = deque([root])
        curr_depth = 1
        
        while queue:
            level_length = len(queue)
            pre = []
            
            for _ in range(level_length):
                node = queue.popleft()
                pre.append(node)
                
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
                    
            if curr_depth + 1 == depth:  # Check for the depth - 1 instead of depth
                for node in pre:
                    # tmp_left = node.left
                    # tmp_right = node.right
                    # node.left = TreeNode(val)  # Use val instead of int
                    # node.left.left = tmp_left
                    # node.right = TreeNode(val)  # Use val instead of int
                    # node.right.right = tmp_right
                    node.left = TreeNode(val, node.left)
                    node.right = TreeNode(val,right= node.right)
            
            curr_depth += 1
            
        return root



# Example:
class Solution:
    def addOneRow(self, root: Optional[TreeNode], val: int, depth: int) -> Optional[TreeNode]:
        if depth == 1:
            return TreeNode(val, root)
            
        depth -= 1
        q = [root]
        while depth > 0:
            tmp = []
            for cur in q:
                if depth == 1:
                    cur.left = TreeNode(val, cur.left)
                    cur.right = TreeNode(val,right= cur.right)
                    
                else:
                    if cur.left:
                        tmp.append(cur.left)
                    if cur.right:
                        tmp.append(cur.right)
            q = tmp
            depth -= 1
        return root