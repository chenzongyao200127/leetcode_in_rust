# 558_Logical_OR_of_Two_Binary_Grids_Represented_as_Quad-Trees
# https://leetcode.cn/problems/logical-or-of-two-binary-grids-represented-as-quad-trees/



# Definition for a QuadTree node.
class Node:
    def __init__(self, val, isLeaf, topLeft, topRight, bottomLeft, bottomRight):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight


class Solution:
    def intersect(self, quadTree1: 'Node', quadTree2: 'Node') -> 'Node':
        if quadTree1.isLeaf:
            return Node(True, True) if quadTree1.val else quadTree2
        
        if quadTree2.isLeaf:
            return self.intersect(quadTree2, quadTree1)
        
        o1 = self.intersect(quadTree1.topLeft, quadTree2.topLeft)
        o2 = self.intersect(quadTree1.topRight, quadTree2.topRight)
        o3 = self.intersect(quadTree1.bottomLeft, quadTree2.bottomLeft)
        o4 = self.intersect(quadTree1.bottomRight, quadTree2.bottomRight)
        
        if o1.isLeaf and o2.isLeaf and o3.isLeaf and o4.isLeaf and o1.val == o2.val == o3.val == o4.val:
            return Node(o1.val, True)
        
        return Node(False, False, o1, o2, o3, o4)


"""
# Definition for a QuadTree node.
class Node:
    def __init__(self, val, isLeaf, topLeft, topRight, bottomLeft, bottomRight):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight
"""

class Solution:
    def intersect(self, quadTree1: 'Node', quadTree2: 'Node') -> 'Node':
        # if quadTree1.isLeaf:
        #     return Node(True, True) if quadTree1.val else quadTree2
        # if quadTree2.isLeaf:
        #     return self.intersect(quadTree2, quadTree1)
        # o1 = self.intersect(quadTree1.topLeft, quadTree2.topLeft)
        # o2 = self.intersect(quadTree1.topRight, quadTree2.topRight)
        # o3 = self.intersect(quadTree1.bottomLeft, quadTree2.bottomLeft)
        # o4 = self.intersect(quadTree1.bottomRight, quadTree2.bottomRight)
        # if o1.isLeaf and o2.isLeaf and o3.isLeaf and o4.isLeaf and o1.val == o2.val == o3.val == o4.val:
        #     return Node(o1.val, True)
        # return Node(False, False, o1, o2, o3, o4)


        if (quadTree1.isLeaf and quadTree1.val) or (quadTree2.isLeaf and not quadTree2.val):
            return quadTree1
        
        elif (quadTree2.isLeaf and quadTree2.val) or (quadTree1.isLeaf and not quadTree1.val):
            return quadTree2
        
        else:
            o1 = self.intersect(quadTree1.topLeft, quadTree2.topLeft)
            o2 = self.intersect(quadTree1.topRight, quadTree2.topRight)
            o3 = self.intersect(quadTree1.bottomLeft, quadTree2.bottomLeft)
            o4 = self.intersect(quadTree1.bottomRight, quadTree2.bottomRight)
            
            if o1.isLeaf and o2.isLeaf and o3.isLeaf and o4.isLeaf and o1.val and o2.val and o3.val and o4.val:
                return o1
            else:
                return Node(None,False,o1,o2,o3,o4)