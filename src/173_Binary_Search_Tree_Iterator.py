# 173_Binary_Search_Tree_Iterator
# https://leetcode.cn/problems/binary-search-tree-iterator/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class BSTIterator:

    def __init__(self, root: Optional[TreeNode]):
        self.order = [-1]
        
        def dfs(node: TreeNode):
            if node is None:
                return node
            
            dfs(node.left)
            
            self.order.append(node.val)
            
            dfs(node.right)
            
        dfs(root)
        self.length = len(self.order)
        self.idx = 0
        

    def next(self) -> int:
        if self.hasNext:
            self.idx += 1
            return self.order[self.idx]


    def hasNext(self) -> bool:
        return self.idx < self.length - 1



# Your BSTIterator object will be instantiated and called as such:
# obj = BSTIterator(root)
# param_1 = obj.next()
# param_2 = obj.hasNext()

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class BSTIterator:

    def __init__(self, root: TreeNode):
        self.stark = []
        while root:
            self.stark.append(root)
            root = root.left

    def next(self) -> int:
        node = self.stark.pop()
        if node.right:
            nnode = node.right
            while nnode:
                self.stark.append(nnode)
                nnode = nnode.left
        return node.val

    def hasNext(self) -> bool:
        return self.stark != []


# Your BSTIterator object will be instantiated and called as such:
# obj = BSTIterator(root)
# param_1 = obj.next()
# param_2 = obj.hasNext()