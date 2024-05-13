# 968_监控二叉树
# https://leetcode.cn/problems/binary-tree-cameras/description/


# 给定一个二叉树，我们在树的节点上安装摄像头。
# 节点上的每个摄影头都可以监视其父对象、自身及其直接子对象。
# 计算监控树的所有节点所需的最小摄像头数量。

from typing import inf
from typing import Optional

# Definition for a binary tree node.


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

# add comments


# Definition for a binary tree node.

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def minCameraCover(self, root: Optional[TreeNode]) -> int:
        """
        Calculate the minimum number of cameras needed to monitor all nodes in a binary tree.

        Args:
        root (Optional[TreeNode]): The root of the binary tree.

        Returns:
        int: The minimum number of cameras required.
        """
        def dfs(node):
            # Base case: If the node is None, return [infinity, 0, 0]
            # which corresponds to the costs of [placing a camera here, one level up (parent), two levels up (grandparent)].
            if not node:
                return [float('inf'), 0, 0]

            # Recursively solve for the left and right subtrees
            la, lb, lc = dfs(node.left)
            ra, rb, rc = dfs(node.right)

            # a: Cost when a camera is placed at the current node
            a = lc + rc + 1
            # b: Minimum cost when the camera is not on the current node but its parent is guaranteed to have a camera
            # either by placing it at the parent or at one of the children.
            b = min(a, la + rb, ra + lb)
            # c: Minimum cost when the camera is neither on the current node nor its parent but guaranteed at the grandparent
            c = min(a, lb + rb)

            # Return the costs for the current node
            return [a, b, c]

        # Start the DFS from the root
        a, b, c = dfs(root)
        # The second value (b) gives the minimum cameras needed with the assumption that root is covered
        return b
