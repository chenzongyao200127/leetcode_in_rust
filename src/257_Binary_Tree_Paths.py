# 257_Binary_Tree_Paths
# https://leetcode.cn/problems/binary-tree-paths/

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def binaryTreePaths(self, root: Optional[TreeNode]) -> List[str]:
        res = []
        
        def dfs(node: TreeNode, road:str):
            if node.left is None and node.right is None:
                res.append(road)
                return
            
            if node.left:
                road1 = road + "->" + str(node.left.val)
                dfs(node.left, road1)
            if node.right:
                road2 = road + "->" + str(node.right.val)
                dfs(node.right, road2)
            
        dfs(root, str(root.val))
        return res
    
    
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
# 我们也可以用广度优先搜索来实现。我们维护一个队列，存储节点以及根到该节点的路径。
# 一开始这个队列里只有根节点。在每一步迭代中，我们取出队列中的首节点，如果它是叶子节点，则将它对应的路径加入到答案中。
# 如果它不是叶子节点，则将它的所有孩子节点加入到队列的末尾。
# 当队列为空时广度优先搜索结束，我们即能得到答案。

class Solution:
    def binaryTreePaths(self, root: Optional[TreeNode]) -> List[str]:
        p = list()
        if not root:
            return p
 
        node_queue = collections.deque([root])
        path_queue = collections.deque([str(root.val)])
 
        while node_queue:
            node = node_queue.popleft()
            path = path_queue.popleft()
 
            if not node.left and not node.right:
                p.append(path)
            else:
                if node.left:
                    node_queue.append(node.left)
                    path_queue.append(path + '->' + str(node.left.val))
                
                if node.right:
                    node_queue.append(node.right)
                    path_queue.append(path + '->' + str(node.right.val))
        return p


class Solution:
    def binaryTreePaths(self, root):
        """
        :type root: TreeNode
        :rtype: List[str]
        """
        def construct_paths(root, path):
            if root:
                path += str(root.val)
                if not root.left and not root.right:  # 当前节点是叶子节点
                    paths.append(path)  # 把路径加入到答案中
                else:
                    path += '->'  # 当前节点不是叶子节点，继续递归遍历
                    construct_paths(root.left, path)
                    construct_paths(root.right, path)

        paths = []
        construct_paths(root, '')
        return paths
# 作者：LeetCode-Solution
# 链接：https://leetcode.cn/problems/binary-tree-paths/solution/er-cha-shu-de-suo-you-lu-jing-by-leetcode-solution/
# 来源：力扣（LeetCode）
# 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。