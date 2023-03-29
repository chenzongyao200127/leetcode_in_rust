# 429_N-ary_Tree_Level_Order_Traversal
# https://leetcode.cn/problems/n-ary-tree-level-order-traversal/


# Definition for a Node.
class Node(object):
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children


class Solution(object):
    def levelOrder(self, root):
        """
        :type root: Node
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
                
                for i in range(len(node.children)):
                    queue.append(node.children[i])
                
            result.append(current_level)
            
        return result
    
    
    
"""
# Definition for a Node.
class Node(object):
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution(object):
    def levelOrder(self, root):
        """
        :type root: Node
        :rtype: List[List[int]]
        """
        results = []
        if not root:
            return results

        from collections import deque
        que = deque([root])

        while que:
            result = []
            for _ in range(len(que)):
                cur = que.popleft()
                result.append(cur.val)
                if cur.children:
                    que.extend(cur.children)
            results.append(result)

        return results
