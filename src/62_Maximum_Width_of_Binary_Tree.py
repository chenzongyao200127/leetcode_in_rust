# // 62_Maximum_Width_of_Binary_Tree
# // https://leetcode.cn/problems/maximum-width-of-binary-tree/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
            
class Solution:
    def widthOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        
        ans = 0
        queue = deque([(root, 0)])
        
        while queue:
            level_length = len(queue)
            level_left_pos = queue[0][1]
            
            for i in range(level_length):
                node, pos = queue.popleft()
                
                if node.left:
                    queue.append((node.left, 2 * pos))
                if node.right:
                    queue.append((node.right, 2 * pos + 1))
            
            ans = max(ans, pos - level_left_pos + 1)
            
        return ans
    
    
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def max_width_of_binary_tree(root: TreeNode) -> int:
    if not root:
        return 0
    
    max_width = 0
    queue = deque([(root, 0)])  # 创建一个队列，存储节点及其位置值
    
    while queue:
        level_length = len(queue)
        level_left_pos = queue[0][1]  # 当前层的最左节点的位置值
        
        for i in range(level_length):
            node, pos = queue.popleft()
            
            if node.left:
                queue.append((node.left, 2 * pos))
            if node.right:
                queue.append((node.right, 2 * pos + 1))
                
        max_width = max(max_width, pos - level_left_pos + 1)  # 更新最大宽度

    return max_width