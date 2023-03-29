# 559_Maximum_Depth_of_N-ary_Tree
# https://leetcode.cn/problems/maximum-depth-of-n-ary-tree/

"""
# Definition for a Node.
class Node(object):
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution(object):
    def maxDepth(self, root):
        """
        :type root: Node
        :rtype: int
        """
        if root is None:
            return 0
        
        queue = deque([root])
        ans = 0
        
        while queue:
            level = len(queue)
            current_level = []
            
            for _ in range(level):
                node = queue.popleft()
                current_level.append(node.val)
                
                for i in range(len(node.children)):
                    queue.append(node.children[i])
                
            ans += 1
            
        return ans
        
        
        
        
# 示例代码        
"""
# Definition for a Node.
class Node(object):
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution(object):
    res = 0
    def maxDepth(self, root):
        """
        :type root: Node
        :rtype: int
        """

        if not root:
            return 0
        self.getDepth(root, 1)
        return self.res

    # Method 2 前序遍历最大深度
    def getDepth(self, node, depth):
        if self.res < depth:
            self.res = depth
        if not node.children:
            return
        for child in node.children:
            depth += 1
            self.getDepth(child, depth)
            depth -= 1
        return
    
    
    
class Solution(object):
    res = 0
    def maxDepth(self, root):
        """
        :type root: Node
        :rtype: int
        """
        if not root:
            return 0
        return self.getDepth(root)

    # Method 1 后序遍历求最大高度=最大深度
    def getDepth(self, node):
        if not node.children:
            return 1
        return 1 + max(self.getDepth(child) for child in node.children)